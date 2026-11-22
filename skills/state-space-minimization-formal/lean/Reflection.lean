/-
Lean formalization of the state-space-minimization calculus that
`../SKILL.md` OWNS. Formalization version: 2026-06-v8 — versioned
INDEPENDENTLY of `../SKILL.md`; it tracks the skill's calculus by
content and review, not by a shared stamp (the `check_claims` version
gate cross-checks only the Lean-internal stamps and never reads
SKILL.md). This module is a DERIVED, NON-NORMATIVE reflection: if it
and `../SKILL.md` disagree, SKILL.md is right. See README.md for
purpose, provenance, determinant, naming parity, scope, and build
instructions.
-/

import Mathlib.Data.Set.Basic
import Mathlib.Order.Basic

set_option autoImplicit false

open Set

universe u v w z

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

/-- SKILL.md § "Inputs": "The construction paths of `A`: the set of
all boundary morphisms that can produce values of `A`. Quantifiers
`∀b` range over this set; it must be enumerated, not assumed."
An enumerated path set is an analysis INPUT (so a sufficiency proof
cannot substitute a different path domain than `m.paths A ps`; any
leaky path retained by `m.paths A ps` cannot be omitted). Whether `ps`
and `m.paths A ps` faithfully reflect every path of real code (including
any paths `m` claims to remove) is an operator-policy residual
(`select.path_enumeration_residual`). -/
structure PathSet (A : Artifact State Obs) where
  Paths : Type w
  path : Paths → Σ U : Type w, Boundary A U

/-- Total reachable state set across all enumerated construction paths. -/
def Reach {A : Artifact State Obs} (ps : PathSet.{u, v, w} A) : Set State :=
  {x | ∃ p : ps.Paths, x ∈ R (ps.path p).2}

/-- SKILL.md § "Inputs": a candidate mechanism `m` inducing both the
re-encoded artifact `m(A)` and its transformed construction paths
`m.paths A ps`. A rank-2 smart constructor holds `S` fixed while
narrowing `paths` to trusted boundaries. The contract pin
`C(m(A)) = C(A)` is required via `BehaviorOK`. -/
structure Mechanism (State : Type u) (Obs : Type v) where
  apply : Artifact State Obs → Artifact State Obs
  paths : (A : Artifact State Obs) →
    PathSet.{u, v, w} A → PathSet.{u, v, w} (apply A)
  rank : Rank

def Residual (m : Mechanism.{u, v, w} State Obs) (A : Artifact State Obs) :
    Set State :=
  I_repr (m.apply A)

def BehaviorOK (m : Mechanism.{u, v, w} State Obs) (A : Artifact State Obs) :
    Prop :=
  ContractPinned A (m.apply A) ∧
  ∃ h : A.C ⊆ (m.apply A).S,
    ContractBehaviorPreserved A (m.apply A) h

/-- SKILL.md § "Encoding Order": `Sufficient(m)` strengthened for
rank-1 mechanisms to `I_repr(m(A)) = ∅`. -/
def SufficientRank1 (m : Mechanism.{u, v, w} State Obs)
    (A : Artifact State Obs) : Prop :=
  I_repr (m.apply A) = ∅

/-- Every enumerated construction path of `m(A)` reaches no invalid
state (`I_reach(m(A), b) = ∅` for every path `b` in `m.paths A ps`). -/
def PathsClean (m : Mechanism.{u, v, w} State Obs) (A : Artifact State Obs)
    (ps : PathSet.{u, v, w} A) : Prop :=
  ∀ p : (m.paths A ps).Paths, I_reach ((m.paths A ps).path p).2 = ∅

/-- SKILL.md § "Encoding Order": `Sufficient(m)` := `I_reach(m(A), b) = ∅`
for every enumerated construction path `b` (given by `m.paths A ps`);
for rank-1 mechanisms (`m.rank.index = 1`) this strengthens to
`I_repr(m(A)) = ∅`. For rank ≥ 2 (e.g., smart constructors), `I_repr`
may remain nonempty while `Sufficient` holds. -/
def Sufficient (m : Mechanism.{u, v, w} State Obs) (A : Artifact State Obs)
    (ps : PathSet.{u, v, w} A) : Prop :=
  (m.rank.index = 1 → SufficientRank1 m A) ∧ PathsClean m A ps

