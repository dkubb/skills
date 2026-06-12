/-
# Lean reflection of the state-space-minimization calculus

Provenance: produced by a Codex (GPT-class) reflection probe of
`../SKILL.md` on 2026-06-12, then graded by compilation under
Lean 4.30.0 + mathlib. Three mechanical patches were applied to the
probe output, each at a spot the probe itself flagged: a mathlib
lemma rename (`Set.notMem_empty`), threading the `A.C ⊆ A'.S` proof
into `ContractBehaviorPreserved` explicitly, and the `.subset`
accessor on `⊂`. No semantic corrections were needed.

This directory is a self-contained lake project: from here, run
`lake exe cache get` once (downloads the mathlib build cache), then
`lake build` to type-check this file.

Determinant: `../SKILL.md` owns the calculus; this file is a derived
reflection used as a comprehension/commitment probe. It is not yet
linked from the skill and is not normative. If the two disagree,
the SKILL.md is right and this file is stale.

Scope: state sets, boundaries and trust theorems, Strict, the
selection objective with `Sufficient` and the fallback, constructive
dominance, and `Obl`. Reception semantics, self-similarity, and the
proof-preservation rules (`Needs` / `Erases` / `Flows`) are not
reflected here — the probe correctly classified them as schematic or
meta-level as stated in the source.
-/

import Mathlib.Data.Set.Basic
import Mathlib.Order.Basic

open Set

universe u v w

/-- An artifact over one common state universe. -/
structure Artifact (State : Type u) (Obs : Type v) where
  S : Set State
  C : Set State
  C_subset_S : C ⊆ S
  B : {x : State // x ∈ S} → Obs

namespace SSM

variable {State : Type u} {Obs : Type v}

def I_repr (A : Artifact State Obs) : Set State :=
  A.S \ A.C

/-- Observable behavior restricted to the contract set. -/
def B_restrict (A : Artifact State Obs) :
    {x : State // x ∈ A.C} → Obs :=
  fun x => A.B ⟨x.1, A.C_subset_S x.2⟩

/-- Boundary morphism carrying explicit evidence. -/
structure Boundary (A : Artifact State Obs) (U : Type w) where
  P : U → Prop
  b : (u : U) → P u → {x : State // x ∈ A.S}

def R {A : Artifact State Obs} {U : Type w}
    (bd : Boundary A U) : Set State :=
  {x | ∃ u, ∃ p : bd.P u, (bd.b u p).1 = x}

def I_reach {A : Artifact State Obs} {U : Type w}
    (bd : Boundary A U) : Set State :=
  R bd ∩ I_repr A

/-- A trust-increasing boundary additionally lands in the contract. -/
structure TrustedBoundary (A : Artifact State Obs) (U : Type w) where
  P : U → Prop
  b : (u : U) → P u → {x : State // x ∈ A.C}

def TrustedBoundary.toBoundary {A : Artifact State Obs} {U : Type w}
    (bd : TrustedBoundary A U) : Boundary A U where
  P := bd.P
  b := fun u p => ⟨(bd.b u p).1, A.C_subset_S (bd.b u p).2⟩

theorem trusted_boundary_reaches_valid
    {A : Artifact State Obs} {U : Type w}
    (bd : TrustedBoundary A U) :
    R bd.toBoundary ⊆ A.C := by
  intro x hx
  rcases hx with ⟨u, p, rfl⟩
  exact (bd.b u p).2

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

def ContractPinned (A A' : Artifact State Obs) : Prop :=
  A'.C = A.C

def ContractStatesPreserved (A A' : Artifact State Obs) : Prop :=
  A.C ⊆ A'.S

def ContractBehaviorPreserved (A A' : Artifact State Obs)
    (h : A.C ⊆ A'.S) : Prop :=
  ∀ x (hx : x ∈ A.C),
    A'.B ⟨x, h hx⟩ =
    A.B ⟨x, A.C_subset_S hx⟩

def Strict (A A' : Artifact State Obs) : Prop :=
  ContractPinned A A' ∧
  ∃ h : A.C ⊆ A'.S,
    I_repr A' ⊂ I_repr A ∧
    ContractBehaviorPreserved A A' h

structure Rank where
  index : Nat
  positive : 0 < index

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

def SufficientRank1 (m : Mechanism State Obs) (A : Artifact State Obs) : Prop :=
  I_repr (m.apply A) = ∅

def SufficientWithPaths
    (m : Mechanism State Obs) (A : Artifact State Obs)
    (Paths : Type w)
    (path : Paths → Σ U : Type w, Boundary (m.apply A) U) : Prop :=
  ∀ p : Paths, I_reach (path p).2 = ∅

def SubsetMinimalOn
    {M : Type w} (Candidates : Set M) (resid : M → Set State) (m : M) : Prop :=
  m ∈ Candidates ∧
  ∀ n ∈ Candidates, resid n ⊆ resid m → resid m ⊆ resid n

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

def ObjectiveChoice
    (Candidates : Set (Mechanism State Obs))
    (A : Artifact State Obs)
    (Sufficient : Mechanism State Obs → Prop)
    (m : Mechanism State Obs) : Prop :=
  EarliestSufficient Candidates A Sufficient m ∧
  CostMinimalAmongTies Candidates A Sufficient m

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

structure ConstructiveDominance
    (Ac Ap : Artifact State Obs) where
  contract_pinned : Ap.C = Ac.C
  constructive : Ac.S = Ac.C
  predicative_wider : Ac.C ⊂ Ap.S
  behavior_eq :
    ∀ x (hx : x ∈ Ac.C),
      Ac.B ⟨x, Ac.C_subset_S hx⟩ =
      Ap.B ⟨x, predicative_wider.subset hx⟩

theorem boundary_introduction
    {A : Artifact State Obs} {U : Type w}
    (bd : TrustedBoundary A U)
    (u : U) (p : bd.P u) :
    (bd.b u p : State) ∈ A.C :=
  (bd.b u p).2

structure Consumer (State : Type u) where
  D : Set State

def Obl
    (A : Artifact State Obs)
    (f : Consumer State)
    {U : Type w}
    (bd : Boundary A U) : Prop :=
  f.D = A.S ∧ R bd ⊆ A.C

end SSM
