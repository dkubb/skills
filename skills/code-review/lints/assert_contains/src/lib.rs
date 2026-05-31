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

//! Deny `assert!(value.contains(...))` loose matchers in punchlist production scope.

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use std::fs;
use std::path::Path;

use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_span::Pos as _;

/// Assertion macros whose first argument is a boolean observation.
const ASSERTION_MACROS: &[&str] = &[
    "assert",
    "assert_eq",
    "debug_assert",
    "debug_assert_eq",
    "prop_assert",
    "prop_assert_eq",
];

/// Frame tracking the current open delimiter while byte-scanning source for
/// macro invocations.
struct DelimiterFrame {
    /// Matching close-delimiter byte for this frame.
    close: u8,
    /// Byte index where the delimiter content starts (one past the opener).
    content_start: usize,
    /// Whether this frame opens a target macro invocation (e.g. `assert!`).
    is_target_macro: bool,
}

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds assertion conditions of the form `assert!(value.contains(expected))`.
    ///
    /// ### Why is this bad?
    ///
    /// Substring assertions let unrelated output drift pass silently. Prefer exact
    /// `assert_eq!` checks, or parse structured text and compare the parsed value.
    ///
    /// ### Known problems
    ///
    /// This lint is deliberately syntactic. It does not decide whether the receiver is a string
    /// or another collection type.
    pub ASSERT_CONTAINS,
    Deny,
    "loose `assert!(value.contains(...))` matcher"
}

impl<'tcx> LateLintPass<'tcx> for AssertContains {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        if let ExprKind::MethodCall(segment, receiver, args, _span) = expr.kind
            && is_contains_method(cx, receiver, segment.ident.name.as_str(), args)
            && !receiver_is_format_macro(cx, receiver)
            && (is_inside_assertion_macro_invocation(cx, expr)
                || is_source_like_variable_membership(cx, receiver, args))
        {
            cx.emit_span_lint(
                ASSERT_CONTAINS,
                expr.span,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("loose `assert!(value.contains(...))` matcher");
                    diag.help("pin the exact value with `assert_eq!` or compare parsed structure");
                }),
            );
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

