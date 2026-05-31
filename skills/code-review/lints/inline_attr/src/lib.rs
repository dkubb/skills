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

//! Deny `#[inline]` attributes on private functions in punchlist production scope.

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use std::path::Path;

use rustc_hir::{
    ImplItem, ImplItemImplKind, ImplItemKind, Item, ItemKind, Node,
    attrs::InlineAttr as CodegenInlineAttr,
};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_middle::ty;
use rustc_span::{Span, def_id::DefId};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds `#[inline]` on private free functions and private inherent methods.
    ///
    /// ### Why is this bad?
    ///
    /// Private functions already have intra-crate inlining available to the
    /// compiler. The project keeps `#[inline]` only where workspace clippy
    /// policy requires it for public items, or where a benchmark justifies a
    /// site-specific exception.
    ///
    /// ### Known problems
    ///
    /// This lint intentionally does not flag public items or trait-impl
    /// methods, because the workspace `missing_inline_in_public_items` policy
    /// currently requires those `#[inline]` attributes.
    pub INLINE_ATTR,
    Deny,
    "unjustified `#[inline]` on private function"
}

impl<'tcx> LateLintPass<'tcx> for InlineAttr {
    fn check_impl_item(&mut self, cx: &LateContext<'tcx>, impl_item: &'tcx ImplItem<'tcx>) {
        if !should_lint_impl_item(cx, impl_item) {
            return;
        }

        if matches!(impl_item.kind, ImplItemKind::Fn(_, _))
            && is_private_inherent_method(cx, impl_item)
            && has_inline_attr(cx, impl_item.owner_id.def_id.to_def_id())
        {
            emit_inline_attr(cx, impl_item.span);
        }
    }

    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if !should_lint_item(cx, item) {
            return;
        }

        if matches!(item.kind, ItemKind::Fn { .. })
            && has_no_explicit_visibility(cx, item.vis_span)
            && has_inline_attr(cx, item.owner_id.def_id.to_def_id())
        {
            emit_inline_attr(cx, item.span);
        }
    }
}

/// Returns true if the item should be examined by this lint.
fn should_lint_item(cx: &LateContext<'_>, item: &Item<'_>) -> bool {
    is_punchlist_production_path(cx, item.span)
        && should_lint_target_source(cx.sess().is_test_crate())
}

/// Returns true if the impl item should be examined by this lint.
fn should_lint_impl_item(cx: &LateContext<'_>, impl_item: &ImplItem<'_>) -> bool {
    is_punchlist_production_path(cx, impl_item.span)
        && should_lint_target_source(cx.sess().is_test_crate())
}

/// Returns true if the current compile target should be linted (always true
/// for this lint — it fires on production source regardless of test mode).
const fn should_lint_target_source(_is_test_crate: bool) -> bool {
    true
}

/// Returns true if the function with `def_id` carries any `#[inline]`
/// attribute (`#[inline]`, `#[inline(always)]`, `#[inline(never)]`).
fn has_inline_attr(cx: &LateContext<'_>, def_id: DefId) -> bool {
    is_inline_attr(cx.tcx.codegen_fn_attrs(def_id).inline)
}

/// Returns true if the parsed inline attribute is anything other than
/// `InlineAttr::None`.
fn is_inline_attr(inline_attr: CodegenInlineAttr) -> bool {
    inline_attr != CodegenInlineAttr::None
}

/// Returns true if `impl_item` is an inherent (non-trait) method on a
/// privately-visible local ADT, with no explicit `pub` visibility.
fn is_private_inherent_method(cx: &LateContext<'_>, impl_item: &ImplItem<'_>) -> bool {
    let ImplItemImplKind::Inherent { vis_span } = impl_item.impl_kind else {
        return false;
    };

    has_no_explicit_visibility(cx, vis_span)
        && inherent_impl_self_type_def_id(cx, impl_item)
            .is_some_and(|def_id| is_private_local_adt(cx, def_id))
}

