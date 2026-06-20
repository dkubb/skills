---
name: find-and-prove
description: >-
  Adversarial review of formal/Lean and other high-stakes artifacts, run as an
  ORACLE HUNT rather than a lens checklist: build a ranked target table of every
  public observation and the hidden predicate it computes, attack the top
  targets with two-run distinguishers and compiled witnesses, and classify each
  result by enforcement rank (kernel theorem / export drill / runtime bridge /
  operator policy / evaluator judgment / honest doc) and threat scope. Presuppose
  defects and find-and-prove them — no claim without a witness; for what you
  can't break, prove it sound. Use to review your own formal work before
  delegating, and as the prompt spine for adversarial subagent reviews. Grounded
  in the formal-methods literature, hardened on a machine-checked Lean model.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  type: review-methodology
---

# Find-and-prove — adversarial review as an oracle hunt

> **How to read this.** A review is an *oracle hunt*, not a taxonomy walk. Build
> the **target table** first (the search procedure below), rank it, and attack
> the top rows. The six technique groups are a *toolbox you pull from on demand*
> to explain the current target — not a checklist to recite. Reach for a
> canonical term only *after* you have a target and a witness; naming literature
> before you have a target produces recitation, not findings.

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
   a predicate over hidden state.*
7. Minimal witness: two states distinguished, or two distinct states
   confused/admitted.
8. Schedule lift: single call → same-run retry → cross-run resubmission →
   crash/restart/fork/concurrency.
9. Proof rank (see Rank classifier): theorem / export drill / runtime bridge /
   operator policy / evaluator judgment / doc.
10. Fix or invariant.

**Rank targets** (attack in descending score):

```text
score ~= claim_strength x hidden_value_importance x adversary_control
         x observation_richness x schedule_amplification x boundary_crossing
         x proof_gap x blast_radius
```

Chosen-query + reusable/restartable + a public tag beats almost everything else.
Attack the top rows; pull the technique groups only when they explain the
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

## Stance

1. **Presuppose defects; FIND-AND-PROVE them.** Open every review/subagent prompt
   with *"there are flaws here — find and prove them; for what you can't break,
   prove it sound."* The presupposition flips bless-bias into a search. A
   **witness** must be one of: a compiled counterexample; a kernel-checked
   theorem exhibiting the bad equality/inequality; an executable small-scope
   counterexample; a public-surface adversary-import that typechecks; or a
   hand-checkable two-state distinguisher when the runtime/bridge is not
   formalized. Anything else is a *suspicion*, not a finding — and the proof
   requirement only reduces confabulation when the witness is mechanically
   checkable and tied to the stated claim.
2. **Model the adversary and its schedule.** Adaptive / chosen-query adversary in
   an adversarial context (read/replay/reorder/inject/drop/retry/fork/
   observe-timing/query-adaptively); always run the schedule amplifier. *Can the
   adversary just run again with a different input and diff the result?*
3. **The exported environment is the artifact, not the theorem** — types,
   constructors, recursors, projections, instances, simp lemmas, coercions,
   notation, macros, theorem *statements*, axioms. Audit the whole surface
   (`references/lean-robustness.md`).
4. **Align to the stated threat model — and be allowed to DOWNGRADE.** If an
   attack only works under a stronger adversary than the artifact claims to
   handle, still give the witness, but classify it as a *threat-model escalation
   / out-of-scope oracle / future-hardening requirement*, not a defect. This
   prevents false severity (e.g. a system that guards accident/manipulation, not
   an adversarial LLM — over-claiming a defect there is its own error).
5. **Run it on your own work first**, then delegate the same hunt to independent
   subagents (me → Claude → Codex), each find-and-prove.

> **The one rule that catches the most.** *Before proving a constructor safe,
> prove that every exported **eliminator** (recursor, projection, deriving
> instance, coercion, `noConfusion`) is intended, harmless, or impossible to call
> by the adversarial role.*

## The technique groups (a toolbox, pulled on demand)

