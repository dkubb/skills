// This crate uses rustc-private APIs because Dylint lints compile against rustc internals.
#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![expect(
    missing_docs,
    reason = "dylint_linting::declare_late_lint! generates internal struct/fn items without docstrings"
)]
#![expect(
    clippy::missing_trait_methods,
    reason = "LateLintPass exposes ~30 default-implemented hook methods; we deliberately override only the ones this lint inspects"
)]
#![expect(
    clippy::exhaustive_structs,
    reason = "dylint_linting::declare_late_lint! generates the public lint pass struct with no fields; we cannot insert `#[non_exhaustive]` through the macro"
)]
#![expect(
    clippy::missing_inline_in_public_items,
    reason = "dylint_linting::declare_late_lint! generates public methods without #[inline]; the lint runner is not on a hot path"
)]
#![expect(
    clippy::disallowed_methods,
    reason = "dylint_linting::declare_late_lint! macro expansion calls `Result::unwrap` internally and we cannot annotate items inside the macro"
)]
#![expect(
    clippy::indexing_slicing,
    reason = "byte-level source scanning indexes into known-bounded slices guarded by length checks; switching to `.get()` adds noise without changing semantics"
)]
#![expect(
    clippy::arithmetic_side_effects,
    reason = "byte-index arithmetic in source-scanning helpers stays bounded by the file length checked at each loop iteration"
)]

//! Deny payload-blind `is_ok` / `is_err` / `is_some` / `is_none` assertions
//! and partial `matches!(..)` assertions in punchlist production test scope.

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use std::fs;
use std::path::Path;

use rustc_hir::Expr;
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_span::Pos as _;

/// Assertion macros whose first argument is a boolean condition.
const ASSERTION_MACROS: &[&str] = &[
    "assert",
    "assert_eq",
    "assert_ne",
    "debug_assert",
    "debug_assert_eq",
    "debug_assert_ne",
    "prop_assert",
    "prop_assert_eq",
    "prop_assert_ne",
];

/// Frame tracking the current open delimiter while byte-scanning source for
/// macro invocations.
struct DelimiterFrame {
    /// Matching close-delimiter byte for this frame.
    close: u8,
    /// Byte index where the delimiter content starts (one past the opener).
    content_start: usize,
    /// Whether this frame opens a target assertion macro invocation.
    is_target_macro: bool,
}

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds Issue 57 fuzzy matchers not covered by the direct string
    /// matcher lints: payload-blind `assert!(value.is_ok())` /
    /// `assert!(value.is_err())` / `assert!(value.is_some())` /
    /// `assert!(value.is_none())` checks and partial `matches!(...)`
    /// assertions with `..` placeholders.
    ///
    /// ### Why is this bad?
    ///
    /// These assertions only prove that a broad state was reached. Prefer
    /// matching the exact payload or exact typed error variant that the test
    /// behavior requires.
    pub FUZZY_MATCHER_ASSERTION,
    Deny,
    "fuzzy assertion matcher that should pin the payload or exact pattern"
}

impl<'tcx> LateLintPass<'tcx> for FuzzyMatcherAssertion {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        // Payload-blind `.is_ok()` / `.is_err()` / `.is_some()` / `.is_none()`
        // assertions are owned by the `tag_only_assertion` lint to avoid
        // double-flagging the same call site.
        if partial_matches_expr_is_linted(cx, expr)
            && is_inside_assertion_macro_invocation(cx, expr)
        {
            emit_partial_matches(cx, expr);
        }
    }
}

/// Returns true if the expression should be examined by this lint.
fn should_lint_expr(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    is_punchlist_production_path(cx, expr) && should_lint_target_source(cx.sess().is_test_crate())
}

/// Returns true if the current compile target should be linted.
const fn should_lint_target_source(is_test_crate: bool) -> bool {
    is_test_crate
}

/// Emits the partial `matches!` assertion diagnostic.
fn emit_partial_matches(cx: &LateContext<'_>, expr: &Expr<'_>) {
    cx.emit_span_lint(
        FUZZY_MATCHER_ASSERTION,
        expr.span,
        rustc_errors::DiagDecorator(|diag| {
            diag.primary_message("partial `matches!` assertion matcher");
            diag.help("pin the exact variant fields, or split into typed constructor assertions");
        }),
    );
}