/// Returns the `DefId` of the type that the inherent impl containing
/// `impl_item` is for, or `None` if the type isn't an ADT/foreign type.
fn inherent_impl_self_type_def_id(cx: &LateContext<'_>, impl_item: &ImplItem<'_>) -> Option<DefId> {
    let impl_owner = cx.tcx.hir_get_parent_item(impl_item.hir_id());
    let impl_ty = cx
        .tcx
        .type_of(impl_owner)
        .instantiate_identity()
        .skip_norm_wip();

    match *impl_ty.kind() {
        ty::Adt(def, _) => Some(def.did()),
        ty::Foreign(def_id) => Some(def_id),
        ty::Bool
        | ty::Char
        | ty::Int(_)
        | ty::Uint(_)
        | ty::Float(_)
        | ty::Str
        | ty::Array(..)
        | ty::Pat(..)
        | ty::Slice(_)
        | ty::RawPtr(..)
        | ty::Ref(..)
        | ty::FnDef(..)
        | ty::FnPtr(..)
        | ty::UnsafeBinder(_)
        | ty::Dynamic(..)
        | ty::Closure(..)
        | ty::CoroutineClosure(..)
        | ty::Coroutine(..)
        | ty::CoroutineWitness(..)
        | ty::Never
        | ty::Tuple(_)
        | ty::Alias(..)
        | ty::Param(_)
        | ty::Bound(..)
        | ty::Placeholder(_)
        | ty::Infer(_)
        | ty::Error(_) => None,
    }
}

/// Returns true if `def_id` names a local ADT declared without an explicit
/// `pub` visibility.
fn is_private_local_adt(cx: &LateContext<'_>, def_id: DefId) -> bool {
    let Some(local_def_id) = def_id.as_local() else {
        return false;
    };

    matches!(
        cx.tcx.hir_node_by_def_id(local_def_id),
        Node::Item(item) if item.is_adt() && has_no_explicit_visibility(cx, item.vis_span)
    )
}

/// Returns true if the source snippet at `vis_span` does not start with `pub`.
fn has_no_explicit_visibility(cx: &LateContext<'_>, vis_span: Span) -> bool {
    cx.sess()
        .source_map()
        .span_to_snippet(vis_span)
        .is_ok_and(|snippet| has_no_explicit_pub_prefix(snippet.as_str()))
}

/// Returns true if the trimmed visibility snippet does not start with `pub`.
fn has_no_explicit_pub_prefix(visibility_snippet: &str) -> bool {
    !visibility_snippet.trim_start().starts_with("pub")
}

/// Emits the `INLINE_ATTR` diagnostic at `span`.
fn emit_inline_attr(cx: &LateContext<'_>, span: Span) {
    cx.emit_span_lint(
        INLINE_ATTR,
        span,
        rustc_errors::DiagDecorator(|diag| {
            diag.primary_message("unjustified `#[inline]` on private function");
            diag.help("remove `#[inline]`, or use `#[expect(inline_attr, reason = \"...\")]` with benchmark evidence");
        }),
    );
}

/// Returns true if the span originates from a punchlist production source
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

    use rustc_hir::attrs::InlineAttr;
    use rustc_span::sym;

    use super::{
        has_no_explicit_pub_prefix, is_inline_attr, is_production_path, should_lint_target_source,
    };

    /// Returns true if the symbol name matches `inline`.
    fn is_inline_attr_name(name: rustc_span::Symbol) -> bool {
        name == sym::inline
    }

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
    fn normal_target_source_is_linted() {
        assert!(should_lint_target_source(false));
    }

    #[test]
    fn harness_source_is_linted() {
        assert!(should_lint_target_source(true));
    }

    #[test]
    fn inline_attribute_name_is_linted() {
        assert!(is_inline_attr_name(sym::inline));
    }

    #[test]
    fn cold_attribute_name_is_skipped() {
        assert!(!is_inline_attr_name(sym::cold));
    }

    #[test]
    fn parsed_inline_attribute_is_linted() {
        assert!(is_inline_attr(InlineAttr::Hint));
    }

    #[test]
    fn missing_inline_attribute_is_skipped() {
        assert!(!is_inline_attr(InlineAttr::None));
    }

    #[test]
    fn empty_visibility_snippet_is_private_syntax() {
        assert!(has_no_explicit_pub_prefix(""));
    }

    #[test]
    fn inherited_visibility_snippet_is_private_syntax() {
        assert!(has_no_explicit_pub_prefix("fn helper() {}"));
    }

    #[test]
    fn public_visibility_snippet_is_skipped() {
        assert!(!has_no_explicit_pub_prefix("pub(crate) fn helper() {}"));
    }
}
