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

//! Deny direct tag-only `assert!(value.is_ok())` style observations.

extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use std::fs;
use std::path::Path;
use std::sync::Mutex;

use rustc_ast::ast::LitKind;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_span::{DUMMY_SP, Pos as _, def_id::LOCAL_CRATE};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds direct assertion conditions of the form `assert!(value.is_ok())`,
    /// `assert!(value.is_err())`, `assert!(value.is_some())`,
    /// `assert!(value.is_none())`, and process lifecycle boolean accessors.
    ///
    /// ### Why is this bad?
    ///
    /// Tag-only assertions prove only the outer variant and let the observable
    /// payload drift. Prefer exact `assert_eq!` comparisons or destructure the
    /// value and compare its payload.
    ///
    /// ### Known problems
    ///
    /// This lint is deliberately syntactic. It only rejects direct `assert!`
    /// conditions in test targets and leaves nested predicate guards for more
    /// semantic exact-matcher passes.
    pub TAG_ONLY_ASSERTION,
    Deny,
    "tag-only result/option assertion"
}

/// Waived hits observed while linting the current test crate.
static OBSERVED_WAIVED_HITS: Mutex<Vec<TagOnlyAssertionHit>> = Mutex::new(Vec::new());

/// Exact waivers for currently excluded punchlist surfaces.
const TAG_ONLY_ASSERTION_WAIVERS: [TagOnlyAssertionWaiver; 9] = [
    TagOnlyAssertionWaiver {
        crate_name: "symbiote_tool_bash",
        line: 839,
        method: "is_none",
        path: "crates/symbiote-tool-bash/src/adapter.rs",
        receiver: "output.next().await",
        snippet: "output.next().await.is_none()",
    },
    TagOnlyAssertionWaiver {
        crate_name: "symbiote_tool_bash",
        line: 1413,
        method: "is_err",
        path: "crates/symbiote-tool-bash/src/adapter.rs",
        receiver: "terminal_before_second",
        snippet: "terminal_before_second.is_err()",
    },
    TagOnlyAssertionWaiver {
        crate_name: "symbiote_tool_bash",
        line: 1448,
        method: "is_none",
        path: "crates/symbiote-tool-bash/src/adapter.rs",
        receiver: "output.next().await",
        snippet: "output.next().await.is_none()",
    },
    TagOnlyAssertionWaiver {
        crate_name: "symbiote_tool_bash",
        line: 1466,
        method: "is_none",
        path: "crates/symbiote-tool-bash/src/adapter.rs",
        receiver: "output.next().await",
        snippet: "output.next().await.is_none()",
    },
    TagOnlyAssertionWaiver {
        crate_name: "symbiote_tool_bash",
        line: 1601,
        method: "is_none",
        path: "crates/symbiote-tool-bash/src/adapter.rs",
        receiver: "output.next().await",
        snippet: "output.next().await.is_none()",
    },
    TagOnlyAssertionWaiver {
        crate_name: "symbiote_tool_bash",
        line: 1649,
        method: "is_none",
        path: "crates/symbiote-tool-bash/src/adapter.rs",
        receiver: "output.next().await",
        snippet: "output.next().await.is_none()",
    },
    TagOnlyAssertionWaiver {
        crate_name: "symbiote_tool_bash",
        line: 1693,
        method: "is_none",
        path: "crates/symbiote-tool-bash/src/adapter.rs",
        receiver: "output.next().await",
        snippet: "output.next().await.is_none()",
    },
    TagOnlyAssertionWaiver {
        crate_name: "symbiote_tool_bash",
        line: 1740,
        method: "is_none",
        path: "crates/symbiote-tool-bash/src/adapter.rs",
        receiver: "output.next().await",
        snippet: "output.next().await.is_none()",
    },
    TagOnlyAssertionWaiver {
        crate_name: "symbiote_tool_bash",
        line: 1797,
        method: "is_none",
        path: "crates/symbiote-tool-bash/src/adapter.rs",
        receiver: "output.next().await",
        snippet: "output.next().await.is_none()",
    },
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

impl<'tcx> LateLintPass<'tcx> for TagOnlyAssertion {
    fn check_crate(&mut self, _cx: &LateContext<'tcx>) {
        OBSERVED_WAIVED_HITS
            .lock()
            // reason: the lint runs single-process and never panics while holding this lock.
            .expect("tag-only assertion waiver observation lock is not poisoned")
            .clear();
    }

    fn check_crate_post(&mut self, cx: &LateContext<'tcx>) {
        if !cx.sess().is_test_crate() {
            return;
        }

        let crate_name_symbol = cx.tcx.crate_name(LOCAL_CRATE);
        let crate_name = crate_name_symbol.as_str();
        let observed = OBSERVED_WAIVED_HITS
            .lock()
            // reason: the lint runs single-process and never panics while holding this lock.
            .expect("tag-only assertion waiver observation lock is not poisoned");

        for waiver in stale_waivers(crate_name, observed.as_slice()) {
            cx.emit_span_lint(
                TAG_ONLY_ASSERTION,
                DUMMY_SP,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("stale tag-only assertion waiver");
                    diag.help(format!(
                        "remove the stale `{}` waiver for `{}` at {}:{}",
                        waiver.method, waiver.snippet, waiver.path, waiver.line
                    ));
                }),
            );
        }
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        if let ExprKind::Lit(literal) = expr.kind
            && matches!(literal.node, LitKind::Bool(_))
            && is_direct_bool_literal_assertion_macro_argument(cx, expr)
        {
            emit_tag_only_assertion(cx, expr);
            return;
        }

        if let ExprKind::MethodCall(segment, receiver, args, _span) = expr.kind
            && is_tag_only_method(segment.ident.name.as_str(), args)
            && is_inside_assert_macro_invocation(cx, expr)
        {
            let Some(hit) = tag_only_assertion_hit(cx, expr, receiver, segment.ident.name.as_str())
            else {
                emit_tag_only_assertion(cx, expr);
                return;
            };

            if waiver_status(&hit) == WaiverStatus::Suppressed {
                OBSERVED_WAIVED_HITS
                    .lock()
                    // reason: the lint runs single-process and never panics while holding this lock.
                    .expect("tag-only assertion waiver observation lock is not poisoned")
                    .push(hit);
                return;
            }

            emit_tag_only_assertion(cx, expr);
        }
    }
}

