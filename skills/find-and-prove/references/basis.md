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

## G2 Keep-side calibration

"The witness depends on it" proves USED, not basis-worthy — the test is
whether the STATEMENT pins a public surface no other headline states. A
cheap proof is not evidence of redundancy (one-step derivability is a
demotion argument only inside the declared universe, G1). An unconsumed
bridge kept as a visible vocabulary-agreement pin is legitimate basis.
Reception may beat bare minimality: keep `= 1` over `≤ 1` when the
stronger form is the contract readers need — document the tiebreak
("semantic headline, honestly labeled") rather than silently keeping the
derivable form.

## G3 Placement rules

- **Projection-straddling**: state the load-bearing theorem in PUBLIC
  vocabulary, prove it by internal induction, and demote the readable
  weakening; at most ONE projection headline per projection (treat
  `trace = revTrace.reverse` as fixed background or give it exactly one
  headline — never inflate every public-trace corollary).
- Per-element relation is the headline; the aggregate view is an
  extensionality corollary.
- **Subsumption-as-theorem, never churn**: when a rung generalizes a
  landed concept, keep the specific one (its pins and mutation tests stay
  green) and add `old ≡ new instance` as a bridge theorem, rather than
  collapsing old into new.
- A non-vacuity reachability witness EARNS basis status (it kills the
  "nothing happens, so all safety holds" mutant — that is a unique kill).
- Split an assumption-free algebraic core from the invariant-concluding
  semantic theorem, and headline the semantic one.
