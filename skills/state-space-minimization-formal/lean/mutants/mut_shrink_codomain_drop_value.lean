/-
MUTANT (must FAIL to compile) for headline
`shrink_codomain_keeps_range`.

Mutation: shrink the codomain BELOW the range (drop a produced
value) — the inadmissible codomain narrowing. The theorem's side
condition `Range f Dom ⊆ K'` is what forbids this. We pick a `K'`
that omits a produced value and try to show every output lands in
`K'`; it fails.

Run:  lake env lean mutants/mut_shrink_codomain_drop_value.lean
Expect: a produced value is not in the shrunken codomain.
-/
import Operations
open SSM.Fn Set
set_option autoImplicit false

def f : Bool → Bool := fun b => b
def Dom : Set Bool := Set.univ
-- K' drops `false`, which `f` produces from `false ∈ Dom`.
def K' : Set Bool := {true}

example : ∀ x ∈ Dom, f x ∈ K' := by
  intro x _hx
  -- goal: x ∈ {true} for all x — false at x = false. Bogus close.
  exact rfl