/// Exact tag-only assertion hit fingerprint.
#[derive(Clone, Debug, Eq, PartialEq)]
struct TagOnlyAssertionHit {
    /// Source line number.
    line: usize,
    /// Method name.
    method: String,
    /// Relative source path.
    path: String,
    /// Receiver expression snippet.
    receiver: String,
    /// Full normalized method-call snippet.
    snippet: String,
}

impl TagOnlyAssertionHit {
    /// Returns true if `waiver` matches this hit exactly.
    fn matches_waiver(&self, waiver: TagOnlyAssertionWaiver) -> bool {
        self.path == waiver.path
            && self.line == waiver.line
            && self.receiver == waiver.receiver
            && self.method == waiver.method
            && self.snippet == waiver.snippet
    }

    /// Creates an exact hit fingerprint.
    fn new(
        path: impl Into<String>,
        line: usize,
        receiver: impl Into<String>,
        method: impl Into<String>,
        snippet: impl Into<String>,
    ) -> Self {
        Self {
            line,
            method: method.into(),
            path: path.into(),
            receiver: receiver.into(),
            snippet: snippet.into(),
        }
    }
}

/// Exact tag-only assertion waiver fingerprint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TagOnlyAssertionWaiver {
    /// Crate that owns the waiver.
    crate_name: &'static str,
    /// Source line number.
    line: usize,
    /// Method name.
    method: &'static str,
    /// Relative source path.
    path: &'static str,
    /// Receiver expression snippet.
    receiver: &'static str,
    /// Full normalized method-call snippet.
    snippet: &'static str,
}

/// Waiver lookup result for a lint hit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WaiverStatus {
    /// The hit exactly matches a recorded waiver.
    Suppressed,
    /// The hit has no exact waiver and must be emitted.
    Unwaived,
}

/// Returns whether `hit` is exactly waived.
fn waiver_status(hit: &TagOnlyAssertionHit) -> WaiverStatus {
    if TAG_ONLY_ASSERTION_WAIVERS
        .iter()
        .copied()
        .any(|waiver| hit.matches_waiver(waiver))
    {
        WaiverStatus::Suppressed
    } else {
        WaiverStatus::Unwaived
    }
}

/// Returns waivers for `crate_name` that did not match an observed hit.
fn stale_waivers(
    crate_name: &str,
    observed: &[TagOnlyAssertionHit],
) -> Vec<TagOnlyAssertionWaiver> {
    TAG_ONLY_ASSERTION_WAIVERS
        .iter()
        .copied()
        .filter(|waiver| waiver.crate_name == crate_name)
        .filter(|waiver| !observed.iter().any(|hit| hit.matches_waiver(*waiver)))
        .collect()
}

