# Boundary Calculus

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

The evidence is what buys trust, and its strength matters: evidence
that discharges the full contract lands the value in `C(A)`, giving
`R(b) ⊆ C(A)` and `I_reach(A,b) = ∅` — only such a boundary is
trust-increasing, and the rule above states that case. Evidence may
also discharge representability only (`b(u,p) ∈ S(A)` with
`R(b) ⊄ C(A)`); such a boundary is an ordinary construction path,
not a trusted one — these are the paths through which `I_reach` can
be nonempty. The evidence-free `b: U -> A` is the weakest case,
trivial `P` discharging representability alone.

$$
\begin{array}{c}
\mathrm{Erases}(g, P(x))
\qquad
\mathrm{Flows}(g, y)
\qquad
\mathrm{Needs}(y,P(x))
\\
\hline
\mathrm{ProofObligation}(y,P(x))
\end{array}
$$

Semantics: `state-space-minimization`
`references/ingress-and-boundaries.md` (boundary parsing,
capability tokens, temporal invariants) and
`references/proof-preservation.md` (proofs that survive
conversions).
