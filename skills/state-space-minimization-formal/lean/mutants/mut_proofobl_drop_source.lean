/-
Try to recover evidence for a raw value without its producer witness.
Only true can be supplied to this erasing operation.
-/
import ProofPreservation
open SSM

private def erase : Operation (fun x : Bool => x = true) := Subtype.val

example : false = true :=
  proofObligation_reconstitute (g := erase) (fun _ => rfl) false