Pull the lens that explains the current target. Canonical terms are latent-space
triggers — use them *after* you have a target and witness. Full lineage:
`references/lineage.md`.

### Decision-oracle extraction — *the operational core of information flow*

- **Public-result partition / decision oracle** — a public result type with
  constructors `c1…cn` induces a *partition* of hidden states; derive it, then
  ask whether the role may learn it. Witness = two hidden states differing only
  in the protected fact that produce different observations. (Replay's
  `ok / intentMismatch / recordExhausted` is not "an error type" — it computes
  `head.intent = actual` and `trace_nonempty`.)
- **Error algebra / diagnostic side channel** — every distinct failure reason
  is a declassification. Prove the role may learn each distinction, or collapse
  candidate-facing errors and keep rich diagnostics offline. Attack `not-found`
  vs `forbidden`, `mismatch` vs `exhausted`, timeout vs denial, parse-error vs
  auth-error, stale-version vs nonexistent.
- **Non-interference** as the two-run / **low-equivalence** theorem
  (`LowEq role s1 s2` ∧ secrets vary → low observations equal ∧ next states
  `LowEq`), proved by **relational Hoare / self-composition / product programs**;
  **unwinding** is the per-step form. **QIF / min-entropy** (Smith): `k`
  distinguishable outputs is a *crude* `log2 k` upper bound, distribution-
  dependent — constructor count is a smell, not a measurement.
- **Chosen-prefix oracle / active automaton learning** — if the adversary drives
  execution to a boundary, each public tag after that prefix is a membership
  query about the hidden trace/policy/state machine. Generalizes cross-run
  replay; catches trace-length, branch-shape, hidden-policy probing.

### Representation & API surface — *is the seal real?*

- **ADT abstraction-barrier leak / public elimination-surface exposure** (the
  precise name for the `casesOn` bug — the type was never abstract). Audit
  recursors, projections (`.1`, parent `toParent`), `noConfusion`, **deriving**
  (`Repr`/`BEq`/`DecidableEq`/`Ord`/`Hashable`/`SizeOf` — each an observer),
  instances + coercions, **`import all`**, reducible aliases, `autoImplicit`.
  Drill in `references/lean-robustness.md`.
- **Least-authority representation** — store the **capability** (a `step`
  closure), not the secret, so a total representation leak yields only the
  capability. Pair with terminality + affine use.
- **Multi-field coherence / evidence binding** — every payload with
  id+args+result+proof+version+authority+parents must prove its fields describe
  the *same* event. Attack by swapping one field while preserving the others;
  require ok-coherence, error-coherence, frame, provenance-binding, hash/payload
  agreement.
- **Contextual equivalence / refinement** — could *any* context distinguish the
  sealed handle from a non-leaking ideal? (Refinement/simulation by default; full
  abstraction only for hostile linking.)

### Authority & resources — *does ownership/budget add up?*

- **Separation logic / resource algebra / ownership** — identify the algebra;
  prove validity & conservation *under composition*; mark resources
  exclusive/fractional/duplicable/affine/linear; **attack aliasing** (two keys ↦
  one resource; one key ↦ two; derived/cached handles; parent/child overlap);
  name the minting boundary if disjointness is an obligation. **Frame rule**:
  prove unrelated capabilities unchanged.
- **POLA / object-capability + confused deputy / ambient authority** — is
  authority ambient (from position/global/session) rather than an explicit
  carried capability? Ambient authority is the confused-deputy /
  prompt-injection enabler.
- **Complete mediation** — every access checked on every path, including
  cached/derived.
- **Forkability / reset attack on linearity** — affine/linear guarantees are void
  unless the use-record cannot be copied, rewound, replayed, or re-decoded. Pure
  values are copyable; runtime state copies via snapshot, crash recovery, branch
  fork, test isolation, serialization, repeated submission. Prove the no-fork
  condition, reify the scheduler, or downgrade to a bridge/operator obligation.
- **Best correct approximation / strongest postcondition** — merely *sound*, or
  the *most precise* sound one? Is the slack an authored choice or baked-in
  over-approximation?