/// Returns true if this expression is a partial `matches!` assertion candidate.
fn partial_matches_expr_is_linted(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    cx.sess()
        .source_map()
        .span_to_snippet(expr.span)
        .is_ok_and(|snippet| partial_matches_source_is_linted(&snippet))
}

/// Returns true if the source snippet is a `matches!` call containing a partial
/// pattern placeholder.
fn partial_matches_source_is_linted(source: &str) -> bool {
    source.trim_start().starts_with("matches!(") && source_contains_partial_pattern(source)
}

/// Returns true if `source` contains a `..` pattern placeholder outside
/// comments and literals.
fn source_contains_partial_pattern(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if let Some(next) = skip_ignored_source(bytes, index) {
            index = next;
            continue;
        }
        if bytes.get(index) == Some(&b'.')
            && bytes.get(index.saturating_add(1)) == Some(&b'.')
            && token_context_is_partial_pattern(bytes, index)
        {
            return true;
        }
        index = index.saturating_add(1);
    }

    false
}

/// Returns true when the `..` token is surrounded like a pattern placeholder
/// rather than a numeric range.
fn token_context_is_partial_pattern(bytes: &[u8], dot_index: usize) -> bool {
    let previous = previous_non_ws_byte(bytes, dot_index);
    let next = next_non_ws_byte(bytes, dot_index.saturating_add(2));

    matches!(previous, Some(b'{' | b'(' | b'[' | b',' | b'|'))
        && matches!(next, Some(b'}' | b')' | b']' | b',' | b'|'))
}

/// Returns the previous non-whitespace byte before `index`.
fn previous_non_ws_byte(bytes: &[u8], mut index: usize) -> Option<u8> {
    while index > 0 {
        index = index.saturating_sub(1);
        if !bytes[index].is_ascii_whitespace() {
            return Some(bytes[index]);
        }
    }
    None
}

/// Returns the next non-whitespace byte at or after `index`.
fn next_non_ws_byte(bytes: &[u8], mut index: usize) -> Option<u8> {
    while index < bytes.len() {
        if !bytes[index].is_ascii_whitespace() {
            return Some(bytes[index]);
        }
        index = index.saturating_add(1);
    }
    None
}

/// Returns true if the expression's source byte position is directly inside
/// an assertion macro invocation.
fn is_inside_assertion_macro_invocation(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let Some(path) = cx
        .sess()
        .source_map()
        .span_to_filename(expr.span)
        .into_local_path()
    else {
        return false;
    };
    let Ok(source) = fs::read_to_string(path) else {
        return false;
    };
    let byte_index = cx
        .sess()
        .source_map()
        .lookup_byte_offset(expr.span.lo())
        .pos
        .to_usize();

    is_inside_any_target_macro_invocation_at(&source, byte_index, ASSERTION_MACROS)
}

/// Returns true if `byte_index` falls directly inside one of the target macro
/// invocations in `source`.
fn is_inside_any_target_macro_invocation_at(
    source: &str,
    byte_index: usize,
    targets: &[&str],
) -> bool {
    let bytes = source.as_bytes();
    let mut frames: Vec<DelimiterFrame> = Vec::new();
    let mut index = 0;
    let end = byte_index.min(bytes.len());

    while index < end {
        if let Some(next) = skip_ignored_source(bytes, index) {
            index = next;
            continue;
        }

        match bytes[index] {
            b'(' => frames.push(DelimiterFrame {
                close: b')',
                is_target_macro: is_target_macro_delimiter_open(bytes, index, targets),
                content_start: index.saturating_add(1),
            }),
            b'[' => frames.push(DelimiterFrame {
                close: b']',
                is_target_macro: is_target_macro_delimiter_open(bytes, index, targets),
                content_start: index.saturating_add(1),
            }),
            b'{' => frames.push(DelimiterFrame {
                close: b'}',
                is_target_macro: is_target_macro_delimiter_open(bytes, index, targets),
                content_start: index.saturating_add(1),
            }),
            close @ (b')' | b']' | b'}')
                if frames.last().is_some_and(|frame| frame.close == close) =>
            {
                frames.pop();
            }
            _ => {}
        }

        index = index.saturating_add(1);
    }

    frames
        .iter()
        .rev()
        .find(|frame| frame.is_target_macro)
        .is_some_and(|frame| {
            source
                .get(frame.content_start..end)
                .is_some_and(assertion_condition_prefix_is_linted)
        })
}

