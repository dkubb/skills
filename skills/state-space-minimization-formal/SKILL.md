---
name: state-space-minimization-formal
description: >-
  Notation and inference rules for state-space minimization:
  S/C/I_repr/I_reach state sets, boundary morphisms, strictness and
  dominance rules, the encoding order, proof obligations, reception
  semantics for text rewrites, and the skill-gap rules.
  Two uses: the output layer when the deliverable is formal, and a
  reasoning mode when it is not — working in the calculus activates
  formal-methods reasoning and surfaces constraints that prose
  analysis misses. Semantics live in the state-space-minimization
  skill's reference modules.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-06-v2"
triggers:
  - "state-space-minimization-formal"
  - "formal state-space minimization"
  - "proof obligation"
  - "inference rule"
  - "artifact calculus"
  - "state-space calculus"
  - "definition-only"
  - "denotational"
  - "formalize"
  - "reason formally"
  - "formal reasoning"
  - "derive formally"
  - "refinement calculus"
  - "refinement step"
  - "refinement order"
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

Broad formal-methods terms ("formalize", "proof obligation",
"denotational", "inference rule") route here only when paired with
state-space intent — state sets, invariant encodings, boundary
morphisms, dominance, thresholds, receiver-state narrowing. A
formal request from another domain belongs to that domain's skill.

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
- Candidate mechanisms `m`, each inducing a re-encoded artifact
  `m(A)` with the contract pinned: `C(m(A)) = C(A)` up to the
  chosen embedding.
- Boundary morphisms `b: (u: U) × P(u) -> A`, consuming raw input `u`
  and evidence `p: P(u)`. The evidence-free `b: U -> A` is the derived
  special case of trivial `P`.
- The construction paths of `A`: the set of all boundary morphisms
  that can produce values of `A`. Quantifiers `∀b` range over this
  set; it must be enumerated, not assumed.
- Functions `f: D -> K`.
- Text `t` with rewrite candidate `t'`, reader/model state `q`,
  plausible reception contexts `Ctx_0` with elements `c` (`Ctx`,
  not `C`: `C` is the contract), plausible prior receiver states
  `Q_0`, and intended receiver-state set `Q_intended`, when text
  reception matters.
- Thresholds `θ` with stricter candidates `θ'` and the current
  evidence, when ratcheting. (`θ`, not `t`: `t` is a text in
  reception semantics.)
- Fact sets and their functional dependencies, when normalizing.
- Evidence set `E`.

## Core Notation

