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
    clippy::arithmetic_side_effects,
    reason = "byte-index arithmetic over return-type snippet stays bounded by the snippet length checked at each loop iteration"
)]

//! Deny `Result<_, String>` function error channels in punchlist production scope.

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use std::path::Path;

use rustc_hir::{Body, FnDecl, FnRetTy, intravisit::FnKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_span::{Span, def_id::LocalDefId};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds function signatures that return `Result<_, String>`.
    ///
    /// ### Why is this bad?
    ///
    /// String error channels erase the machine-actionable failure variant. Prefer a typed
    /// domain-specific error enum with stable variants and a `Display` implementation for logs.
    ///
    /// ### Known problems
    ///
    /// This lint is syntactic and intentionally mirrors the punchlist gate. It does not inspect
    /// type aliases.
    pub RESULT_STRING_ERROR,
    Deny,
    "function returns stringly typed `Result<_, String>`"
}

impl<'tcx> LateLintPass<'tcx> for ResultStringError {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        _body: &'tcx Body<'tcx>,
        span: Span,
        _id: LocalDefId,
    ) {
        if matches!(kind, FnKind::Closure) || !should_lint_function(cx, span) {
            return;
        }

        let FnRetTy::Return(return_type) = decl.output else {
            return;
        };
        let Ok(snippet) = cx.sess().source_map().span_to_snippet(return_type.span) else {
            return;
        };

        if is_result_string_return_type(&snippet) {
            cx.emit_span_lint(
                RESULT_STRING_ERROR,
                return_type.span,
                rustc_errors::DiagDecorator(|diag| {
                    diag.primary_message("function returns `Result<_, String>`");
                    diag.help("replace the string error channel with a typed error enum");
                }),
            );
        }
    }
}

/// Returns true if the function should be examined by this lint.
fn should_lint_function(cx: &LateContext<'_>, span: Span) -> bool {
    is_punchlist_production_path(cx, span) && should_lint_target_source(cx.sess().is_test_crate())
}

/// Returns true if the current compile target should be linted.
const fn should_lint_target_source(is_test_crate: bool) -> bool {
    is_test_crate
}

/// Returns true if the return-type snippet names a `Result<_, String>`
/// channel.
fn is_result_string_return_type(return_type: &str) -> bool {
    let normalized = normalize_type_text(return_type);
    let Some(arguments) = result_generic_arguments(&normalized) else {
        return false;
    };
    let Some((_ok_type, error_type)) = split_result_arguments(arguments) else {
        return false;
    };

    is_string_type(error_type)
}

/// Strips all whitespace from `source` so the type-shape match can compare
/// canonical text without formatter sensitivity.
fn normalize_type_text(source: &str) -> String {
    source.chars().filter(|ch| !ch.is_whitespace()).collect()
}

/// Returns the substring inside the outermost `Result<...>` generics, or
/// `None` if `return_type` does not name a `Result` at the outer position.
fn result_generic_arguments(return_type: &str) -> Option<&str> {
    let result_start = return_type.rfind("Result<")?;
    if !is_result_path_boundary(return_type, result_start) {
        return None;
    }
    let args_start = result_start + "Result<".len();
    let args_end = matching_angle_bracket(return_type, args_start)?;

    return_type.get(args_start..args_end)
}

/// Returns true if `result_start` sits at a path boundary (start of string or
/// preceded by `::`), so we don't match e.g. `MyResult<...>`.
fn is_result_path_boundary(return_type: &str, result_start: usize) -> bool {
    result_start == 0
        || return_type
            .get(..result_start)
            .is_some_and(|prefix| prefix.ends_with("::"))
}

/// Returns the byte index of the `>` that closes the generics opened at
/// `args_start`, or `None` if the source has no matching close.
fn matching_angle_bracket(source: &str, args_start: usize) -> Option<usize> {
    let mut depth = 1_usize;
    for (offset, byte) in source
        .as_bytes()
        .iter()
        .copied()
        .enumerate()
        .skip(args_start)
    {
        match byte {
            b'<' => depth = depth.saturating_add(1),
            b'>' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(offset);
                }
            }
            _ => {}
        }
    }

    None
}

/// Splits comma-separated `Result<T, E>` arguments at the outer-level comma,
/// returning `(ok_type, error_type)` substrings or `None` for malformed input.
fn split_result_arguments(arguments: &str) -> Option<(&str, &str)> {
    let mut depth = 0_usize;
    for (index, byte) in arguments.as_bytes().iter().copied().enumerate() {
        match byte {
            b'<' => depth = depth.saturating_add(1),
            b'>' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => {
                let ok_type = arguments.get(..index)?;
                let error_type = arguments.get(index.checked_add(1)?..)?;
                if ok_type.is_empty() || error_type.is_empty() {
                    return None;
                }
                return Some((ok_type, error_type));
            }
            _ => {}
        }
    }

    None
}

/// Returns true if `error_type` names one of the canonical `String` types.
fn is_string_type(error_type: &str) -> bool {
    matches!(
        error_type,
        "String" | "std::string::String" | "alloc::string::String"
    )
}

/// Returns true if `span` originates from a punchlist production source
/// path (excludes `tests/` directories and `build.rs`).
fn is_punchlist_production_path(cx: &LateContext<'_>, span: Span) -> bool {
    let Some(path) = cx
        .sess()
        .source_map()
        .span_to_filename(span)
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

    use super::{is_production_path, is_result_string_return_type, should_lint_target_source};

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
    fn result_string_return_type_is_linted() {
        assert!(is_result_string_return_type("Result<User, String>"));
    }

    #[test]
    fn async_result_string_return_type_is_linted() {
        assert!(is_result_string_return_type("Result<Option<User>, String>"));
    }

    #[test]
    fn qualified_result_string_return_type_is_linted() {
        assert!(is_result_string_return_type(
            "std::result::Result<User, std::string::String>"
        ));
    }

    #[test]
    fn result_typed_error_return_type_is_skipped() {
        assert!(!is_result_string_return_type(
            "Result<User, UserLookupError>"
        ));
    }

    #[test]
    fn non_result_return_type_is_skipped() {
        assert!(!is_result_string_return_type("Option<String>"));
    }

    #[test]
    fn one_argument_result_alias_is_skipped() {
        assert!(!is_result_string_return_type("Result<User>"));
    }

    #[test]
    fn nested_string_inside_ok_type_is_skipped() {
        assert!(!is_result_string_return_type(
            "Result<Vec<String>, UserLookupError>"
        ));
    }

    #[test]
    fn wrapped_result_string_return_type_is_skipped() {
        assert!(!is_result_string_return_type(
            "Option<Result<User, String>>"
        ));
    }

    #[test]
    fn nested_result_string_ok_type_is_skipped() {
        assert!(!is_result_string_return_type(
            "Result<Result<User, String>, UserLookupError>"
        ));
    }
}
