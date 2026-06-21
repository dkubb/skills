/-
Function range/gap/preimage, the six operations, and the Invariants
of the state-space-minimization calculus (SKILL.md § "Core Notation"
`R(f)`/`G(f)`/`f⁻¹`, § "Operations", § "Invariants"). Calculus
version: 2026-06-v8.

The structural content of each operation (domain/codomain narrowing,
preimage, normalization determinants) is kernel rank. The operations'
*semantics* (when an estimated bound may move the other way, ratchet
policy) are owned by `state-space-minimization` references and carried
as residuals where no command can enforce them.
-/

import Mathlib.Data.Set.Basic

set_option autoImplicit false

open Set

universe u v w

namespace SSM.Fn

variable {D : Type u} {K : Type v}

/-- SKILL.md § "Core Notation": `R(f) := {f(x) | x ∈ D(f)}` — the
range of `f` over a domain set. -/
def Range (f : D → K) (Dom : Set D) : Set K :=
  {y | ∃ x ∈ Dom, f x = y}

/-- SKILL.md § "Core Notation": `G(f) := K(f) \ R(f)` — the gap, the
codomain values `f` never produces. -/
def Gap (f : D → K) (Dom : Set D) (Cod : Set K) : Set K :=
  Cod \ Range f Dom

/-- SKILL.md § "Core Notation": `f⁻¹(F) := {x ∈ D(f) | f(x) ∈ F}` —
the preimage (failure preimage when `F` is the bad set). -/
def Preimage (f : D → K) (Dom : Set D) (F : Set K) : Set D :=
  {x | x ∈ Dom ∧ f x ∈ F}

/-- Proves the defining relation between range and gap: a codomain
value is in the gap iff it is in the codomain and not in the range.
Pins that `Gap` is exactly the complement of the range within the
codomain. -/
theorem mem_gap_iff
    {f : D → K} {Dom : Set D} {Cod : Set K} {y : K} :
    y ∈ Gap f Dom Cod ↔ y ∈ Cod ∧ y ∉ Range f Dom :=
  Iff.rfl

/-- SKILL.md § "Operations" (shrink codomain): replacing `K` by `K'`
with `R(f) ⊆ K' ⊊ K` is admissible exactly when the range still fits.
This theorem proves the side condition `R(f) ⊆ K'` is what keeps the
narrowed codomain total — a codomain that drops a produced value is
rejected. -/
theorem shrink_codomain_keeps_range
    {f : D → K} {Dom : Set D} {K' : Set K}
    (hsub : Range f Dom ⊆ K') :
    ∀ x ∈ Dom, f x ∈ K' := by
  intro x hx
  exact hsub ⟨x, hx, rfl⟩

/-- SKILL.md § "Operations" (shrink domain): replacing `D` by
`D' ⊊ D` shrinks the range monotonically (it never adds values). -/
theorem shrink_domain_range_mono
    {f : D → K} {Dom Dom' : Set D} (h : Dom' ⊆ Dom) :
    Range f Dom' ⊆ Range f Dom := by
  rintro y ⟨x, hx, rfl⟩
  exact ⟨x, h hx, rfl⟩

/-- SKILL.md § "Invariants", Totality: every exposed function is
total on its declared domain. In Lean a `D → K` is already total on
all of `D`; modeled as: on any declared domain set, `f` produces a
codomain value for every input. This is the structural witness that
totality is discharged by the function space, with `Range ⊆ Cod` as
the well-typedness obligation. -/
def Total (f : D → K) (Dom : Set D) (Cod : Set K) : Prop :=
  ∀ x ∈ Dom, f x ∈ Cod

/-- SKILL.md § "Invariants", Single source: each fact has one
determinant. Modeled over a functional dependency `X → Y`: a relation
`r` is single-source when the determinant `key` fixes the value, i.e.
`r` is the graph of a function of `key`. -/
def SingleSource {Fact Det Val : Type u}
    (key : Fact → Det) (val : Fact → Val) : Prop :=
  ∀ a b, key a = key b → val a = val b

/-- Proves single-source normalization is well defined: under a
single-source dependency, the value is a genuine function of the
determinant — two facts with the same determinant cannot disagree.
This is the no-disagreement content (a second determinant for one
fact is unrepresentable). -/
theorem singleSource_functional {Fact Det Val : Type u}
    {key : Fact → Det} {val : Fact → Val}
    (h : SingleSource key val)
    {a b : Fact} (hk : key a = key b) :
    val a = val b :=
  h a b hk

/-- SKILL.md § "Invariants", normalization: a normalizer `nf` is
*idempotent* when re-normalizing a normalized value is a no-op
(`nf (nf x) = nf x`), i.e. `nf` is a retraction onto its image. This
is the only property modeled here; it does NOT claim that distinct
reduction orders converge (that would require a rewrite relation and a
diamond/Newman argument, which are out of scope — see the residual
note below). -/
def Idempotent {S : Type u} (nf : S → S) : Prop :=
  ∀ x, nf (nf x) = nf x

/-- Proves the fixed-point content of idempotence: every normalized
value `nf x` is a fixed point of `nf`, so the image of `nf` is exactly
its set of fixed points. This is the honest, provable consequence —
re-applying the normalizer never moves a normalized value. -/
theorem idempotent_fixes_image {S : Type u} {nf : S → S}
    (h : Idempotent nf) (x : S) :
    nf (nf x) = nf x :=
  h x

/- SKILL.md § "Invariants", Exhaustiveness, is NOT given a Lean
declaration here: it is enforced *structurally by the Lean kernel
itself* (a `match` on a closed inductive that omits a constructor
does not typecheck; a catch-all is only needed for open sums). A
project-level `Prop` would either restate that compiler fact
vacuously (`True`) or be unfalsifiable, so exhaustiveness is carried
as an allowlisted residual — `exhaustiveness_residual` in
claims.toml — whose enforcement rank is the type checker, not a
theorem. -/

end SSM.Fn
