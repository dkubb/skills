# Self-Similarity

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
