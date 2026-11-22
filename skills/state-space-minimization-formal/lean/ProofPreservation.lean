/-
Proof-preservation rules of the state-space-minimization calculus
(SKILL.md § "Boundary Calculus", proof-obligation rule, and
§ "Proof-preservation corollary"). Calculus version: 2026-06-v9.

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

/-- The output of an operation on evidence-carrying inputs is a raw value.
An identity-preserving operation forgets the evidence at its interface. -/
abbrev Operation (P : State → Prop) := Subtype P → State

/-- A consumer declares `P` as a necessary condition for admitting a value.
Its domain alone says nothing about an arbitrary raw input. -/
def Needs (y : Consumer State) (P : State → Prop) : Prop :=
  ∀ x ∈ y.D, P x

/-- Erasing the proof component must preserve the underlying value. -/
def Erases {P : State → Prop} (g : Operation P) : Prop :=
  ∀ input, g input = input.val

/-- Every output of this producer falls in the consumer's domain. -/
def Flows {P : State → Prop} (g : Operation P) (y : Consumer State) : Prop :=
  ∀ input, g input ∈ y.D

/-- These premises identify a consumer obligation. Establishing the required
predicate for a particular raw value still needs evidence of its provenance. -/
structure ProofObligation {P : State → Prop} (g : Operation P)
    (y : Consumer State) : Prop where
  erased : Erases g
  flows : Flows g y
  needs : Needs y P

theorem proofObligation_intro {P : State → Prop} {g : Operation P}
    {y : Consumer State} (erased : Erases g) (flows : Flows g y)
    (needs : Needs y P) : ProofObligation g y :=
  ⟨erased, flows, needs⟩

/-- This projection describes the declared consumer contract. It requires
independent evidence that `raw` is admissible. -/
theorem proofObligation_needs_holds {P : State → Prop} {g : Operation P}
    {y : Consumer State} (ob : ProofObligation g y)
    (raw : State) (admitted : raw ∈ y.D) : P raw :=
  ob.needs raw admitted

/-- Knowing which verified input produced a raw value recovers its evidence.
The consumer's contract is irrelevant to this step: the producer preserved
the value, and the input carried a proof before its type erased it. -/
theorem proofObligation_reconstitute {P : State → Prop} {g : Operation P}
    (erased : Erases g) (raw : State) (source : ∃ input, g input = raw) :
    P raw := by
  obtain ⟨input, rfl⟩ := source
  rw [erased input]
  exact input.property

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

/-- The provenance recorded for this particular input suffices to recover
its contract membership even with a wider representable state space. -/
theorem obligation_implies_contract {A : Artifact State Obs} {f : Consumer State}
    {U : Type w} {bd : TrustedBoundary A U} {x : State}
    (provenance : Obl A f bd x) : x ∈ A.C :=
  trusted_boundary_reaches_valid bd provenance.2.2

end SSM
