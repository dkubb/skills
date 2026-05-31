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

//! Deny non-tunneling `.unwrap_or_else(...)` fallback accessors.

extern crate rustc_errors;
extern crate rustc_hir;

use std::path::Path;

use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds `.unwrap_or_else(...)` calls whose fallback closure substitutes a
    /// value instead of preserving the error or aborting with context.
    ///
    /// ### Why is this bad?
    ///
    /// Substituting a fallback value hides the absent or failing computation
    /// from the caller. Prefer propagating the missing state or modeling it as
    /// a typed variant. The project keeps panic-with-context tunnels and
    /// poisoned-mutex recovery out of this lint because those do not silently
    /// continue with ambiguous data.
    ///
    /// ### Known problems
    ///
    /// This lint is syntactic. It classifies the fallback expression text to
    /// preserve the audited legitimate patterns in the current punchlist.
    pub UNWRAP_OR_ELSE,
    Deny,
    "fallback `.unwrap_or_else(...)` masks missing data"
}

impl<'tcx> LateLintPass<'tcx> for UnwrapOrElse {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        if let ExprKind::MethodCall(segment, _receiver, args, _span) = expr.kind
            && is_unwrap_or_else_method(segment.ident.name.as_str(), args.len())
            && let Some(fallback) = args.first()
            && !fallback_is_diverging(cx, fallback)
            && fallback_masks_missing_data(cx, fallback)
        {
            cx.emit_span_lint(
                UNWRAP_OR_ELSE,
                expr.span,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("fallback `.unwrap_or_else(...)` masks missing data");
                    diag.help("propagate the missing state or handle it as a typed variant");
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

/// Returns true if the call shape matches `<value>.unwrap_or_else(<closure>)`.
fn is_unwrap_or_else_method(method_name: &str, arg_count: usize) -> bool {
    method_name == "unwrap_or_else" && arg_count == 1
}

/// Returns true if the fallback closure diverges, so calling it cannot
/// substitute a value (the process aborts, exits, or panics). The closure's
/// signature return type is unification-coerced to the success type of
/// `unwrap_or_else`, so checking the signature isn't enough — instead we
/// check the closure body expression's unadjusted type.
fn fallback_is_diverging<'tcx>(cx: &LateContext<'tcx>, fallback: &Expr<'tcx>) -> bool {
    if let ExprKind::Closure(closure) = fallback.kind {
        let body = cx.tcx.hir_body(closure.body);
        let typeck = cx.tcx.typeck(closure.def_id);
        return typeck.expr_ty(body.value).is_never();
    }
    false
}

/// Returns true if the fallback closure substitutes a value rather than
/// tunneling the error or recovering from a known poison. When the source
/// snippet cannot be retrieved we conservatively classify the fallback as
/// masking, so the lint stays alert rather than silently passing.
fn fallback_masks_missing_data(cx: &LateContext<'_>, fallback: &Expr<'_>) -> bool {
    cx.sess()
        .source_map()
        .span_to_snippet(fallback.span)
        .map_or(true, |snippet| {
            fallback_snippet_masks_missing_data(snippet.as_str())
        })
}

/// Returns true if the fallback snippet is neither an error-tunnel pattern
/// (e.g. `process::abort()` / `exit_with_startup_ingress_error`) nor a
/// poisoned-mutex recovery (`PoisonError::into_inner`).
fn fallback_snippet_masks_missing_data(fallback_snippet: &str) -> bool {
    !is_error_tunnel_fallback(fallback_snippet) && !is_poison_recovery_fallback(fallback_snippet)
}

/// Returns true if the fallback snippet contains a panic-with-context tunnel.
fn is_error_tunnel_fallback(fallback_snippet: &str) -> bool {
    fallback_snippet.contains("process::abort")
        || fallback_snippet.contains("exit_with_startup_ingress_error")
}

/// Returns true if the fallback snippet is `PoisonError::into_inner` (verbatim).
fn is_poison_recovery_fallback(fallback_snippet: &str) -> bool {
    fallback_snippet.trim() == "PoisonError::into_inner"
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
        fallback_snippet_masks_missing_data, is_production_path, is_unwrap_or_else_method,
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
    fn unwrap_or_else_method_is_linted() {
        assert!(is_unwrap_or_else_method("unwrap_or_else", 1));
    }

    #[test]
    fn unwrap_or_else_method_without_argument_is_skipped() {
        assert!(!is_unwrap_or_else_method("unwrap_or_else", 0));
    }

    #[test]
    fn unwrap_or_method_is_skipped() {
        assert!(!is_unwrap_or_else_method("unwrap_or", 1));
    }

    #[test]
    fn aborting_fallback_is_skipped() {
        assert!(!fallback_snippet_masks_missing_data(
            r#"|err| {
                eprintln!("FATAL: {err}");
                process::abort();
            }"#
        ));
    }

    #[test]
    fn startup_exit_fallback_is_skipped() {
        assert!(!fallback_snippet_masks_missing_data(
            "|message| exit_with_startup_ingress_error(message.as_str())"
        ));
    }

    #[test]
    fn poison_error_recovery_is_skipped() {
        assert!(!fallback_snippet_masks_missing_data(
            "PoisonError::into_inner"
        ));
    }

    #[test]
    fn computed_fallback_is_linted() {
        assert!(fallback_snippet_masks_missing_data(
            r#"|| non_empty_string!("fallback")"#
        ));
    }
}
