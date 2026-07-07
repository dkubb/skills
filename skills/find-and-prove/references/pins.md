# Unpinned surface — defect class B (it holds, but binds nothing)

Load when a theorem proves true but may not bind the surface it advertises:
hidden wrappers, existential conclusions decoupled from their witnesses,
headlines that pin a caller instead of the helper. The class's core is the
`_iff` pin family.

## B1 The `_iff` pin family

The highest-yield family in the harvest. Three FIRE-ONs:

- **Every new erasure/refinement RELATION on the mutable surface** — prove
  `R ↔ <explicit intended content>` as a headline, every field/conjunct
  spelled out. A forward-simulation theorem
  (`relates pre → step → ∃ post, relates post`) survives BOTH a too-WEAK
  relation (the post-state omits a field — a preservation claim silently
  lost) AND a too-RESTRICTIVE one (the pre-state over-narrowed — the bridge
  silently stops being general), because the theorem is conditional on the
  relation it is defined over. Projection lemmas kill too-weak but not
  too-restrictive; the iff kills both. Distinguish forward refinement from
  behaviour-EQUIVALENCE (the latter needs a converse theorem); never let the
  docs say "same behaviour" when only forward simulation is proved.
- **Every new wrapper `def P … : Prop` or `structure … : Prop` consumed
  downstream** — prove `P_iff : P ↔ <its definition body>` (often
  `Iff.rfl`). Witnesses plus projections leave four mutants alive:
  hidden-extra-condition (`P := <real conjunct> ∧ <unrelated
  true-on-the-witness thing>`), drop-a-field (downstream theorems that don't
  project that field stay green), hidden-`False` field (every consumer goes
  vacuous while a sibling non-vacuity witness still passes), and
  field-type weakening (drops a sequential-consumption step; passes whenever
  keys don't collide). Only the iff kills all four; DEMOTE the witnesses and
  projections to exported corollaries. A "theorem proving P holds" is never
  enough when `P`'s definition is mutable surface.
- **Every laws-carrying record FIELD (instance honesty)** — a dishonest
  instance (`spec := the decoder's own graph`) makes the generic contract
  law a tautology while every named-predicate theorem stays green: those
  theorems re-derive from the carried laws independently of the field, so
  per-definition mutation STRUCTURALLY cannot catch instance wiring. Require
  the defeq lock `C.field ≡ named-predicate`, proved by `Iff.rfl`, as a
  headline — and verify it reddens under the graph-swap mutant. A derived
  public statement through the lock stays a headline as consumer-facing
  surface but is TAGGED an "export twin", never cited later as independent
  basis.

## B2 Producer pin

FIRE on every type-level injectivity / no-collision headline: the theorem is
about the TYPE — it does not bite unless a theorem also pins that the live
RULE advances/records using the full structured identity. The surviving
mutant: collapse the rule's produced value to a PARTIAL key (a constant
occurrence — the exact collision class the slice exists to close); the
type-injectivity headline still passes because nothing reads the rule's
output value (well-formedness counts are blind to it, and diamond theorems
carry the frontier opaquely, using the SAME mutated producer on both sides,
so they are invariant to the mutation). Fix = the producer pin:
`produced-token-with-full-identity ∈ ruleOutput`, and make the HEADLINE
producer-grounded — read the events off the rule's output, THEN assert
distinctness. Also pin a kept lossy projection's NON-DEGENERACY:
`proj _ := constant` survives unless a theorem forces the projection to
separate its inputs. "The type can't collide" is not "the rule can't
collide." When executing these mutants, apply detector isolation
(`references/evidence.md`, H1): mutate only the def body, never the pin.

## B3 Cardinality conjunct

FIRE on every universal value claim over a linear/unique resource:
`∀ x, P x → x = good` is forged by TWO copies of the right thing — the
most-recurring forgery class in the harvest. `count = 1` is first-class,
never derived from a `∀`-uniqueness reading; and the `∃ rest` decomposition
form (`state = good :: rest`) needs an explicit not-in-rest clause
(`good ∉ rest`), else the second copy hides in the rest. The deciding
witness is the two-copy state: it satisfies every uniqueness-as-`∀` reading
and fails only the count / not-in-rest form. [Anchors: exactly-once
semantics; multiplicity.]

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

**The grammatical-subject half (B5b).** FIRE on every `∃`-headline: every
conjunct's grammatical SUBJECT must be the bound variable, never a ground
helper term that happens to equal the witness (`∃ t, Reads t ∧ P (helper x)`
where `helper x` equals `t` at the exhibited witness). Both forms are defeq
AT THE WITNESS, so no mutant can distinguish them — the mutation gate is
structurally blind here, and the defect and its fix are syntactic: rewrite
each conjunct so it constrains the bound variable directly (`∃ t, Reads t ∧
P t`). A conjunct whose subject is a ground term binds the helper, not the
existential — a different witness satisfying `Reads` is left unconstrained.

## B7 rfl-headline symbolic passthrough

FIRE on every exact `_eq` headline proved by `rfl` whose RHS names a helper:
the headline pins the CALLER's shape, not the helper's — mutating the helper
moves BOTH sides of the definitional equation, so the headline stays green
under every helper mutant. The helper needs its own `_iff`/`_eq` pin against
spelled-out content (not against another name that unfolds with it). And
calibrate every "this theorem reddens mutant X" claim to the locus that
actually reddens — the caller — never to the helper the docstring names; a
kill-claim at the wrong locus overstates the helper's coverage.
