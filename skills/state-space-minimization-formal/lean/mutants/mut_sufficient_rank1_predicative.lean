/-
MUTANT (must FAIL to compile) for headline
`rank1_sufficient_iff` / def `Sufficient` (rank-1 strengthening).

Mutation: claim `Sufficient` holds for a rank-1 mechanism whose
output artifact is still predicative (`I_repr ≠ ∅`), even though its
only construction path is trusted (`I_reach = ∅`). Per SKILL.md
§ "Encoding Order", rank-1 sufficiency strengthens to `I_repr = ∅`,
so the rank-1 guard rejects a predicative representation.

Run:  lake env lean mutants/mut_sufficient_rank1_predicative.lean
Expect: single error on `SufficientRank1` (`I_repr Apred = ∅` is false).
-/
import Reflection
open SSM Set
set_option autoImplicit false

def Apred : Artifact Bool Unit where
  S := Set.univ
  C := {true}
  C_subset_S := fun _ _ => trivial
  B := fun _ => ()

def trusted : TrustedBoundary Apred Unit where
  P := fun _ => True
  b := fun _ _ => ⟨true, rfl⟩

def smartRank1 : Mechanism.{0, 0, 0} Bool Unit where
  apply := fun _ => Apred
  paths := fun _ _ =>
    { Paths := Unit
      path := fun _ => ⟨Unit, trusted.toBoundary⟩ }
  rank := ⟨1, Nat.zero_lt_one⟩

-- All paths are clean (`I_reach = ∅`), but `rank = 1` demands
-- `I_repr Apred = ∅`, which is false (`false ∈ I_repr Apred`).
example (ps : PathSet.{0, 0, 0} Apred) : Sufficient smartRank1 Apred ps := by
  refine ⟨fun _ => ?_, fun _ => trusted_boundary_no_reachable_invalid trusted⟩
  -- goal: I_repr Apred = ∅ — false at `false`; bogus close.
  exact rfl
