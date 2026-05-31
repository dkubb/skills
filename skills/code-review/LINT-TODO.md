# LINT-TODO

> **Model: generic lint suite.** The lints bundled under `lints/` are the
> *generic* Rust lints that all of the user's Rust projects adhere to. They
> must be able to fire on any Rust project, so nothing here may couple to a
> specific project's types, paths, or ledgers. Project-specific dylints live
> in each project's own tree, not here.
>
> For this reason, five formerly-bundled lints that were hard-coupled to one
> project's types and issue ledgers (`issue24_tool_call_name_proof`,
> `issue33_expect_valid`, `unbounded_boundary_event_accumulator`,
> `negative_space_assertion_candidate`, `source_ratchet_contains`) were
> pruned from this suite. The backlog below is likewise restricted to
> generic lints.

Rules from `references/languages/rust.md` and `references/allow-vs-expect.md`
that should move from manual review to automated linting. Target tool:
**[dylint](https://github.com/trailofbits/dylint)** for project-local custom
lints. Where stock clippy already covers a rule, configure via `clippy.toml`
or `[lints.clippy]` in `Cargo.toml` rather than reimplement.

## Implementation primer

- The suite is a Cargo workspace of **one cdylib crate per lint**. Each lint
  lives in its own crate directory `lints/<lint_name>/`, listed in the
  `[workspace] members` array of `lints/Cargo.toml`, and registers its pass
  with `dylint_linting::declare_late_lint!` (or
  `declare_pre_expansion_lint!`). There is no single shared `dylint_library!`
  library; every crate compiles to its own `cdylib` that Dylint discovers by
  the `lib<name>@<toolchain>.dylib` naming convention in the release output.
- Most rules below are `LateLintPass` (HIR access, type information). The
  one early-pass case is `expect_not_allow` (attribute matching only).
- The lint toolchain is pinned by `lints/run-dylint` (`DYLINT_TOOLCHAIN`) in
  lockstep with the pinned Dylint revision, so the rustc-private APIs
  (`rustc_hir`, `rustc_ast`, `rustc_middle`, `rustc_session`, `rustc_lint`)
  remain stable across lint authors.
- For each lint, ship:
  1. The lint registration and pass impl in `lints/<lint_name>/src/lib.rs`.
  2. UI tests under the crate's `ui/` directory with paired `.stderr`
     snapshots (use `dylint_testing::ui_test_example`).
  3. At least one **positive** fixture (lint fires) and one **negative**
     fixture (lint stays silent).
  4. A `cargo expand`-friendly fixture for any attribute-driven cases.
- Diagnostic style: short title + `help:` suggestion + span on the offending
  item, not the whole file. Match clippy's tone.

## Workspace layout

```text
lints/
├── Cargo.toml                   # [workspace] members = one entry per lint crate
├── clippy.toml                  # disallowed-methods config shared by the lints
├── run-dylint                   # bootstrap + build + run script (pins toolchain)
├── <lint_name>/                 # one cdylib crate per lint
│   ├── Cargo.toml               # crate-type = ["cdylib"], rustc-private deps
│   ├── src/
│   │   └── lib.rs               # declare_late_lint! { ... } + pass impl
│   └── ui/                      # UI fixtures + .stderr snapshots
│       ├── <lint_name>.rs
│       └── <lint_name>.stderr
└── ...                          # remaining lint crates, same shape
```

---

## Phase 1 — Syntactic / attribute, near-zero FP

### `pub_in_crate_shorthand`

**Status:** not yet shipped — open backlog item.

**Source:** `rust.md` "API and ownership".

**Rule.** Prefer `pub(crate)` over `pub(in crate)`.

**Pass.** `EarlyLintPass` (visibility tokens are visible at the AST).

**Detection algorithm.**

```rust
impl EarlyLintPass for PubInCrateShorthand {
    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &ast::Item) {
        if let ast::VisibilityKind::Restricted { path, .. } = &item.vis.kind {
            if path.segments.len() == 1
                && path.segments[0].ident.name == kw::Crate
            {
                span_lint_and_sugg(
                    cx, PUB_IN_CRATE_SHORTHAND, item.vis.span,
                    "prefer `pub(crate)` shorthand",
                    "replace with",
                    "pub(crate)".to_string(),
                    Applicability::MachineApplicable,
                );
            }
        }
    }
}
```

**Positive (lints fire):**

```rust
pub(in crate) fn helper() {}                  // fires
pub(in crate) struct Internal;                // fires
```

**Negative (no fire):**

```rust
pub(crate) fn helper() {}
pub(in crate::foo::bar) fn deep() {}          // genuine module path
```

**Config:** none.

---

### `serde_deny_unknown_fields`

**Status:** not yet shipped. (The bundled `serde_xor_derive` crate is a
different rule — it enforces serde derive symmetry, not
`deny_unknown_fields`.)

**Source:** `rust.md` "Conversions and typing".

**Rule.** Any `#[derive(Deserialize)]` on a struct requires
`#[serde(deny_unknown_fields)]` on the same struct.

**Pass.** `LateLintPass::check_item` over `ItemKind::Struct`.

**Detection algorithm.**

```rust
impl<'tcx> LateLintPass<'tcx> for SerdeDenyUnknownFields {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx hir::Item<'tcx>) {
        let hir::ItemKind::Struct(_, _) = item.kind else { return };

        // Walk derive list on this item.
        let derives_deserialize = cx.tcx.hir().attrs(item.hir_id())
            .iter()
            .filter_map(|attr| attr.meta_item_list())
            .flatten()
            .any(|meta| {
                meta.path().map_or(false, |path| {
                    // path is `serde::Deserialize` or `Deserialize`
                    path.segments.last().map_or(false, |seg|
                        seg.ident.name == Symbol::intern("Deserialize"))
                })
            });
        if !derives_deserialize { return; }

        let has_deny = cx.tcx.hir().attrs(item.hir_id())
            .iter()
            .filter(|a| a.has_name(Symbol::intern("serde")))
            .filter_map(|a| a.meta_item_list())
            .flatten()
            .any(|n| n.has_name(Symbol::intern("deny_unknown_fields")));

        if !has_deny {
            span_lint_and_help(
                cx, SERDE_DENY_UNKNOWN_FIELDS, item.span,
                "`#[derive(Deserialize)]` without `#[serde(deny_unknown_fields)]`",
                None,
                "add `#[serde(deny_unknown_fields)]` to reject extra fields at the boundary",
            );
        }
    }
}
```

**Positive (lints fire):**

```rust
#[derive(Deserialize)]
struct Config { host: String }               // fires

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Other { x: u32 }                      // fires (rename_all is not deny_unknown_fields)
```

**Negative (no fire):**

```rust
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Good { host: String }

