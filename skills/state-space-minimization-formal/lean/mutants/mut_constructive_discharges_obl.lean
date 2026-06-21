/-
MUTANT (must FAIL to compile) for headline
`constructive_discharges_obl`.

Mutation: drop the constructive premise `S(A) = C(A)` (use a
predicative artifact) and still claim the consumer obligation is
discharged by the type. For predicative `A`, a state in `f.D = A.S`
need not be in `A.C`, so the discharge fails.

Run:  lake env lean mutants/mut_constructive_discharges_obl.lean
Expect: a representable-invalid state is not in C.
-/
import ProofPreservation
open SSM Set
set_option autoImplicit false

def Apred : Artifact Bool Unit where
  S := Set.univ
  C := {true}          -- predicative: S ⊋ C
  C_subset_S := fun _ _ => trivial
  B := fun _ => ()

def fcons : Consumer Bool := { D := Set.univ }

-- Without the constructive premise, membership in f.D does NOT give
-- membership in C. The mutant asserts it does.
example (x : Bool) (hx : x ∈ fcons.D) : x ∈ Apred.C := by
  -- goal: x ∈ {true} for arbitrary x — false at false. Bogus close.
  exact hx
