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

//! Deny `assert!(value.starts_with(...))` loose matchers in punchlist production scope.

extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use std::fs;
use std::path::Path;

use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_span::Pos as _;

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
    /// Finds direct assertion conditions of the form `assert!(value.starts_with(expected))`.
    ///
    /// ### Why is this bad?
    ///
    /// Prefix assertions let unrelated output drift pass silently. Prefer exact
    /// `assert_eq!` checks, or parse structured text and compare the parsed value.
    ///
    /// ### Known problems
    ///
    /// This lint is deliberately syntactic. It does not decide whether the receiver is a string
    /// or another collection type.
    pub ASSERT_STARTS_WITH,
    Deny,
    "loose `assert!(value.starts_with(...))` matcher"
}

impl<'tcx> LateLintPass<'tcx> for AssertStartsWith {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        if let ExprKind::MethodCall(segment, _receiver, args, _span) = expr.kind
            && is_starts_with_method(segment.ident.name.as_str(), args.len())
            && !argument_is_path_prefix(args.first())
            && is_inside_assert_macro_invocation(cx, expr)
        {
            cx.emit_span_lint(
                ASSERT_STARTS_WITH,
                expr.span,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("loose `assert!(value.starts_with(...))` matcher");
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

/// Returns true if the `starts_with` argument is a path/URL prefix string
/// literal — a value like `"/tmp/run/"` or `"https://"` where the suffix
/// is intentionally variable (PID, identifier, etc.) and `assert_eq!`
/// would be impossible. These are the legitimate uses of `starts_with`.
fn argument_is_path_prefix(arg: Option<&Expr<'_>>) -> bool {
    let Some(arg) = arg else { return false };
    let ExprKind::Lit(ref lit) = arg.kind else { return false };
    let rustc_ast::LitKind::Str(symbol, _) = lit.node else { return false };
    let text = symbol.as_str();
    text.starts_with('/') || text.contains("://")
}

/// Returns true if the call shape matches `<value>.starts_with(<arg>)`.
fn is_starts_with_method(method_name: &str, arg_count: usize) -> bool {
    method_name == "starts_with" && arg_count == 1
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
        is_inside_target_macro_invocation_at, is_production_path, is_starts_with_method,
        should_lint_target_source,
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
    fn starts_with_method_is_linted() {
        assert!(is_starts_with_method("starts_with", 1));
    }

    #[test]
    fn starts_with_method_without_argument_is_skipped() {
        assert!(!is_starts_with_method("starts_with", 0));
    }

    #[test]
    fn ends_with_method_is_skipped() {
        assert!(!is_starts_with_method("ends_with", 1));
    }

    #[test]
    fn direct_starts_with_call_is_inside_assert_macro_invocation() {
        let source = "assert!(output.starts_with(\"prefix\"));";
        // reason: the literal source string contains `output.starts_with` by construction.
        let byte_index = source.find("output.starts_with").expect("call exists");

        assert!(is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn direct_starts_with_call_with_message_is_inside_assert_macro_invocation() {
        let source = "assert!(output.starts_with(\"prefix\"), \"bad prefix\");";
        // reason: the literal source string contains `output.starts_with` by construction.
        let byte_index = source.find("output.starts_with").expect("call exists");

        assert!(is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn negated_starts_with_call_is_not_direct_assert_macro_condition() {
        let source = "assert!(!output.starts_with(\"prefix\"));";
        // reason: the literal source string contains `output.starts_with` by construction.
        let byte_index = source.find("output.starts_with").expect("call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn nested_starts_with_call_is_not_direct_assert_macro_condition() {
        let source =
            "assert!(matches!(action, UsageError { message } if message.starts_with(\"--\")));";
        // reason: the literal source string contains `message.starts_with` by construction.
        let byte_index = source.find("message.starts_with").expect("call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }

    #[test]
    fn starts_with_call_outside_assert_macro_invocation_is_skipped() {
        let source = "let configured = output.starts_with(\"prefix\");";
        // reason: the literal source string contains `output.starts_with` by construction.
        let byte_index = source.find("output.starts_with").expect("call exists");

        assert!(!is_inside_target_macro_invocation_at(
            source, byte_index, "assert"
        ));
    }
}
