/-
Self-similarity rules of the state-space-minimization calculus
(SKILL.md § "Self-Similarity"). Calculus version: 2026-06-v8.

The set-difference structure is kernel rank: the skill is an
artifact, its distinctions are states, and the gap analogy
`G = K(Skill) \ Req(E)` is a set difference. The "`E*` spans repeated
use" premise of `SharpeningCandidate` is an allowlisted residual
(empirical — see claims.toml `selfsim_spans_repeated_use_residual`):
we model the structural rule and carry the premise as an abstract
hypothesis, never asserting it holds.
-/

import Mathlib.Data.Set.Basic

set_option autoImplicit false

open Set

universe u

namespace SSM.SelfSimilarity

variable {Distinction : Type u}

/-- SKILL.md § "Self-Similarity": `K(Skill)` — the distinctions
expressible by the skill (the skill viewed as an artifact, its rules
as representable states). This is the codomain in the gap analogy. -/
def K (Skill : Set Distinction) : Set Distinction := Skill

/-- SKILL.md § "Self-Similarity": `Req(E)` — the distinctions the
evidence `E` exercised or required. Modeled as the set the evidence
ranges over. -/
def Req (E : Set Distinction) : Set Distinction := E

/-- SKILL.md § "Self-Similarity": `MissingDistinction(d)` — an
expressiveness deficit, `d ∈ Req(E) \ K(Skill)`. This is NOT the gap
`G`; it is the reverse difference. -/
def MissingDistinction
    (Skill E : Set Distinction) (d : Distinction) : Prop :=
  d ∈ Req E \ K Skill

/-- SKILL.md § "Self-Similarity": the gap `G` applied to the skill —
`G(Skill, E) := K(Skill) \ Req(E)`, unexercised expressiveness. The
range/gap analogy: with codomain `K(Skill)` and "range" `Req(E)`,
this is exactly `K \ Req`. -/
def Gap (Skill E : Set Distinction) : Set Distinction :=
  K Skill \ Req E

/-- SKILL.md § "Self-Similarity": `SharpeningCandidate(d)` — `d` is
expressible but went unexercised across accumulated evidence `E*`.
The "`E*` spans repeated use" premise is carried as the abstract
hypothesis `spansRepeatedUse` (an allowlisted residual: it is an
empirical claim about evidence accumulation, not a Lean-checkable
fact). The structural content — `d ∈ G(Skill, E*)` — is kernel. -/
def SharpeningCandidate
    (Skill Estar : Set Distinction) (spansRepeatedUse : Prop)
    (d : Distinction) : Prop :=
  spansRepeatedUse ∧ d ∈ Gap Skill Estar

/-- Proves SKILL.md § "Self-Similarity": the two directions are
genuinely different — a `MissingDistinction` is never in the gap `G`,
and a gap element is never missing. Pins that the set-difference
structure is asymmetric (the most common slip — conflating the two —
is unrepresentable). -/
theorem missing_not_gap
    {Skill E : Set Distinction} {d : Distinction}
    (h : MissingDistinction Skill E d) :
    d ∉ Gap Skill E := by
  intro hg
  exact h.2 hg.1

/-- Converse corollary of `missing_not_gap`: a gap element is not a
missing distinction. Each direction proves the other in one line
(`missing_not_gap` is the headline; this is its dual), so the disjoint
guarantee is carried by `missing_not_gap` alone. -/
theorem gap_not_missing
    {Skill E : Set Distinction} {d : Distinction}
    (h : d ∈ Gap Skill E) :
    ¬ MissingDistinction Skill E d :=
  fun hm => missing_not_gap hm h

/-- Proves the residual premise is load-bearing for
`SharpeningCandidate`: dropping `spansRepeatedUse` (taking it `True`)
would let the rule fire on a single case. This theorem extracts the
premise, witnessing that it is carried, not assumed away. -/
theorem sharpening_requires_span
    {Skill Estar : Set Distinction} {spans : Prop} {d : Distinction}
    (h : SharpeningCandidate Skill Estar spans d) :
    spans :=
  h.1

end SSM.SelfSimilarity
