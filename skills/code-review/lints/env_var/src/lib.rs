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

//! Deny ambient environment reads outside harness startup.

extern crate rustc_errors;
extern crate rustc_hir;

use std::path::Path;

use rustc_hir::{Expr, ExprKind, ItemKind, Node, Path as HirPath, QPath};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds calls to `env::var`, `std::env::var`, `env::vars`, and
    /// `std::env::vars` outside the harness startup boundary.
    ///
    /// ### Why is this bad?
    ///
    /// Ambient configuration should be collapsed into explicit typed values at
    /// startup. Adapters and the core should receive frozen configuration
    /// instead of reading process environment during execution.
    ///
    /// ### Known problems
    ///
    /// This lint is syntactic. It mirrors the retired punchlist gate and does
    /// not resolve whether an `env` path names `std::env` or another module.
    pub ENV_VAR,
    Deny,
    "ambient environment read outside harness startup"
}

impl<'tcx> LateLintPass<'tcx> for EnvVar {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        if let ExprKind::Call(callee, args) = expr.kind
            && is_env_read_call(callee, args.len())
        {
            cx.emit_span_lint(
                ENV_VAR,
                expr.span,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("ambient environment read outside harness startup");
                    diag.help(
                        "collapse ambient configuration into typed startup config and pass it in",
                    );
                }),
            );
        }
    }
}

/// Returns true if the expression should be examined by this lint.
fn should_lint_expr(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    is_punchlist_production_path(cx, expr)
        && should_lint_target_source(cx.sess().is_test_crate())
        && !is_allowed_harness_startup_env_read(cx, expr)
}

/// Returns true if the current compile target should be linted.
const fn should_lint_target_source(is_test_crate: bool) -> bool {
    is_test_crate
}

/// Returns true if `callee` is a call expression naming an env-read path
/// (`env::var`, `std::env::var`, `env::vars`, `std::env::vars`) with the
/// expected argument count.
fn is_env_read_call(callee: &Expr<'_>, arg_count: usize) -> bool {
    let ExprKind::Path(qpath) = callee.kind else {
        return false;
    };
    let QPath::Resolved(None, path) = qpath else {
        return false;
    };

    is_env_read_path(path, arg_count)
}

/// Returns true if the resolved HIR path matches an env-read path.
fn is_env_read_path(path: &HirPath<'_>, arg_count: usize) -> bool {
    let segments: Vec<&str> = path
        .segments
        .iter()
        .map(|segment| segment.ident.name.as_str())
        .collect();

    is_env_read_segments(&segments, arg_count)
}

/// Returns true if `segments` and `arg_count` match a known env-read shape.
fn is_env_read_segments(segments: &[&str], arg_count: usize) -> bool {
    matches!(segments, ["env", "var"] | ["std", "env", "var"]) && arg_count == 1
        || matches!(segments, ["env", "vars"] | ["std", "env", "vars"]) && arg_count == 0
}

/// Returns true if the expression is an allowed startup-module env read in
/// the harness entrypoint.
fn is_allowed_harness_startup_env_read(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    is_harness_entrypoint_expr(cx, expr) && is_inside_harness_startup_module(cx, expr)
}

/// Returns true if the expression originates from the harness entrypoint file.
fn is_harness_entrypoint_expr(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let Some(path) = cx
        .sess()
        .source_map()
        .span_to_filename(expr.span)
        .into_local_path()
    else {
        return false;
    };

    is_harness_entrypoint_path(path.as_path())
}

/// Returns true if the path names the harness entrypoint file.
fn is_harness_entrypoint_path(path: &Path) -> bool {
    path.ends_with("crates/symbiote-harness/src/main.rs")
}

/// Returns true if the expression appears inside the documented harness
/// startup module.
fn is_inside_harness_startup_module(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let mut is_inside_startup_module = false;
    for (_hir_id, node) in cx.tcx.hir_parent_iter(expr.hir_id) {
        let Node::Item(item) = node else {
            continue;
        };
        if !matches!(item.kind, ItemKind::Mod(..)) {
            continue;
        }

        let item_name = cx.tcx.item_name(item.owner_id.def_id.to_def_id());
        let module_name = item_name.as_str();
        if is_harness_test_module_name(module_name) {
            return false;
        }
        if is_harness_startup_module_name(module_name) {
            is_inside_startup_module = true;
        }
    }

    is_inside_startup_module
}

/// Returns true if the module name is the documented harness startup boundary.
fn is_harness_startup_module_name(name: &str) -> bool {
    name == "startup"
}

/// Returns true if the module name is an in-file test module.
fn is_harness_test_module_name(name: &str) -> bool {
    name == "tests"
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
        is_env_read_segments, is_harness_startup_module_name, is_harness_test_module_name,
        is_production_path, should_lint_target_source,
    };

    #[test]
    fn production_source_file_is_linted() {
        assert!(is_production_path(Path::new("crates/example/src/lib.rs")));
    }

    #[test]
    fn harness_entrypoint_is_linted() {
        assert!(is_production_path(Path::new(
            "crates/symbiote-harness/src/main.rs"
        )));
    }

    #[test]
    fn absolute_harness_entrypoint_is_linted() {
        assert!(is_production_path(Path::new(
            "/repo/crates/symbiote-harness/src/main.rs"
        )));
    }

    #[test]
    fn harness_startup_module_name_is_allowed() {
        assert!(is_harness_startup_module_name("startup"));
    }

    #[test]
    fn startup_prefixed_function_name_is_not_a_module_boundary() {
        assert!(!is_harness_startup_module_name(
            "startup_model_adapter_from_runtime_config"
        ));
    }

    #[test]
    fn tests_module_name_is_not_a_startup_boundary() {
        assert!(is_harness_test_module_name("tests"));
        assert!(!is_harness_startup_module_name("tests"));
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
    fn env_var_call_is_linted() {
        assert!(is_env_read_segments(&["env", "var"], 1));
    }

    #[test]
    fn std_env_var_call_is_linted() {
        assert!(is_env_read_segments(&["std", "env", "var"], 1));
    }

    #[test]
    fn env_vars_call_is_linted() {
        assert!(is_env_read_segments(&["env", "vars"], 0));
    }

    #[test]
    fn std_env_vars_call_is_linted() {
        assert!(is_env_read_segments(&["std", "env", "vars"], 0));
    }

    #[test]
    fn env_var_call_with_missing_argument_is_skipped() {
        assert!(!is_env_read_segments(&["env", "var"], 0));
    }

    #[test]
    fn env_vars_call_with_argument_is_skipped() {
        assert!(!is_env_read_segments(&["env", "vars"], 1));
    }

    #[test]
    fn env_var_os_call_is_skipped() {
        assert!(!is_env_read_segments(&["env", "var_os"], 1));
    }

    #[test]
    fn other_var_call_is_skipped() {
        assert!(!is_env_read_segments(&["config", "var"], 1));
    }
}