/-- Optional liveness property (separate from `Sufficient`): every
contract state remains constructible through some enumerated path.
Not baked into `Sufficient` or `EligibleMechanisms` because SKILL.md
imposes no universal reachability requirement (e.g., conservative
parsers may have `Reach ⊊ C`). -/
def ContractReachable {A : Artifact State Obs}
    (ps : PathSet.{u, v, w} A) : Prop :=
  A.C ⊆ Reach ps

/-- Proves `PathsClean` is equivalent to the aggregate reachable set
being disjoint from `I_repr(m(A))`. -/
theorem pathsClean_iff_reach_inter
    {m : Mechanism.{u, v, w} State Obs} {A : Artifact State Obs}
    {ps : PathSet.{u, v, w} A} :
    PathsClean m A ps ↔ Reach (m.paths A ps) ∩ I_repr (m.apply A) = ∅ := by
  constructor
  · intro h
    ext x
    constructor
    · rintro ⟨⟨p, hx⟩, hI⟩
      have hmem : x ∈ I_reach ((m.paths A ps).path p).2 := ⟨hx, hI⟩
      rw [h p] at hmem
      exact Set.notMem_empty x hmem
    · intro hx
      exact (Set.notMem_empty x hx).elim
  · intro h p
    ext x
    constructor
    · rintro ⟨hx, hI⟩
      have hmem : x ∈ Reach (m.paths A ps) ∩ I_repr (m.apply A) := ⟨⟨p, hx⟩, hI⟩
      rw [h] at hmem
      exact Set.notMem_empty x hmem
    · intro hx
      exact (Set.notMem_empty x hx).elim

/-- Constructive sufficiency (`I_repr(m(A)) = ∅`) implies `Sufficient`
for EVERY input path set `ps` — constructive encodings survive the
addition of arbitrary construction paths. -/
theorem rank1_sufficient_any_paths
    {m : Mechanism.{u, v, w} State Obs} {A : Artifact State Obs}
    (h : SufficientRank1 m A) (ps : PathSet.{u, v, w} A) :
    Sufficient m A ps := by
  refine ⟨fun _ => h, ?_⟩
  intro p
  ext x
  constructor
  · rintro ⟨_, hI⟩
    rw [show I_repr (m.apply A) = ∅ from h] at hI
    exact Set.notMem_empty x hI
  · intro hx
    exact (Set.notMem_empty x hx).elim

/-- SKILL.md § "Encoding Order": for rank-1 mechanisms, `Sufficient`
is equivalent to `SufficientRank1` (`I_repr(m(A)) = ∅`). -/
theorem rank1_sufficient_iff
    {m : Mechanism.{u, v, w} State Obs} {A : Artifact State Obs}
    {ps : PathSet.{u, v, w} A} (hr1 : m.rank.index = 1) :
    Sufficient m A ps ↔ SufficientRank1 m A :=
  ⟨fun h => h.1 hr1, fun h => rank1_sufficient_any_paths h ps⟩

/-- Anti-vacuity refutation: if ANY enumerated path in `m.paths A ps`
reaches ANY invalid state `x ∈ I_repr(m(A))`, then `m` is NOT
sufficient — a leaky path cannot be hidden or dropped. -/
theorem leaky_path_not_sufficient
    {m : Mechanism.{u, v, w} State Obs} {A : Artifact State Obs}
    {ps : PathSet.{u, v, w} A}
    (p : (m.paths A ps).Paths)
    {x : State}
    (hxR : x ∈ R ((m.paths A ps).path p).2)
    (hxI : x ∈ I_repr (m.apply A)) :
    ¬ Sufficient m A ps := by
  intro h
  have hmem : x ∈ I_reach ((m.paths A ps).path p).2 := ⟨hxR, hxI⟩
  rw [h.2 p] at hmem
  exact Set.notMem_empty x hmem

/-- Liveness refutation: an empty path set fails `ContractReachable`
whenever the contract `A.C` is nonempty. -/
theorem empty_paths_not_contract_reachable
    {A : Artifact State Obs} (ps : PathSet.{u, v, w} A)
    (hEmpty : IsEmpty ps.Paths)
    {x : State} (hx : x ∈ A.C) :
    ¬ ContractReachable ps := by
  intro h
  obtain ⟨p, _⟩ := h hx
  exact hEmpty.false p