#[derive(Serialize)]
struct Only { x: u32 }                       // no Deserialize → no fire
```

**Edge cases.**

- `#[serde(flatten)]` on a field doesn't disable the requirement; the
  outer struct still needs `deny_unknown_fields`. (Note: serde itself
  emits a warning when `flatten` is combined with `deny_unknown_fields`,
  so this case may need a `#[expect]` opt-out per field.)
- `enum` deserialization is out of scope for this lint. The `deny_unknown_fields`
  attribute applies to struct variants individually; consider a follow-up
  lint after this one ships.

**Config:** none required.

---

### `single_variant_pub_enum`

**Status:** not yet shipped — open backlog item.

**Source:** `rust.md` "Structural review".

**Rule.** A `pub enum` with exactly one variant requires `#[non_exhaustive]`
(with a doc comment naming the reserved future variant). Otherwise
convert to `pub struct`.

**Pass.** `LateLintPass::check_item` over `ItemKind::Enum`.

**Detection algorithm.**

```rust
impl<'tcx> LateLintPass<'tcx> for SingleVariantPubEnum {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx hir::Item<'tcx>) {
        let hir::ItemKind::Enum(ref enum_def, _) = item.kind else { return };

        // Visibility check: only flag `pub` enums.
        let vis = cx.tcx.visibility(item.owner_id);
        if !vis.is_public() { return; }

        if enum_def.variants.len() != 1 { return; }

        let has_non_exhaustive = cx.tcx.hir().attrs(item.hir_id())
            .iter()
            .any(|a| a.has_name(Symbol::intern("non_exhaustive")));
        if has_non_exhaustive { return; }

        span_lint_and_help(
            cx, SINGLE_VARIANT_PUB_ENUM, item.span,
            "`pub enum` with one variant is a struct in disguise",
            None,
            "either mark `#[non_exhaustive]` with a doc comment naming the reserved \
             future variant, or convert to `pub struct`",
        );
    }
}
```

**Positive (lints fire):**

```rust
pub enum Status {
    Active,
}                                            // fires

pub enum Wrap {
    Inner(u32),
}                                            // fires
```

**Negative (no fire):**

```rust
#[non_exhaustive]
pub enum Status {
    Active,                                  // reserved: ::Inactive
}

pub enum Status {
    Active,
    Inactive,
}                                            // 2 variants

enum Private { Only }                        // not pub
```

**Config:** none. The lint doesn't audit the doc-comment rationale; that
stays manual.

---

## Phase 2 — Structural HIR, low FP

### `usize_in_pub_error_variant`

**Status:** not yet shipped — open backlog item.

**Source:** `rust.md` "Conversions and typing".

**Rule.** Public error enum variants must not carry counts as `usize`.
Use explicit-width integers (`u32`, `u16`, `i64`, etc.) at the public
boundary. `usize` is reserved for slice indexing, pointer arithmetic,
`mem::size_of`, and FFI.

**Pass.** `LateLintPass::check_item` over `ItemKind::Enum`.

**Detection algorithm.**

```rust
impl<'tcx> LateLintPass<'tcx> for UsizeInPubErrorVariant {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx hir::Item<'tcx>) {
        let hir::ItemKind::Enum(ref enum_def, _) = item.kind else { return };
        if !cx.tcx.visibility(item.owner_id).is_public() { return; }