/// Returns true if the call shape matches a loose substring observation.
fn is_contains_method(
    cx: &LateContext<'_>,
    receiver: &Expr<'_>,
    method_name: &str,
    args: &[Expr<'_>],
) -> bool {
    method_name == "contains"
        && args.len() == 1
        && (args.first().is_some_and(is_literal_argument) || receiver_is_source_like(cx, receiver))
}

/// Returns true for source-ratchet membership filters such as
/// `.filter(|target| source.contains(target))`.
fn is_source_like_variable_membership(
    cx: &LateContext<'_>,
    receiver: &Expr<'_>,
    args: &[Expr<'_>],
) -> bool {
    receiver_is_source_like(cx, receiver)
        && args.first().is_some_and(|arg| !is_literal_argument(arg))
}

/// Returns true if the argument is a direct literal expression.
const fn is_literal_argument(argument: &Expr<'_>) -> bool {
    matches!(argument.kind, ExprKind::Lit(_))
}

/// Returns true if the receiver is a rendered Debug/Display string where
/// `.contains(...)` is the legitimate substring tool.
///
/// Three shapes count as rendered text:
/// 1. `format!(...)` macro invocation (catches `format!("{x:?}").contains(...)`).
/// 2. A method chain containing `.to_string()` (catches `error.to_string().contains(...)`).
/// 3. A bare variable whose name is `debug`, `*_debug`, or `*_string`
///    (convention: variable explicitly holds rendered output).
///
/// For these patterns the lint's "prefer `assert_eq!`" advice doesn't apply
/// — the alternative (matching a whole Debug/Display rendering) is far more
/// brittle than a phrase-presence check.
fn receiver_is_format_macro(cx: &LateContext<'_>, receiver: &Expr<'_>) -> bool {
    // After macro expansion, the receiver's `span` may carry a synthetic
    // location. `source_callsite()` walks the expansion chain back to the
    // user's source location so we can inspect the textual form.
    let user_source_span = receiver.span.source_callsite();
    let Ok(snippet) = cx.sess().source_map().span_to_snippet(user_source_span) else {
        return false;
    };
    let trimmed = snippet.trim();
    if trimmed.starts_with("format!(") || trimmed.starts_with("format !(") {
        return true;
    }
    if trimmed.contains(".to_string()") {
        return true;
    }
    if trimmed.starts_with("String::from_utf8(") || trimmed.starts_with("String::from_utf8_lossy(") {
        return true;
    }
    // Bare variable convention: variables holding rendered text typically
    // use names like `debug`, `stdout`, `stderr`, `output`, or end in
    // `_debug` / `_string` / `_output`. Anything with `.`, `(`, or
    // whitespace is not a bare identifier (and is caught by the snippet
    // checks above if relevant).
    if !trimmed.contains(|c: char| c == '.' || c == '(' || c.is_whitespace()) {
        return matches!(trimmed, "debug" | "stdout" | "stderr" | "output")
            || trimmed.ends_with("_debug")
            || trimmed.ends_with("_string")
            || trimmed.ends_with("_output");
    }
    false
}

/// Returns true if the receiver source looks like source/rendered/debug proof text.
fn receiver_is_source_like(cx: &LateContext<'_>, receiver: &Expr<'_>) -> bool {
    let Ok(snippet) = cx.sess().source_map().span_to_snippet(receiver.span) else {
        return false;
    };

    receiver_name_is_source_like(snippet.trim())
}

/// Returns true if a receiver name should be treated as source-like proof text.
fn receiver_name_is_source_like(receiver: &str) -> bool {
    matches!(
        receiver,
        "source" | "production_source" | "rendered" | "rendered_source"
    ) || receiver.ends_with("_source")
}

/// Returns true if the expression's source byte position is inside an
/// `assert!` macro invocation.
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

/// Returns true if `byte_index` falls directly inside a `target!(...)` macro
/// invocation in `source` (i.e., not nested under another macro/expression).
#[cfg(test)]
fn is_inside_target_macro_invocation_at(source: &str, byte_index: usize, target: &str) -> bool {
    is_inside_any_target_macro_invocation_at(source, byte_index, &[target])
}

/// Returns true if `byte_index` falls directly inside any `targets` macro
/// invocation in `source`.
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
                is_target_macro: is_any_target_macro_delimiter_open(bytes, index, targets),
                content_start: index.saturating_add(1),
            }),
            b'[' => frames.push(DelimiterFrame {
                close: b']',
                is_target_macro: is_any_target_macro_delimiter_open(bytes, index, targets),
                content_start: index.saturating_add(1),
            }),
            b'{' => frames.push(DelimiterFrame {
                close: b'}',
                is_target_macro: is_any_target_macro_delimiter_open(bytes, index, targets),
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
                .is_some_and(|window| matches!(window.trim(), "" | "!"))
        })
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

/// Skips from `//` through the next newline (or end of input).
fn skip_line_comment(bytes: &[u8], mut index: usize) -> usize {
    while index < bytes.len() && bytes[index] != b'\n' {
        index = index.saturating_add(1);
    }
    index
}

/// Skips a (possibly nested) `/* ... */` block comment.
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

/// Skips a Rust raw string literal `r#"..."#` (with any number of `#`s),
/// returning the byte index just past the closing delimiter.
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

/// Returns true if the byte before `open_index` (after whitespace) is `!` and
/// the identifier preceding the `!` matches `target`.
fn is_target_macro_delimiter_open(bytes: &[u8], open_index: usize, target: &str) -> bool {
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

    bytes.get(macro_name_start..macro_name_end) == Some(target.as_bytes())
}

