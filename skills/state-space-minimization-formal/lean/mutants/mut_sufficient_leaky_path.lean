/-
MUTANT (must FAIL to compile) for headline
`leaky_path_not_sufficient` / def `Sufficient`.

Mutation: claim `Sufficient` holds for a rank-2 mechanism whose
transformed `PathSet` retains a leaky construction path alongside a
trusted one. Because the path set is fixed by `m.paths A ps` (not
chosen by the claimant), the leaky path cannot be omitted and
`I_reach` on the leaky branch is nonempty (`false ∈ I_reach`), so
`Sufficient` is unprovable.

Run:  lake env lean mutants/mut_sufficient_leaky_path.lean
Expect: single error on the leaky path (`false ∈ ∅`).
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

def leaky : Boundary Apred Unit where
  P := fun _ => True
  b := fun _ _ => ⟨false, trivial⟩

def adapterLeaky : Mechanism.{0, 0, 0} Bool Unit where
  apply := fun _ => Apred
  paths := fun _ _ =>
    { Paths := Bool
      path := fun i => ⟨Unit, if i then trusted.toBoundary else leaky⟩ }
  rank := ⟨2, Nat.zero_lt_two⟩

-- With the leaky path present in `adapterLeaky.paths`, claiming
-- `Sufficient` fails on the `false` branch: `false ∈ I_reach leaky`,
-- which cannot equal `∅`.
example (ps : PathSet.{0, 0, 0} Apred) : Sufficient adapterLeaky Apred ps := by
  refine ⟨fun h => (by cases h), ?_⟩
  intro p
  cases p
  · -- goal: I_reach leaky = ∅ — false at `false`; bogus close.
    exact rfl
  · exact trusted_boundary_no_reachable_invalid trusted