/// Builds an exact hit fingerprint from a tag-only assertion method call.
fn tag_only_assertion_hit(
    cx: &LateContext<'_>,
    expr: &Expr<'_>,
    receiver: &Expr<'_>,
    method: &str,
) -> Option<TagOnlyAssertionHit> {
    let source_map = cx.sess().source_map();
    let source_path = source_map.span_to_filename(expr.span).into_local_path()?;
    let relative_path = repository_relative_path(source_path.to_string_lossy().as_ref());
    let line = source_map.lookup_char_pos(expr.span.lo()).line;
    let receiver_snippet =
        normalized_snippet(source_map.span_to_snippet(receiver.span).ok()?.as_str());
    let snippet = normalized_snippet(source_map.span_to_snippet(expr.span).ok()?.as_str());

    Some(TagOnlyAssertionHit::new(
        relative_path,
        line,
        receiver_snippet,
        method.to_owned(),
        snippet,
    ))
}

/// Returns `path` relative to the repository root when possible.
fn repository_relative_path(path: &str) -> String {
    path.split_once("/crates/").map_or_else(
        || path.to_owned(),
        |(_prefix, suffix)| format!("crates/{suffix}"),
    )
}

/// Returns the stable source snippet used by exact waiver fingerprints.
fn normalized_snippet(snippet: &str) -> String {
    snippet.trim().to_owned()
}

/// Emits the tag-only assertion diagnostic.
fn emit_tag_only_assertion(cx: &LateContext<'_>, expr: &Expr<'_>) {
    cx.emit_span_lint(
        TAG_ONLY_ASSERTION,
        expr.span,
        rustc_errors::DiagDecorator(|diag| {
            diag.primary_message("tag-only result/option assertion");
            diag.help(
                "compare the exact value with `assert_eq!` or destructure and assert the payload",
            );
        }),
    );
}

/// Returns true if the expression should be examined by this lint.
fn should_lint_expr(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    is_punchlist_production_path(cx, expr) && should_lint_target_source(cx.sess().is_test_crate())
}

/// Returns true if the current compile target should be linted.
const fn should_lint_target_source(is_test_crate: bool) -> bool {
    is_test_crate
}

/// Returns true if the method call only observes a Result/Option tag.
fn is_tag_only_method(method_name: &str, args: &[Expr<'_>]) -> bool {
    is_tag_only_method_shape(method_name, args.len())
}

/// Returns true if the method name and arity describe a tag-only predicate.
fn is_tag_only_method_shape(method_name: &str, argument_count: usize) -> bool {
    argument_count == 0
        && matches!(
            method_name,
            "is_ok" | "is_err" | "is_some" | "is_none" | "alive" | "termination_requested"
        )
}

/// Returns true if the expression's source byte position is inside an
/// `assert!` macro invocation.
fn is_inside_assert_macro_invocation(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
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

    is_inside_target_macro_invocation_at(&source, byte_index, "assert")
}

/// Returns true if `byte_index` falls directly inside a `target!(...)` macro
/// invocation in `source` (i.e., not nested under another macro/expression).
fn is_inside_target_macro_invocation_at(source: &str, byte_index: usize, target: &str) -> bool {
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
                is_target_macro: is_target_macro_delimiter_open(bytes, index, target),
                content_start: index.saturating_add(1),
            }),
            b'[' => frames.push(DelimiterFrame {
                close: b']',
                is_target_macro: is_target_macro_delimiter_open(bytes, index, target),
                content_start: index.saturating_add(1),
            }),
            b'{' => frames.push(DelimiterFrame {
                close: b'}',
                is_target_macro: is_target_macro_delimiter_open(bytes, index, target),
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
                .is_some_and(|window| window.trim().is_empty())
        })
}

/// Returns true if `byte_index` falls inside a direct argument to any target
/// assertion macro invocation.
fn is_direct_target_macro_argument_at(source: &str, byte_index: usize, targets: &[&str]) -> bool {
    let bytes = source.as_bytes();
    let Some((content_start, content_end)) = target_macro_content_range(bytes, byte_index, targets)
    else {
        return false;
    };

    is_top_level_argument_byte(bytes, content_start, content_end, byte_index)
}

/// Returns the content range of the innermost target macro containing
/// `byte_index`.
fn target_macro_content_range(
    bytes: &[u8],
    byte_index: usize,
    targets: &[&str],
) -> Option<(usize, usize)> {
    let mut frames: Vec<DelimiterFrame> = Vec::new();
    let mut target_range = None;
    let mut index = 0;

    while index < bytes.len() {
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
                if let Some(frame) = frames.pop()
                    && frame.is_target_macro
                    && frame.content_start <= byte_index
                    && byte_index < index
                {
                    target_range = Some((frame.content_start, index));
                }
            }
            _ => {}
        }

        index = index.saturating_add(1);
    }

    target_range
}

