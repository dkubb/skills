# Reception Semantics

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
\mathrm{supp}([[t']]^O_{q,c}) \setminus Q_{\mathrm{intended}}
\subseteq \mathrm{supp}([[t]]^O_{q,c})
\\
\hline
\mathrm{AdmissibleRewrite}(t,t')
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
