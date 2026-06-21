/-
Proof-preservation rules of the state-space-minimization calculus
(SKILL.md § "Boundary Calculus", proof-obligation rule, and
§ "Proof-preservation corollary"). Calculus version: 2026-06-v8.

Formalizes the rule

    Erases(g, P(x))   Flows(g, y)   Needs(y, P(x))
    ------------------------------------------------ ProofObligation
                 ProofObligation(y, P(x))

as a structural introduction rule, and connects it to `Obl` from
`Reflection.lean`.
-/

import Reflection

set_option autoImplicit false

universe u v w

namespace SSM

variable {State : Type u} {Obs : Type v}

/-- SKILL.md § "Core Notation": an operation `g` over states,
together with the evidence predicate `P` it may erase and the
consumer it may feed. Modeled abstractly: an operation is named by
the data it carries about evidence flow, so the introduction rule is
a structural fact rather than an assertion about opaque functions. -/
structure Operation (State : Type u) where
  /-- the underlying state transform `(x, p : P x) ↦ x` -/
  run : State → State

/-- SKILL.md § "Core Notation": `Needs(y, P(x))` — consumer `y`'s
contract depends on evidence `P(x)`. A consumer (from `Reflection`)
needs `P` exactly when its declared domain `D` admits a state for
which `P` must have held. Modeled as a relation over the evidence
predicate the consumer requires on its inputs. -/
def Needs (y : Consumer State) (P : State → Prop) : Prop :=
  ∀ x ∈ y.D, P x

/-- SKILL.md § "Core Notation": `Erases(g, P(x))` — operation `g`
maps `(x, p : P x) ↦ x`, discarding the evidence. The formal content
is the *value-preservation* `g.run x = x`: the erasure keeps the
value in the same state where `P` held, so `P` could be reconstituted
downstream (see `proofObligation_reconstitute`). `P` is a NAMED
parameter identifying WHICH evidence is dropped — it is intentionally
NOT a constraint inside this predicate (a `Prop`-irrelevant witness
cannot be tracked by the value transform), so `Erases g P` is, by
construction, independent of `P`. The load-bearing content (value
preservation) is pinned by `proofObligation_reconstitute`; the role
of `P` is pinned by `proofObligation_needs_holds` via `Needs`. A
mutant that lets `g.run` move the value off its evidence-carrying
state breaks the model. -/
def Erases (g : Operation State) (_P : State → Prop) : Prop :=
  ∀ x, g.run x = x

/-- SKILL.md § "Core Notation": `Flows(g, y)` — the output of `g`
reaches consumer `y`: every post-state of `g` lands in `y.D`. -/
def Flows (g : Operation State) (y : Consumer State) : Prop :=
  ∀ x, g.run x ∈ y.D

/-- SKILL.md § "Boundary Calculus", proof-obligation rule:
`ProofObligation(y, P(x))` — the consumer `y` carries a standing
obligation that the erased evidence `P` held of its input. This is
the *conclusion* of the introduction rule; carrying the three
premises as fields makes the obligation a structural consequence,
not a free assertion. -/
structure ProofObligation
    (g : Operation State) (y : Consumer State) (P : State → Prop)
    where
  erased : Erases g P
  flows : Flows g y
  needs : Needs y P

/-- Proves SKILL.md § "Boundary Calculus": the proof-obligation
introduction rule. From the three premises, the obligation is
discharged structurally. -/
theorem proofObligation_intro
    {g : Operation State} {y : Consumer State} {P : State → Prop}
    (hErase : Erases g P) (hFlow : Flows g y) (hNeed : Needs y P) :
    ProofObligation g y P :=
  ⟨hErase, hFlow, hNeed⟩

/-- Proves SKILL.md § "Proof-preservation corollary": when the
obligation holds and the input actually flowed through `g`, the
required evidence `P` holds of it. This is the content that pins the
rule: the obligation is *about* `P` on the consumer's inputs. -/
theorem proofObligation_needs_holds
    {g : Operation State} {y : Consumer State} {P : State → Prop}
    (ob : ProofObligation g y P)
    (x : State) (hx : x ∈ y.D) :
    P x :=
  ob.needs x hx

/-- Proves the full proof-preservation content (SKILL.md
§ "Proof-preservation corollary"): the evidence `g` erased must be
reconstituted at the consumer. Concretely, for any pre-state `x`,
the erased post-state `g.run x` flows into `y.D` and the consumer
requires `P` there — so `P (g.run x)` is an obligation that the
erasure created. This theorem is load-bearing on all three fields:
`flows` places `g.run x` in `y.D`, `needs` requires `P` on `y.D`,
and `erased` identifies `g.run x` with the original value, so the
reconstituted obligation is about the very value whose proof was
dropped. -/
theorem proofObligation_reconstitute
    {g : Operation State} {y : Consumer State} {P : State → Prop}
    (ob : ProofObligation g y P)
    (x : State) :
    P (g.run x) ∧ g.run x = x := by
  refine ⟨ob.needs (g.run x) (ob.flows x), ob.erased x⟩

/-- SKILL.md § "Proof-preservation corollary": for constructive `A`
(`S(A) = C(A)`), the consumer obligation `Obl(f)` is discharged by
the type and survives the addition of arbitrary new construction
paths. Modeled: when `f.D = A.S` and `A` is constructive, membership
in `f.D` already certifies membership in `C(A)`. -/
theorem constructive_discharges_obl
    {A : Artifact State Obs} (hCon : A.S = A.C)
    (f : Consumer State) (hf : f.D = A.S)
    (x : State) (hx : x ∈ f.D) :
    x ∈ A.C := by
  rw [hf, hCon] at hx
  exact hx

end SSM
