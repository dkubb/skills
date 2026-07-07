# Resources & composition — defect class F

Load when the target's guarantee is authority/resource conservation, or when
a per-component claim must survive composition (union, append, concurrency,
runtime lowering). The confidentiality half of the hunt is
`references/information-flow.md`.

## F1 Conservation shape

FIRE on every resource/authority law: the law is an aggregate SUM
(`childL + childR ≤ parent`), never pointwise containment —
each-child-≤-parent admits finite-authority duplication (every child
individually within bounds while the sum is unbounded). In Lean, derive
`draws ≤ ceiling` BEFORE any truncated subtraction: `Nat.sub` silently masks
over-draw (`ceiling - draws = 0` reads as exhaustion, not violation). Read
"remaining" off the REACHED state, never a static expression. The deciding
mutant: split a parent into two children each equal to the parent —
pointwise containment passes, the sum law reddens.

## F2 Split-vs-copy

FIRE on every fork-like rule: it may COPY information (causal history is a
fact both children legitimately hold) but must SPLIT consumable authority
(budget, capability, fuel). Decision rule: split iff the invariant bounds a
SUM over all parties — anything the conservation law adds up must divide,
anything it doesn't may replicate. The deciding mutant: let the fork copy
the summed resource; the aggregate conservation law (F1) reddens while
every per-child law stays green. [Anchor: linear logic exponentials — `!A`
marks what may be duplicated; everything unmarked is linear.]

## F3 Control multiplication costs budget

FIRE on every rule that creates control (fork/spawn/loop-back): it consumes
a budget unit and is FORBIDDEN at zero — else a zero-budget actor
multiplies unboundedly and every per-actor bound survives (each child is
individually within bounds; the population is not). Verify with a
potential-function measure: potential strictly decreases across every
control-creating step; a fork that pays nothing shows up as
non-decreasing potential. [Anchor: the potential method of amortized
analysis.]

## Authority & resources — *does ownership/budget add up?*

- **Separation logic / resource algebra / ownership** — identify the algebra;
  prove validity & conservation *under composition*; mark resources
  exclusive/fractional/duplicable/affine/linear; **attack aliasing** (two keys ↦
  one resource; one key ↦ two; derived/cached handles; parent/child overlap);
  name the minting boundary if disjointness is an obligation. **Per minting
  rule, ask what it mints and what same-uniqueness-family token is already
  present that key-freshness does NOT rule out** — a fresh per-*key* guard stops
  a duplicate key but not a duplicate owner-scoped token (`pending pid k` minted
  onto a strand already holding `pending pid k'`); close the gap with a separate
  phase precondition (a per-owner guard), not a negative premise in the rule.
  **Frame rule**: prove unrelated capabilities unchanged.
- **POLA / object-capability + confused deputy / ambient authority** — is
  authority ambient (from position/global/session) rather than an explicit
  carried capability? Ambient authority is the confused-deputy /
  prompt-injection enabler.
- **Complete mediation** — every access checked on every path, including
  cached/derived.
- **Forkability / reset attack on linearity** (rubric F4) — affine/linear
  guarantees are void
  unless the use-record cannot be copied, rewound, replayed, or re-decoded. Pure
  values are copyable; runtime state copies via snapshot, crash recovery, branch
  fork, test isolation, serialization, repeated submission. Prove the no-fork
  condition, reify the scheduler, or downgrade to a bridge/operator obligation.
  The published rank ceiling is **fork consistency / fork-linearizability**
  (SUNDR): an untrusted store cannot be *prevented* from forking views, only
  forced to keep the fork forever — detection plus commitment, not prevention.
- **Resource-or-fact (the dual of fork)** — fork *admits* invalid states by
  treating a copyable value as affine; the dual *deletes* valid states by treating
  a FACT as affine. A derived datum / SSA value with multiple legitimate readers
  (a later call AND an audit/export step) is a fact, not a one-shot resource —
  modeling it linear starves the second reader (an artificial consumer race that
  the contract never asked for). Ask *"affine resource or fact?"* before imposing
  linearity; a fact is persistent + owner-scoped with a write-once `≤1`
  coherence bound, never consumed.
- **Best correct approximation / strongest postcondition** — merely *sound*, or
  the *most precise* sound one? Is the slack an authored choice or baked-in
  over-approximation?

## Dynamics & composition — *across time, substitution, runtime*

- **Safety / liveness / fairness** (Alpern–Schneider) — is "authorization safety"
  silently assuming progress, freshness, cleanup, revocation?
- **Refinement mapping / simulation / bisimulation** (Abadi–Lamport;
  history/prophecy) — the way to prove a bridge (a shared helper is not a proof).
  Caveat: trace refinement preserves trace properties and *subset-closed*
  hyperproperties, not arbitrary ones — and no liveness: in the CSP hierarchy
  only failures/divergences refinement carries progress claims through, so a
  liveness claim riding a traces-level refinement is an overclaim.
- **Linearizability / linearization points**; **injective agreement** (Lowe —
  unique/fresh, not just "something similar ran"); crash / idempotence.
- **Prove the run, not the store (the store-transplant mutant)** (rubric
  F4) — FIRE on every cross-rung composition theorem: importing a prior
  stage's FINAL STORE into a fresh prestate proves nothing about
  reachability — the transplant fabricates a state no run produces, and
  every store-shaped invariant still holds of it. Thread the ACTUAL run:
  the composition theorem's hypothesis is the prior run object (or its
  run invariant), never the transplanted store. Soundness caveat
  (physical-execution coupling): couple fuel/physical execution only when
  the headline EXHIBITS a run; a universal over a SUPPLIED run is
  legitimately fuel-orthogonal.
- **Composition non-monotonicity (the A∪B trap)** (rubric F4) — a property
  proven per-component does NOT transfer to a union/append:
  `Inv A ∧ Inv B ⇏ Inv (A ++ B)` when A and B live in different worlds
  (recorded-run vs candidate-run frontiers; two writers; two sessions
  reusing an id). Any theorem over `A ++ B` needs a BRIDGE invariant
  naming the cross term, and the certificate must be STRONGER than "the
  lossy projections matched" (else it assumes what it proves — the bridge
  must say prior facts matched at the real seam, not just that a lossy
  view agreed). State the bridge shape
  (`perRunInj ∧ crossAgreement → unionInj`) now; prove the pure ALGEBRA
  now, and defer only the semantic implication discharging its
  hypotheses. Do not let a future rung quietly prove
  only the weaker per-side facts.