### Dynamics & composition — *across time, substitution, runtime*

- **Safety / liveness / fairness** (Alpern–Schneider) — is "authorization safety"
  silently assuming progress, freshness, cleanup, revocation?
- **Refinement mapping / simulation / bisimulation** (Abadi–Lamport;
  history/prophecy) — the way to prove a bridge (a shared helper is not a proof).
  Caveat: trace refinement preserves trace properties and *subset-closed*
  hyperproperties, not arbitrary ones.
- **Linearizability / linearization points**; **injective agreement** (Lowe —
  unique/fresh, not just "something similar ran"); crash / idempotence.
- **Codec / canonicalization lawfulness** — round-trip, idempotence,
  soundness+completeness, canonical uniqueness. A bad canonicalizer is a
  *replay-semantics* bug. **Runtime-bridge laws**: lowering preserves/narrows
  authority; "Lean proved it" ≠ "Rust does it".
- **Evaluator / provenance poisoning** (record-now-judge-later) — can a candidate
  produce a trace that's *safe as execution* but *misleading as evidence*
  (labels, region boundaries, version mapping, divergence spans, check
  identities, corpus, evaluator prompts) while staying within hard stops?

### Claim / spec truth — *is this the right theorem, honestly stated?*

- **Vacuity + mutation** — break a definition / weaken a hypothesis; does it still
  prove? Un-killed mutant = spec too weak. (See the vacuity test below.)
- **Adequacy of encodings** — is the model the *real* object, or one with extra
  inhabitants / collapsed distinctions?
- **Kernel conservativity** (did a def/quotient/axiom enlarge what the *kernel*
  proves?) vs **elaboration-surface stability** (did public instances/simp/
  reducibility change what downstream constructs or what proofs *mean*?).
- **Axiom / TCB budget** — `#print axioms`; `Classical.choice` is debt only under
  a constructive/extraction gate; `sorryAx`/`native_decide`/unsafe are the real
  red flags.

### Evidence generation

- **Small-scope / property-based** (Alloy small-scope; QuickChick) — instantiate
  finite parameters to small bounds and search (`decide`/`#eval`/enumerators)
  BEFORE proving; require a non-degenerate witness per headline. **Mutation ·
  metamorphic · differential · CEGAR · Hughes property taxonomy.**

## Calibration