/// Returns true if the delimiter opens any target macro invocation.
fn is_any_target_macro_delimiter_open(bytes: &[u8], open_index: usize, targets: &[&str]) -> bool {
    targets
        .iter()
        .any(|target| is_target_macro_delimiter_open(bytes, open_index, target))
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

/// Returns true if the path is treated as production source for the punchlist
/// gate (excludes `tests/` directories and `build.rs`).
fn is_production_path(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|file_name| file_name != "build.rs")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{
        is_inside_any_target_macro_invocation_at, is_inside_target_macro_invocation_at,
        is_production_path, receiver_name_is_source_like, should_lint_target_source,
    };

    #[test]
    fn production_source_file_is_linted() {
        assert!(is_production_path(Path::new("crates/example/src/lib.rs")));
    }

    #[test]
    fn in_file_test_module_path_is_linted() {
        assert!(is_production_path(Path::new("crates/example/src/tests.rs")));
    }

    #[test]
    fn integration_test_file_is_linted() {
        assert!(is_production_path(Path::new(
            "crates/example/tests/integration.rs"
        )));
    }

    #[test]
    fn build_script_is_skipped() {
        assert!(!is_production_path(Path::new("crates/example/build.rs")));
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
    fn assert_contains_method_shape_is_linted() {
        assert!(is_method_shape_linted("contains", "value", true, 1));
    }

    #[test]
    fn assert_contains_method_without_literal_argument_is_linted() {
        assert!(is_method_shape_linted("contains", "source", false, 1));
    }

    #[test]
    fn collection_contains_method_without_literal_argument_is_skipped() {
        assert!(!is_method_shape_linted(
            "contains",
            "FLAGGED_METHODS",
            false,
            1
        ));
    }

    #[test]
    fn assert_contains_method_with_chained_receiver_is_linted() {
        assert!(is_method_shape_linted("contains", "dockerfile", true, 1));
    }

    #[test]
    fn starts_with_method_is_skipped() {
        assert!(!is_method_shape_linted("starts_with", "source", true, 1));
    }

    #[test]
    fn production_source_name_is_source_like() {
        assert!(receiver_name_is_source_like("production_source"));
        assert!(receiver_name_is_source_like("included_source"));
        assert!(!receiver_name_is_source_like("FLAGGED_METHODS"));
    }

    #[test]
    fn assert_contains_call_is_inside_assert_macro_invocation() {
        let source = "assert!(dockerfile.contains(\"git config\"));";
        // reason: the literal source string contains `dockerfile.contains` by construction.
        let byte_index = source
            .find("dockerfile.contains")
            .expect("contains call exists");

        assert!(is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn assert_contains_call_with_message_is_inside_assert_macro_invocation() {
        let source = "assert!(dockerfile.contains(\"git config\"), \"missing git config\");";
        // reason: the literal source string contains `dockerfile.contains` by construction.
        let byte_index = source
            .find("dockerfile.contains")
            .expect("contains call exists");

        assert!(is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn negated_contains_call_is_direct_assert_macro_condition() {
        let source = "assert!(!cache.contains(&key));";
        // reason: the literal source string contains `cache.contains` by construction.
        let byte_index = source.find("cache.contains").expect("contains call exists");

        assert!(is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn nested_contains_call_is_not_direct_assert_macro_condition() {
        let source =
            "assert!(matches!(action, UsageError { message } if message.contains(\"--bogus\")));";
        // reason: the literal source string contains `message.contains` by construction.
        let byte_index = source
            .find("message.contains")
            .expect("contains call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn contains_call_outside_assert_macro_invocation_is_skipped() {
        let source = "let configured = dockerfile.contains(\"git config\");";
        // reason: the literal source string contains `dockerfile.contains` by construction.
        let byte_index = source
            .find("dockerfile.contains")
            .expect("contains call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn source_like_contains_call_outside_assert_macro_is_linted() {
        assert!(is_method_shape_linted("contains", "source", false, 1));
        assert!(receiver_name_is_source_like("source"));
    }

    #[test]
    fn source_like_literal_contains_call_outside_assert_macro_is_skipped() {
        assert!(is_method_shape_linted("contains", "source", true, 1));
        assert!(receiver_name_is_source_like("source"));
    }

    #[test]
    fn non_source_like_contains_call_outside_assert_macro_is_skipped() {
        assert!(!receiver_name_is_source_like("raw_output"));
    }

    #[test]
    fn contains_call_inside_debug_assert_macro_invocation_is_skipped() {
        let source = "debug_assert!(dockerfile.contains(\"git config\"));";
        // reason: the literal source string contains `dockerfile.contains` by construction.
        let byte_index = source
            .find("dockerfile.contains")
            .expect("contains call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn contains_call_inside_assert_eq_macro_invocation_is_linted() {
        let source = "assert_eq!(source.contains(fragment.as_str()), true);";
        // reason: the literal source string contains `source.contains` by construction.
        let byte_index = source
            .find("source.contains")
            .expect("contains call exists");

        assert!(is_inside_any_target_macro_invocation_at(
            source,
            byte_index,
            &["assert", "assert_eq"]
        ));
    }

    #[test]
    fn assert_like_text_inside_string_is_skipped() {
        let source = "let text = \"assert!(dockerfile.contains(\\\"git config\\\"))\";\nlet configured = dockerfile.contains(\"git config\");";
        // reason: the literal source string contains `dockerfile.contains` by construction.
        let byte_index = source
            .rfind("dockerfile.contains")
            .expect("contains call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    fn is_method_shape_linted(
        method_name: &str,
        receiver: &str,
        has_literal_argument: bool,
        argument_count: usize,
    ) -> bool {
        method_name == "contains"
            && argument_count == 1
            && (has_literal_argument || receiver_name_is_source_like(receiver))
    }
}
