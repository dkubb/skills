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

//! Deny bare `.clone()` calls in punchlist production scope.

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use std::fs;
use std::path::Path;

use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_middle::ty;
use rustc_span::{Pos as _, SyntaxContext, sym};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds `value.clone()` on `Arc<T>` and `Rc<T>` receivers.
    ///
    /// ### Why is this bad?
    ///
    /// On `Arc` / `Rc` the `.clone()` reads ambiguously: it could be a
    /// refcount bump (cheap, the usual intent) or a data clone (expensive,
    /// usually a bug). Prefer `Arc::clone(&value)` / `Rc::clone(&value)`
    /// to make the refcount bump explicit at the call site.
    ///
    /// For other types (`String`, `PathBuf`, `Vec`, custom structs, …) the
    /// `.clone()` shape is unambiguous and idiomatic; this lint deliberately
    /// stays quiet there.
    ///
    /// ### Known problems
    ///
    /// This lint inspects the receiver's resolved type. It does not decide
    /// whether the clone is redundant.
    pub BARE_CLONE,
    Deny,
    "bare `.clone()` on Arc/Rc receiver"
}

impl<'tcx> LateLintPass<'tcx> for BareClone {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion()
            || expr.span.ctxt() != SyntaxContext::root()
            || !should_lint_expr(cx, expr)
        {
            return;
        }

        if let ExprKind::MethodCall(segment, receiver, args, _span) = expr.kind
            && segment.ident.name.as_str() == "clone"
            && args.is_empty()
            && receiver_is_arc_or_rc(cx, receiver)
            && !is_inside_source_macro_invocation(cx, expr)
        {
            cx.emit_span_lint(
                BARE_CLONE,
                expr.span,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("bare `.clone()` on Arc/Rc receiver");
                    diag.help(
                        "use `Arc::clone(&value)` / `Rc::clone(&value)` to make the \
                     refcount bump explicit and disambiguate from a data clone",
                    );
                }),
            );
        }
    }
}

/// Returns true if the receiver expression's type is `Arc<T>` or `Rc<T>`.
fn receiver_is_arc_or_rc(cx: &LateContext<'_>, receiver: &Expr<'_>) -> bool {
    let receiver_ty = cx.typeck_results().expr_ty(receiver).peel_refs();
    if let ty::Adt(adt_def, _) = receiver_ty.kind() {
        let did = adt_def.did();
        cx.tcx.is_diagnostic_item(sym::Arc, did) || cx.tcx.is_diagnostic_item(sym::Rc, did)
    } else {
        false
    }
}

/// Frame tracking the current open delimiter while byte-scanning source for
/// macro invocations.
#[derive(Clone, Copy)]
struct DelimiterFrame {
    /// Matching close-delimiter byte for this frame.
    close: u8,
    /// Whether this frame opens a `name!` macro invocation.
    is_macro: bool,
}

/// Returns true if the expression should be examined by this lint.
fn should_lint_expr(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    is_punchlist_production_path(cx, expr) && should_lint_target_source(cx.sess().is_test_crate())
}

/// Returns true if the current compile target should be linted.
const fn should_lint_target_source(is_test_crate: bool) -> bool {
    is_test_crate
}

/// Returns true if the expression's source byte position is inside any
/// `name!(...)` source-level macro invocation (so we don't fire inside e.g.
/// `vec![value.clone()]` or `assert_eq!(left.clone(), right)`).
fn is_inside_source_macro_invocation(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
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

    is_inside_macro_invocation_at(&source, byte_index)
}

/// Returns true if `byte_index` is inside any open macro-invocation delimiter.
fn is_inside_macro_invocation_at(source: &str, byte_index: usize) -> bool {
    let bytes = source.as_bytes();
    let mut frames = Vec::new();
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
                is_macro: is_macro_delimiter_open(bytes, index),
            }),
            b'[' => frames.push(DelimiterFrame {
                close: b']',
                is_macro: is_macro_delimiter_open(bytes, index),
            }),
            b'{' => frames.push(DelimiterFrame {
                close: b'}',
                is_macro: is_macro_delimiter_open(bytes, index),
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

    frames.iter().any(|frame| frame.is_macro)
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

/// Skips a Rust raw string literal `r#"..."#`, returning the byte index just
/// past the closing delimiter.
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

/// Returns true if the byte before `open_index` (after whitespace) is `!`
/// preceded by an identifier or expression continuation.
fn is_macro_delimiter_open(bytes: &[u8], open_index: usize) -> bool {
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
    macro_name_end > 0
        && matches!(
            bytes[macro_name_end - 1],
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b')' | b']'
        )
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

    use super::{is_inside_macro_invocation_at, is_production_path, should_lint_target_source};

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
    fn direct_source_is_not_inside_macro_invocation() {
        let source = "let cloned = value.clone();";
        // reason: the literal source string contains `value.clone()` by construction.
        let byte_index = source.find("value.clone()").expect("clone exists");

        assert!(!is_inside_macro_invocation_at(source, byte_index));
    }

    #[test]
    fn vec_macro_argument_is_inside_macro_invocation() {
        let source = "let values = vec![value.clone()];";
        // reason: the literal source string contains `value.clone()` by construction.
        let byte_index = source.find("value.clone()").expect("clone exists");

        assert!(is_inside_macro_invocation_at(source, byte_index));
    }

    #[test]
    fn multiline_macro_argument_is_inside_macro_invocation() {
        let source = "assert_eq!(\n    left.clone(),\n    right,\n);";
        // reason: the literal source string contains `left.clone()` by construction.
        let byte_index = source.find("left.clone()").expect("clone exists");

        assert!(is_inside_macro_invocation_at(source, byte_index));
    }

    #[test]
    fn source_after_macro_invocation_is_not_inside_macro_invocation() {
        let source = "assert_eq!(left, right);\nlet cloned = value.clone();";
        // reason: the literal source string contains `value.clone()` by construction.
        let byte_index = source.find("value.clone()").expect("clone exists");

        assert!(!is_inside_macro_invocation_at(source, byte_index));
    }
}