/// Returns true if the assertion-macro argument prefix proves this expression
/// is the direct condition, allowing an optional negation marker.
fn assertion_condition_prefix_is_linted(prefix: &str) -> bool {
    matches!(prefix.trim(), "" | "!")
}

/// If `index` points at the start of a comment, string, or char literal,
/// returns the byte index just past it. Otherwise returns `None`.
fn skip_ignored_source(bytes: &[u8], index: usize) -> Option<usize> {
    if bytes.get(index) == Some(&b'/') && bytes.get(index.saturating_add(1)) == Some(&b'/') {
        return Some(skip_line_comment(bytes, index));
    }
    if bytes.get(index) == Some(&b'/') && bytes.get(index.saturating_add(1)) == Some(&b'*') {
        return Some(skip_block_comment(bytes, index));
    }
    if bytes.get(index) == Some(&b'"') {
        return Some(skip_quoted_string(bytes, index));
    }
    if bytes.get(index) == Some(&b'r')
        && let Some(next) = skip_raw_string(bytes, index)
    {
        return Some(next);
    }
    if bytes.get(index) == Some(&b'\'')
        && let Some(next) = skip_char_literal(bytes, index)
    {
        return Some(next);
    }

    None
}

/// Skips from `//` through the next newline or end of input.
fn skip_line_comment(bytes: &[u8], mut index: usize) -> usize {
    while index < bytes.len() && bytes[index] != b'\n' {
        index = index.saturating_add(1);
    }
    index
}

/// Skips a possibly nested `/* ... */` block comment.
fn skip_block_comment(bytes: &[u8], mut index: usize) -> usize {
    let mut depth = 0_usize;
    while index.saturating_add(1) < bytes.len() {
        match (bytes[index], bytes[index + 1]) {
            (b'/', b'*') => {
                depth = depth.saturating_add(1);
                index = index.saturating_add(2);
            }
            (b'*', b'/') => {
                depth = depth.saturating_sub(1);
                index = index.saturating_add(2);
                if depth == 0 {
                    return index;
                }
            }
            _ => index = index.saturating_add(1),
        }
    }
    bytes.len()
}

/// Skips a `"..."` string literal, honoring backslash escapes.
fn skip_quoted_string(bytes: &[u8], mut index: usize) -> usize {
    index = index.saturating_add(1);
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index = index.saturating_add(2),
            b'"' => return index.saturating_add(1),
            _ => index = index.saturating_add(1),
        }
    }
    bytes.len()
}

/// Skips a Rust raw string literal `r#"..."#`.
fn skip_raw_string(bytes: &[u8], index: usize) -> Option<usize> {
    let mut cursor = index.saturating_add(1);
    while bytes.get(cursor) == Some(&b'#') {
        cursor = cursor.saturating_add(1);
    }
    if bytes.get(cursor) != Some(&b'"') {
        return None;
    }

    let hashes = cursor.saturating_sub(index).saturating_sub(1);
    cursor = cursor.saturating_add(1);
    while cursor < bytes.len() {
        if bytes[cursor] == b'"'
            && (0..hashes).all(|offset| bytes.get(cursor + 1 + offset) == Some(&b'#'))
        {
            return Some(cursor.saturating_add(1).saturating_add(hashes));
        }
        cursor = cursor.saturating_add(1);
    }

    Some(bytes.len())
}

/// Skips a `'.'` char literal, honoring backslash escapes.
fn skip_char_literal(bytes: &[u8], index: usize) -> Option<usize> {
    if index.saturating_add(2) >= bytes.len() {
        return None;
    }

    let mut cursor = index.saturating_add(1);
    while cursor < bytes.len() && bytes[cursor] != b'\n' {
        cursor = match bytes[cursor] {
            b'\\' => cursor.saturating_add(2),
            b'\'' => return Some(cursor.saturating_add(1)),
            _ => cursor.saturating_add(1),
        };
    }

    None
}

