---
name: state-space-minimization-formal
description: >-
  Notation and inference rules for state-space minimization:
  S/C/I_repr/I_reach state sets, boundary morphisms, strictness and
  dominance rules, the encoding order, and proof obligations.
  Two uses: the output layer when the deliverable is formal, and a
  reasoning mode when it is not — working in the calculus activates
  formal-methods reasoning and surfaces constraints that prose
  analysis misses. Semantics live in the state-space-minimization
  skill's reference modules.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-05-v1"
triggers:
  - "state-space-minimization-formal"
  - "formal state-space minimization"
  - "proof obligation"
  - "inference rule"
  - "calculus"
  - "artifact calculus"
  - "definition-only"
  - "denotational"
---

# State Space Minimization Formal

This skill is the notation-and-inference-rule layer over
`state-space-minimization`. That skill's reference modules are
determinant for semantics — what each concept means and when it
applies. This skill is determinant for notation and inference
rules — how the obligations are written and discharged. Where a
section cites a module, the module owns the meaning.

The calculus is standard formal-methods machinery under local
names: `Strict(A, A')` is a refinement step (contract preserved
while the representable set shrinks); "invalid states
unrepresentable" is a safety invariant discharged structurally;
each rule's premises are proof obligations in the
refinement-calculus sense; the inclusion order on encodings is a
refinement order. Reason in that register — exhaustive case
analysis, every obligation discharged or explicitly recorded as
open, no claim without a premise.

The notation is also compression: a derivation in the calculus
packs more reasoning into fewer tokens than its prose equivalent,
and committing to symbols keeps later steps anchored to exact
referents. Prefer it for internal reasoning and scratch work even
when nothing formal was requested. Notation surfacing in
intermediate output is acceptable; only the final deliverable must
match the requested form.

## When to Activate

- Required output is formal, theoretical, proof-like, or definition-only.
- The task asks for state spaces, invariants, morphisms, ranges, gaps,
  receiver-state effects, or proof obligations.
- The ordinary `state-space-minimization` skill is too operational or verbose
  for the requested form.
- The analysis needs more rigor than prose is producing — even when the
  deliverable is ordinary code or design. Switch to the calculus as a
  reasoning mode, derive in notation, then translate back. The notation
  forces commitments prose lets slide: `S` and `C` must be stated to be
  related, every premise of a rule must be discharged, and the
  `I_repr` / `I_reach` distinction cannot be blurred. Reasoning in this
  register tends to find further narrowings of the state space that
  informal analysis misses.

## When Not to Use

- The user wants examples, implementation tactics, or idiom catalogues.
- A concrete language module is required.
- The task needs explanatory prose more than formal structure.

Using the calculus internally as a reasoning step and translating the
result into the deliverable's form is always admissible; these bullets
gate the output form, not the reasoning mode.

## Inputs

- Artifact `A`.
- Contract `C(A)`.
- Boundary morphisms `b: (u: U) × P(u) -> A`, consuming raw input `u`
  and evidence `p: P(u)`. The evidence-free `b: U -> A` is the derived
  special case of trivial `P`.
- Functions `f: D -> K`.
- Reader/model state `q` and intended receiver-state set
  `Q_intended`, when text reception matters.
- Thresholds `t` with stricter candidates `t'` and the current
  evidence, when ratcheting.
- Evidence set `E`.

## Core Notation