/-- Non-collapse separation witness (SKILL.md § "Constructive vs
predicative"): there exists a rank-2 predicative mechanism (`I_repr ≠ ∅`)
that is `Sufficient` (and `ContractReachable`) for every input path set.
Proves `Sufficient` does NOT collapse into `SufficientRank1`. -/
theorem predicative_sufficient_separation :
    ∃ (A : Artifact Bool Unit) (m : Mechanism.{0, 0, 0} Bool Unit),
      m.rank.index = 2 ∧
      I_repr (m.apply A) ≠ ∅ ∧
      ∀ (ps : PathSet.{0, 0, 0} A),
        Sufficient m A ps ∧ ContractReachable (m.paths A ps) := by
  let Apred : Artifact Bool Unit :=
    { S := Set.univ
      C := {true}
      C_subset_S := fun _ _ => trivial
      B := fun _ => () }
  let trusted : TrustedBoundary Apred Unit :=
    { P := fun _ => True
      b := fun _ _ => ⟨true, rfl⟩ }
  let smart : Mechanism.{0, 0, 0} Bool Unit :=
    { apply := fun _ => Apred
      paths := fun _ _ =>
        { Paths := Unit
          path := fun _ => ⟨Unit, trusted.toBoundary⟩ }
      rank := ⟨2, Nat.zero_lt_two⟩ }
  refine ⟨Apred, smart, rfl, ?_, ?_⟩
  · intro hEmpty
    have hFalse : false ∈ I_repr Apred := ⟨trivial, fun h => by cases h⟩
    rw [hEmpty] at hFalse
    exact Set.notMem_empty false hFalse
  · intro ps
    refine ⟨⟨fun h => (by cases h), ?_⟩, ?_⟩
    · intro _
      exact trusted_boundary_no_reachable_invalid trusted
    · intro x hx
      exact ⟨(), (), trivial, (show x = true from hx).symm⟩

/-- SKILL.md § "Artifact Calculus": ⊆-minimality of the residual —
inclusion, never cardinality; incomparable residuals stay
unordered. -/
def SubsetMinimalOn
    {M : Type z} (Candidates : Set M) (resid : M → Set State) (m : M) : Prop :=
  m ∈ Candidates ∧
  ∀ n ∈ Candidates, resid n ⊆ resid m → resid m ⊆ resid n

/-- SKILL.md § "Encoding Order": "Only sufficient mechanisms compete
for rank" — eligibility gates the whole selection, wired directly to
the concrete `Sufficient` predicate over the input `PathSet`. -/
def EligibleMechanisms
    (Candidates : Set (Mechanism.{u, v, w} State Obs))
    (A : Artifact State Obs)
    (ps : PathSet.{u, v, w} A) :
    Set (Mechanism.{u, v, w} State Obs) :=
  {n | n ∈ Candidates ∧ Sufficient n A ps ∧ BehaviorOK n A}

/-- SKILL.md § "Artifact Calculus": the selection objective — among
eligible mechanisms, ⊆-minimal residual then earliest rank. -/
def EarliestSufficient
    (Candidates : Set (Mechanism.{u, v, w} State Obs))
    (A : Artifact State Obs)
    (ps : PathSet.{u, v, w} A)
    (m : Mechanism.{u, v, w} State Obs) : Prop :=
  m ∈ EligibleMechanisms Candidates A ps ∧
  SubsetMinimalOn
    (EligibleMechanisms Candidates A ps) (fun n => Residual n A) m ∧
  ∀ n ∈ EligibleMechanisms Candidates A ps,
    Residual n A = Residual m A → m.rank.index ≤ n.rank.index

/-- Proves the selection lattice is normatively sound: an
`EarliestSufficient` mechanism is genuinely `Sufficient` (over `ps`)
and preserves behavior (`BehaviorOK`). -/
theorem earliestSufficient_admissible
    {Candidates : Set (Mechanism.{u, v, w} State Obs)}
    {A : Artifact State Obs}
    {ps : PathSet.{u, v, w} A}
    {m : Mechanism.{u, v, w} State Obs}
    (h : EarliestSufficient Candidates A ps m) :
    Sufficient m A ps ∧ BehaviorOK m A :=
  ⟨h.1.2.1, h.1.2.2⟩

/-- SKILL.md § "Encoding Order": the fallback when no mechanism is
sufficient — earliest mechanism that detects, documents, or rejects
the targeted invalid states; the remainder is the residual gap. -/
def FallbackChoice
    (Candidates : Set (Mechanism.{u, v, w} State Obs))
    (TargetInvalid : Set State)
    (DetectsRejectsOrDocuments : Mechanism.{u, v, w} State Obs → Set State → Prop)
    (Sufficient : Mechanism.{u, v, w} State Obs → Prop)
    (m : Mechanism.{u, v, w} State Obs) : Prop :=
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
