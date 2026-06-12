/-
Lean reflection of the state-space-minimization calculus in
`../SKILL.md`. See README.md for purpose, provenance, determinant,
naming parity, scope, and build instructions.
-/

import Mathlib.Data.Set.Basic
import Mathlib.Order.Basic

open Set

universe u v w

/-- SKILL.md § "Core Notation": `S(A)`, `C(A)` with `C(A) ⊆ S(A)`
structural, and `B(A)` as behavior over representable state. One
common `State` universe is the chosen embedding, fixed up front per
§ "Artifact Calculus". -/
structure Artifact (State : Type u) (Obs : Type v) where
  S : Set State
  C : Set State
  C_subset_S : C ⊆ S
  B : {x : State // x ∈ S} → Obs

namespace SSM

variable {State : Type u} {Obs : Type v}

/-- SKILL.md § "Core Notation": `I_repr(A) = S(A) \ C(A)`. -/
def I_repr (A : Artifact State Obs) : Set State :=
  A.S \ A.C

/-- SKILL.md § "Core Notation": `B(A)|_C` — behavior restricted to
the contract set. -/
def B_restrict (A : Artifact State Obs) :
    {x : State // x ∈ A.C} → Obs :=
  fun x => A.B ⟨x.1, A.C_subset_S x.2⟩

/-- SKILL.md § "Boundary Calculus": `b: (u: U) × P(u) -> A`. The
subtype codomain makes `R(b) ⊆ S(A)` structural — constructive
rather than a side-condition field. -/
structure Boundary (A : Artifact State Obs) (U : Type w) where
  P : U → Prop
  b : (u : U) → P u → {x : State // x ∈ A.S}

/-- SKILL.md § "Core Notation": `R(b)`, the reachable states. -/
def R {A : Artifact State Obs} {U : Type w}
    (bd : Boundary A U) : Set State :=
  {x | ∃ u, ∃ p : bd.P u, (bd.b u p).1 = x}

/-- SKILL.md § "Core Notation": `I_reach(A,b) = R(b) ∩ I_repr(A)`. -/
def I_reach {A : Artifact State Obs} {U : Type w}
    (bd : Boundary A U) : Set State :=
  R bd ∩ I_repr A

/-- SKILL.md § "Boundary Calculus", boundary-introduction rule:
discharged evidence lands the value in `C(A)` — here by shape, so a
trusted value without contract evidence is unrepresentable. -/
structure TrustedBoundary (A : Artifact State Obs) (U : Type w) where
  P : U → Prop
  b : (u : U) → P u → {x : State // x ∈ A.C}

def TrustedBoundary.toBoundary {A : Artifact State Obs} {U : Type w}
    (bd : TrustedBoundary A U) : Boundary A U where
  P := bd.P
  b := fun u p => ⟨(bd.b u p).1, A.C_subset_S (bd.b u p).2⟩

/-- Proves SKILL.md § "Boundary Calculus": a trusted boundary gives
`R(b) ⊆ C(A)`. -/
theorem trusted_boundary_reaches_valid
    {A : Artifact State Obs} {U : Type w}
    (bd : TrustedBoundary A U) :
    R bd.toBoundary ⊆ A.C := by
  intro x hx
  rcases hx with ⟨u, p, rfl⟩
  exact (bd.b u p).2

/-- Proves SKILL.md § "Constructive vs predicative": a trusted
boundary yields `I_reach(A,b) = ∅`. -/
theorem trusted_boundary_no_reachable_invalid
    {A : Artifact State Obs} {U : Type w}
    (bd : TrustedBoundary A U) :
    I_reach bd.toBoundary = ∅ := by
  ext x
  constructor
  · intro hx
    rcases hx with ⟨hR, hInvalid⟩
    exact False.elim (hInvalid.2 (trusted_boundary_reaches_valid bd hR))
  · intro hx
    exact False.elim (Set.notMem_empty x hx)

/-- SKILL.md § "Artifact Calculus", Strict premise `C(A') = C(A)`:
"the contract is pinned". -/
def ContractPinned (A A' : Artifact State Obs) : Prop :=
  A'.C = A.C

/-- SKILL.md § "Artifact Calculus", Strict premise `C(A) ⊆ S(A')`:
the well-formedness obligation on the embedding. -/
def ContractStatesPreserved (A A' : Artifact State Obs) : Prop :=
  A.C ⊆ A'.S

/-- SKILL.md § "Artifact Calculus", Strict premise
`B(A')|_C = B(A)|_C`: contract behavior preserved, restricted on
both sides. -/
def ContractBehaviorPreserved (A A' : Artifact State Obs)
    (h : A.C ⊆ A'.S) : Prop :=
  ∀ x (hx : x ∈ A.C),
    A'.B ⟨x, h hx⟩ =
    A.B ⟨x, A.C_subset_S hx⟩

/-- SKILL.md § "Artifact Calculus": the `Strict(A, A')` rule — a
refinement step in which `I_repr` strictly shrinks. -/
def Strict (A A' : Artifact State Obs) : Prop :=
  ContractPinned A A' ∧
  ∃ h : A.C ⊆ A'.S,
    I_repr A' ⊂ I_repr A ∧
    ContractBehaviorPreserved A A' h

/-- SKILL.md § "Encoding Order": ranks are positive positions in a
per-architecture derived order — hence not a closed enum. -/
structure Rank where
  index : Nat
  positive : 0 < index

/-- SKILL.md § "Inputs": a candidate mechanism `m` inducing `m(A)`.
The contract pin `C(m(A)) = C(A)` is required via `BehaviorOK`. -/
structure Mechanism (State : Type u) (Obs : Type v) where
  apply : Artifact State Obs → Artifact State Obs
  rank : Rank
  cost : Nat

def Residual (m : Mechanism State Obs) (A : Artifact State Obs) : Set State :=
  I_repr (m.apply A)

def BehaviorOK (m : Mechanism State Obs) (A : Artifact State Obs) : Prop :=
  ContractPinned A (m.apply A) ∧
  ∃ h : A.C ⊆ (m.apply A).S,
    ContractBehaviorPreserved A (m.apply A) h

/-- SKILL.md § "Encoding Order": `Sufficient(m)` strengthened for
rank-1 mechanisms to `I_repr(m(A)) = ∅`. -/
def SufficientRank1 (m : Mechanism State Obs) (A : Artifact State Obs) : Prop :=
  I_repr (m.apply A) = ∅

/-- SKILL.md § "Encoding Order": `Sufficient(m)` — `I_reach = ∅` for
every construction path. Per § "Inputs", `Paths` must be the
enumerated construction-path set, not a convenient subset. -/
def SufficientWithPaths
    (m : Mechanism State Obs) (A : Artifact State Obs)
    (Paths : Type w)
    (path : Paths → Σ U : Type w, Boundary (m.apply A) U) : Prop :=
  ∀ p : Paths, I_reach (path p).2 = ∅

/-- SKILL.md § "Artifact Calculus": ⊆-minimality of the residual —
inclusion, never cardinality; incomparable residuals stay
unordered. -/
def SubsetMinimalOn
    {M : Type w} (Candidates : Set M) (resid : M → Set State) (m : M) : Prop :=
  m ∈ Candidates ∧
  ∀ n ∈ Candidates, resid n ⊆ resid m → resid m ⊆ resid n

/-- SKILL.md § "Encoding Order": "Only sufficient mechanisms compete
for rank" — eligibility gates the whole selection. -/
def EligibleMechanisms
    (Candidates : Set (Mechanism State Obs))
    (A : Artifact State Obs)
    (Sufficient : Mechanism State Obs → Prop) :
    Set (Mechanism State Obs) :=
  {n | n ∈ Candidates ∧ Sufficient n ∧ BehaviorOK n A}

def EarliestSufficient
    (Candidates : Set (Mechanism State Obs))
    (A : Artifact State Obs)
    (Sufficient : Mechanism State Obs → Prop)
    (m : Mechanism State Obs) : Prop :=
  m ∈ EligibleMechanisms Candidates A Sufficient ∧
  SubsetMinimalOn
    (EligibleMechanisms Candidates A Sufficient) (fun n => Residual n A) m ∧
  ∀ n ∈ EligibleMechanisms Candidates A Sufficient,
    Residual n A = Residual m A → m.rank.index ≤ n.rank.index

def CostMinimalAmongTies
    (Candidates : Set (Mechanism State Obs))
    (A : Artifact State Obs)
    (Sufficient : Mechanism State Obs → Prop)
  (m : Mechanism State Obs) : Prop :=
  ∀ n ∈ EligibleMechanisms Candidates A Sufficient,
    Residual n A = Residual m A → n.rank.index = m.rank.index →
    m.cost ≤ n.cost

/-- SKILL.md § "Artifact Calculus": the lexicographic objective —
invalidity (⊆-minimal), then earliest sufficient rank, then cost. -/
def ObjectiveChoice
    (Candidates : Set (Mechanism State Obs))
    (A : Artifact State Obs)
    (Sufficient : Mechanism State Obs → Prop)
    (m : Mechanism State Obs) : Prop :=
  EarliestSufficient Candidates A Sufficient m ∧
  CostMinimalAmongTies Candidates A Sufficient m

/-- SKILL.md § "Encoding Order": the fallback when no mechanism is
sufficient — earliest mechanism that detects, documents, or rejects
the targeted invalid states; the remainder is the residual gap. -/
def FallbackChoice
    (Candidates : Set (Mechanism State Obs))
    (TargetInvalid : Set State)
    (DetectsRejectsOrDocuments : Mechanism State Obs → Set State → Prop)
    (Sufficient : Mechanism State Obs → Prop)
    (m : Mechanism State Obs) : Prop :=
  (∀ n ∈ Candidates, ¬ Sufficient n) ∧
  m ∈ Candidates ∧
  DetectsRejectsOrDocuments m TargetInvalid ∧
  ∀ n ∈ Candidates,
    DetectsRejectsOrDocuments n TargetInvalid → m.rank.index ≤ n.rank.index

/-- SKILL.md § "Constructive dominance at rank 1": `A_c ≺ A_p` when
the constructive encoding of the same pinned contract exists. -/
structure ConstructiveDominance
    (Ac Ap : Artifact State Obs) where
  contract_pinned : Ap.C = Ac.C
  constructive : Ac.S = Ac.C
  predicative_wider : Ac.C ⊂ Ap.S
  behavior_eq :
    ∀ x (hx : x ∈ Ac.C),
      Ac.B ⟨x, Ac.C_subset_S hx⟩ =
      Ap.B ⟨x, predicative_wider.subset hx⟩

/-- Proves SKILL.md § "Boundary Calculus", the boundary-introduction
rule: with `p : P(u)` discharged, `b(u,p) ∈ C(A)`. -/
theorem boundary_introduction
    {A : Artifact State Obs} {U : Type w}
    (bd : TrustedBoundary A U)
    (u : U) (p : bd.P u) :
    (bd.b u p : State) ∈ A.C :=
  (bd.b u p).2

structure Consumer (State : Type u) where
  D : Set State

/-- SKILL.md § "Proof-preservation corollary": `Obl(f)` — the
obligation that this consumer's actual input came through the
trusted boundary. Provenance per value, not boundary validity. -/
def Obl
    (A : Artifact State Obs)
    (f : Consumer State)
    {U : Type w}
    (bd : TrustedBoundary A U)
    (x : State) : Prop :=
  f.D = A.S ∧ x ∈ f.D ∧ x ∈ R bd.toBoundary

end SSM