- `S(A)` — representable states (structural; what the type's shape permits).
- `C(A)` — valid states (contract). ("Valid", matching
  `principles.md`; "admissible" is reserved for judgments about
  transformations and rewrites.)
- `R(b)` — reachable states under a boundary constructor
  `b: (u: U) × P(u) -> A`.
- `I_repr(A)` — representable invalid: `S(A) \ C(A)`.
- `I_reach(A,b)` — reachable invalid: `R(b) ∩ I_repr(A)`.
- `B(A)` — contract behavior: the artifact's observable behavior as a
  function of representable state `S(A)`. `B(A)|_C` restricts that
  function to the states in `C`, where it does real work.
- `D(f)`, `K(f)`, `R(f)`, `G(f)` — domain, codomain, range, gap.
- `cost(m)` — the audit burden of mechanism `m`: the trusted surface it
  adds plus its ergonomic and toolchain cost. Semantics:
  `state-space-minimization` `references/principles.md` § "Core
  principle" (audit cost and drift surface) and § "Encode invariants
  into types" (ergonomic and toolchain cost per rung).
- `supp(μ)` — the support of distribution `μ`: the outcomes with
  nonzero probability. (`μ`, not `P`: `P(u)` is the boundary
  evidence predicate.)
- `Needs(y, P(x))` — consumer `y`'s contract depends on evidence
  `P(x)`.
- `Erases(g, P(x))` — operation `g` maps `(x, p: P(x)) ↦ x`,
  discarding the evidence.
- `Flows(g, y)` — the output of `g` reaches consumer `y`.
- `[[t]]^D`, `[[t]]^O_{q,c}` — denotation, and operational reception
  from prior state `q` in context `c`.
- `c ∈ Ctx_0`, `q'` — a reception context, and the receiver state
  after reading `t`.

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

For predicative `A`, every consumer `f` with `D(f) = S(A)` whose
contractual behavior is specified only on `C(A)` carries an
obligation `Obl(f)` that the input came through `R(b)`. (`Obl(f)`, not `P_f`: `P` is the boundary
evidence predicate.) Adding a second construction path `b' ≠ b` with
`R(b') ⊄ C(A)` voids `Obl(f)` globally.

For constructive `A`, `Obl(f)` is discharged by the type and survives
the addition of arbitrary new construction paths.

## Artifact Calculus

- Minimization is contract-preserving invalidity reduction.
- Strictness is admissible only as a contract-preserving transformation.
- Mechanism choice is one lexicographic order: invalidity first, then
  the encoding-order rank, then `cost(m)` as the final tiebreak.
- A mechanism that detects, documents, or rejects invalid states does not
  necessarily shrink `S(A)`. Smart constructors, validators, and
  refinement-library wrappers narrow `R(b)`, not `S`.

$$
\begin{array}{c}
C(A') = C(A)
\qquad
C(A) \subseteq S(A')
\qquad
I_{\mathrm{repr}}(A') \subsetneq I_{\mathrm{repr}}(A)
\qquad
B(A')|_{C(A)} = B(A)|_{C(A)}
\\
\hline
\operatorname{Strict}(A,A')
\end{array}
$$

The contract is pinned: `C(A') = C(A)` (up to the chosen embedding)
forbids discharging the rule by widening the contract until nothing
is invalid. `C(A) ⊆ S(A')` is the well-formedness obligation on the
chosen embedding — every contract state must be carried into
`S(A')`, so a strictness step cannot delete a state the contract
requires. It is listed to be checked, not because it is independent
of the other premises.

$$
\begin{array}{rl}
\text{among } m \text{ with} & \operatorname{Sufficient}(m) \;\wedge\;
B(m(A))|_{C(A)} = B(A)|_{C(A)}: \\
\text{choose} & I_{\mathrm{repr}}(m(A)) \;\; \subseteq\text{-minimal} \\
\text{then} & \text{earliest encoding-order rank} \\
\text{then} & \operatorname{cost}(m) \text{ minimal among those}
\end{array}
$$

Eligibility gates the whole selection, not just the rank step: an
insufficient or behavior-breaking mechanism may not participate in —
or block — the minimality comparison. When no mechanism is eligible,
use the fallback rule in § "Encoding Order".

Invalidity is ordered by inclusion, not cardinality: `m` dominates
`m'` only when `I_repr(m(A)) ⊊ I_repr(m'(A))`. Incomparable
residual sets are not ranked — a count would rank them, and could
prefer the numerically smaller set that still contains the
dangerous states. The inclusion order is taken up to a chosen
embedding into a common universe of states (`S(NonEmptyString)` is
not literally a subset of `S(String)`; the embedding is part of
the analysis). Fix one embedding per candidate into one common
universe before checking any premise; switching embeddings between
premises is inadmissible.

When several candidates are ⊆-minimal with incomparable residuals,
no rule orders them — neither rank nor cost may break the tie. The
choice is an open obligation: name the states in the symmetric
difference, decide by which residual contains the more dangerous
states, and record the decision with the residual gap in Outputs.

## Operations

- Shrink domain: replace `D` by `D' ⊊ D`.
- Bound ranges and cardinality: replace an unbounded component by a
  finite, ordered, or measured subset. (Value intervals and counts,
  not `R(f)`: `R` is the image.)
- Shrink codomain: replace `K` by `K'` where `R ⊆ K' ⊊ K`.
- Remove intermediate: compose `g . f` and eliminate exposed state `W` when
  `W` has no independent contract role. (`W`, not `X`: `X` is
  reserved for functional-dependency determinants `X → Y` in
  `state-space-minimization` `references/normalization.md`.)
- Normalize: decompose facts into determinants, remove transitively derivable
  facts, recompose along use.
- Ratchet: replace threshold `θ` by stricter `θ'` when current evidence
  satisfies `θ'`.

These are the six operations of `state-space-minimization`
`references/principles.md` § "Six operations"; `normalization.md`
and `ratchet.md` there own the last two in depth (including when an
estimated bound may move the other way).

## Encoding Order

The **encoding order** ranks mechanism classes. The table below is
derived for an application-owned artifact, not an axiom — derive
the ranks per architecture using the metric that follows it. Use
the earliest sufficient mechanism:

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

`Sufficient(m)` := `I_reach(m(A), b) = ∅` for every construction
path `b`; for rank-1 mechanisms this strengthens to
`I_repr(m(A)) = ∅`. Only sufficient mechanisms compete for rank:

$$
\begin{array}{c}
\operatorname{Sufficient}(m_i)
\qquad
\operatorname{Sufficient}(m_j)
\qquad
I_{\mathrm{repr}}(m_i(A)) = I_{\mathrm{repr}}(m_j(A))
\qquad
B(m_i(A))|_{C(A)} = B(A)|_{C(A)}
\qquad
B(m_j(A))|_{C(A)} = B(A)|_{C(A)}
\qquad
i < j
\\
\hline
m_i \prec m_j
\end{array}
$$

When no mechanism is sufficient, choose the earliest mechanism that
detects, documents, or rejects every targeted invalid state, and record what
remains as the residual gap in Outputs. Ranks 5–7 are these
fallback positions — a test oracle, documentation claim, or runtime
assertion never makes `I_reach` empty by itself (semantics:
`constructive-vs-predicative.md` § "Hard cases for constructive
modeling", which falls back to lints, tests, and runtime
configuration).

The rank metric: a mechanism ranks earlier when it governs more of
the artifact's construction paths; ties break by earlier detection
phase. The table is the derived order for an
application-owned artifact, not an axiom — derive the ranks per
architecture. In a database-first system with multiple writers, a
schema constraint governs every write path while an application
boundary adapter governs one — the adapter fails `Sufficient`, so
the schema constraint is the earliest sufficient mechanism. Rank order is
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
the chosen embedding). A predicative scheme is a composite — a
rank-1 representation whose sufficiency is supplied by its rank-2
constructor; this rule compares the representations, not the
mechanisms. A constructive encoding `A_c` with
`S(A_c) = C(A)` strictly dominates any predicative encoding `A_p`
with `S(A_p) ⊋ C(A)` — then `S(A_c) ⊊ S(A_p)` — even when every
`I_reach(A_p, b)` is empty.

$$
\begin{array}{c}
S(A_c) = C(A)
\qquad
S(A_p) \supsetneq C(A)
\qquad
B(A_c) = B(A_p)|_{C(A)}
\\
\hline
A_c \prec A_p
\end{array}
$$

Rationale: `A_c` discharges every consumer proof obligation `Obl(f)`
structurally and is monotone under the addition of new construction
paths. `A_p` discharges `Obl(f)` only through the trusted boundary `b`,
and any new `b'` with `R(b') ⊄ C(A)` voids `Obl(f)`. A swap that holds `I_reach`
constant while holding `S` and `R(b)` fixed (predicative-to-predicative
mechanism swap, e.g., relocating the check from a hand-written
constructor to a refinement library) is **not** a narrowing; it changes
neither `S` nor `R(b)`.

## Invariants

- Contract preservation: `B(A')|_{C(A)} = B(A)|_{C(A)}`.
- Boundary monotonicity: trust may increase only through
  `b: (u: U) × P(u) -> A`.
- Proof preservation: no downstream operation depends on erased proof.
- Totality: every exposed function is total on its declared domain.
- Exhaustiveness: closed sums are matched without a catch-all branch; a
  catch-all is admissible only when the sum is explicitly open.
- Single source: each fact has one determinant.
- Confluence: normalization order does not affect normal form.

Semantics: totality `references/total-functions.md`; exhaustiveness
`references/defensive-code.md`; single source and confluence
`references/normalization.md` — all in `state-space-minimization`.

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
b(u,p) \in C(A)
\end{array}
$$

The evidence is what buys trust: with `p : P(u)` discharged, the
constructed value lands in `C(A)`, so `R(b) ⊆ C(A)` and
`I_reach(A,b) = ∅`. The evidence-free derived form `b: U -> A`
reaches only `S(A)` and is not trust-increasing.

$$
\begin{array}{c}
\operatorname{Erases}(g, P(x))
\qquad
\operatorname{Flows}(g, y)
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
- Rewrite text only when denotation is preserved and no new
  unintended reading is introduced; aim for strict reception
  narrowing.

$$
\begin{array}{rcl}
[[t]]^D &:=& \text{denotation of } t \\
[[t]]^O_{q,c} &:=& \Pr(q' \mid q,t,c) \\
t \equiv_D t' &:=& [[t]]^D = [[t']]^D
\end{array}
$$

$$
\begin{array}{c}
t \equiv_D t'
\qquad
\forall q \in Q_0,\, c \in Ctx_0:\;
\operatorname{supp}([[t']]^O_{q,c}) \setminus Q_{\mathrm{intended}}
\subseteq \operatorname{supp}([[t]]^O_{q,c})
\\
\hline
\operatorname{AdmissibleRewrite}(t,t')
\end{array}
$$

The premise admits new *intended* readings (repairing a text no
reader currently understands is admissible) while forbidding new
unintended ones. Reception completeness —
`supp([[t']]^O_{q,c}) ⊆ Q_intended` — is the discharge condition of
the reception obligation, not an admissibility premise: when it
fails, record the remaining unintended readings as the residual gap
in Outputs.

`ReceptionNarrowing(t, t')` strengthens admissibility with a strict
witness: additionally `∃(q, c) ∈ Q_0 × Ctx_0` where
`supp([[t']]^O_{q,c}) \ Q_intended ⊊ supp([[t]]^O_{q,c}) \ Q_intended`.
A rewrite that leaves the unintended readings unchanged is
admissible but is not a narrowing — the same standard as the
predicative mechanism swap.

The quantifiers forbid discharging the premises against one
favorable prior reader or one favorable context; the rewrite must
hold for every plausible `(q, c)` pair.

The support premises are proof obligations like every other
premise: discharged by argument about the plausible readings of
`t'`, not computed. A rewrite is admissible only when it preserves
denotation and introduces no new unintended readings; reception
completeness — every remaining reading intended — is the
obligation's discharge condition, not the admissibility bar.

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
\operatorname{Req}(E) &:=& \text{distinctions the evidence } E \text{ exercised or required} \\
K(\operatorname{Skill}) &:=& \text{distinctions expressible by the skill}
\end{array}
$$

`Req(E)` is named apart from the range notation `R(f)` / `R(b)` —
`R` stays reserved for ranges — but it plays the range's role in
the gap analogy: with codomain `K(Skill)`, the set
`K(Skill) \ Req(E)` is the gap `G` applied to the skill itself.
The two directions differ: `Req(E) \ K(Skill)` is an
expressiveness deficit (`MissingDistinction`), not a gap;
`K(Skill) \ Req(E*)` is unexercised expressiveness
(`SharpeningCandidate`) — only the latter is `G`.

$$
\begin{array}{c}
d \in \operatorname{Req}(E) \setminus K(\operatorname{Skill})
\\
\hline
\operatorname{MissingDistinction}(d)
\end{array}
$$

$$
\begin{array}{c}
d \in K(\operatorname{Skill})
\qquad
d \notin \operatorname{Req}(E^{*})
\qquad
E^{*} \text{ spans repeated use}
\\
\hline
\operatorname{SharpeningCandidate}(d)
\end{array}
$$

A single case exercises few distinctions, so the premise must range
over accumulated evidence `E*`, not one case — otherwise the rule
fires vacuously on every use. Materiality and the report gate are
owned by `references/skill-refinement.md`.

## Process

1. State `S`, `C`, `I_repr = S \ C`, and, when boundaries exist,
   `R(b)` and `I_reach`. Never report `I = ∅` without naming which.
2. Locate the boundary morphisms `b: (u: U) × P(u) -> A`.
3. Compute domain, codomain, range, gaps, and failure preimages.
4. Select the least encoding that removes the invalid set; when no
   mechanism is sufficient, apply the fallback rule and record the
   residual gap. When choosing among rank-1 encodings, prefer the one
   whose `S` is strictly included in the alternative's (constructive
   dominance); when choosing among predicative encodings of equal `S`
   and `R(b)`, the swap is not a narrowing and the dominance rule does
   not apply.
5. Prove contract preservation.
6. Preserve or reconstitute proofs across morphisms.
7. Normalize duplicated determinants; where the artifact exposes
   functions, closed sums, or a rewrite order, check totality,
   exhaustiveness, and confluence.
8. If text reception matters, model `[[t]]^D` and `[[t]]^O_{q,c}`.
9. In output-layer mode, emit only definitions, equations, derived
   obligations, and the selection justification (the dominance
   reason). In reasoning mode, translate the derivation into the
   deliverable's form; the notation remains as intermediate work.

## Outputs

In output-layer mode, emit the items below. In reasoning mode they
are derived internally and translated into the deliverable's form.

- Invalid-state sets: `I_repr`, and `I_reach` per boundary.
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