- `S(A)` — representable states (structural; what the type's shape permits).
- `C(A)` — admissible states (contract).
- `R(b)` — reachable states under a boundary constructor
  `b: (u: U) × P(u) -> A`.
- `I_repr(A)` — representable invalid: `S(A) \ C(A)`.
- `I_reach(A,b)` — reachable invalid: `R(b) ∩ I_repr(A)`.
- `B(A)` — contract behavior.
- `D(f)`, `K(f)`, `R(f)`, `G(f)` — domain, codomain, range, gap.
- `cost(m)` — the audit burden of mechanism `m`: the trusted surface it
  adds plus its ergonomic and toolchain cost. Semantics:
  `state-space-minimization` `references/least-power.md`.
- `supp(P)` — the support of distribution `P`: the outcomes with
  nonzero probability.
- `Needs(y, P(x))` — consumer `y`'s contract depends on evidence
  `P(x)`.
- `[[t]]^D`, `[[t]]^O_c` — denotation and operational reception.

$$
\begin{array}{rcl}
S(A) &:=& \text{representable states (structural)} \\
C(A) &\subseteq& S(A) \\
R(b) &:=& \{\, b(u,p) \mid u \in U,\; p \in P(u) \,\} \subseteq S(A) \\
I_{\mathrm{repr}}(A) &:=& S(A) \setminus C(A) \\
I_{\mathrm{reach}}(A,b) &:=& R(b) \cap I_{\mathrm{repr}}(A)
\end{array}
$$

Always subscript the invalid set. `I_repr` and `I_reach` answer
different questions, and an unsubscripted `I` silently equates a
predicative encoding with a constructive one.

$$
\begin{array}{rcl}
R(f) &:=& \{\, f(x) \mid x \in D(f) \,\} \\
G(f) &:=& K(f) \setminus R(f) \\
f^{-1}(F) &:=& \{\, x \in D(f) \mid f(x) \in F \,\}
\end{array}
$$

### Constructive vs predicative

- **Constructive encoding**: `S(A) = C(A)`, so `I_repr(A) = ∅`. The proof
  lives in the type's shape and is preserved across every consumer.
- **Predicative encoding**: `S(A) ⊋ C(A)`, so `I_repr(A) ≠ ∅`. A trusted
  boundary `b` restricts `R(b) ⊆ C(A)`, so `I_reach(A,b) = ∅`, but `I_repr`
  remains and proof obligations must be reconstituted at every consumer
  that reasons about `S(A)`.

A smart constructor narrows `R(b)`, not `S`. Equating `I_reach = ∅` with
`I_repr = ∅` is the most common slip in informal SSM analysis.

Semantics: `state-space-minimization`
`references/constructive-vs-predicative.md` (intrinsic vs extrinsic
safety, the trusted-boundary audit, the hard-case fallbacks).

### Proof-preservation corollary

For predicative `A`, every consumer `f` whose contract `B(f)` is defined
on `C(A)` but typed on `S(A)` carries an obligation `P_f` that the input
came through `R(b)`. Adding a second construction path `b' ≠ b` with
`R(b') ⊄ C(A)` voids `P_f` globally.

For constructive `A`, `P_f` is discharged by the type and survives the
addition of arbitrary new construction paths.

## Artifact Calculus

- Minimization is contract-preserving invalidity reduction.
- Strictness is admissible only as a contract-preserving transformation.
- Mechanism choice is lexicographic: invalidity first, cost second.
- A mechanism that detects, documents, or rejects invalid states does not
  necessarily shrink `S(A)`. Smart constructors, validators, and
  refinement-library wrappers narrow `R(b)`, not `S`.

$$
\begin{array}{c}
C(A) \subseteq S(A')
\qquad
I_{\mathrm{repr}}(A') \subset I_{\mathrm{repr}}(A)
\qquad
B(A') = B(A)|_{C(A)}
\\
\hline
\operatorname{Strict}(A,A')
\end{array}
$$

The first premise is explicit, not derived: a strictness step may
not delete any state the contract requires. `B(A') = B(A)|_{C(A)}`
presupposes it, but the obligation stands on its own.

$$
\begin{array}{rl}
\text{choose } m: & I_{\mathrm{repr}}(m(A)) \;\; \subseteq\text{-minimal} \\
\text{then} & \operatorname{cost}(m) \text{ minimal among those} \\
\text{s.t.} & B(m(A)) = B(A)|_{C(A)}
\end{array}
$$

Invalidity is ordered by inclusion, not cardinality: `m` dominates
`m'` only when `I_repr(m(A)) ⊊ I_repr(m'(A))`. Incomparable
residual sets are not ranked — a count would rank them, and could
prefer the numerically smaller set that still contains the
dangerous states. The inclusion order is taken up to a chosen
embedding into a common universe of states (`S(NonEmptyString)` is
not literally a subset of `S(String)`; the embedding is part of
the analysis).

## Operations

- Shrink domain: replace `D` by `D' subset D`.
- Bound range: replace unbounded component by finite, ordered, or measured
  subset.
- Shrink codomain: replace `K` by `K'` where `R subset K' subset K`.
- Remove intermediate: compose `g . f` and eliminate exposed state `W` when
  `W` has no independent contract role. (`W`, not `X`: `X` is
  reserved for functional-dependency determinants `X → Y` in
  `state-space-minimization` `references/normalization.md`.)
- Normalize: decompose facts into determinants, remove transitively derivable
  facts, recompose along use.
- Ratchet: replace threshold `t` by stricter `t'` when current evidence
  satisfies `t'`.

These are the six operations of `state-space-minimization`
`references/principles.md` § "Six operations"; `normalization.md`
and `ratchet.md` there own the last two in depth (including when an
estimated bound may move the other way).

## Encoding Order

The **encoding order** ranks mechanism classes. Use the earliest
sufficient mechanism:

$$
\begin{array}{c|c}
k & \text{mechanism} \\
\hline
1 & \text{type or representation} \\
2 & \text{constructor or parser} \\
3 & \text{boundary adapter} \\
4 & \text{schema constraint} \\
5 & \text{test oracle} \\
6 & \text{documentation claim} \\
7 & \text{runtime assertion}
\end{array}
$$

$$
\begin{array}{c}
I_{\mathrm{repr}}(m_i(A)) = I_{\mathrm{repr}}(m_j(A))
\qquad
B(m_i(A)) = B(A)|_{C(A)}
\qquad
B(m_j(A)) = B(A)|_{C(A)}
\qquad
i < j
\\
\hline
m_i \prec m_j
\end{array}
$$

The rank metric: a mechanism ranks earlier when it governs more of
the artifact's construction paths and detects violations at an
earlier phase. The table is the derived order for an
application-owned artifact, not an axiom — derive the ranks per
architecture. In a database-first system with multiple writers, a
schema constraint governs every write path while an application
boundary adapter governs one, and outranks it. Rank order is
least-power order under this metric: the earliest sufficient
mechanism is the least powerful one that still removes the invalid
set (`state-space-minimization` `references/least-power.md` owns
the principle).

The choice among rank-1 type-level mechanisms (enums, constructive
datatypes, typestate, phantom tags, GADTs, refinement and
dependent types) is ordered by the **mechanism ladder** in
`state-space-minimization` `references/principles.md` § "Encode
invariants into types"; that ladder's lowest rungs (runtime
checks, smart constructors) correspond to this table's later
ranks. The two ladders are distinct: the encoding order ranks
mechanism classes, the mechanism ladder ranks concrete mechanisms
by guarantee strength.

### Constructive dominance at rank 1

Within rank 1, encodings are ordered by inclusion of `S` (up to
the chosen embedding). A constructive encoding `A_c` with
`S(A_c) = C(A)` strictly dominates any predicative encoding `A_p`
with `S(A_p) ⊋ C(A)` — then `S(A_c) ⊊ S(A_p)` — even when
`I_reach(A_p, b) = ∅`.

$$
\begin{array}{c}
S(A_c) = C(A)
\qquad
S(A_p) \supsetneq C(A)
\qquad
I_{\mathrm{reach}}(A_p, b) = \varnothing
\\
\hline
A_c \prec A_p
\end{array}
$$

Rationale: `A_c` discharges every consumer proof obligation `P_f`
structurally and is monotone under the addition of new construction
paths. `A_p` discharges `P_f` only through the trusted boundary `b`, and
any new `b'` with `R(b') ⊄ C(A)` voids `P_f`. A swap that holds `I_reach`
constant while holding `S` and `R(b)` fixed (predicative-to-predicative
mechanism swap, e.g., relocating the check from a hand-written
constructor to a refinement library) is **not** a narrowing; it changes
neither `S` nor `R(b)`.

## Invariants

- Contract preservation: `B(A') = B(A)|_{C(A)}`.
- Boundary monotonicity: trust may increase only through
  `b: (u: U) × P(u) -> A`.
- Proof preservation: no downstream operation depends on erased proof.
- Totality: every exposed function is total on its declared domain.
- Exhaustiveness: closed sums have no catch-all branch without an external
  codomain.
- Single source: each fact has one determinant.
- Confluence: normalization order does not affect normal form.

## Boundary Calculus

- Trust increases only through boundary morphisms.
- Proof erasure is admissible only when no downstream consumer requires the
  erased proof.

$$
\begin{array}{c}
u \in U
\qquad
b: (u: U) \times P(u) \to A
\qquad
p: P(u)
\\
\hline
b(u,p) \in A
\end{array}
$$

$$
\begin{array}{c}
(x,p) \mapsto x
\qquad
\operatorname{Needs}(y,P(x))
\\
\hline
\operatorname{ProofObligation}(y,P(x))
\end{array}
$$

Semantics: `state-space-minimization`
`references/ingress-and-boundaries.md` (boundary parsing,
capability tokens, temporal invariants) and
`references/proof-preservation.md` (proofs that survive
conversions).

## Reception Semantics

- Text has denotational content and operational effect.
- Prose, ordering, notation, omissions, and repetition are state controls.
- Rewrite text only when denotation is preserved and intended receiver states
  are narrowed.

$$
\begin{array}{rcl}
[[t]]^D &:=& \text{denotation of } t \\
[[t]]^O_c &:=& \Pr(q' \mid q,t,c) \\
t \equiv_D t' &:=& [[t]]^D = [[t']]^D
\end{array}
$$

$$
\begin{array}{c}
t \equiv_D t'
\qquad
\operatorname{supp}([[t']]^O_c) \subseteq \operatorname{supp}([[t]]^O_c)
\qquad
\operatorname{supp}([[t']]^O_c) \subseteq Q_{\mathrm{intended}}
\\
\hline
\operatorname{ReceptionNarrowing}(t,t')
\end{array}
$$

The support premises are proof obligations like every other
premise: discharged by argument about the plausible readings of
`t'`, not computed. A rewrite is admissible only when it preserves
denotation and every reading it leaves open is an intended one.

Semantics: `state-space-minimization`
`references/documentation.md` (the reader's mental model as the
state space a text constrains).

## Self-Similarity

- The skill is an artifact.
- Its rules are representable states.
- Concrete cases reveal whether the codomain is too wide, too narrow, or
  missing a distinction.

Semantics: `state-space-minimization` `references/principles.md`
§ "Self-similarity"; the operational audit discipline is
`references/skill-refinement.md`.

$$
\begin{array}{rcl}
S(\operatorname{Skill}) &:=& \text{representable rules} \\
\operatorname{Req}(E) &:=& \text{distinctions required by evidence } E \\
K(\operatorname{Skill}) &:=& \text{distinctions expressible by the skill}
\end{array}
$$

`Req(E)` is named apart from the range notation `R(f)`/`R(b)`: it
is a requirement set, not a range. With `Req(E)` as the skill's
range and `K(Skill)` as its codomain, `K(Skill) \ Req(E)` is the
gap `G` applied to the skill itself.

$$
\begin{array}{c}
\operatorname{Req}(E) \not\subseteq K(\operatorname{Skill})
\\
\hline
\operatorname{MissingDistinction}
\end{array}
$$

$$
\begin{array}{c}
K(\operatorname{Skill}) \setminus \operatorname{Req}(E) \neq \varnothing
\\
\hline
\operatorname{SharpeningCandidate}
\end{array}
$$

## Process

1. State `S`, `C`, `I_repr = S \ C`, and, when boundaries exist,
   `R(b)` and `I_reach`. Never report `I = ∅` without naming which.
2. Locate the boundary morphisms `b: (u: U) × P(u) -> A`.
3. Compute domain, codomain, range, gaps, and failure preimages.
4. Select the least encoding that removes the invalid set. When choosing
   among rank-1 encodings, prefer the one whose `S` is strictly included
   in the alternative's (constructive dominance); when choosing among
   predicative encodings of equal `S` and `R(b)`, the swap is not a
   narrowing and the dominance rule does not apply.
5. Prove contract preservation.
6. Preserve or reconstitute proofs across morphisms.
7. Normalize duplicated determinants.
8. If text reception matters, model `[[t]]^D` and `[[t]]^O_c`.
9. In output-layer mode, emit only definitions, equations, derived
   obligations, and the selection justification (the dominance
   reason). In reasoning mode, translate the derivation into the
   deliverable's form; the notation remains as intermediate work.

## Outputs

In output-layer mode, emit the items below. In reasoning mode they
are derived internally and translated into the deliverable's form.

- Invalid-state set.
- Chosen encoding and dominance reason.
- Boundary morphisms.
- Contract-preservation obligation.
- Proof-preservation obligation.
- Reception-narrowing obligation, when text is the artifact.
- Normal form.
- Residual gap, if nonempty.

## Validation Checklist

- `S`, `C`, `I_repr = S \ C`, and (where boundaries exist) `R(b)` and
  `I_reach = R(b) ∩ I_repr` are explicit. No bare `I = ∅` claims.
- Every narrowing is contract-preserving.
- Predicative-to-predicative mechanism swaps are not counted as
  narrowings; constructive dominance applies only when `S` strictly
  shrinks by inclusion.
- Every boundary changes trust explicitly.
- Every proof dependency is preserved.
- Every duplicate fact has one determinant.
- Text rewrites preserve denotation while narrowing reception states.
- The selected mechanism is least-power among sufficient mechanisms.
- In output-layer mode the answer is formal, concise, and
  self-contained at the notation level; in reasoning mode the
  derivation is translated faithfully into the deliverable's form.