        // Heuristic: type name ends in `Error`.
        if !item.ident.as_str().ends_with("Error") { return; }

        for variant in enum_def.variants {
            for field in variant.data.fields() {
                let ty = cx.tcx.type_of(field.def_id).skip_binder();
                if ty.is_usize() {
                    span_lint_and_help(
                        cx, USIZE_IN_PUB_ERROR_VARIANT, field.ty.span,
                        "`usize` in public error variant",
                        None,
                        "use an explicit width (e.g. `u32`) at the public boundary; \
                         `usize` is platform-dependent",
                    );
                }
            }
        }
    }
}
```

**Positive (lints fire):**

```rust
pub enum FrameError {
    TooManyBytes { max: usize },                          // fires
    TooManyEvents(usize),                                  // fires
}
```

**Negative (no fire):**

```rust
pub enum FrameError {
    TooManyBytes { max: u32 },                             // ok
}

pub(crate) enum Internal {
    TooMany { count: usize },                              // not pub
}

pub enum NotAnError {                                      // doesn't end in Error
    A { len: usize },
}
```

**Edge case.** Slice-length fields that intentionally carry the native
machine word may opt out via `#[expect(usize_in_pub_error_variant, reason = "...")]`.
A future config could allow a per-type allowlist via `dylint.toml`.

**Config (`dylint.toml`):**

```toml
[code_review_lints.usize_in_pub_error_variant]
# Optional: extend the type-name suffix filter.
suffixes = ["Error", "Failure"]
# Optional: allowlisted variants where usize is genuinely correct.
allow = ["MyType::SliceLen"]
```

---

### `identity_passthrough_method`

**Status:** not yet shipped — open backlog item.

**Source:** `rust.md` "Structural review".

**Rule.** Methods whose entire body returns the single input value
unchanged are shims. Three shapes to detect:

1. `fn x(self) -> Self { self }`
2. `fn x(self) -> Self { *self }` (deref of `Copy` self)
3. `fn from_x(x: Self) -> Self { x }`

**Pass.** `LateLintPass::check_fn`.

**Detection algorithm.**

```rust
impl<'tcx> LateLintPass<'tcx> for IdentityPassthroughMethod {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        sig: &'tcx hir::FnDecl<'tcx>,
        body: &'tcx hir::Body<'tcx>,
        span: Span,
        _: hir::HirId,
    ) {
        // Look for one statement / expression that returns the single input.
        let body_expr = peel_blocks(&body.value);

        // Case 1: `self` returned by value.
        // Case 2: `*self` returned (Copy).
        // Case 3: single named param returned.
        let returned_local = match &body_expr.kind {
            hir::ExprKind::Path(qpath) => resolve_local(cx, qpath),
            hir::ExprKind::Unary(hir::UnOp::Deref, inner) => match &inner.kind {
                hir::ExprKind::Path(qpath) => resolve_local(cx, qpath),
                _ => None,
            },
            _ => None,
        };
        let Some(local_id) = returned_local else { return };

        // The local must be one of the function's parameters.
        let params = &body.params;
        if params.len() != 1 { return; }

        let param_pat = &params[0].pat;
        if !pat_binds(param_pat, local_id) { return; }

        // The function must return Self (i.e. parameter type == return type).
        let param_ty = cx.typeck_results().pat_ty(param_pat);
        let body_ty  = cx.typeck_results().expr_ty(body_expr);
        if param_ty != body_ty { return; }

        span_lint_and_help(
            cx, IDENTITY_PASSTHROUGH_METHOD, span,
            "method body returns its input unchanged",
            None,
            "delete the method and inline at callers; the type alias \
             (or generic) already attests the equivalence",
        );
    }
}
```

Helper notes:

- `peel_blocks` strips `{ expr }` and `return expr;` wrappers.
- `resolve_local` walks `Res::Local(hir_id)` from `cx.qpath_res(qpath, expr.hir_id)`.
- `pat_binds(pat, hir_id)` checks whether the pat binds that HIR id (handles
  `self`, `mut self`, `&self`, etc.).

**Positive (lints fire):**

```rust
impl Foo {
    pub const fn from_foo(foo: Self) -> Self { foo }       // fires
    pub const fn action(self) -> Self { self }             // fires
    pub const fn action(&self) -> Self { *self }           // fires (Copy)
}
```

**Negative (no fire):**

```rust
impl Foo {
    pub fn into_inner(self) -> Inner { self.0 }            // returns sub-field
    pub fn dup(&self) -> Foo { Foo { /* ... */ } }         // builds new value
    pub fn touch(self) -> Self {                           // mutates then returns
        let mut s = self;
        s.timestamp = now();
        s
    }
}
```

**Allow path:** `#[expect(identity_passthrough, reason = "API stability — keep across plugin boundary")]`.

