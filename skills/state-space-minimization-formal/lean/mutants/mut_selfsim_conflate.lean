/-
MUTANT (must FAIL to compile) for headline
`missing_not_gap` / defs `MissingDistinction` / `Gap`.

Mutation: conflate the two directions of the set difference — treat a
`MissingDistinction` (in `Req(E) \ K`) as if it were in the gap
`G = K \ Req(E)`. The theorem proves they are disjoint, so the
conflation is false. We exhibit a distinction that is missing and try
to place it in the gap; it fails.

Run:  lake env lean mutants/mut_selfsim_conflate.lean
Expect: cannot prove the gap membership.
-/
import SelfSimilarity
open SSM.SelfSimilarity Set
set_option autoImplicit false

-- Skill expresses {true}; evidence requires {false}. So `false` is a
-- MissingDistinction (in Req \ K) and is NOT in the gap (K \ Req).
def skill : Set Bool := {true}
def ev : Set Bool := {false}

-- `false ∈ K skill = {true}` is already false, so the gap membership
-- cannot hold. The mutant tries to assert it via `rfl` for the first
-- conjunct (`false = true`), which fails.
example : (false : Bool) ∈ K skill := rfl
