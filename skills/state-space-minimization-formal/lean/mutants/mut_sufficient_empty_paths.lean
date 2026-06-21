/-
MUTANT (must FAIL to compile) for headline
`sufficientWithPaths_imp_irepr_empty` / def `SufficientWithPaths`.

Mutation: weaken the construction-path set to the empty/subset form
the OLD vacuous definition allowed (`Paths = Empty`). With the
coverage field, `Empty` cannot cover an artifact whose `S` is
nonempty, so the `covers` obligation is unprovable. This is the
rejecting mutant for the closed vacuity hole: it demonstrates that an
empty/subset path set can no longer discharge sufficiency.

Run:  lake env lean mutants/mut_sufficient_empty_paths.lean
Expect: a type error on `covers` (False is not provable).
-/
import Reflection
open SSM Set
set_option autoImplicit false

def Abad : Artifact Bool Unit where
  S := Set.univ
  C := {true}
  C_subset_S := fun _ _ => trivial
  B := fun _ => ()

def midemo : Mechanism Bool Unit where
  apply := fun _ => Abad
  rank := ⟨1, Nat.zero_lt_one⟩
  cost := 0

-- The empty path set cannot satisfy coverage of S = univ.
def emptyPaths : ConstructionPaths midemo Abad where
  Paths := Empty
  path := fun e => e.elim
  covers := by
    intro x hx
    -- goal: ∃ p : Empty, x ∈ R ...  — unprovable; the mutant tries a bogus close.
    exact absurd hx (by intro _; exact trivial)