**Config:** none. Lint scope: only methods inside `impl` blocks; skip
trait method implementations (those carry the trait's signature, not the
project's choice).

---

### `constant_method_ignores_self`

**Status:** not yet shipped — open backlog item.

**Source:** `rust.md` "Structural review".

**Rule.** A method receiving `self`/`&self`/`&mut self` whose body is a
literal or constant, with no HIR reference to `self`, is documentation in
function form. Either rewrite as an exhaustive `match self { ... }` or
delete.

**Pass.** `LateLintPass::check_fn`.

**Detection algorithm.**

```rust
impl<'tcx> LateLintPass<'tcx> for ConstantMethodIgnoresSelf {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _kind: FnKind<'tcx>,
        _sig: &'tcx hir::FnDecl<'tcx>,
        body: &'tcx hir::Body<'tcx>,
        span: Span,
        _: hir::HirId,
    ) {
        // Has a `self` receiver?
        let Some(first_param) = body.params.first() else { return };
        let hir::PatKind::Binding(_, _, ident, _) = first_param.pat.kind else { return };
        if ident.name != Symbol::intern("self") { return; }
        let self_hir_id = first_param.pat.hir_id;

        // Body is a literal or const path?
        let body_expr = peel_blocks(&body.value);
        match &body_expr.kind {
            hir::ExprKind::Lit(_) => {}
            hir::ExprKind::Path(qpath) => {
                let Res::Def(DefKind::Const | DefKind::AssocConst, _) =
                    cx.qpath_res(qpath, body_expr.hir_id) else { return };
            }
            _ => return,
        }

        // `self` must not appear in the body (use a visitor).
        if references_local(body_expr, self_hir_id) { return; }

        span_lint_and_help(
            cx, CONSTANT_METHOD_IGNORES_SELF, span,
            "method takes `self` but returns a constant; `self` is unused",
            None,
            "rewrite as `match self { Variant1 | Variant2 => CONST }` so a new \
             variant forces the question, or delete the method",
        );
    }
}
```

`references_local(expr, target_hir_id)`: HIR visitor checking every
`ExprKind::Path` against `target_hir_id` via `cx.qpath_res(...)` =>
`Res::Local(...)`.

**Positive (lints fire):**

```rust
impl Decision {
    pub const fn launches_duplicate_work(self) -> bool { false }     // fires

    pub const fn is_complete(&self) -> bool { true }                 // fires

    pub fn category(&self) -> &'static str { "transient" }           // fires
}
```

**Negative (no fire):**

```rust
impl Decision {
    pub fn launches_duplicate_work(self) -> bool {
        match self {
            Self::ReturnCached | Self::Attach => false,
            Self::StartValidation(_) => true,                         // matches self
        }
    }

    pub const fn capacity(&self) -> u32 { self.cap }                  // refs self
}
```

**Config:** none.

---

## Phase 3 — Heuristic / cross-item, higher FP

### `try_from_never_errors`

**Status:** not yet shipped — open backlog item.

**Source:** `rust.md` "Conversions and typing".

**Rule.** `impl TryFrom<X> for Y` whose `try_from` body never returns
`Err(...)` should be `impl From<X> for Y`.

**Pass.** `LateLintPass::check_impl_item` (the `fn try_from` method
inside an `impl TryFrom<...>` block).

**Detection algorithm.**

```rust
impl<'tcx> LateLintPass<'tcx> for TryFromNeverErrors {
    fn check_impl_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx hir::ImplItem<'tcx>) {
        if item.ident.name != Symbol::intern("try_from") { return; }
        let parent = cx.tcx.hir().get_parent_item(item.hir_id());
        let parent_item = cx.tcx.hir().expect_item(parent.def_id);
        let hir::ItemKind::Impl(impl_block) = parent_item.kind else { return };
        let Some(of_trait) = impl_block.of_trait else { return };
        // Match `TryFrom` by path.
        let trait_path = of_trait.path.segments.last().map(|s| s.ident.as_str());
        if trait_path != Some("TryFrom") { return; }

        let hir::ImplItemKind::Fn(_, body_id) = item.kind else { return };
        let body = cx.tcx.hir().body(body_id);

        let mut visitor = ErrReturnVisitor { found: false };
        intravisit::walk_expr(&mut visitor, &body.value);

        if !visitor.found {
            span_lint_and_help(
                cx, TRY_FROM_NEVER_ERRORS, item.span,
                "`TryFrom::try_from` never returns `Err`",
                None,
                "implement `From` instead, since the conversion is infallible",
            );
        }
    }
}

struct ErrReturnVisitor { found: bool }
impl<'v> Visitor<'v> for ErrReturnVisitor {
    fn visit_expr(&mut self, ex: &'v hir::Expr<'v>) {
        if let hir::ExprKind::Call(callee, _) = ex.kind {
            if let hir::ExprKind::Path(hir::QPath::Resolved(_, path)) = callee.kind {
                if path.segments.last().map_or(false, |s| s.ident.name == Symbol::intern("Err")) {
                    self.found = true;
                    return;
                }
            }
        }
        // Also detect `?` operator: `expr?` desugars to `match expr { Err(e) => return Err(e), ... }`.
        // The desugaring produces a `Call` to `Err` so the above catches it; double-check via testing.
        intravisit::walk_expr(self, ex);
    }
}
```

**Positive (lints fire):**

```rust
impl TryFrom<u32> for Wrap {
    type Error = std::convert::Infallible;
    fn try_from(x: u32) -> Result<Self, Self::Error> {
        Ok(Wrap(x))
    }
}
```

**Negative (no fire):**

```rust
impl TryFrom<u32> for Wrap {
    type Error = WrapError;
    fn try_from(x: u32) -> Result<Self, Self::Error> {
        if x == 0 { return Err(WrapError::Zero); }
        Ok(Wrap(x))
    }
}
```

**Caveats.**

- `?` operator desugars through `Err`; the visitor catches it.
- A `try_from` that always panics is technically infallible at the type
  level — we lint it. The panic should be a real `From` impl that
  documents the precondition. (Or, better, fix the precondition.)

**Config:** none.

---

### `bit_identical_enum_bodies`

**Status:** not yet shipped — open backlog item.

**Source:** `rust.md` "Structural review".

**Rule.** Two `pub enum` types in the same crate with field-identical
variant lists likely encode the same state space under different names.

**Pass.** Crate-wide late pass — accumulate shapes during item visits,
report at the end.

**Detection algorithm.**

```rust
#[derive(Default)]
pub struct BitIdenticalEnumBodies {
    // shape -> [(span, type name)]
    seen: HashMap<EnumShape, Vec<(Span, String)>>,
}

#[derive(Hash, PartialEq, Eq)]
struct EnumShape {
    variants: Vec<VariantShape>,
}

#[derive(Hash, PartialEq, Eq)]
struct VariantShape {
    name: String,                          // variant ident
    fields: Vec<(Option<String>, String)>, // (field name, normalized type)
}

impl<'tcx> LateLintPass<'tcx> for BitIdenticalEnumBodies {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx hir::Item<'tcx>) {
        let hir::ItemKind::Enum(ref enum_def, _) = item.kind else { return };
        if !cx.tcx.visibility(item.owner_id).is_public() { return; }

        let shape = shape_of(cx, enum_def);
        self.seen.entry(shape).or_default()
            .push((item.span, item.ident.to_string()));
    }

    fn check_crate_post(&mut self, cx: &LateContext<'_>) {
        for (_shape, types) in self.seen.drain() {
            if types.len() < 2 { continue; }
            let names: Vec<_> = types.iter().map(|(_, n)| n.clone()).collect();
            for (span, name) in &types {
                span_lint_and_help(
                    cx, BIT_IDENTICAL_ENUM_BODIES, *span,
                    &format!("`{name}` has the same variants as: {}", names.join(", ")),
                    None,
                    "collapse via type alias, generic over the payload, or phantom tag; \
                     a distinguishing tag belongs on the success type, not on a parallel error",
                );
            }
        }
    }
}

fn shape_of<'tcx>(cx: &LateContext<'tcx>, enum_def: &'tcx hir::EnumDef<'tcx>) -> EnumShape {
    let mut variants: Vec<_> = enum_def.variants.iter().map(|v| {
        let mut fields: Vec<_> = v.data.fields().iter().map(|f| {
            let ty = cx.tcx.type_of(f.def_id).skip_binder();
            (f.ident.as_str().to_string().into(), normalize_ty(cx, ty))
        }).collect();
        // For tuple variants, field names are positional indices; that's fine.
        fields.sort();
        VariantShape { name: v.ident.to_string(), fields }
    }).collect();
    variants.sort_by(|a, b| a.name.cmp(&b.name));
    EnumShape { variants }
}

fn normalize_ty<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> String {
    // Use the type's `Display` impl, but resolve through `tcx.normalize_erasing_regions`
    // so that lifetimes and ADT paths normalize uniformly.
    cx.tcx.normalize_erasing_regions(cx.param_env, ty).to_string()
}
```

**Positive (lints fire):**

```rust
// Two crate-local pub enums with the same shape.
pub enum TreeHashOutputParseError {
    NonUtf8,
    MissingHash,
    ExtraOutput,
    InvalidHash(ObjectHashParseError),
}

pub enum ObjectHashOutputParseError {
    NonUtf8,
    MissingHash,
    ExtraOutput,
    InvalidHash(ObjectHashParseError),
}                                            // both fire, referencing each other
```

**Negative (no fire):**

```rust
pub enum A { X, Y }
pub enum B { X, Y, Z }                       // different variant counts

pub enum A { X(u32) }
pub enum B { X(u64) }                        // different field types
```

**Caveats.**

- Cross-crate detection is out of scope; only intra-crate.
- Some twin enums are intentional (per-command-kind discriminators with
  distinct trait impls). Treat as advisory — per-type opt-out via
  `#[expect(bit_identical_enum_bodies, reason = "...")]` on either of the
  twins suffices.
- Variant order is normalized (sort by name) so reordering doesn't escape
  detection.

**Config (`dylint.toml`):**

```toml
[code_review_lints.bit_identical_enum_bodies]
# Optional: types to ignore even when their shapes collide.
ignore = ["MyCrate::SomeIntentionalTwin"]
```

---

## Shipped

Reference specs for lints that already ship in the suite. Listed
alphabetically; kept here so their detection logic and fixtures stay
discoverable even though they are off the open backlog.

### `expect_justified` (partial — was `expect_not_allow`)

**Status:** the `#[expect]`-carries-a-`reason` half of this rule ships today
as the `expect_justified` lint crate. The "`#[allow]` is forbidden in favor
of `#[expect]`" half is left to stock clippy's `allow_attributes` (see the
clippy table below); it does not enforce a `reason`, which `expect_justified`
adds.

