# Unpinned surface — defect class B (it holds, but binds nothing)

Load when a theorem proves true but may not bind the surface it advertises:
hidden wrappers, existential conclusions decoupled from their witnesses,
headlines that pin a caller instead of the helper. The class's core is the
`_iff` pin family (graduating per the formal-review-suite plan).

## B5 Existential coupling / witness-hiding

Fire on every corollary of shape
"given a relation hiding witnesses, `∃ <new witnesses>, P`". When a theorem
CONSUMES a relation/`∃` that binds its witnesses INTERNALLY and RETURNS fresh
existential witnesses, ask: *does the CONCLUSION explicitly relate the two sets
of witnesses — or could an UNRELATED witness already present in the state
satisfy it?* The exactness that lives only in the proof BODY does not bind a
client: a body that unpacks the firing's exact `(pid,bind,v)` but concludes
only `∃ pid bind v, Origin …` is satisfiable by a DECOY origin (separating
state: two persistent vals, firing reads the 2nd, conclusion met by the 1st).
Opaque-contract corollary: *would a downstream client know the claimed
correspondence from the theorem's TYPE alone, without inspecting the proof
body?* If the docstring says "the EXACT triple it reads" but the type is a bare
`∃`, that load-bearing word is untested against the type. Fix-trigger: move the
identifying indices into explicit parameters (an indexed read relation,
`Reads code G G' pid bind v → Origin …`, or
`∃ triple, Reads triple ∧ Origin triple`) so the conclusion type-couples to
the exact witnesses.
