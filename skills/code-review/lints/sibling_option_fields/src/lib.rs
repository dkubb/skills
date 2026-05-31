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

//! Flag structs whose field list contains two or more sibling
//! `Option<T>` fields — or a discriminator field plus an optional key.
//! Both shapes usually encode a closed sum that should be an `enum`.

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use std::path::Path;

use rustc_hir::{Item, ItemKind, VariantData};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_span::Span;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Flags `struct` definitions whose field list contains either two or
    /// more fields whose source-level type is `Option<...>` (including
    /// `core::option::Option<...>` and `std::option::Option<...>`), or a
    /// discriminator field plus an optional key field such as
    /// `(port_name, parent_intent_id: Option<_>)`.
    ///
    /// ### Why is this bad?
    ///
    /// Two `Option<T>` sibling fields are `2^2 = 4` representable
    /// permutations, but only a closed subset of those is usually valid.
    /// A discriminator-plus-optional-key pair has the same problem: the
    /// discriminator claims a variant while the optional key may be missing
    /// or may name a key from the wrong aggregate. Per
    /// [state-space-minimization], the right encoding is a closed `enum`
    /// whose variants name the valid combinations, leaving the invalid
    /// combinations unrepresentable by construction.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// struct TokenShape {
    ///     access_token: Option<String>,
    ///     account_id: Option<String>,
    /// }
    /// ```
    ///
    /// Encodes four combinations, but only `(Some, Some)` is ever valid.
    /// Replace with:
    ///
    /// ```rust,ignore
    /// enum TokenShape {
    ///     Codex { access_token: String, account_id: String },
    /// }
    /// ```
    ///
    /// ### Known problems
    ///
    /// The check operates on the source-level type snippet, not the
    /// resolved HIR type. Aliases that hide `Option` behind a typedef
    /// will not be flagged. Wire DTOs that intentionally mirror an
    /// external sibling-`Option` shape SHOULD suppress this lint with
    /// `#[expect(sibling_option_fields, reason = "...")]` on the struct,
    /// with an inline comment explaining why the `TryFrom` boundary
    /// converts the raw DTO into a closed enum before any caller sees
    /// the multi-Option permutations.
    pub SIBLING_OPTION_FIELDS,
    // Calibrated 2026-05-10: 7 production-path hits documented in
    // PUNCHLIST.md §5.29. Kept at Allow because the per-site suppression
    // pattern requires a cfg_attr(allow(unknown_lints, ...)) umbrella
    // that conflicts with the workspace clippy::allow_attributes deny.
    // The lint runs at Allow as an on-demand checklist (flip to Warn
    // locally to inspect inventory) and serves as the regression
    // prevention target once each cited site has migrated to a closed
    // enum.
    Allow,
    "struct with sibling optional state should be a closed enum"
}

impl<'tcx> LateLintPass<'tcx> for SiblingOptionFields {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        let ItemKind::Struct(_ident, _generics, variant_data) = item.kind else {
            return;
        };
        if !is_punchlist_production_span(cx, item.span) {
            return;
        }
        let option_count = count_option_fields(cx, &variant_data);
        let has_discriminator_optional_key = has_discriminator_plus_optional_key(cx, &variant_data);
        if option_count >= 2 {
            emit_sibling_options(cx, item.span, option_count);
            return;
        }
        if has_discriminator_optional_key {
            emit_discriminator_optional_key(cx, item.span);
        }
    }
}

/// Counts struct fields whose type-source snippet is an `Option<...>`
/// (or a qualified path that resolves to `core::option::Option<...>`
/// / `std::option::Option<...>`).
fn count_option_fields(cx: &LateContext<'_>, variant_data: &VariantData<'_>) -> usize {
    let VariantData::Struct { fields, .. } = *variant_data else {
        return 0;
    };
    fields
        .iter()
        .filter(|field| field_ty_is_option(cx, field.ty.span))
        .count()
}

/// Returns true when a struct carries a discriminator field plus an
/// optional key field such as `(port_name, parent_intent_id:
/// Option<_>)`.
fn has_discriminator_plus_optional_key(
    cx: &LateContext<'_>,
    variant_data: &VariantData<'_>,
) -> bool {
    let VariantData::Struct { fields, .. } = *variant_data else {
        return false;
    };
    let has_optional_key = fields.iter().any(|field| {
        field_name_is_optional_key(field.ident.as_str()) && field_ty_is_option(cx, field.ty.span)
    });
    let has_discriminator = fields.iter().any(|field| {
        field_name_is_discriminator(field.ident.as_str())
            && field_ty_is_discriminator(cx, field.ty.span)
    });
    has_optional_key && has_discriminator
}

/// Returns true if the type-source snippet at `span` is one of the
/// recognised `Option<...>` forms.
fn field_ty_is_option(cx: &LateContext<'_>, span: Span) -> bool {
    if span.from_expansion() {
        return false;
    }
    cx.sess()
        .source_map()
        .span_to_snippet(span)
        .is_ok_and(|snippet| snippet_is_option(&snippet))
}

/// Returns true if the source snippet at `span` looks like one of the
/// local stringly discriminator value types.
fn field_ty_is_discriminator(cx: &LateContext<'_>, span: Span) -> bool {
    if span.from_expansion() {
        return false;
    }
    cx.sess()
        .source_map()
        .span_to_snippet(span)
        .is_ok_and(|snippet| snippet_is_discriminator_type(&snippet))
}

