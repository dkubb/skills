/-
MUTANT (must FAIL to compile) for def `Strict` /
`sufficientWithPaths_imp_irepr_empty` neighbourhood — the
contract-pin premise `C(A') = C(A)`.

Mutation: discharge `Strict` by WIDENING the contract until nothing
is invalid, instead of preserving it. The calculus forbids this:
`ContractPinned` requires `A'.C = A.C`. We try to build a `Strict`
step whose `A'` widens `C` to all of `S`, making `I_repr A' = ∅`
trivially, and show the pin premise cannot be met.

Run:  lake env lean mutants/mut_strict_drop_pin.lean
Expect: ContractPinned (A'.C = A.C) is unprovable.
-/
import Reflection
open SSM Set
set_option autoImplicit false

def A0 : Artifact Bool Unit where
  S := Set.univ
  C := {true}
  C_subset_S := fun _ _ => trivial
  B := fun _ => ()

-- A' widens the contract to univ (the forbidden move): I_repr A' = ∅.
def A1 : Artifact Bool Unit where
  S := Set.univ
  C := Set.univ
  C_subset_S := fun _ _ => trivial
  B := fun _ => ()

-- The contract pin `A1.C = A0.C` is FALSE (univ ≠ {true}).
example : ContractPinned A0 A1 := by
  show A1.C = A0.C
  -- goal: (univ : Set Bool) = {true} — false; bogus close.
  rfl