**A real finding has all five** (else it's a nitpick): (1) violates an explicit
claim or necessary invariant; (2) names an in-scope role/adversary, or an
explicit threat escalation; (3) a reachable public observation or constructible
invalid state; (4) a minimal witness (two states distinguished / confused, or one
invalid state admitted); (5) impact — leaks a protected fact, fabricates
evidence, widens authority, loses conservation, corrupts replay/history, blocks
progress, or poisons the evaluator. A nitpick "only strengthens a theorem beyond
the claim" or "only changes proof style" or "relies on an out-of-scope threat
without saying so".

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

**Theorem vs document:** *theorem* when the property is semantic over modeled
values and a bad impl is expressible as a failing mutant; *export-surface drill*
when it's syntactic non-definability; *runtime/operator obligation* when it
depends on single-use/terminality/crash/campaign; *honest doc* when the system
intentionally doesn't defend it (and the threat model says so).

**Stop when** every high-score target has one of: proven defect + witness; proven
invariant; killed mutant; an explicit bridge/operator/evaluator obligation; or a
declared out-of-scope escalation. **Keep going if** a public observation has no
allowed-declassification story; a strong boundary phrase has no enforcement rank;
a theorem has no bad mutant it rejects; a resource has no
conservation/disjointness/anti-fork story; or a bridge claim hasn't been
schedule-lifted. Do **not** stop because all lenses were mentioned, nor continue
because another lens exists.

## Application sequence

1. Extract the strongest claims (the quoted words above) and, for each, the
   protected thing + role.
2. Build the **oracle table** — every public observation and the hidden predicate
   it computes.
3. Rank by adversary-control × schedule-amplification × claim-strength ×
   proof-gap.
4. Attack the top target: the two-run distinguisher / forgery pair /
   admitted-invalid-state; then run the **schedule amplifier**.
5. Only now pull lenses: codec law for canonicalization targets; export-surface
   drill for sealed targets; resource algebra for authority/fuel; non-interference
   for confidentiality; simulation for bridge/runtime; mutation/small-scope for
   theorem adequacy; error-algebra for failure tags; provenance for
   record-now-judge-later.
6. **Classify by rank** (theorem / export drill / runtime bridge / operator
   policy / evaluator / doc) and by threat scope (in-scope defect vs escalation vs
   out-of-scope).
7. Read proof bodies LAST — after the target and expected theorem shape are known.

## Reusable subagent prompts (find-and-prove)

```text
Build the ORACLE TABLE first; do NOT list lenses. For each exported function/
result/error/instance/log/timing/termination: hidden variable | public
observation | predicate computed | adversary control | schedule lift | allowed
declassification? | witness-or-invariant. Then attack only the top THREE rows by
score, producing for each: a compiled witness / two-run distinguisher / forgery
pair / rejected-attack theorem. Classify each by enforcement rank and threat
scope.
```

```text
For each headline theorem: write the bad MUTANT it is supposed to reject. Does
the theorem FAIL under that mutant (compile it)? If not, classify vacuous or
under-scoped. If yes, state the exact PUBLIC behavior it constrains.
```

```text
Then sweep the IMPLEMENTATION mechanically — do NOT reason about coverage,
EXECUTE it. The dumb brute-force check ("delete/change this, recompile")
finds what targeted reasoning rationalizes away ("surely the types forbid
that" — when they don't). For each load-bearing definition apply each
operator — each replaces a construct with a smaller-state-space form (full
catalog: lean-robustness "Mutation operators"): weaken a carrier (⊆→=),
degenerate a field to a constant (children→nil), swap a constructor
(ordinary→authoring), off-by-one a count (≤→<), identity a transform (map→id),
flip a branch/Option — then recompile against the WHOLE theorem set. A mutant
that compiles GREEN means the lower-power form passes every theorem, forcing a
binary: EITHER adopt it (the power was unneeded — shrink the code) OR add the
theorem that forces the dropped behavior (the gap). Never rewrite to mask it.
Sweep the full set, not one def. A kill counts only if SEMANTIC — silence
incidental warningAsError lints (rename _x) first, or you log a false kill
that overstates adequacy.
```

```text
For each claim containing "private/sealed/affine/terminal/only/never/deferred":
assign its enforcement rank (theorem / export-drill / runtime-bridge /
operator-policy / evaluator / doc). If the artifact enforces it at the WRONG
rank, give the minimal witness or the reason.
```

```text
Audit the exported environment as a hostile downstream module: #check rec/recOn/
casesOn/noConfusion/projections; #synth Repr/BEq/DecidableEq/Hashable/Inhabited;
try .1/.2/parent projections/coercions/structure-update/import-all. Prove what
each leaks; anything past the role's authority is a defect.
```

## References (load on demand)

- `references/lean-robustness.md` — the Lean export-surface attack catalog, the
  `#check`/`#synth` adversary-import drill, and robust-Lean habits
  (capability-sealed handles, four theorem families, `LowEq`/two-run,
  `DocClaims`-pinning, `autoImplicit false`, intentional mutation).
- `references/lineage.md` — published-lineage table: every technique → canonical
  name + reference (including the niche framings kept off the main spine:
  Dolev–Yao, full abstraction, the PER/DCC unification, intransitive/robust
  declassification).

## Relationship to other skills

- `state-space-minimization-formal` — the state-validity calculus; this adds the
  oracle hunt, information-flow, capability/resource-separation, and
  representation-leak reasoning on top.
- `code-review` — general review; this is the adversarial/formal specialization.
- Memory `feedback_adversarial_review_lenses` points here.