**Source:** `references/allow-vs-expect.md`; `rust.md` "Review focus".

**Rule.** `#[expect(..., reason = "...")]` only. No `#[allow(...)]`. Every
`#[expect]` carries a `reason` meta. Inner attributes (`#![allow(...)]`)
are also covered.

**Pass.** `EarlyLintPass`. Attributes are visible at the AST stage before
macro expansion, which is what we want.

**Detection algorithm.**

```rust
impl EarlyLintPass for ExpectNotAllow {
    fn check_attribute(&mut self, cx: &EarlyContext<'_>, attr: &ast::Attribute) {
        let Some(meta) = attr.meta() else { return };
        match meta.name_or_empty() {
            sym::allow => {
                span_lint_and_help(
                    cx, EXPECT_NOT_ALLOW, attr.span,
                    "use `#[expect(..., reason = \"...\")]` instead of `#[allow(...)]`",
                    None,
                    "the project policy in `references/allow-vs-expect.md` requires \
                     forcing suppressions to be removed once obsolete",
                );
            }
            sym::expect => {
                // Walk the meta list. If no nested meta has name `reason`, lint.
                let has_reason = meta.meta_item_list()
                    .map(|list| list.iter().any(|nested|
                        nested.has_name(Symbol::intern("reason"))
                    ))
                    .unwrap_or(false);
                if !has_reason {
                    span_lint_and_help(
                        cx, EXPECT_NOT_ALLOW, attr.span,
                        "`#[expect]` without `reason`",
                        None,
                        "add `reason = \"...\"` explaining why the lint is suppressed",
                    );
                }
            }
            _ => {}
        }
    }
}
```

**Positive (lints fire):**

```rust
#[allow(clippy::needless_return)]
fn f() -> i32 { return 1; }                  // fires: bare allow

