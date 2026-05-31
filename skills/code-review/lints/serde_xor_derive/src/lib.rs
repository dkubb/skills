// This crate uses rustc-private APIs because Dylint lints compile against rustc internals.
#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![expect(
    missing_docs,
    reason = "dylint_linting::declare_late_lint! generates internal struct/fn items without docstrings"
)]
#![expect(
    clippy::missing_trait_methods,
    reason = "LateLintPass exposes many default-implemented hook methods; this lint overrides only the item hook it inspects"
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

//! Deny public serde derive asymmetry without an explicit no-serde comment.

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use std::fs::read_to_string;
use std::path::Path;

use rustc_hir::{Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_span::Pos as _;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Finds public structs and enums deriving only one side of serde's
    /// `Serialize` / `Deserialize` pair without an adjacent justification
    /// comment.
    ///
    /// ### Why is this bad?
    ///
    /// Asymmetric serde DTOs are often accidental boundary leaks. A public
    /// serialized type should normally round-trip in both directions; deliberate
    /// write-only or read-only DTOs must say so with `// no-deserialize: ...` or
    /// `// no-serialize: ...`.
    ///
    /// ### Known problems
    ///
    /// This lint reads the source around a public item and is deliberately
    /// scoped to public struct/enum items in non-excluded punchlist surfaces.
    pub SERDE_XOR_DERIVE,
    Deny,
    "public serde derive asymmetry without justification"
}

impl<'tcx> LateLintPass<'tcx> for SerdeXorDerive {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if !is_struct_or_enum(item) || !is_public_item_source(cx, item) || !is_linted_path(cx, item)
        {
            return;
        }

        let Some(source) = source_before_item(cx, item) else {
            return;
        };
        let derive = serde_derives(source.as_str());
        let Some(missing) = derive.missing_side() else {
            return;
        };
        if has_required_justification(source.as_str(), missing) {
            return;
        }

        cx.emit_span_lint(
            SERDE_XOR_DERIVE,
            item.span,
            rustc_errors::DiagDecorator(|diag| {
                diag.primary_message("public serde derive asymmetry without justification");
                diag.help(format!(
                    "derive both Serialize and Deserialize, or add an adjacent `// {missing}: ...` comment"
                ));
            }),
        );
    }
}

/// Serde derive state found near an item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SerdeDerives {
    /// Whether `Deserialize` appears in a derive attribute.
    deserialize: bool,
    /// Whether `Serialize` appears in a derive attribute.
    serialize: bool,
}

impl SerdeDerives {
    /// Returns the required justification token when exactly one side is
    /// derived.
    const fn missing_side(self) -> Option<&'static str> {
        match (self.serialize, self.deserialize) {
            (true, false) => Some("no-deserialize"),
            (false, true) => Some("no-serialize"),
            (true, true) | (false, false) => None,
        }
    }
}

/// Returns true if `item` is a struct or enum.
const fn is_struct_or_enum(item: &Item<'_>) -> bool {
    matches!(item.kind, ItemKind::Struct(..) | ItemKind::Enum(..))
}

/// Returns true if the item source starts with a public struct or enum
/// declaration.
fn is_public_item_source(cx: &LateContext<'_>, item: &Item<'_>) -> bool {
    cx.sess()
        .source_map()
        .span_to_snippet(item.span)
        .is_ok_and(|snippet| is_public_item_snippet(snippet.as_str()))
}

/// Returns true if `snippet` starts with a public struct or enum declaration.
fn is_public_item_snippet(snippet: &str) -> bool {
    let trimmed = snippet.trim_start();
    trimmed.starts_with("pub struct ") || trimmed.starts_with("pub enum ")
}

/// Returns true if `item` is inside the linted punchlist surface.
fn is_linted_path(cx: &LateContext<'_>, item: &Item<'_>) -> bool {
    let Some(path) = cx
        .sess()
        .source_map()
        .span_to_filename(item.span)
        .into_local_path()
    else {
        return false;
    };

    is_included_path(path.as_path())
}

/// Returns true when `path` is inside the linted source surface.
fn is_included_path(path: &Path) -> bool {
    path.file_name()
        .is_none_or(|file_name| file_name != "build.rs")
}

/// Returns the source text immediately preceding and including an item.
fn source_before_item(cx: &LateContext<'_>, item: &Item<'_>) -> Option<String> {
    let source_map = cx.sess().source_map();
    let path = source_map.span_to_filename(item.span).into_local_path()?;
    let source = read_to_string(path).ok()?;
    let start = source_map.lookup_byte_offset(item.span.lo()).pos.to_usize();
    let window_start = start.saturating_sub(4096);
    let item_prefix = source.get(window_start..start)?;
    Some(item_prefix.to_owned())
}

