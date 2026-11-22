# Mechanism selection

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
\mathrm{Strict}(A,A')
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
\text{among } m \text{ with} & \mathrm{Sufficient}(m) \;\wedge\;
B(m(A))|_{C(A)} = B(A)|_{C(A)}: \\
\text{choose} & I_{\mathrm{repr}}(m(A)) \;\; \subseteq\text{-minimal} \\
\text{then} & \text{earliest encoding-order rank} \\
\text{then} & \mathrm{cost}(m) \text{ minimal among those}
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
\mathrm{Sufficient}(m_i)
\qquad
\mathrm{Sufficient}(m_j)
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

## Constructive dominance at rank 1

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