/// Returns true if the trimmed snippet starts with `Option<`,
/// `core::option::Option<`, or `std::option::Option<`.
fn snippet_is_option(snippet: &str) -> bool {
    let trimmed = snippet.trim_start();
    trimmed.starts_with("Option<")
        || trimmed.starts_with("core::option::Option<")
        || trimmed.starts_with("std::option::Option<")
}

/// Returns true for source snippets used as stringly discriminators in
/// persistence and wire DTOs.
fn snippet_is_discriminator_type(snippet: &str) -> bool {
    let trimmed = snippet.trim_start();
    matches!(
        trimmed,
        "PortName" | "String" | "NonEmptyString" | "RawDiscriminator"
    ) || trimmed.ends_with("Name")
        || trimmed.ends_with("Kind")
        || trimmed.ends_with("State")
}

/// Returns true when a field name is commonly used as a closed-sum
/// discriminator.
fn field_name_is_discriminator(field_name: &str) -> bool {
    matches!(field_name, "kind" | "port_name" | "state" | "type_")
        || field_name.ends_with("_kind")
        || field_name.ends_with("_name")
        || field_name.ends_with("_state")
}

/// Returns true when an optional field name is key-shaped and therefore
/// likely tied to a discriminator.
fn field_name_is_optional_key(field_name: &str) -> bool {
    field_name.ends_with("_id") || field_name.ends_with("_key")
}

/// Emits one diagnostic for a struct with 2+ sibling `Option` fields.
fn emit_sibling_options(cx: &LateContext<'_>, span: Span, option_count: usize) {
    cx.emit_span_lint(
        SIBLING_OPTION_FIELDS,
        span,
        rustc_errors::DiagDecorator(move |diag| {
            diag.primary_message(format!(
                "struct has {option_count} sibling `Option<_>` fields"
            ));
            diag.help(
                "convert to a closed `enum` whose variants name the valid \
                 combinations; raw serde DTOs that mirror an external \
                 sibling-Option shape may suppress with \
                 `#[expect(sibling_option_fields, reason = \"...\")]`",
            );
        }),
    );
}

/// Emits one diagnostic for a struct with a discriminator plus an
/// optional key field.
fn emit_discriminator_optional_key(cx: &LateContext<'_>, span: Span) {
    cx.emit_span_lint(
        SIBLING_OPTION_FIELDS,
        span,
        rustc_errors::DiagDecorator(|diag| {
            diag.primary_message("struct has a discriminator plus an optional key field");
            diag.help(
                "convert to a closed `enum` whose variants pair the \
                 discriminator with the required key; raw serde DTOs that \
                 mirror an external product shape may suppress with \
                 `#[expect(sibling_option_fields, reason = \"...\")]`",
            );
        }),
    );
}

/// Returns true if the span originates from a production source path
/// (excludes `tests/` directories and `build.rs`).
fn is_punchlist_production_span(cx: &LateContext<'_>, span: Span) -> bool {
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

/// Returns true if the path is treated as production source for the
/// punchlist gate (excludes `tests/` directories and `build.rs`).
fn is_production_path(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|file_name| file_name != "build.rs")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{
        field_name_is_discriminator, field_name_is_optional_key, is_production_path,
        snippet_is_discriminator_type, snippet_is_option,
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
    fn unqualified_option_snippet_is_option() {
        assert!(snippet_is_option("Option<String>"));
    }

    #[test]
    fn core_option_path_snippet_is_option() {
        assert!(snippet_is_option("core::option::Option<u32>"));
    }

    #[test]
    fn std_option_path_snippet_is_option() {
        assert!(snippet_is_option("std::option::Option<u32>"));
    }

    #[test]
    fn leading_whitespace_is_trimmed_before_option_check() {
        assert!(snippet_is_option("    Option<String>"));
    }

    #[test]
    fn unrelated_type_is_not_option() {
        assert!(!snippet_is_option("Vec<String>"));
    }

    #[test]
    fn empty_snippet_is_not_option() {
        assert!(!snippet_is_option(""));
    }

    #[test]
    fn option_substring_inside_other_type_is_not_option() {
        assert!(!snippet_is_option("MyOption<String>"));
    }

    #[test]
    fn port_name_is_discriminator_name() {
        assert!(field_name_is_discriminator("port_name"));
    }

    #[test]
    fn state_suffix_is_discriminator_name() {
        assert!(field_name_is_discriminator("outbound_state"));
    }

    #[test]
    fn unrelated_field_is_not_discriminator_name() {
        assert!(!field_name_is_discriminator("rendered_bytes"));
    }

    #[test]
    fn parent_intent_id_is_optional_key_name() {
        assert!(field_name_is_optional_key("parent_intent_id"));
    }

    #[test]
    fn rendered_bytes_is_not_optional_key_name() {
        assert!(!field_name_is_optional_key("rendered_bytes"));
    }

    #[test]
    fn port_name_type_is_discriminator_type() {
        assert!(snippet_is_discriminator_type("PortName"));
    }

    #[test]
    fn name_suffix_type_is_discriminator_type() {
        assert!(snippet_is_discriminator_type("OutboundState"));
    }

    #[test]
    fn optional_key_type_is_not_discriminator_type() {
        assert!(!snippet_is_discriminator_type("Option<PositiveBigInt>"));
    }
}