#[expect(clippy::needless_return)]
fn g() -> i32 { return 1; }                  // fires: missing reason

#![allow(unused)]                            // fires: bare allow at file level
```

**Negative (no fire):**

```rust
#[expect(clippy::needless_return, reason = "explicit early return reads clearer")]
fn h() -> i32 { return 1; }
```

**Similar clippy lint to copy from:** `clippy::allow_attributes` (already
warns on `#[allow]` in favor of `#[expect]`, but doesn't require `reason`).
Source: `clippy_lints/src/attrs/allow_attributes.rs` in `rust-lang/rust-clippy`.

**Config:** none required.

---

### Fallback-method family (was `test_fallback_helpers`)

**Status:** shipped. The single proposed `test_fallback_helpers` lint was
split into per-method lint crates that each catch one disallowed fallback
helper: `unwrap_or`, `unwrap_or_default`, `unwrap_or_else`, `or_default`,
and `get_or_insert`. Together they cover the fallback-helper rule below.

**Source:** `rust.md` "Test strictness".

**Rule.** In test scope, do not use `unwrap_or`, `unwrap_or_default`,
`unwrap_or_else`, `map_or`, `map_or_else`, `or`, `or_else`, `get_or_insert`,
`get_or_insert_with`, `or_default`. Also forbid the `Iterator` chain
`next().unwrap_or(...)`.

**Pass.** `LateLintPass::check_expr` restricted to test scope.

**Test scope detection.** A function or module is in test scope if it is
under any of:

- `#[test]` attribute on the enclosing fn,
- `#[cfg(test)]` attribute on an enclosing item,
- A `mod tests` (or `mod test`) module ancestor,
- Located under a `tests/` integration-test directory (the crate root file
  starts with `tests/`).

dylint helper: use `cx.tcx.opt_local_def_id_to_hir_id` and walk parent
items via `cx.tcx.hir().parent_iter(...)` to find enclosing modules and
their attrs.

**Detection algorithm.**

```rust
const BLACKLIST: &[&str] = &[
    "unwrap_or", "unwrap_or_default", "unwrap_or_else",
    "map_or", "map_or_else",
    "or", "or_else",
    "get_or_insert", "get_or_insert_with", "or_default",
];

impl<'tcx> LateLintPass<'tcx> for TestFallbackHelpers {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx hir::Expr<'tcx>) {
        let hir::ExprKind::MethodCall(path_seg, _, _, _) = expr.kind else { return };
        if !BLACKLIST.contains(&path_seg.ident.as_str()) { return; }

        if !in_test_scope(cx, expr.hir_id) { return; }

        span_lint_and_help(
            cx, TEST_FALLBACK_HELPERS, expr.span,
            &format!("`.{}` in test code hides missing data", path_seg.ident),
            None,
            "tests should fail on missing data; use `expect(\"...\")` or match the variant directly",
        );
    }
}

