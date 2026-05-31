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

//! Deny unbounded async-IO read accumulators without an explicit byte cap.

extern crate rustc_errors;
extern crate rustc_hir;

use std::path::Path;

use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds calls to `read_until`, `read_to_end`, and `read_to_string` whose
    /// receiver is not first capped with `.take(MAX + 1)` (or another bounded
    /// reader adapter).
    ///
    /// ### Why is this bad?
    ///
    /// An unbounded `read_until`/`read_to_end`/`read_to_string` call against an
    /// adversarial peer can grow the destination buffer without limit before
    /// any downstream smart constructor sees the bytes. Cap the reader with
    /// `.take(MAX + 1)` (the `+1` lets the smart constructor distinguish "at
    /// most MAX" from "exceeded MAX") and let the typed value reject anything
    /// past the cap.
    ///
    /// ### Known problems
    ///
    /// This lint is syntactic. It looks at the receiver expression's source
    /// snippet for an explicit `.take(` segment. If the cap is enforced
    /// elsewhere (e.g. through a wrapper newtype with no `.take(` in the
    /// source line) the lint still fires; in that case suppress with a
    /// per-site `#[expect(unbounded_io_accumulator, reason = "...")]` that
    /// names the enforcing type.
    pub UNBOUNDED_IO_ACCUMULATOR,
    // Track A5 registration: flipped Allow -> Deny after Track A4
    // (`read_capped_line`, commit 32d324e8) closed the cited Issue 32
    // read sites and Track A3 (b3637cc6, 1e379404, 90bc1623, 3ac7ff01,
    // ff6cf536) bounded the boundary-event accumulators.
    Deny,
    "unbounded async-IO read accumulator without a `take(MAX + 1)` cap"
}

/// Async-IO read methods this lint inspects.
const FLAGGED_METHODS: &[&str] = &["read_until", "read_to_end", "read_to_string"];

impl<'tcx> LateLintPass<'tcx> for UnboundedIoAccumulator {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        let ExprKind::MethodCall(path_segment, receiver, _args, _span) = expr.kind else {
            return;
        };

        let method_name = path_segment.ident.name.as_str();
        if !FLAGGED_METHODS.contains(&method_name) {
            return;
        }

        if receiver_is_take_capped(cx, receiver) {
            return;
        }

        cx.emit_span_lint(
            UNBOUNDED_IO_ACCUMULATOR,
            expr.span,
            rustc_errors::DiagDecorator(|diag| {
                diag.primary_message(
                    "unbounded async-IO read accumulator without a `take(MAX + 1)` cap",
                );
                diag.help(
                    "wrap the reader with `.take(MAX + 1)` and let a smart constructor reject \
                     anything past the cap",
                );
            }),
        );
    }
}

/// Returns true if the expression is in punchlist production scope.
fn should_lint_expr(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    is_punchlist_production_path(cx, expr)
}

/// Returns true if the receiver expression is preceded by a `.take(...)` call.
fn receiver_is_take_capped(cx: &LateContext<'_>, receiver: &Expr<'_>) -> bool {
    if let ExprKind::MethodCall(path_segment, _, _, _) = receiver.kind
        && path_segment.ident.name.as_str() == "take"
    {
        return true;
    }

    if receiver.span.from_expansion() {
        return false;
    }

    cx.sess()
        .source_map()
        .span_to_snippet(receiver.span)
        .is_ok_and(|snippet| snippet_contains_take_call(&snippet))
}

/// Returns true if the snippet contains a `.take(` method call.
fn snippet_contains_take_call(source: &str) -> bool {
    let bytes = source.as_bytes();
    bytes
        .windows(b".take(".len())
        .any(|window| window == b".take(")
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

    use super::{FLAGGED_METHODS, is_production_path, snippet_contains_take_call};

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
    fn flagged_methods_cover_async_read_family() {
        assert_eq!(
            FLAGGED_METHODS,
            &["read_until", "read_to_end", "read_to_string"]
        );
    }

    #[test]
    fn snippet_with_take_is_treated_as_capped() {
        assert!(snippet_contains_take_call("reader.take(MAX + 1)"));
    }

    #[test]
    fn snippet_without_take_is_treated_as_uncapped() {
        assert!(!snippet_contains_take_call("reader"));
    }

    #[test]
    fn snippet_with_take_in_identifier_is_treated_as_uncapped() {
        assert!(!snippet_contains_take_call("reader.intake(MAX + 1)"));
    }
}
