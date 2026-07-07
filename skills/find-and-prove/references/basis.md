# Basis curation — defect class G (headline vs corollary)

Load when deciding which theorems deserve headline status: redundant
(interderivable) headlines, demotions to corollary, and what earns a place in
the exported basis.

## G1 Theorem-set minimality (the delete-a-headline mutant)

Extend mutation from *definitions* to the *theorem set itself*: delete each
headline and recompile the rest. If it re-derives from the others plus
already-pinned lemmas, the delete-mutant SURVIVES — it pins nothing new, so
it is redundant coverage, not an independent guarantee. Keep the headline
that names the load-bearing invariant (or the one nearest a public
observation); DEMOTE the one-step-derivable twin to a plain corollary.
Canonical case: two readings of one fact linked by carried laws (e.g.
`decode-origin` vs `spec-conformance` via `sound`/`complete`) — a minimal
basis keeps one. Caveat: theorem-SET redundancy is a *prune / minimal-basis*
finding, NOT vacuity or a soundness defect, and does not weaken
*definition*-mutation resistance (a definition mutant is still caught, by one
or both); flag it as such, don't overstate severity.
