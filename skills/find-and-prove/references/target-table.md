# Review target table

## What a guarantee is (don't over-unify)

A guarantee is one of these — keep them distinct so you don't contort a concrete
attack into the wrong frame:

- **Confidentiality** — forbidden distinctions are not observable by the role.
- **Integrity** — required distinctions/bindings are preserved (no forgery, no
  confusion, no collision).
- **Authority / resource** — ownership and consumption are conserved *under
  composition*.
- **Progress** — required good events eventually happen under stated
  fairness / fuel.
- **Provenance** — evidence faithfully binds who/what/when/why to the event it
  certifies.
- **Refinement** — the implementation exposes no more behavior than the spec at
  the chosen observer.

Most can be *expressed* relationally (preservation of a dependency /
indistinguishability relation at a stated *(role, observation-level)*; the
published unification is the PER model — Sabelfeld–Sands — and the Dependency
Core Calculus — Abadi et al.), and the three classic failure shapes are: **too
fine → leak**, **too coarse → forgery/confusion**, **no progress → stall**.
But do not force every claim into one relation if that hides the concrete public
predicate, binding, or resource equation. Provenance, adequacy, and
evaluator-poisoning resist the single frame — chase the concrete attack.

## The search procedure: oracle-first, lens-second

Do not start by walking the taxonomy. Build the **target map**, then aim.

**A public observation is anything a role can see or branch on:** return
constructors, errors, booleans, equality/ordering, repr/hash/serialization,
logs, timing/termination, resource exhaustion, changed state, admission
accept/reject, trace shape, the *statement* of an exported theorem, an available
*eliminator*, a synthesizable typeclass instance.

For each, fill a **target table** row:

1. Claim being tested (quote the strong word: "sealed / only / never /
   unrepresentable / redacted / canonical / exactly / first / terminal / affine
   / complete / sound / no-fabrication / deferred").
2. Role / adversary, and whether they're in the artifact's stated threat model.
3. Hidden variable: secret, authority, parent set, recorded head, trace length,
   validity proof, resource balance, identity, semantic version, evaluator
   judgment.
4. Public observation: which constructor / tag / value / timing / termination /
   exported eliminator.
5. Adversary control: chosen input? chosen prefix? replay? retry? fork?
   cross-run adaptivity?
6. **Predicate leaked or confused** — the hidden predicate this observation
   computes (e.g. `head.intent = actual`, `trace_nonempty`, `grant_has cap`,
   `canon xs = canon ys`, `key aliases physical resource`). *Every public tag is
   a predicate over hidden state.* The knowledge-based reading (gradual
   release): each observation shrinks the adversary's knowledge set; an
   *allowed* declassification is a stated bound on how fast it may shrink —
   demand that bound, not a judgment call.
7. Minimal witness: two states distinguished, or two distinct states
   confused/admitted.
8. Schedule lift: single call → same-run retry → cross-run resubmission →
   crash/restart/fork/concurrency.
9. Proof rank (see Rank classifier): theorem / export drill / runtime bridge /
   operator policy / evaluator judgment / doc.
10. Fix or invariant.

**Rank targets.** The strongest-claim → public-oracle triage list below is the
authoritative order; the score is a tiebreaker mnemonic *within* a triage tier
— do not compute it numerically (that produces fake precision):

```text
score ~= claim_strength x hidden_value_importance x adversary_control
         x observation_richness x schedule_amplification x boundary_crossing
         x proof_gap x blast_radius
```

Chosen-query + reusable/restartable + a public tag beats almost everything else.
Attack the top rows; pull a rubric class only when it explains the
current target. Local algebraic lemmas that don't cross a role boundary are
lowest priority — until you see they feed identity, authority, replay, or an
exported observation (a canonicalization law matters *because* it decides
identity equality).

**Strongest-claim → public-oracle triage** (what to attack first, what to ignore
under budget):

1. "sealed / private / redacted / no more than" + any public eliminator or
   result tag.
2. "never fabricates / request-keyed / first-mismatch" + replay, retry, restart.
3. "unrepresentable" + exported constructors / recursors / coercions / deriving.
4. "canonical / same identity" + equality, hashing, serialization,
   normalization.
5. "exactly / least / only" authority + aliases, derived/cached handles,
   delegation.
6. "bridge preserves" + runtime lowering, crash recovery, concurrency, logs.
7. Local algebraic lemmas with no role boundary — last.

### The schedule amplifier (run after EVERY candidate oracle)

A mitigation only closes the schedules it controls. For every predicate `P`
learned in one step, rerun:

- **single call** — can one query distinguish?
- **same-run repeated** — can the program retry with the same handle?
  (affine/terminal closes this.)
- **cross-run adaptive** — can the attacker submit a new run/candidate after
  seeing the last result? (affine/terminal does NOT close this — needs non-secret
  values or campaign controls.)
- **reset / fork** — clone or rewind state and re-query?
- **crash / restart** — does recovery replay the query, double-consume, or reveal
  a phase bit?
- **concurrency** — two actors racing the same affine/linear capability?
- **check-then-use (TOCTOU)** — can the hidden state change between an
  admission check and the consumption of its result? A passed check is
  itself a stale observation.
- **batching** — does the evaluator reveal per-case or only aggregate results?

### The rank classifier (assign every claim exactly one)

Enforcing a claim at the wrong rank is itself a defect.

- **Kernel theorem** — a *semantic* property over modeled values and exported
  behavior.
- **Export-surface seal** — a *syntactic*/API fact (no public projection, no
  leaky instance), checked by the `#check`/`#synth` adversary-import drill.
  Ordinary Lean props can't quantify over "publicly definable downstream code";
  don't fake it as a theorem.
- **Runtime bridge obligation** — depends on single-use, terminality, crash
  behavior, process isolation, scheduling. Unprovable in a pure-value model
  unless the scheduler is reified.
- **Operator / harness policy** — rate limits, campaign batching, non-adaptive
  submission, access control.
- **Evaluator judgment** — external semantic acceptance, not substrate truth.
- **Honest documentation** — an intentional non-guarantee; acceptable *only* when
  the threat model says it's out of scope.

Rank mismatch: proving an API-surface fact as a definitional theorem =
**vacuity**; documenting a modeled semantic invariant = **under-enforcement**;
proving a runtime scheduling property in a pure model = **impossible** (reify the
scheduler or downgrade the rank).

**The bad-lowerer test (kernel theorem vs lowering obligation).** For any
claimed run-OUTPUT property: can a bad lowerer falsify it while every reducer
rule behaves correctly? Then it is a lowering obligation carried in the
statement, not a kernel theorem — [scope, D1](scope.md#d1-the-bad-lowerer-test-kernel-theorem-vs-lowering-obligation).
