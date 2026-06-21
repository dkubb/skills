/-
MUTANT (must FAIL to compile) for headline
`proofObligation_reconstitute` / def `ProofObligation`.

Mutation: drop the `needs` premise (the consumer no longer requires
the evidence `P`). Without `needs`, `P x` cannot be concluded for an
arbitrary consumer input. We build a ProofObligation-shaped value
WITHOUT a `needs` witness and try to extract `P x`; it fails.

Run:  lake env lean mutants/mut_proofobl_drop_needs.lean
Expect: cannot prove `P x` without `needs`.
-/
import ProofPreservation
open SSM
set_option autoImplicit false

-- A consumer whose domain admits a state for which P is FALSE.
def cons : Consumer Bool := { D := Set.univ }
def g : Operation Bool := { run := fun x => x }
def P : Bool → Prop := fun b => b = true

-- Erases and Flows hold; Needs does NOT (false ∈ D but ¬ P false).
example : P false := by
  have hErase : Erases g P := fun _ => rfl
  have hFlow : Flows g cons := fun _ => trivial
  -- The mutant drops `needs`; with only erase+flow it tries to close
  -- `P false`, which is `false = true`.
  exact rfl
