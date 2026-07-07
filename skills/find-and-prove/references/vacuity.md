# Vacuity — defect class A (does the suite prove anything?)

Load when auditing whether a theorem set constrains anything at all: trivial
models, unpinned annotations, unsatisfiable antecedents, fuel opt-outs, and
models unfaithful to the real object. Mutation mechanics (how to run the
sweeps) live in `references/evidence.md`.

## Vacuity + mutation

Break a definition / weaken a hypothesis; does it still prove? Un-killed
mutant = spec too weak. (See the vacuous-vs-binding tells below; operators and
sweep mechanics in `references/evidence.md`.)

## A1 Trivial-model realizability (the do-nothing mutant)

The dual of the leaky-mutant test: can the DO-NOTHING implementation (empty
trace, reject-everything, no-op step) satisfy the whole advertised theorem
set? A suite the trivial model passes constrains nothing it claims to — the
refinement-calculus "miracle": an infeasible spec refines to anything, so
feasibility is a proof obligation, not a given. One `#eval`/`decide` sweep
instantiating every headline at the trivial model; the all-constant
annotation sweep below is its per-annotation form.

## A2 Predicate-collapse lattice

FIRE on every NEW predicate, including one in CONCLUSION position: a "the
predicate is real" theorem must be pinned against the WHOLE collapse lattice.
Descend all seven levels — each is a distinct surviving-mutant class; assume
each level survives until you have built the killing mutant:

1. **Truth-value** — test `:= True` AND `:= False`. An existential
   counter-witness (`∃ C, ¬ P C`) kills only the `True` mutant; `:= False`
   survives because consumers go VACUOUS for the interesting case, not false
   — the asymmetry hides it. Kill: a two-sided distinguishing theorem
   (positive AND negative for the predicate).
2. **Quantifier scope** — head-only / last-only / first-only mutants, plus
   wrong-request (the predicate ignores which request was delivered). A
   finite `[good, bad]` witness puts `bad` in a fixed position, so a
   head-only check still rejects it. Kill: a request-sensitivity witness plus
   mixed-list negatives.
3. **Value restriction** — `P o := o = c ∧ …` for any FIXED `c` survives a
   hardcoded witness value. Kill: parametrize the witness over an ARBITRARY
   caller-chosen value.
4. **Position/cardinality** — last-only / singleton-only mutants pass finite
   examples. Kill: negatives parametric over position
   (`pre ++ bad :: post`) — bad anywhere, neither end special.
5. **OVER-restriction (the symmetric blind spot)** — fully parametric
   negatives with finite positives let a predicate survive that accepts only
   the narrow positive SHAPES the finite witness lists. Kill: positives
   exactly as parametric as negatives (`∀ all-good structure → accepted`).
   Symmetry is the convergence criterion.
6. **Witness shape** — `∃!` (unique-witness) and identity-parse
   over-restrictions survive even arbitrary-value, non-identity witnesses
   (each can be built with exactly one accepting raw). Kill: level 7.
7. **Close the class** — pin the INTRODUCTION RULE as a theorem
   (`evidence → predicate`, e.g. `decode raw = some o → P o`) instead of
   chasing one more witness; one theorem states the actual invariant and
   kills the whole over-restriction class at the predicate level. Headline it
   iff the predicate definition is mutable design surface in the declared
   mutation universe (`references/basis.md`, G1).

[Anchors: specification mutation — Ammann–Black; spec coverage metrics —
Chockler–Kupferman–Vardi.] The worked origin was a seven-round hunt on a
decode-image predicate, one lattice level per round — walking the lattice up
front replaces the rounds.

## A3 Annotation / label vacuity (the all-constant sweep)

Fire on every decorated relation/step/run: whenever a relation/step/run is
decorated with a label, tag, event, or annotation, ask: *can ALL my advertised
theorems survive if every annotation is replaced by ONE constant, or by
`none` / a no-op?* If yes, the annotation's CORRECTNESS is UNPINNED — erasure
+ lifting + single-valuedness together prove only "the labeled relation is the
graph of a single-valued partial function refining the base," and the constant
function is in that set (the `StepWithEvent := Step ∧ e = none` countermodel
goes green RECORDING NOTHING). Single-valuedness proves DETERMINISTIC
annotation, not CORRECT annotation — "deterministic garbage passes." A label
is real only once one theorem READS IT BACK against what the step produced.
Fix-trigger: require at least one theorem that EXACTLY ties the label to an
independent observable (an exact +1 count delta against the real durable
residue: `some x` → exactly that residue +1, `none` → all unchanged), and
confirm it reddens BOTH the all-constant mutant AND a swapped-annotation
mutant. Apply the sweep across the whole annotation SUITE, not per-theorem —
the per-theorem vacuity check passes each one yet the suite records nothing.

## A4 Echo lens

FIRE on every theorem whose conclusion shares atoms with a hypothesis: a
conclusion conjunct restating a hypothesis does no work but reads as proven —
the theorem's advertised strength includes a clause its hypotheses hand it
for free. Decide by the delete-BOTH counter-mutant: drop the hypothesis AND
the echoing conjunct together; if the remainder still proves, the conjunct
was an echo (inherent vacuity), not a guarantee. Sibling probe: swap the
exhibited run witness for the trivial one and require failure — a
non-vacuity witness that the do-nothing run also satisfies exhibits nothing.

[Anchor: inherent vacuity — vacuity present in every model of interest, not
just the artifact at hand.]

## A5 Fuel / partiality vacuity

A liveness-flavored headline over a fuel-indexed definition is vacuously
conditional when the fuel is existential (`∃ fuel`) or a fixed constant never
related to input size: it proves reachability-in-principle, not progress.
Demand `∀ fuel ≥ bound(input)` with an explicit bound function, or downgrade
the claim. `partial def` (and any `termination_by` / `decreasing_by` hole)
opts the definition out of the theorem story entirely — flag every headline
whose subject is `partial` while the prose still claims total coverage.

## Adequacy of encodings

Is the model the *real* object, or one with extra inhabitants / collapsed
distinctions? Don't leave this as judgment: build the **bidirectional
coverage table** — every real-world event / behavior / failure mode ↔ the
model constructor or rule representing it. An unmatched row on EITHER side is
a finding (unmodeled behavior, or a model inhabitant with no real counterpart
— the extra inhabitant IS the adequacy gap). The algebraic-specification
names for the two directions: **junk** — a model inhabitant with no real
counterpart; **confusion** — two real things the model identifies ("no junk,
no confusion"). Where both a model interpreter and an implementation run, add
differential execution on shared inputs. This is the adequacy analogue of the
mutation sweep: executable, not argued.

## The vacuous-vs-binding tells

**A theorem is probably VACUOUS if:** it proves by `rfl` after unfolding the impl
under review; a deliberately *leaky* impl still satisfies it; it mentions private
names the adversary can't use; its antecedent is unsatisfiable / degenerate; it
only covers the exact constructor path the impl uses; or its conclusion restates
the definition instead of constraining a *public* observation. **It is BINDING
if:** its statement is on the public surface (or over a reified adversary
language); it compares ≥2 runs/states/impls or proves a public constructor
impossible; it FAILS under the leaky mutant; and its conclusion is in the role's
observations, not private representation. *Always state the bad mutant a theorem
is meant to reject; if no mutant fails, it's vacuous.*
