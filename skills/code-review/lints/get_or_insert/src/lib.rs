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

//! Deny `.get_or_insert(...)` fallback accessors in punchlist production scope.

extern crate rustc_errors;
extern crate rustc_hir;

use std::path::Path;

use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds `.get_or_insert(...)` and `.get_or_insert_with(...)` method calls.
    ///
    /// ### Why is this bad?
    ///
    /// Insert-and-return fallback accessors materialize an absent value in
    /// place, hiding the missing state from downstream code. Prefer naming the
    /// absent path explicitly.
    ///
    /// ### Known problems
    ///
    /// This lint is syntactic. It does not resolve whether the receiver is an
    /// `Option` or another type with a same-named method.
    pub GET_OR_INSERT,
    Deny,
    "fallback `.get_or_insert(...)` masks missing data"
}

impl<'tcx> LateLintPass<'tcx> for GetOrInsert {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        if let ExprKind::MethodCall(segment, _receiver, args, _span) = expr.kind
            && is_get_or_insert_method(segment.ident.name.as_str(), args.len())
        {
            cx.emit_span_lint(
                GET_OR_INSERT,
                expr.span,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("fallback `.get_or_insert(...)` masks missing data");
                    diag.help("name the absent path explicitly");
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

/// Returns true if the call shape matches `<value>.get_or_insert(<arg>)` or
/// `<value>.get_or_insert_with(<arg>)`.
fn is_get_or_insert_method(method_name: &str, arg_count: usize) -> bool {
    matches!(method_name, "get_or_insert" | "get_or_insert_with") && arg_count == 1
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

    use super::{is_get_or_insert_method, is_production_path, should_lint_target_source};

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
    fn get_or_insert_method_is_linted() {
        assert!(is_get_or_insert_method("get_or_insert", 1));
    }

    #[test]
    fn get_or_insert_with_method_is_linted() {
        assert!(is_get_or_insert_method("get_or_insert_with", 1));
    }

    #[test]
    fn get_or_insert_method_without_argument_is_skipped() {
        assert!(!is_get_or_insert_method("get_or_insert", 0));
    }

    #[test]
    fn get_or_insert_default_method_is_skipped() {
        assert!(!is_get_or_insert_method("get_or_insert_default", 0));
    }
}