/// Returns true if `byte_index` starts a whole top-level bool argument.
fn is_top_level_argument_byte(
    bytes: &[u8],
    content_start: usize,
    content_end: usize,
    byte_index: usize,
) -> bool {
    let mut depth = 0_usize;
    let mut argument_start = content_start;
    let mut index = content_start;

    while index < content_end {
        if let Some(next) = skip_ignored_source(bytes, index) {
            index = next;
            continue;
        }

        if bytes[index] == b',' && depth == 0 {
            if is_exact_bool_argument(bytes, argument_start, index, byte_index) {
                return true;
            }
            argument_start = index.saturating_add(1);
            index = index.saturating_add(1);
            continue;
        }

        match bytes[index] {
            b'(' | b'[' | b'{' if index != content_start => depth = depth.saturating_add(1),
            b')' | b']' | b'}' if depth > 0 => depth = depth.saturating_sub(1),
            _ => {}
        }

        index = index.saturating_add(1);
    }

    is_exact_bool_argument(bytes, argument_start, content_end, byte_index)
}

/// Returns true if the argument slice trims exactly to `true` or `false` and
/// the literal begins at `byte_index`.
fn is_exact_bool_argument(bytes: &[u8], start: usize, end: usize, byte_index: usize) -> bool {
    let mut trimmed_start = start;
    while trimmed_start < end && bytes[trimmed_start].is_ascii_whitespace() {
        trimmed_start = trimmed_start.saturating_add(1);
    }

    let mut trimmed_end = end;
    while trimmed_start < trimmed_end && bytes[trimmed_end - 1].is_ascii_whitespace() {
        trimmed_end = trimmed_end.saturating_sub(1);
    }

    trimmed_start == byte_index
        && matches!(
            bytes.get(trimmed_start..trimmed_end),
            Some(b"true" | b"false")
        )
}

/// Returns true if the expression is a direct boolean literal argument to an
/// equality/inequality assertion macro.
fn is_direct_bool_literal_assertion_macro_argument(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
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

    is_direct_bool_literal_assertion_macro_argument_at(&source, byte_index)
}

