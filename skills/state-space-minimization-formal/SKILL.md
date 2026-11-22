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
  version: "2026-06-v3"
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

Define the representable, valid, and reachable sets before comparing encodings.
[Read the rule](references/core-notation.md).

### Constructive vs predicative

Distinguish the shape of a representation from the values its constructors admit.
[Read the rule](references/core-notation.md#constructive-vs-predicative).

### Proof-preservation corollary

Identify the evidence each consumer needs from its construction path.
[Read the rule](references/core-notation.md#proof-preservation-corollary).

## Artifact Calculus

Pin the contract and compare residual invalidity in a common state space.
[Read the rule](references/selection.md#artifact-calculus).

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

Choose among sufficient mechanisms or record a fallback and its residual.
[Read the rule](references/selection.md#encoding-order).

### Constructive dominance at rank 1

Compare the representable state sets of competing type-level encodings.
[Read the rule](references/selection.md#constructive-dominance-at-rank-1).

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

Track how evidence enters the system and where a consumer needs it again.
[Read the rule](references/boundary-calculus.md).

### Transitions as boundary morphisms

Check step closure, causal origin, and the cross-component premise.
[Read the rule](references/transitions.md).

## Reception Semantics

Compare a text and its rewrite across plausible readers and contexts.
[Read the rule](references/reception.md).

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
\mathrm{Req}(E) &:=& \text{distinctions the evidence } E
\text{ exercised or required} \\
K(\mathrm{Skill}) &:=& \text{distinctions expressible by the skill}
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
d \in \mathrm{Req}(E) \setminus K(\mathrm{Skill})
\\
\hline
\mathrm{MissingDistinction}(d)
\end{array}
$$

$$
\begin{array}{c}
d \in K(\mathrm{Skill})
\qquad
d \notin \mathrm{Req}(E^{\ast})
\qquad
E^{\ast} \text{ spans repeated use}
\\
\hline
\mathrm{SharpeningCandidate}(d)
\end{array}
$$

A single case exercises few distinctions, so the premise must range
over accumulated evidence `E*`, not one case — otherwise the rule
fires vacuously on every use. Materiality and the report gate are
owned by `references/skill-refinement.md`.

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

## Utilities

Run `./check_claims` from the [Lean project](lean/README.md) to verify
the declarations, theorems, and residuals in its [manifest](lean/claims.toml).
The prose rules also require evidence outside that manifest:

- For equally ranked mechanisms with equal residuals, compare their audit
  costs using the architecture and toolchain under review. The Lean
  selection relation checks residuals and rank.
- For confluence, prove that alternative reduction orders reach a common
  normal form. Lean checks idempotence of a single normalizer, which
  establishes that applying it again preserves its result.
- For output guarantees that depend on a lowering, model the lowering or
  carry its well-formedness as a hypothesis. The calculus gives the
  bad-lowerer test as a review procedure.
- For transitions and composition, supply a preservation argument for
  each enabled step, identify the source of an arriving value, and
  establish cross-component compatibility. The boundary theorems alone
  do not prove these properties of a particular transition system.

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
