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

//! Deny `.unwrap_or(...)` fallback accessors in punchlist production scope.

extern crate rustc_errors;
extern crate rustc_hir;

use std::path::Path;

use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds `.unwrap_or(...)` method calls.
    ///
    /// ### Why is this bad?
    ///
    /// Substituting a fallback value hides the absent or failing computation
    /// from the caller. Prefer propagating the missing state or modeling it as
    /// a typed variant.
    ///
    /// ### Known problems
    ///
    /// This lint is syntactic. It does not resolve whether the receiver is an
    /// `Option`, `Result`, or another type with a same-named method.
    pub UNWRAP_OR,
    Deny,
    "fallback `.unwrap_or(...)` masks missing data"
}

impl<'tcx> LateLintPass<'tcx> for UnwrapOr {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        if let ExprKind::MethodCall(segment, receiver, args, _span) = expr.kind
            && is_unwrap_or_method(segment.ident.name.as_str(), args.len())
            && !receiver_is_safe_strip(receiver)
        {
            cx.emit_span_lint(
                UNWRAP_OR,
                expr.span,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("fallback `.unwrap_or(...)` masks missing data");
                    diag.help("propagate the missing state or handle it as a typed variant");
                }),
            );
        }
    }
}

/// Returns true if the receiver is a method call to a safe-strip method whose
/// `None` result means "no match" rather than "data missing". `.unwrap_or` on
/// these is the idiomatic "optionally strip" pattern, not a value substitution.
fn receiver_is_safe_strip(receiver: &Expr<'_>) -> bool {
    if let ExprKind::MethodCall(segment, _, _, _) = receiver.kind {
        matches!(segment.ident.name.as_str(), "strip_prefix" | "strip_suffix")
    } else {
        false
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

/// Returns true if the call shape matches `<value>.unwrap_or(<arg>)`.
fn is_unwrap_or_method(method_name: &str, arg_count: usize) -> bool {
    method_name == "unwrap_or" && arg_count == 1
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

    use super::{is_production_path, is_unwrap_or_method, should_lint_target_source};

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
    fn unwrap_or_method_is_linted() {
        assert!(is_unwrap_or_method("unwrap_or", 1));
    }

    #[test]
    fn unwrap_or_method_without_argument_is_skipped() {
        assert!(!is_unwrap_or_method("unwrap_or", 0));
    }

    #[test]
    fn unwrap_or_default_method_is_skipped() {
        assert!(!is_unwrap_or_method("unwrap_or_default", 0));
    }
}