/// Returns the serde derive state in the source window before an item.
fn serde_derives(source: &str) -> SerdeDerives {
    let mut derive = SerdeDerives {
        serialize: false,
        deserialize: false,
    };

    for attribute in derive_attributes(source) {
        derive.serialize |= has_derive_name(attribute, "Serialize");
        derive.deserialize |= has_derive_name(attribute, "Deserialize");
    }

    derive
}

/// Returns all derive attribute payloads in `source`.
fn derive_attributes(source: &str) -> impl Iterator<Item = &str> {
    source
        .lines()
        .rev()
        .take_while(|line| is_item_prefix_line(line))
        .filter_map(derive_attribute_payload)
}

/// Returns true if a reversed line can still belong to an item prefix.
fn is_item_prefix_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.is_empty()
        || trimmed.starts_with("#[")
        || trimmed.starts_with("///")
        || trimmed.starts_with("//")
}

/// Extracts the payload of a single-line `#[derive(...)]` attribute.
fn derive_attribute_payload(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    let payload = trimmed.strip_prefix("#[derive(")?.strip_suffix(")]")?;
    Some(payload)
}

/// Returns true if `payload` contains the derive name.
fn has_derive_name(payload: &str, name: &str) -> bool {
    payload
        .split(',')
        .map(str::trim)
        .any(|segment| segment.rsplit("::").next() == Some(name))
}

/// Returns true if `source` contains the comment required for `missing`.
fn has_required_justification(source: &str, missing: &str) -> bool {
    source
        .lines()
        .rev()
        .take_while(|line| is_item_prefix_line(line))
        .any(|line| line.trim_start().starts_with(&format!("// {missing}:")))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{
        SerdeDerives, has_required_justification, is_included_path, is_public_item_snippet,
        serde_derives,
    };

    #[test]
    fn paired_serde_derives_are_balanced() {
        let source = "#[derive(Debug, Deserialize, Serialize)]\n";

        assert_eq!(
            serde_derives(source),
            SerdeDerives {
                deserialize: true,
                serialize: true,
            }
        );
        assert_eq!(serde_derives(source).missing_side(), None);
    }

    #[test]
    fn serialize_only_requires_no_deserialize_comment() {
        let source = "#[derive(Debug, Serialize)]\n";

        assert_eq!(serde_derives(source).missing_side(), Some("no-deserialize"));
        assert!(!has_required_justification(source, "no-deserialize"));
    }

    #[test]
    fn deserialize_only_requires_no_serialize_comment() {
        let source = "#[derive(Debug, serde::Deserialize)]\n";

        assert_eq!(serde_derives(source).missing_side(), Some("no-serialize"));
        assert!(!has_required_justification(source, "no-serialize"));
    }

    #[test]
    fn adjacent_no_serialize_comment_justifies_deserialize_only() {
        let source = "// no-serialize: external provider response DTO\n#[derive(Deserialize)]\n";

        assert!(has_required_justification(source, "no-serialize"));
    }

    #[test]
    fn adjacent_no_deserialize_comment_justifies_serialize_only() {
        let source = "// no-deserialize: outbound recording payload\n#[derive(Serialize)]\n";

        assert!(has_required_justification(source, "no-deserialize"));
    }

    #[test]
    fn formerly_excluded_paths_are_linted() {
        assert!(is_included_path(Path::new(
            "crates/symbiote-tool-bash/src/adapter.rs"
        )));
        assert!(is_included_path(Path::new(
            "crates/symbiote-adapter-interface/src/lib.rs"
        )));
        assert!(is_included_path(Path::new(
            "crates/symbiote-core/src/parent_invocation.rs"
        )));
        assert!(is_included_path(Path::new(
            "crates/symbiote-persistence/src/boundary_event.rs"
        )));
    }

    #[test]
    fn ordinary_paths_are_linted() {
        assert!(is_included_path(Path::new(
            "crates/symbiote-adapters/src/recording.rs"
        )));
    }

    #[test]
    fn public_struct_and_enum_snippets_are_linted() {
        assert!(is_public_item_snippet("pub struct Public;"));
        assert!(is_public_item_snippet("pub enum Public {}"));
        assert!(!is_public_item_snippet("pub(crate) struct CrateVisible;"));
        assert!(!is_public_item_snippet("struct Private;"));
    }
}
