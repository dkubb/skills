---
name: state-space-minimization-formal
description: >-
  Apply state-space minimization as a compact formal calculus: domains,
  codomains, ranges, invariants, morphisms, normal forms, and
  proof-preserving boundaries.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-05-v1"
triggers:
  - "state-space-minimization-formal"
  - "formal state-space minimization"
  - "formal illegal states unrepresentable"
  - "formal invalid states impossible"
  - "formal parse don't validate"
  - "formal domain codomain range"
  - "formal invariant encoding"
  - "formal proof preservation"
  - "formal normal form"
  - "formal refinement type"
  - "formal typestate"
  - "formal capability"
  - "formal least power"
---

# State Space Minimization Formal

## When to Activate

- Required output is formal, theoretical, proof-like, or definition-only.
- The task asks for state spaces, invariants, morphisms, ranges, gaps,
  receiver-state effects, or proof obligations.
- The ordinary `state-space-minimization` skill is too operational or verbose
  for the requested form.

## When Not to Use

- The user wants examples, implementation tactics, or idiom catalogues.
- A concrete language module is required.
- The task needs explanatory prose more than formal structure.

## Inputs

- Artifact `A`.
- Contract `C(A)`.
- Boundary morphisms `b: U -> T`.
- Functions `f: D -> K`.
- Reader/model state `q`, when text reception matters.
- Evidence set `E`.

## Core Notation

- `S(A)` — representable states (structural; what the type's shape permits).
- `C(A)` — admissible states (contract).
- `R(b)` — reachable states under a boundary constructor `b: U -> A`.
- `I_repr(A)` — representable invalid: `S(A) \ C(A)`.
- `I_reach(A,b)` — reachable invalid: `R(b) ∩ I_repr(A)`.
- `B(A)` — contract behavior.
- `D(f)`, `K(f)`, `R(f)`, `G(f)` — domain, codomain, range, gap.
- `[[t]]^D`, `[[t]]^O_c` — denotation and operational reception.

$$
\begin{array}{rcl}
S(A) &:=& \text{representable states (structural)} \\
C(A) &\subseteq& S(A) \\
R(b) &:=& \{\, b(u) \mid u \in U \,\} \subseteq S(A) \\
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
I_{\mathrm{repr}}(A') \subset I_{\mathrm{repr}}(A)
\qquad
B(A') = B(A)|_{C(A)}
\\
\hline
\operatorname{Strict}(A,A')
\end{array}
$$

$$
\begin{array}{rl}
\min\limits_m & (\;|I_{\mathrm{repr}}(m(A))|,\operatorname{cost}(m)\;) \\
\text{s.t.} & B(m(A)) = B(A)|_{C(A)}
\end{array}
$$

## Operations

- Shrink domain: replace `D` by `D' subset D`.
- Bound range: replace unbounded component by finite, ordered, or measured
  subset.
- Shrink codomain: replace `K` by `K'` where `R subset K' subset K`.
- Remove intermediate: compose `g . f` and eliminate exposed state `X` when
  `X` has no independent contract role.
- Normalize: decompose facts into determinants, remove transitively derivable
  facts, recompose along use.
- Ratchet: replace threshold `t` by stricter `t'` when current evidence
  satisfies `t'`.

## Encoding Order

Use the earliest sufficient mechanism:

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

### Constructive dominance at rank 1

Within rank 1, encodings are ordered by `|S|`. A constructive encoding
`A_c` with `S(A_c) = C(A)` strictly dominates any predicative encoding
`A_p` with `S(A_p) ⊋ C(A)`, even when `I_reach(A_p, b) = ∅`.

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
constant while keeping `|S|` constant (predicative-to-predicative
mechanism swap, e.g., relocating the check from a hand-written
constructor to a refinement library) is **not** a narrowing; it changes
neither `S` nor `R(b)`.

## Invariants

- Contract preservation: `behavior(A') = behavior(A)` on `C(A)`.
- Boundary monotonicity: trust may increase only through `b: U -> T`.
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
b: U \to T
\qquad
p: P(u)
\\
\hline
b(u,p) \in T
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
H([[t']]^O_c) \le H([[t]]^O_c)
\qquad
\delta_M(q,t') \in Q_{\mathrm{intended}}
\\
\hline
\operatorname{ReceptionNarrowing}(t,t')
\end{array}
$$

## Self-Similarity

- The skill is an artifact.
- Its rules are representable states.
- Concrete cases reveal whether the codomain is too wide, too narrow, or
  missing a distinction.

$$
\begin{array}{rcl}
S(\operatorname{Skill}) &:=& \text{representable rules} \\
R(E) &:=& \text{distinctions required by evidence } E \\
K(\operatorname{Skill}) &:=& \text{distinctions expressible by the skill}
\end{array}
$$

$$
\begin{array}{c}
R(E) \not\subseteq K(\operatorname{Skill})
\\
\hline
\operatorname{MissingDistinction}
\end{array}
$$

$$
\begin{array}{c}
K(\operatorname{Skill}) \setminus R(E) \neq \varnothing
\\
\hline
\operatorname{SharpeningCandidate}
\end{array}
$$

## Process

1. State `S`, `C`, `I_repr = S \ C`, and, when boundaries exist,
   `R(b)` and `I_reach`. Never report `I = ∅` without naming which.
2. Locate boundaries `U -> T`.
3. Compute domain, codomain, range, gaps, and failure preimages.
4. Select the least encoding that removes the invalid set. When choosing
   among rank-1 encodings, prefer the one with smaller `|S|` (constructive
   dominance); when choosing among predicative encodings of equal `S` and
   `R(b)`, the swap is not a narrowing and the dominance rule does not
   apply.
5. Prove contract preservation.
6. Preserve or reconstitute proofs across morphisms.
7. Normalize duplicated determinants.
8. If text reception matters, model `[[t]]^D` and `[[t]]^O`.
9. Emit only definitions, equations, and derived obligations.

## Outputs

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
  narrowings; constructive dominance applies only when `|S|` strictly
  decreases.
- Every boundary changes trust explicitly.
- Every proof dependency is preserved.
- Every duplicate fact has one determinant.
- Text rewrites preserve denotation while narrowing reception states.
- The selected mechanism is least-power among sufficient mechanisms.
- The answer is formal, concise, and self-contained.
