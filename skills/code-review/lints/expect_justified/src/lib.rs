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

//! Deny unjustified `.expect()` calls in punchlist production scope.

extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use std::fs;
use std::path::Path;

use rustc_hir::{Expr, ExprKind, Node};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_middle::ty;
use rustc_span::{Span, SyntaxContext, sym};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds `.expect()` calls without an immediately preceding invariant comment.
    ///
    /// ### Why is this bad?
    ///
    /// Runtime `.expect()` panics should explain the proof that makes the panic unreachable.
    ///
    /// ### Known problems
    ///
    /// This lint intentionally inspects source comments, so formatting affects whether a call is
    /// treated as justified.
    pub EXPECT_JUSTIFIED,
    Deny,
    "unjustified `.expect()` call in punchlist production scope"
}

impl<'tcx> LateLintPass<'tcx> for ExpectJustified {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion()
            || expr.span.ctxt() != SyntaxContext::root()
            || !should_lint_expr(cx, expr)
        {
            return;
        }

        let ExprKind::MethodCall(segment, receiver, args, _method_span) = expr.kind else {
            return;
        };

        if segment.ident.name.as_str() != "expect"
            || args.len() != 1
            || !is_option_or_result_receiver(cx, receiver)
        {
            return;
        }

        // A non-trivial expect message is itself the justification. The rule's
        // intent is "explain the proof that makes the panic unreachable" —
        // `expect("usize should fit into u64")` does explain it, even without
        // a separate `// reason:` comment.
        if let Some(message_arg) = args.first()
            && message_is_self_justifying(message_arg)
        {
            return;
        }

        let Some(method_line) = source_expect_method_line(cx, expr.span) else {
            return;
        };

        let boundary_span = expectation_boundary_span(cx, expr);
        if has_expect_justification(cx, expr.span, method_line, boundary_span) {
            return;
        }

