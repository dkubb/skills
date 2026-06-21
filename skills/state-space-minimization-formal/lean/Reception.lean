/-
Reception semantics of the state-space-minimization calculus
(SKILL.md § "Reception Semantics"). Calculus version: 2026-06-v8.

Formalizes the *structural* content: denotational equivalence `≡_D`
and the support-subset admissibility/narrowing relations as set
relations (kernel rank). The operational measure `Pr(q'|q,t,c)` is
evaluator-judgment / honest-doc — modeled only by the SHAPE of its
support; the probabilistic premise itself is an allowlisted residual
(see claims.toml `reception_operational_residual`). No probabilistic
theorem is asserted here.
-/

import Mathlib.Data.Set.Basic

set_option autoImplicit false

open Set

universe u v w

namespace SSM.Reception

variable {Den : Type u} {Q : Type v} {Ctx : Type w}

/-- A text artifact, abstractly: it carries a denotation `[[t]]^D`
and the SHAPE of its operational reception — for each prior receiver
state `q` and context `c`, the *support* `supp([[t]]^O_{q,c})` of the
reception distribution, i.e. the set of receiver states it can
induce. The numeric distribution is deliberately not modeled (its
premise is the residual); only the support, over which the calculus's
admissibility relations are set inclusions. -/
structure Text (Den : Type u) (Q : Type v) (Ctx : Type w) where
  /-- `[[t]]^D` — the denotation of `t`. -/
  den : Den
  /-- `supp([[t]]^O_{q,c})` — the support of the operational
  reception distribution from prior state `q` in context `c`. -/
  supp : Q → Ctx → Set Q

/-- SKILL.md § "Reception Semantics": `t ≡_D t' := [[t]]^D = [[t']]^D`. -/
def DenotEq (t t' : Text Den Q Ctx) : Prop :=
  t.den = t'.den

/-- SKILL.md § "Reception Semantics", the `AdmissibleRewrite` rule:
denotation preserved, and for every plausible `(q, c)` the new
unintended readings are bounded by the old support:

    supp([[t']]^O_{q,c}) \ Q_intended ⊆ supp([[t]]^O_{q,c}).

The quantifier ranges over the plausible prior states `Q0` and
contexts `Ctx0` (here the full carriers; restrict by intersecting in
the caller). New *intended* readings are admitted; new *unintended*
ones are forbidden. -/
def AdmissibleRewrite
    (Q_intended : Set Q) (t t' : Text Den Q Ctx) : Prop :=
  DenotEq t t' ∧
  ∀ q c, (t'.supp q c \ Q_intended) ⊆ t.supp q c

/-- SKILL.md § "Reception Semantics": `ReceptionNarrowing` strengthens
admissibility with a strict witness — some `(q, c)` at which the
unintended-reading set strictly shrinks. A rewrite that leaves the
unintended readings unchanged is admissible but not a narrowing. -/
def ReceptionNarrowing
    (Q_intended : Set Q) (t t' : Text Den Q Ctx) : Prop :=
  AdmissibleRewrite Q_intended t t' ∧
  ∃ q c,
    (t'.supp q c \ Q_intended) ⊂ (t.supp q c \ Q_intended)

/-- SKILL.md § "Reception Semantics": reception *completeness* — every
remaining reading is intended. This is the discharge condition of the
reception obligation, NOT an admissibility premise. -/
def ReceptionComplete (Q_intended : Set Q) (t' : Text Den Q Ctx) : Prop :=
  ∀ q c, t'.supp q c ⊆ Q_intended

/-- Proves admissibility is reflexive: a text is an admissible rewrite
of itself (denotation equal; no new readings). Pins that the relation
is non-degenerate (a vacuous `AdmissibleRewrite := False` fails). -/
theorem admissibleRewrite_refl
    (Q_intended : Set Q) (t : Text Den Q Ctx) :
    AdmissibleRewrite Q_intended t t := by
  refine ⟨rfl, ?_⟩
  intro q c x hx
  exact hx.1

/-- Proves SKILL.md § "Reception Semantics": narrowing is strictly
stronger than admissibility — every narrowing is admissible. -/
theorem narrowing_imp_admissible
    {Q_intended : Set Q} {t t' : Text Den Q Ctx}
    (h : ReceptionNarrowing Q_intended t t') :
    AdmissibleRewrite Q_intended t t' :=
  h.1

/-- Proves the discharge-condition relation: when a rewrite is
admissible AND reception-complete, it introduces NO unintended
reading at all (the residual gap is empty). This pins that
completeness is the obligation's discharge — the kernel content of
"every remaining reading intended". -/
theorem complete_imp_no_unintended
    {Q_intended : Set Q} {t t' : Text Den Q Ctx}
    (_ha : AdmissibleRewrite Q_intended t t')
    (hc : ReceptionComplete Q_intended t') :
    ∀ q c, (t'.supp q c \ Q_intended) = ∅ := by
  intro q c
  ext x
  constructor
  · rintro ⟨hxs, hxni⟩
    exact (hxni (hc q c hxs)).elim
  · intro hx
    exact (Set.notMem_empty x hx).elim

/-- Proves transitivity of admissibility under a shared intended set:
admissible rewrites compose, so a chain of denotation-preserving,
reception-narrowing edits is itself admissible. Load-bearing on the
support-subset premise (a mutant dropping it breaks composition). -/
theorem admissibleRewrite_trans
    {Q_intended : Set Q} {t t' t'' : Text Den Q Ctx}
    (h1 : AdmissibleRewrite Q_intended t t')
    (h2 : AdmissibleRewrite Q_intended t' t'') :
    AdmissibleRewrite Q_intended t t'' := by
  refine ⟨h1.1.trans h2.1, ?_⟩
  intro q c x hx
  -- x ∈ supp(t'') \ Q_intended ⊆ supp(t') ; and x ∉ Q_intended, so
  -- x ∈ supp(t') \ Q_intended ⊆ supp(t).
  have hx' : x ∈ t'.supp q c := h2.2 q c hx
  have hx'' : x ∈ t'.supp q c \ Q_intended := ⟨hx', hx.2⟩
  exact h1.2 q c hx''

end SSM.Reception