fn in_test_scope<'tcx>(cx: &LateContext<'tcx>, hir_id: hir::HirId) -> bool {
    // 1. Function `#[test]`?
    for (parent_hir_id, _) in cx.tcx.hir().parent_iter(hir_id) {
        let attrs = cx.tcx.hir().attrs(parent_hir_id);
        if attrs.iter().any(|a| a.has_name(Symbol::intern("test"))) { return true; }
        if attrs.iter().any(|a|
            a.meta_item_list()
             .map_or(false, |list| list.iter().any(|n| n.has_name(sym::test)))
        ) { return true; }
        // Module named tests?
        if let Some(node) = cx.tcx.hir().find(parent_hir_id) {
            if let hir::Node::Item(item) = node {
                if matches!(item.kind, hir::ItemKind::Mod(_))
                    && (item.ident.name == Symbol::intern("tests")
                        || item.ident.name == Symbol::intern("test"))
                {
                    return true;
                }
            }
        }
    }
    false
}
```

**Positive (lints fire):**

```rust
#[test]
fn example() {
    let map: HashMap<&str, u32> = HashMap::new();
    let value = map.get("x").unwrap_or(&0);          // fires: unwrap_or
    let first = items.iter().next().unwrap_or(&0);   // fires: unwrap_or
}

#[cfg(test)]
mod tests {
    fn helper() {
        let v: Option<u32> = None;
        let _ = v.unwrap_or_default();                // fires
        let _ = v.or(Some(0)).expect("ok");           // fires: or
    }
}
```

**Negative (no fire):**

```rust
// In production code:
pub fn parse_or_default(raw: &str) -> Config {
    parse(raw).unwrap_or_default()                    // not test scope
}

#[test]
fn example() {
    let v: Option<u32> = Some(1);
    let x = v.expect("setup should produce Some");    // expect with message: ok
    assert_eq!(x, 1);
}
```

**Similar clippy lint:** `clippy::disallowed_methods` (configurable via
`clippy.toml`) covers all-scope matching but cannot restrict to test
scope. The test-scope restriction is the value-add here.

**Config (`dylint.toml`):**

```toml
[code_review_lints.test_fallback_helpers]
# Optional: extend the blacklist per project.
extra_methods = []
```

---

### `inline_attr` (was `inline_requires_rationale`)

**Status:** shipped as the `inline_attr` lint crate.

**Source:** `rust.md` "API and ownership".

**Rule.** `#[inline]` and `#[inline(always)]` require an adjacent
justification: either a `//` rationale comment on the line above, or a
`reason = "..."` if the attribute is suppressed via `#[expect]`.

**Pass.** `EarlyLintPass::check_attribute`.

**Detection algorithm.**

```rust
impl EarlyLintPass for InlineRequiresRationale {
    fn check_attribute(&mut self, cx: &EarlyContext<'_>, attr: &ast::Attribute) {
        if !attr.has_name(sym::inline) { return; }

        // Look for a line comment immediately preceding the attribute span.
        let source_map = cx.sess().source_map();
        let snippet_above = source_map.span_to_snippet(
            source_map.span_extend_to_prev_str(attr.span, "//", false, false)
        );
        let has_comment = snippet_above
            .map(|s| s.trim_start().starts_with("//"))
            .unwrap_or(false);

        if !has_comment {
            span_lint_and_help(
                cx, INLINE_REQUIRES_RATIONALE, attr.span,
                "`#[inline]` without a justification comment",
                None,
                "add a comment above naming the benchmark, repo lint, or other reason; \
                 otherwise remove `#[inline]` and let the compiler decide",
            );
        }
    }
}
```

**Positive (lints fire):**

```rust
#[inline]
pub fn small() -> u32 { 1 }                   // fires: no rationale

#[inline(always)]
pub fn hot() {}                                // fires: no rationale
```

**Negative (no fire):**

```rust
// Hot path: inlining shaves ~12% off the per-frame loop (see benches/frame.rs).
#[inline]
pub fn small() -> u32 { 1 }