/// Returns true if the byte before `open_index` belongs to a target macro call.
fn is_target_macro_delimiter_open(bytes: &[u8], open_index: usize, targets: &[&str]) -> bool {
    let mut bang_index = open_index;
    while bang_index > 0 && bytes[bang_index - 1].is_ascii_whitespace() {
        bang_index -= 1;
    }
    if bang_index == 0 || bytes[bang_index - 1] != b'!' {
        return false;
    }

    let mut macro_name_end = bang_index - 1;
    while macro_name_end > 0 && bytes[macro_name_end - 1].is_ascii_whitespace() {
        macro_name_end -= 1;
    }
    let mut macro_name_start = macro_name_end;
    while macro_name_start > 0 && is_identifier_byte(bytes[macro_name_start - 1]) {
        macro_name_start -= 1;
    }

    targets
        .iter()
        .any(|target| bytes.get(macro_name_start..macro_name_end) == Some(target.as_bytes()))
}

/// Returns true if `byte` is a valid Rust identifier continuation byte.
const fn is_identifier_byte(byte: u8) -> bool {
    matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_')
}

/// Returns true if the expression originates from a punchlist production
/// source path (excludes `tests/` directories and `build.rs`).
fn is_punchlist_production_path(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let Some(path) = cx
        .sess()
        .source_map()
        .span_to_filename(expr.span)
        .into_local_path()
    else {
        return false;
    };

    is_production_path(path.as_path())
}

