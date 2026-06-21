/-
MUTANT (must FAIL to compile) for headline
`trusted_boundary_reaches_valid`.

Mutation: a "trusted" boundary that lands values in `S(A)` only (not
`C(A)`) — the leaky weakening the theorem must reject. We try to prove
`R(b) ⊆ C(A)` for a boundary whose value is a representable-invalid
state, which is false.

Run:  lake env lean mutants/mut_trusted_reaches_valid.lean
Expect: a type/proof error — the invalid value is not in C.
-/
import Reflection
open SSM Set
set_option autoImplicit false

def Awide : Artifact Bool Unit where
  S := Set.univ
  C := {true}
  C_subset_S := fun _ _ => trivial
  B := fun _ => ()

-- An ordinary (untrusted) boundary that reaches `false` ∈ I_repr.
def leaky : Boundary Awide Unit where
  P := fun _ => True
  b := fun _ _ => ⟨false, trivial⟩

-- The theorem's guarantee `R(b) ⊆ C(A)` is FALSE for this boundary:
-- `false ∈ R leaky` but `false ∉ C(Awide) = {true}`.
example : R leaky ⊆ Awide.C := by
  intro x hx
  obtain ⟨u, p, rfl⟩ := hx
  -- goal: false ∈ {true} — unprovable; bogus close to force a failure.
  exact rfl