// `clippy::missing_inline_in_public_items` is enabled at the crate root.
#[inline]
pub fn other() {}
```

**Caveats.**

- The lint cannot evaluate whether the rationale is accurate; it defers
  to the author. The point is to make the cost visible at review time.

**Config:** none.

---

## Already covered by stock clippy — configure, don't reimplement

| Rule | Clippy lint | Source |
|---|---|---|
| Avoid needless `.clone()` | `clone_on_copy`, `redundant_clone` | `rust.md` "API and ownership" |
| Prefer `checked_*` arithmetic | `arithmetic_side_effects` | `rust.md` "Review focus" |
| `.unwrap()` in production | `unwrap_used` | implicit |
| Wide numeric casts | `cast_possible_truncation`, `cast_sign_loss`, `cast_possible_wrap` | `rust.md` "Conversions and typing" |
| `Default::default()` calls (general) | `default_trait_access` | `rust.md` "Test strictness" (partial) |
| Disallowed methods (general scope) | `disallowed_methods` via `clippy.toml` | `rust.md` "Test strictness" (no test-only scope) |
| `panic!` outside tests | `panic` | implicit |
| `#[allow]` in favor of `#[expect]` | `allow_attributes` | `allow-vs-expect.md` (no `reason` enforcement) |

Configure via `[lints.clippy]` in workspace `Cargo.toml`:

```toml
[lints.clippy]
clone_on_copy = "warn"
redundant_clone = "warn"
arithmetic_side_effects = "warn"
unwrap_used = "warn"
cast_possible_truncation = "warn"
cast_sign_loss = "warn"
cast_possible_wrap = "warn"
default_trait_access = "warn"
allow_attributes = "warn"
panic = "warn"

# Disallow methods (project list); see clippy.toml for the actual blacklist.
disallowed_methods = "warn"
```

In `clippy.toml`:

```toml
disallowed-methods = [
    { path = "std::option::Option::unwrap_or", reason = "use match or expect" },
    # ...
]
```

---

## Out of scope for lints (manual review)

These rules need intent or domain knowledge and stay in the manual
checklist:

- **Smart-constructor design.** Private fields, validation logic, contract
  docs — decisions tied to each type's invariants.
- **`NonEmptyString` / `NonEmpty<T>` adoption.** "Empty is invalid"
  requires per-field domain knowledge.
- **Bounded source → unbounded `Vec` returns.** The bound isn't always
  expressed in the source type signature.
- **`unwrap_or(literal)` outside tests.** Production usage may be
  intentional clamping; needs review.
- **Misleading field names.** Pure semantic concern.
- **Method ordering / file organization.** Cosmetic; mechanical detection
  produces noise.

---

## References

### Rules

- `references/languages/rust.md` — Rust review guidelines.
- `references/allow-vs-expect.md` — lint suppression policy.

### Tools

- **dylint:** [`https://github.com/trailofbits/dylint`](https://github.com/trailofbits/dylint) — workspace and lint registration macros.
- **Clippy lint index:** [`https://rust-lang.github.io/rust-clippy/master/`](https://rust-lang.github.io/rust-clippy/master/) — stock lint catalogue.
- **Clippy source (reference implementations):** [`https://github.com/rust-lang/rust-clippy/tree/master/clippy_lints/src`](https://github.com/rust-lang/rust-clippy/tree/master/clippy_lints/src) — read for AST/HIR matching patterns.

### Lint API entry points (rustc-private)

- `rustc_lint::{EarlyLintPass, LateLintPass, EarlyContext, LateContext}`
- `rustc_lint::LintStore::register_late_pass`
- `rustc_hir::{Item, ItemKind, Expr, ExprKind, Body, FnDecl, EnumDef, ImplItem, ImplItemKind}`
- `rustc_ast::{Attribute, MetaItem, NestedMetaItem, Visibility, VisibilityKind}`
- `rustc_span::{Symbol, sym, kw}`
- `rustc_middle::ty::{Ty, TyCtxt}` (for resolved types via `cx.typeck_results()`)

### Useful clippy lints to read as templates

- `clippy::allow_attributes` — early-pass attribute matching.
- `clippy::derive_partial_eq_without_eq` — late-pass derive list inspection.
- `clippy::disallowed_methods` — config-driven method-name blacklist.
- `clippy::manual_non_exhaustive_enum` — single-variant-with-hidden-sentinel
  pattern (different rule, similar shape).
- `clippy::cast_possible_truncation` — numeric-type inspection through HIR.

### Diagnostic helpers

- `clippy_utils::diagnostics::{span_lint, span_lint_and_help, span_lint_and_sugg}`
  if depending on `clippy_utils`. Otherwise use `cx.span_lint(...)` directly.

---

## Implementation order

Open backlog — the generic lints still to implement here, in priority order
(lowest false-positive risk first; structurally-similar lints kept adjacent):

1. **Phase 1** — syntactic / attribute, near-zero FP:
   `pub_in_crate_shorthand`, `serde_deny_unknown_fields`,
   `single_variant_pub_enum`.
2. **Phase 2** — structural HIR, low FP: `usize_in_pub_error_variant`,
   `identity_passthrough_method`, `constant_method_ignores_self`.
3. **Phase 3** — heuristic / cross-item, higher FP: `try_from_never_errors`,
   `bit_identical_enum_bodies`.

Each lint that lands moves rules out of the manual checklist (per `SKILL.md`
"Rule Steps" item 2).

For lints with legitimate exceptions, **prefer
`#[expect(lint_name, reason = "...")]`** so future cleanups force exception
removal.
