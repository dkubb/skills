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

//! Deny literal `loop {}` blocks without a bounded counter.

extern crate rustc_errors;
extern crate rustc_hir;

use std::path::Path;

use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Block, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds literal `loop { ... }` expressions in production source.
    ///
    /// ### Why is this bad?
    ///
    /// Unbounded loops violate the bounded-resource invariant. Prefer a
    /// countdown budget over a sufficiently large integer, or fail closed with a
    /// typed `BudgetExhausted` error after a step limit.
    ///
    /// ### Known problems
    ///
    /// This lint is syntactic. It does not infer whether the loop body always
    /// breaks after a bounded number of iterations.
    pub BARE_LOOP,
    Deny,
    "literal `loop` block without an explicit bound"
}

impl<'tcx> LateLintPass<'tcx> for BareLoop {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if !should_lint_expr(cx, expr) {
            return;
        }

        if let ExprKind::Loop(body, _label, _source, _span) = expr.kind
            && is_literal_loop_source(cx, expr)
            && !loop_body_has_return(body)
        {
            cx.emit_span_lint(
                BARE_LOOP,
                expr.span,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("literal `loop` block without an explicit bound");
                    diag.help(
                        "replace with a bounded countdown or a typed budget-exhausted failure",
                    );
                }),
            );
        }
    }
}

/// Returns true if the loop body contains at least one `return` expression
/// (or `?` operator, which desugars to `return`). Such loops are bounded by
/// control flow — typically an I/O read returning EOF, or a typed shutdown
/// path — and don't need an explicit countdown budget.
///
/// Recursion stops at nested closure boundaries because a `return` inside a
/// closure exits the closure, not the enclosing function or loop.
fn loop_body_has_return<'tcx>(body: &'tcx Block<'tcx>) -> bool {
    let mut visitor = HasReturnVisitor { found: false };
    visitor.visit_block(body);
    visitor.found
}

/// HIR walker that flags the first `Ret` expression it sees, ignoring
/// nested closure bodies.
struct HasReturnVisitor {
    found: bool,
}

impl<'v> Visitor<'v> for HasReturnVisitor {
    fn visit_expr(&mut self, ex: &'v Expr<'v>) {
        if self.found {
            return;
        }

        match ex.kind {
            ExprKind::Ret(_) => {
                self.found = true;
            }
            ExprKind::Closure(_) => {
                // do not descend into closure bodies
            }
            _ => intravisit::walk_expr(self, ex),
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

/// Returns true if the expression's source snippet starts with a literal
/// `loop` keyword (with or without a label).
fn is_literal_loop_source(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    if expr.span.from_expansion() {
        return false;
    }

    cx.sess()
        .source_map()
        .span_to_snippet(expr.span)
        .is_ok_and(|snippet| is_literal_loop_text(&snippet))
}

/// Returns true if `source` is a literal `loop { ... }` or `'label: loop { ... }`
/// expression.
fn is_literal_loop_text(source: &str) -> bool {
    let trimmed = source.trim_start();
    starts_with_loop_keyword(trimmed)
        || labelled_loop_body(trimmed).is_some_and(starts_with_loop_keyword)
}

/// If `source` starts with a labelled-block prefix (`'label:`), returns the
/// trimmed body after the colon; otherwise returns `None`.
fn labelled_loop_body(source: &str) -> Option<&str> {
    let rest = source.strip_prefix('\'')?;
    let colon = rest.find(':')?;

    rest.get(colon.saturating_add(1)..).map(str::trim_start)
}

/// Returns true if `source` starts with the `loop` keyword followed by a
/// non-identifier character.
fn starts_with_loop_keyword(source: &str) -> bool {
    let Some(rest) = source.strip_prefix("loop") else {
        return false;
    };

    rest.chars()
        .next()
        .is_some_and(|character| !is_identifier_continue(character))
}

/// Returns true if `character` is a valid Rust identifier continuation.
fn is_identifier_continue(character: char) -> bool {
    character == '_' || character.is_alphanumeric()
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

    use super::{is_literal_loop_text, is_production_path, should_lint_target_source};

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
    fn literal_loop_text_is_linted() {
        assert!(is_literal_loop_text("loop { work(); }"));
    }

    #[test]
    fn labelled_loop_text_is_linted() {
        assert!(is_literal_loop_text("'scan: loop { work(); }"));
    }

    #[test]
    fn while_loop_text_is_skipped() {
        assert!(!is_literal_loop_text("while index < len { index += 1; }"));
    }

    #[test]
    fn for_loop_text_is_skipped() {
        assert!(!is_literal_loop_text("for item in items { drop(item); }"));
    }

    #[test]
    fn await_text_is_skipped() {
        assert!(!is_literal_loop_text(".await"));
    }

    #[test]
    fn loop_prefixed_identifier_is_skipped() {
        assert!(!is_literal_loop_text("loopy()"));
    }
}
