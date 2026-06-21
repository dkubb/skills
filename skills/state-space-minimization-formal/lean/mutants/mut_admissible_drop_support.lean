/-
MUTANT (must FAIL to compile) for headline
`admissibleRewrite_trans` / def `AdmissibleRewrite`.

Mutation: drop the support-subset premise (keep only denotational
equivalence). Then "admissibility" no longer bounds new unintended
readings, and transitivity's conclusion — the support-subset chain —
cannot be reconstructed. We exhibit two rewrites that agree
denotationally but introduce a new unintended reading, and try to
prove the support-subset conclusion; it fails.

Run:  lake env lean mutants/mut_admissible_drop_support.lean
Expect: cannot prove the support-subset goal.
-/
import Reception
open SSM.Reception Set
set_option autoImplicit false

-- One denotation, one prior state/context, three texts where t'' adds
-- an unintended reading absent from t.
def t0 : Text Unit Bool Unit := { den := (), supp := fun _ _ => {true} }
def t2 : Text Unit Bool Unit := { den := (), supp := fun _ _ => {true, false} }

-- With the support premise DROPPED, admissibility would be just
-- `den = den`. The genuine `AdmissibleRewrite` premise fails here:
-- `t2.supp \ ∅ = {true,false} ⊄ t0.supp = {true}`.
example : AdmissibleRewrite (∅ : Set Bool) t0 t2 := by
  refine ⟨rfl, ?_⟩
  intro q c x hx
  -- goal: x ∈ {true}; but x may be `false`. Bogus close.
  exact hx.1
