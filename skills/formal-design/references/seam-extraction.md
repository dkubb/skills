# Seam / interface extraction — without changing the theorems it abstracts

Run before factoring duplicated theorems into an interface/typeclass. The
12-question set; each answer is a checkable artifact:

1. Did the abstraction PROVE a previously-duplicated theorem, or merely sit
   beside the old proofs?
2. Do existing public statements stay byte-stable, with only proof terms
   changing? (Mechanize with `#guard_msgs` snapshots — find-and-prove H3.)
3. Does at least one concrete instance re-derive the old laws through the
   interface?
4. Does a downstream theorem consume the re-derived laws TRANSITIVELY —
   does deleting the abstraction redden a real consumer?
5. Is the interface exactly as strong as the downstream needs — especially
   the CROSS-object cases, not the same-object conveniences?
6. Did the generic theorem preserve the full ARITY of the concrete claim?
   (N distinct objects need C(N,2) pairwise inequalities — the arity audit,
   find-and-prove C2.)
7. Are convenience corollaries exported but kept OUT of the headline basis
   when one-step derivable?
8. Is future-shape documentation truly docs — no new constructor, token,
   state, or replay key smuggled in?
9. Did the extraction avoid widening unrelated identity surfaces?
10. Is global freshness clearly separated from structural namespace laws,
    not silently overclaimed?
11. Can a pointwise/local-law model satisfy the weakened interface while
    breaking the multi-object claim? (Compile the model; if it satisfies
    the interface, the interface is too weak.)
12. Does the abstraction make the next consumer cheaper WITHOUT forcing
    premature re-parameterization?

**Don't over-generalize.** The tempting generic theorem is usually too
high: if making a theorem generic forces re-parameterizing existing state
(`Token`/`Step`/WF off the concrete type), it belongs to a later refactor,
not the seam rung. Generalize the SMALLEST theorem that pins the
abstraction (the address-only namespace law, not the whole preservation
theorem), and get non-vacuity from routing the concrete laws through the
instance — not from lifting a high theorem.