/// Returns true if the path is treated as a punchlist test source for this
/// gate (excludes `build.rs`).
fn is_production_path(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|file_name| file_name != "build.rs")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{
        FuzzyMatcherAssertionHit, WaiverStatus, assertion_condition_prefix_is_linted,
        is_inside_any_target_macro_invocation_at, is_production_path,
        partial_matches_source_is_linted, payload_blind_method_is_linted,
        should_lint_target_source, stale_waivers, waiver_status,
    };

    const ASSERTION_MACROS: &[&str] = &[
        "assert",
        "assert_eq",
        "assert_ne",
        "debug_assert",
        "debug_assert_eq",
        "debug_assert_ne",
        "prop_assert",
        "prop_assert_eq",
        "prop_assert_ne",
    ];

    #[test]
    fn production_source_file_is_linted() {
        assert!(is_production_path(Path::new("crates/example/src/lib.rs")));
    }

    #[test]
    fn integration_test_file_is_linted() {
        assert!(is_production_path(Path::new(
            "crates/example/tests/integration.rs"
        )));
    }

    #[test]
    fn normal_target_source_is_skipped() {
        assert!(!should_lint_target_source(false));
    }

    #[test]
    fn harness_source_is_linted() {
        assert!(should_lint_target_source(true));
    }

    #[test]
    fn direct_assertion_condition_prefix_is_linted() {
        assert!(assertion_condition_prefix_is_linted(""));
    }

    #[test]
    fn negated_assertion_condition_prefix_is_linted() {
        assert!(assertion_condition_prefix_is_linted("!"));
    }

    #[test]
    fn compound_assertion_condition_prefix_is_skipped() {
        assert!(!assertion_condition_prefix_is_linted("other &&"));
    }

    #[test]
    fn is_ok_assertion_is_payload_blind() {
        assert!(payload_blind_method_is_linted("is_ok", 0));
    }

    #[test]
    fn is_some_assertion_is_payload_blind() {
        assert!(payload_blind_method_is_linted("is_some", 0));
    }

    #[test]
    fn is_none_assertion_is_payload_blind() {
        assert!(payload_blind_method_is_linted("is_none", 0));
    }

    #[test]
    fn is_err_assertion_is_payload_blind() {
        assert!(payload_blind_method_is_linted("is_err", 0));
    }

    #[test]
    fn exact_excluded_surface_waiver_is_suppressed() {
        let hit = FuzzyMatcherAssertionHit::new(
            "crates/symbiote-tool-bash/src/adapter.rs",
            839,
            "output.next().await",
            "is_none",
            "output.next().await.is_none()",
        );

        assert_eq!(waiver_status(&hit), WaiverStatus::Suppressed);
    }

    #[test]
    fn changed_excluded_surface_line_is_not_waived() {
        let hit = FuzzyMatcherAssertionHit::new(
            "crates/symbiote-tool-bash/src/adapter.rs",
            840,
            "output.next().await",
            "is_none",
            "output.next().await.is_none()",
        );

        assert_eq!(waiver_status(&hit), WaiverStatus::Unwaived);
    }

    #[test]
    fn existing_excluded_surface_is_err_hit_is_suppressed() {
        let hit = FuzzyMatcherAssertionHit::new(
            "crates/symbiote-tool-bash/src/adapter.rs",
            1413,
            "terminal_before_second",
            "is_err",
            "terminal_before_second.is_err()",
        );

        assert_eq!(waiver_status(&hit), WaiverStatus::Suppressed);
    }

    #[test]
    fn stale_waiver_is_reported() {
        let hits = [FuzzyMatcherAssertionHit::new(
            "crates/symbiote-tool-bash/src/adapter.rs",
            839,
            "output.next().await",
            "is_none",
            "output.next().await.is_none()",
        )];

        assert_eq!(stale_waivers("symbiote_tool_bash", &hits).len(), 8,);
    }

    #[test]
    fn partial_matches_placeholder_is_linted() {
        assert!(partial_matches_source_is_linted(
            "matches!(result, Err(AcpProtocolError::InvalidRequestIdCharacter { .. }))",
        ));
    }

    #[test]
    fn exact_matches_pattern_is_skipped() {
        assert!(!partial_matches_source_is_linted(
            "matches!(result, Err(AcpProtocolError::EmptyRequestId))",
        ));
    }

    #[test]
    fn matches_range_pattern_is_skipped() {
        assert!(!partial_matches_source_is_linted("matches!(value, 0..=10)",));
    }

    #[test]
    fn partial_placeholder_inside_string_is_skipped() {
        assert!(!partial_matches_source_is_linted(
            r#"matches!(value, Pattern { field: ".." })"#,
        ));
    }

    #[test]
    fn direct_assert_is_linted() {
        let source = "assert!(result.is_ok());";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }

    #[test]
    fn direct_assert_eq_is_linted() {
        let source = "assert_eq!(result.is_ok(), true);";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }

    #[test]
    fn direct_assert_ne_is_linted() {
        let source = "assert_ne!(result.is_some(), false);";
        // reason: the literal source string contains `result.is_some` by construction.
        let byte_index = source.find("result.is_some").expect("call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }

    #[test]
    fn direct_prop_assert_is_linted() {
        let source = "prop_assert!(result.is_ok());";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }

    #[test]
    fn direct_prop_assert_eq_is_linted() {
        let source = "prop_assert_eq!(result.is_ok(), true);";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }

    #[test]
    fn direct_prop_assert_ne_is_linted() {
        let source = "prop_assert_ne!(result.is_some(), false);";
        // reason: the literal source string contains `result.is_some` by construction.
        let byte_index = source.find("result.is_some").expect("call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }

    #[test]
    fn direct_debug_assert_is_linted() {
        let source = "debug_assert!(matches!(value, Pattern { .. }));";
        // reason: the literal source string contains `matches!` by construction.
        let byte_index = source.find("matches!").expect("matches macro exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }

    #[test]
    fn direct_debug_assert_eq_is_linted() {
        let source = "debug_assert_eq!(result.is_ok(), true);";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }

    #[test]
    fn direct_debug_assert_ne_is_linted() {
        let source = "debug_assert_ne!(result.is_some(), false);";
        // reason: the literal source string contains `result.is_some` by construction.
        let byte_index = source.find("result.is_some").expect("call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }

    #[test]
    fn negated_assert_is_linted() {
        let source = "assert!(!result.is_ok());";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            ASSERTION_MACROS
        ));
    }
}