        cx.emit_span_lint(
            EXPECT_JUSTIFIED,
            expr.span,
            rustc_errors::DiagDecorator(|diag| {
                diag.primary_message("unjustified `.expect()` call");
                diag.help(
                    "add an immediately preceding `// SAFETY:`, `// INVARIANT:`, \
                     `// reason:`, or `// PROOF:` comment explaining why the panic is impossible",
                );
            }),
        );
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

/// Returns true if the expect message itself explains the proof that the
/// panic is unreachable. A non-empty string literal counts; any dynamic
/// expression (`format!`, variable, function call) also counts because the
/// developer is composing a deliberate diagnostic. Empty or whitespace-only
/// string literals do not count.
fn message_is_self_justifying(arg: &Expr<'_>) -> bool {
    match arg.kind {
        ExprKind::Lit(ref lit) => match lit.node {
            rustc_ast::LitKind::Str(symbol, _) => !symbol.as_str().trim().is_empty(),
            _ => true,
        },
        _ => true,
    }
}

/// Returns true if the receiver expression is typed as `Option<T>` or
/// `Result<T, E>` (the only types whose `.expect()` we lint).
fn is_option_or_result_receiver(cx: &LateContext<'_>, receiver: &Expr<'_>) -> bool {
    let receiver_type = cx.typeck_results().expr_ty(receiver).peel_refs();
    matches!(
        receiver_type.kind(),
        ty::Adt(adt_def, _)
            if cx.tcx.is_diagnostic_item(sym::Option, adt_def.did())
                || cx.tcx.is_diagnostic_item(sym::Result, adt_def.did())
    )
}

/// Returns the enclosing statement span for `expr`, or its own span if no
/// statement parent exists. The returned span gives the "boundary" line
/// from which a justification comment may precede a multi-line expression.
fn expectation_boundary_span(cx: &LateContext<'_>, expr: &Expr<'_>) -> Span {
    for (_hir_id, node) in cx.tcx.hir_parent_iter(expr.hir_id) {
        if let Node::Stmt(stmt) = node {
            return stmt.span;
        }
    }

    expr.span
}

/// Returns the 1-based line number of the `.expect(` token within the
/// snippet of the full method-chain expression, accounting for multi-line
/// receivers.
fn source_expect_method_line(cx: &LateContext<'_>, expr_span: Span) -> Option<usize> {
    let start_line = cx.sess().source_map().lookup_char_pos(expr_span.lo()).line;
    let snippet = cx.sess().source_map().span_to_snippet(expr_span).ok()?;

    expect_method_line_from_snippet(&snippet, start_line)
}

/// Computes the 1-based line of the `.expect(` segment within `snippet`,
/// given that the snippet starts at `start_line` in the original file.
fn expect_method_line_from_snippet(snippet: &str, start_line: usize) -> Option<usize> {
    let expect_offset = snippet.rfind(".expect(")?;
    let line_offset = snippet
        .get(..expect_offset)?
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count();

    start_line.checked_add(line_offset)
}

/// Returns true if the file containing `expr_span` has a justification
/// comment on the line immediately preceding either the `.expect(` line or
/// the enclosing statement boundary.
fn has_expect_justification(
    cx: &LateContext<'_>,
    expr_span: Span,
    method_line: usize,
    boundary_span: Span,
) -> bool {
    let Some(path) = cx
        .sess()
        .source_map()
        .span_to_filename(expr_span)
        .into_local_path()
    else {
        return false;
    };
    let Ok(source) = fs::read_to_string(path) else {
        return false;
    };

    let boundary_line = cx
        .sess()
        .source_map()
        .lookup_char_pos(boundary_span.lo())
        .line;

    has_source_justification(&source, method_line, boundary_line)
}

/// Returns true if `source` has a justification comment on the line
/// immediately preceding either `method_line` or `boundary_line`.
fn has_source_justification(source: &str, method_line: usize, boundary_line: usize) -> bool {
    preceding_line_has_justification(source, method_line)
        || boundary_line != method_line && preceding_line_has_justification(source, boundary_line)
}

/// Returns true if the line immediately before `one_based_line` is a
/// justification comment.
fn preceding_line_has_justification(source: &str, one_based_line: usize) -> bool {
    let Some(zero_based_index) = one_based_line.checked_sub(2) else {
        return false;
    };

    source
        .lines()
        .nth(zero_based_index)
        .is_some_and(has_justification_comment)
}

/// Returns true if `line` (after leading whitespace) starts with one of the
/// recognized justification prefixes.
fn has_justification_comment(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("// SAFETY:")
        || trimmed.starts_with("// INVARIANT:")
        || trimmed.starts_with("// reason:")
        || trimmed.starts_with("// PROOF:")
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
        expect_method_line_from_snippet, has_justification_comment, has_source_justification,
        is_production_path, should_lint_target_source,
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
    fn safety_comment_is_justification() {
        assert!(has_justification_comment(
            "// SAFETY: fixed literal is non-empty"
        ));
    }

    #[test]
    fn invariant_comment_is_justification() {
        assert!(has_justification_comment(
            "    // INVARIANT: counter starts below the limit"
        ));
    }

    #[test]
    fn reason_comment_is_justification() {
        assert!(has_justification_comment(
            "// reason: test fixture is valid"
        ));
    }

    #[test]
    fn proof_comment_is_justification() {
        assert!(has_justification_comment(
            "// PROOF: constructor already narrowed input"
        ));
    }

    #[test]
    fn lowercase_safety_comment_is_not_justification() {
        assert!(!has_justification_comment(
            "// safety: case-sensitive prefix"
        ));
    }

    #[test]
    fn single_line_expect_method_line_is_snippet_start_line() {
        assert_eq!(
            expect_method_line_from_snippet("value.expect(\"present\")", 8),
            Some(8)
        );
    }

    #[test]
    fn multiline_expect_method_line_tracks_expect_line() {
        let snippet = "value\n    .map(identity)\n    .expect(\"present\")";

        assert_eq!(expect_method_line_from_snippet(snippet, 8), Some(10));
    }

    #[test]
    fn macro_expansion_without_source_expect_has_no_method_line() {
        assert_eq!(
            expect_method_line_from_snippet("assert_eq!(left, right)", 8),
            None
        );
    }

    #[test]
    fn expect_err_is_not_expect_method_line() {
        assert_eq!(
            expect_method_line_from_snippet("value.expect_err(\"error\")", 8),
            None
        );
    }

    #[test]
    fn preceding_method_line_comment_justifies_expect() {
        let source =
            "let value = Some(1)\n    // SAFETY: fixed option is Some\n    .expect(\"some\");\n";

        assert!(has_source_justification(source, 3, 1));
    }

    #[test]
    fn preceding_boundary_line_comment_justifies_expect() {
        let source = "// INVARIANT: the fixture vector is non-empty\nlet value = values.first().expect(\"first\");\n";

        assert!(has_source_justification(source, 2, 2));
    }

    #[test]
    fn missing_preceding_comment_is_unjustified() {
        let source = "let value = values.first().expect(\"first\");\n";

        assert!(!has_source_justification(source, 1, 1));
    }

    #[test]
    fn separated_comment_is_unjustified() {
        let source = "// SAFETY: too far away\n\nlet value = values.first().expect(\"first\");\n";

        assert!(!has_source_justification(source, 3, 3));
    }
}