/// Returns true if `byte_index` points at a direct boolean literal argument to
/// an equality/inequality assertion macro.
fn is_direct_bool_literal_assertion_macro_argument_at(source: &str, byte_index: usize) -> bool {
    is_direct_target_macro_argument_at(
        source,
        byte_index,
        &[
            "assert_eq",
            "assert_ne",
            "prop_assert_eq",
            "prop_assert_ne",
            "debug_assert_eq",
            "debug_assert_ne",
        ],
    )
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

/// Returns true if the delimiter opens any of the target macro invocations.
fn is_any_target_macro_delimiter_open(bytes: &[u8], open_index: usize, targets: &[&str]) -> bool {
    targets
        .iter()
        .any(|target| is_target_macro_delimiter_open(bytes, open_index, target))
}

/// Returns true if `byte` is a valid Rust identifier continuation byte.
const fn is_identifier_byte(byte: u8) -> bool {
    matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_')
}

/// Returns true if the expression originates from a punchlist Rust source path.
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

/// Returns true if the path is treated as Rust source for the punchlist gate.
fn is_production_path(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|file_name| file_name != "build.rs")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{
        TagOnlyAssertionHit, WaiverStatus, is_direct_bool_literal_assertion_macro_argument_at,
        is_inside_target_macro_invocation_at, is_production_path, is_tag_only_method_shape,
        should_lint_target_source,
    };

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
    fn excluded_bash_tool_source_is_linted_before_waiver_matching() {
        assert!(is_production_path(Path::new(
            "crates/symbiote-tool-bash/src/adapter.rs"
        )));
    }

    #[test]
    fn exact_excluded_surface_waiver_is_suppressed() {
        let hit = TagOnlyAssertionHit::new(
            "crates/symbiote-tool-bash/src/adapter.rs",
            839,
            "output.next().await",
            "is_none",
            "output.next().await.is_none()",
        );

        assert_eq!(super::waiver_status(&hit), WaiverStatus::Suppressed);
    }

    #[test]
    fn changed_excluded_surface_code_is_reported() {
        let hit = TagOnlyAssertionHit::new(
            "crates/symbiote-tool-bash/src/adapter.rs",
            839,
            "output.next().await",
            "is_none",
            "output.next() . await . is_none()",
        );

        assert_eq!(super::waiver_status(&hit), WaiverStatus::Unwaived);
    }

    #[test]
    fn new_excluded_surface_hit_is_reported() {
        let hit = TagOnlyAssertionHit::new(
            "crates/symbiote-tool-bash/src/adapter.rs",
            840,
            "output.next().await",
            "is_none",
            "output.next().await.is_none()",
        );

        assert_eq!(super::waiver_status(&hit), WaiverStatus::Unwaived);
    }

    #[test]
    fn stale_waiver_is_reported() {
        let hits = [];

        assert_eq!(
            super::stale_waivers("symbiote_tool_bash", &hits),
            super::TAG_ONLY_ASSERTION_WAIVERS
        );
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
    fn result_ok_tag_check_is_linted() {
        assert!(is_tag_only_method_shape("is_ok", 0));
    }

    #[test]
    fn result_err_tag_check_is_linted() {
        assert!(is_tag_only_method_shape("is_err", 0));
    }

    #[test]
    fn option_some_tag_check_is_linted() {
        assert!(is_tag_only_method_shape("is_some", 0));
    }

    #[test]
    fn option_none_tag_check_is_linted() {
        assert!(is_tag_only_method_shape("is_none", 0));
    }

    #[test]
    fn worker_liveness_tag_check_is_linted() {
        assert!(is_tag_only_method_shape("alive", 0));
    }

    #[test]
    fn worker_termination_tag_check_is_linted() {
        assert!(is_tag_only_method_shape("termination_requested", 0));
    }

    #[test]
    fn other_predicate_is_skipped() {
        assert!(!is_tag_only_method_shape("is_empty", 0));
    }

    #[test]
    fn method_with_arguments_is_skipped() {
        assert!(!is_tag_only_method_shape("is_ok", 1));
    }

    #[test]
    fn tag_check_call_is_inside_assert_macro_invocation() {
        let source = "assert!(result.is_ok());";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("is_ok call exists");

        assert!(is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn tag_check_call_with_message_is_inside_assert_macro_invocation() {
        let source = "assert!(result.is_ok(), \"result should pass\");";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("is_ok call exists");

        assert!(is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn negated_tag_check_is_not_direct_assert_macro_condition() {
        let source = "assert!(!result.is_ok());";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("is_ok call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn nested_tag_check_is_not_direct_assert_macro_condition() {
        let source = "assert!(matches!(result, Ok(value) if value.is_some()));";
        // reason: the literal source string contains `value.is_some` by construction.
        let byte_index = source.find("value.is_some").expect("is_some call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn tag_check_outside_assert_macro_invocation_is_skipped() {
        let source = "let passed = result.is_ok();";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("is_ok call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn tag_check_inside_debug_assert_macro_invocation_is_skipped() {
        let source = "debug_assert!(result.is_ok());";
        // reason: the literal source string contains `result.is_ok` by construction.
        let byte_index = source.find("result.is_ok").expect("is_ok call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn assert_eq_bool_literal_argument_is_linted() {
        let source = "assert_eq!(transport.is_write_vectored(), true);";
        // reason: the literal source string contains `true` by construction.
        let byte_index = source.find("true").expect("bool literal exists");

        assert!(is_direct_bool_literal_assertion_macro_argument_at(
            source, byte_index
        ));
    }

    #[test]
    fn prop_assert_ne_bool_literal_argument_is_linted() {
        let source = "prop_assert_ne!(false, scenario.completed_successfully());";
        // reason: the literal source string contains `false` by construction.
        let byte_index = source.find("false").expect("bool literal exists");

        assert!(is_direct_bool_literal_assertion_macro_argument_at(
            source, byte_index
        ));
    }

    #[test]
    fn nested_bool_literal_argument_is_skipped() {
        let source = "assert_eq!(predicate(true), expected);";
        // reason: the literal source string contains `true` by construction.
        let byte_index = source.find("true").expect("bool literal exists");

        assert!(!is_direct_bool_literal_assertion_macro_argument_at(
            source, byte_index
        ));
    }

    #[test]
    fn bool_literal_inside_expected_array_is_skipped() {
        let source = "assert_eq!(observed, [true, false]);";
        // reason: the literal source string contains `true` by construction.
        let byte_index = source.find("true").expect("bool literal exists");

        assert!(!is_direct_bool_literal_assertion_macro_argument_at(
            source, byte_index
        ));
    }

    #[test]
    fn non_assertion_bool_literal_argument_is_skipped() {
        let source = "assert!(transport.is_write_vectored() == true);";
        // reason: the literal source string contains `true` by construction.
        let byte_index = source.find("true").expect("bool literal exists");

        assert!(!is_direct_bool_literal_assertion_macro_argument_at(
            source, byte_index
        ));
    }
}
