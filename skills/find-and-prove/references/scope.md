# Mis-scoping — defect class D (right claim, wrong boundary or rank)

Load when a claim is true but enforced at the wrong boundary, rank, or run
position: kernel theorems that are really lowering obligations, invariants
that are not step-closed, properties stated over the wrong slice of the run.
The rank classifier itself (assign every claim exactly one enforcement rank)
lives in SKILL.md.

## D1 The bad-lowerer test (kernel theorem vs lowering obligation)

For any claimed run-OUTPUT property, ask: *can a bad lowerer falsify this
while every reducer rule behaves correctly?* If yes, the property reads from
AUTHORED program data the reducer consumes but does not compute (an
occurrence id, a slot index, a template), so it is conditional on lowering
well-formedness — a *lowering / admitted-code obligation*, not a kernel
theorem. Carry the conditional in the *statement* ("…under
lowered-occurrence uniqueness"), never only in the docs; the uniqueness stays
a hypothesis until the lowering relation is itself modeled and proved. This
is the read-from-program asymmetry: a reducer theorem quantifies over all
programs; a property a bad program can break is predicated on the program
being well-formed.

## D2 Reach-past-boundary

FIRE on every success/positive witness: a premise about a step LATER than
the property being pinned is the mechanical tell of overbuild. Stop the
witness AT the boundary — a boundary guarantee's positive witness terminates
right after reaching it, carrying no premise about continuation; the
full-run version DEMOTES to an integration corollary. The headline fixture
is the SMALLEST net that reaches the new boundary and stops right after it —
never a prefix of a richer fixture (a cross-layer witness must be a
same-shape machine that terminates). The tell is mechanical: scan each
witness's hypotheses for references to steps past the pinned property; each
one found is either overbuild to strip or a mis-scoped claim to re-state.

## D3 Two-step phase-closure test (a counterexample-to-induction hunt)

Don't only test a hand-inserted bad state: start CLEAN, take one LEGAL step,
then ask whether the next enabled instruction invalidates the side invariant
the next step needs. Catches an invariant true of reachable states but not
INDUCTIVE (not closed under the relation); the breaking state is a
counterexample-to-induction — the object IC3/PDR mines — and the two-step
test is k-induction at k = 1: take k legal steps when one doesn't expose the
gap (two back-to-back `emitAwait` on one strand → the no-pending
precondition is false at step 2, yet the raw rule still fires). A consumed
invariant that isn't step-closed is a LOWERING / PHASE discipline, not a run
invariant: bundle it into a named `PhaseWF` with a preservation/lowering
theorem, OR keep the looser run invariant but stop calling it "sufficient"
and name the obligation. Keep it OUT of the rule premise (preserve
orthogonality); make it a NAMED admitted-code obligation, never an informal
comment.

**WF-uniqueness ≠ semantic cleanliness.** Multiplicity bounds admit orphan
garbage: a `≤1` well-formedness clause permits the one orphan it bounds, so
uniqueness invariants do not prove the state is clean. A needed
no-orphan condition is a named `Clean` predicate consumed as a PHASE
obligation, never a negative premise inside a rule.

**Persistent-fact re-entry.** For every rule adding a PERSISTENT
(accumulating) fact under a `≤1` cardinality clause, ask: can history
already contain the fact I am about to add? A WF state may already hold the
slot's one permitted occupant, so firing makes two — the invariant is not
step-preserved alone. Fix = a freshness/phase precondition as a SEPARATE
consumed phase invariant (`NoValAtAwaitBind`), keeping the rule orthogonal;
if slots are provably non-revisited (single assignment), freshness is
derived and no guard is needed. This is the two-step test generalized to
persistent facts.

## D4 Channel coverage

FIRE on every "full X for every Y" claim: does the mechanism actually RUN
on every channel quantified over? If one channel bypasses it (an effect
kind the check never sees, a path that skips the boundary), either
scope-and-rename the claim to the covered channels or fix the mechanism
first. Classify a bypassing channel as a MODEL BOUNDARY, not a proof bug —
the theorems are true of the model; the model just doesn't route that
channel through the mechanism.
