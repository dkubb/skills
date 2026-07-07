# Basis curation — defect class G (headline vs corollary)

Load when deciding which theorems deserve headline status: redundant
(interderivable) headlines, demotions to corollary, and what earns a place in
the exported basis.

## G1 Declared-universe irredundancy

FIRE before any KEEP/DEMOTE judgment: irredundancy is UNDEFINED until the
mutation universe is declared — name what is fixed background vs mutable
design surface first. The same headline flips KEEP↔DEMOTE with the
declaration (a "defeq, so no content" demotion is true only if the unfolded
def is FIXED background; if it is mutable surface, the headline pins it).
KEEP is proven by a unique-kill witness model: an admissible design variant
that satisfies all the OTHER headlines and falsifies this one — compile it,
never argue it. DEMOTE is proven by derivation from the others plus fixed
defs (named-set ≠ exported: a demoted theorem may stay exported as a
corollary). Apply every judgment against the DECLARED universe, not
intuition. The delete-a-headline mutant below is the mechanical half; the
witness model decides what deletion alone cannot — whether the survivor
uniquely pins mutable surface.

### The delete-a-headline mutant (theorem-set minimality)

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
