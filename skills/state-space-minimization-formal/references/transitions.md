# Transitions as boundary morphisms

A rewrite / transition rule `t: A -> A` that mints state is a
construction path, so it has a range `R(t)` and a reachable-invalid set
`I_reach(A,t) = R(t) ∩ I_repr(A)` like any boundary. To find it,
**enumerate each minted token against each uniqueness bound in `C(A)`**: a
per-*key* freshness premise narrows `R(t)` on the by-key family but not on
an owner-scoped family, so a pre-existing owner-scoped token plus the mint
gives `R(t) ∩ I_repr(A) ≠ ∅`. For example, a fresh request ID permits
two pending requests for the same account. If each account may have only
one pending request, require the account to be idle before accepting one.
Close the gap with this per-owner *phase* premise on `t`, never a contract
widening and never a negative premise that couples
the rule to global state.

**Step-closure of a consumed premise.** A premise `φ` that `t` consumes is
sound for one application; soundness of a *run* requires `φ` closed under
the transition relation. If it is not, `φ` is predicative — held by a
boundary/lowering, not by the carrier — and the run invariant that omits
its preservation is under-scoped.

$$
\begin{array}{c}
\varphi(A)
\qquad
t(A, A')
\qquad
\neg\,\mathrm{Preserves}(t, \varphi)
\\
\hline
\varphi \text{ is a lowering/phase discipline, not a run invariant}
\end{array}
$$

Discharge `Preserves(t, φ)` and fold `φ` into the run invariant, OR keep
`φ` as a named admitted-code obligation and stop asserting the smaller run
invariant is *sufficient*.
To check whether a legal first step consumes the premise needed
for the next one, use the two-step phase-closure test (D3) in
[find-and-prove's scope checks](../../find-and-prove/references/scope.md).

**Reachability is not arrival.** `x ∈ R(t)` says the state is *reachable*,
not that a token *arrived* by `t`. In an unordered carrier a
supposedly-later token may inhabit the initial state, so a reachability
theorem `Reaches(A_0, A_f)` proves compatibility along a schedule, not
causal sourcing: "`t` sources `y`" holds only for the `y` that `t` mints;
exogenous start-state tokens are inputs, not outputs.

**Composition is non-monotone (the union trap).** For a carrier
composition `⊕` (append, union),

$$
\begin{array}{c}
\varphi(A)
\qquad
\varphi(B)
\qquad
A, B \text{ independently constructed}
\\
\hline
\varphi(A \oplus B) \text{ does not follow}
\end{array}
$$

when `A` and `B` come from different worlds (recorded vs candidate run,
two writers). The union needs a cross term `χ(A,B)` with
`φ(A) ∧ φ(B) ∧ χ(A,B) → φ(A ⊕ B)`. Name `χ` before applying any
per-component result; `χ` must be stronger than "the lossy projections
agreed" (else it assumes the conclusion).
