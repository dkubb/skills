# Plan: the formal-review skill suite (find-and-prove v3 + formal-design)

**Audience.** A future LLM session (Claude/Fable orchestrator, Codex, or a
Pro-driving subagent) tasked with (a) building this suite, or (b) reviewing /
designing Lean 4 and similar formally-proven code before the suite exists.
Section 3 is directly usable as a review rubric today — start there if you are
reviewing, not building. **If executing this plan as a goal, read §7's
execution protocol first** — it tells you how to locate the current phase
from the repo state and what you may do without asking the operator.

**Sources consolidated.**

1. `skills/find-and-prove/` at 2026-07-06 (post PR #4 — canonical-name +
   new-technique commits).
2. The symbiote harvest catalog
   (`~/.claude/projects/-Users-dkubb-workspace-dkubb-symbiote/memory/reference_pro_reasoning_principles.md`,
   ~150 techniques from Pro/Codex/Fable review rounds, audited 2026-07-06:
   ~25 fully graduated, ~35 partial, ~70 missing, rest other-skill territory).
3. Published literature not yet in either (marked NEW below).

**Verdict that motivated this plan.** The catalog is sound (3 minor caveats,
all noted below) and ~4x richer than the current skill. Its most
battle-hardened items — rubric #12/#14/#15/#16/#17, the `_iff` pin family, the
Fable out-finds — are precisely the ones not yet graduated. A whole defect
class recurs that the mutation gate *structurally cannot catch*
(name↔referent, rfl-passthrough, instance wiring, existential subject,
proof-term false kills): statement-surface defects need statement-level
audits, not more mutants.

---

## 1 Design principles (binding for the build)

1. **LLM-first vocabulary.** Every technique is named by its published term of
   art where one exists (latent-space anchor), with the operational house name
   in apposition. New lineage rows for every new anchor.
2. **Triggers, not lenses.** The catalog's hardest meta-lesson: reviewers had
   the lens and missed the *application site*. Every rubric item states its
   FIRE-ON condition ("on every new predicate", "on every ∃-headline", "on
   every constructor addition"). An item without a trigger is not done.
3. **Witness discipline.** A finding requires a compiled witness / two-state
   distinguisher / typechecking adversary import; a technique requires the
   named mutant it kills. Unchanged from v2; extended by the false-kill rule
   (H1).
4. **Execute, don't argue.** Anything checkable by compile/probe/linter is
   never discharged by reasoning. New: a mechanical floor (H3) of Lean-native
   checks that runs before any judgment lens.
5. **Rank honesty.** Every claim gets exactly one enforcement rank (kernel
   theorem / export drill / runtime bridge / operator policy / evaluator /
   honest doc). Unchanged from v2.
6. **Two modes, two skills.** Attacking an artifact (review-time) and choosing
   a design (build-time) are different activities with different triggers;
   they get sibling skills that cross-reference.
7. **Acceptance = blind reproduction.** A graduated technique is validated by
   handing a fresh subagent a filter-safe puzzle at the pre-fix state, armed
   only with the technique, and getting the known finding on attempt 1 (the
   symbiote protocol). The winning prompts ship in the skill.

---

## 2 Architecture

### The suite

| Skill | Mode | Owns |
|---|---|---|
| `find-and-prove` (keep name) | review-time | stance, oracle hunt, **the binding rubric** (§3), evidence mechanics, enforcement ranks, calibration, adjudication, subagent prompt library, Lean export-surface reference |
| `formal-design` (new) | build-time | pre-flight decision procedures and keyed question sets run BEFORE authoring a slice: identity design, carrier/representation choice, primitive forcing, increment scoping, seam/interface extraction |
| `state-space-minimization` (+`-formal`) | unchanged | encoding order, constructive encoding, normalization, boundary parsing — find-and-prove cites, never duplicates |

**Names — BINDING DEFAULTS.** Keep `find-and-prove`; name the sibling
`formal-design`. An executor proceeds on these without asking, unless the
operator has said otherwise in the session or a memory. Rationale:
`find-and-prove` is a coinage but encodes the stance, is already
cross-referenced (memories, SSM, the symbiote plan), and the latent-space
work is done by the technique names inside, not the skill slug. Recorded
alternatives if the operator ever renames: `proof-audit`,
`adversarial-spec-review`; `theorem-design`, `slice-design`.

### Internal structure of find-and-prove v3

SKILL.md stays a dispatch layer (~its current size); depth moves to reference
modules. New spine:

```text
SKILL.md
  Stance (v2 stance 1-6, incl. reviewer-directed prompt injection)
  The hunt (oracle table, triage, schedule amplifier, ranks — v2, slimmed)
  THE BINDING RUBRIC — one line per item + FIRE-ON + reference pointer (§3)
  Mechanical floor (H3 summary — run these before judging)
  Calibration (v2 + nitpick/ergonomics rule)
  Adjudication (v2 + HEAD-check, keep-list, challenge→authority, reproduction)
  Application sequence
references/
  rubric.md            — full rubric with witnesses, mutants, worked examples
  vacuity.md           — defect class A (the collapse lattice is the core)
  pins.md              — defect class B (the _iff family is the core)
  reception.md         — defect class C (the mutation-blind class)
  scope.md             — defect class D
  identity.md          — defect class E
  resources.md         — defect class F
  basis.md             — defect class G
  evidence.md          — defect class H (mutation mechanics, witness rules,
                         mechanical floor; absorbs lean-robustness "Mutation
                         operators" + "Mutate intentionally")
  information-flow.md  — defect class I (confidentiality: the oracle-hunt's
                         leak lenses — partition oracle, error algebra,
                         non-interference/LowEq, QIF, declassification,
                         chosen-prefix; representation-seal: least-authority,
                         contextual equivalence); lean-robustness is its
                         Lean drill
  lean-robustness.md   — export-surface catalog + robust-Lean habits (kept)
  lineage.md           — extended with every new anchor in §3
  prompts.md           — subagent prompts incl. reproduced winning prompts
```

---

## 3 THE BINDING RUBRIC v1 (usable as a guide today)

Format per item: **FIRE ON** → check → the mutant/witness that decides it.
[Anchors] are the latent-space terms; ★n = origin in the symbiote catalog's
Pro-rubric item n; (NEW) = not in catalog or current skill.

### A — Vacuity: does the suite prove anything?

- **A1 Trivial-model sweep** [feasibility; Morgan's *miracle*] — FIRE once per
  suite: can the do-nothing implementation (empty trace / reject-everything /
  no-op) satisfy every advertised theorem? One `#eval`/`decide` sweep.
- **A2 Predicate-collapse lattice** [specification mutation, Ammann–Black
  1999 (NEW anchor); coverage metrics, Chockler–Kupferman–Vardi (NEW)] — FIRE
  on every NEW predicate, including one in CONCLUSION position (★10): descend
  all seven levels, each a distinct surviving-mutant class:
  1. truth-value: test `:= True` AND `:= False` (False makes consumers
     vacuous, not false — the asymmetry hides it);
  2. quantifier scope: head-only / last-only / wrong-request mutants;
  3. value restriction: a hardcoded witness value lets `o = c ∧ …` survive —
     parametrize over a caller-chosen value;
  4. position/cardinality: negatives parametric over position
     (`pre ++ bad :: post`);
  5. OVER-restriction: positives must be exactly as parametric as negatives
     (symmetry is the convergence criterion);
  6. witness shape: `∃!`, identity-parse over-restriction;
  7. close the class: pin the INTRODUCTION rule (`evidence → predicate`)
     instead of chasing witnesses.
- **A3 Annotation-constant sweep** (graduated; keep) — FIRE on every decorated
  relation/step: all labels → one constant across the WHOLE suite; a label is
  real only when a theorem reads it back by exact count delta.
- **A4 Echo lens** [inherent vacuity (NEW anchor)] — FIRE on every theorem
  whose conclusion shares atoms with a hypothesis: a conclusion conjunct
  restating a hypothesis does no work but reads as proven. Decide by the
  delete-BOTH counter-mutant (drop hypothesis and conjunct; still proves ⇒
  echo). Sibling: swap the exhibited run witness for the trivial one and
  require failure.
- **A5 Fuel/partiality vacuity** (graduated; keep) — `∃ fuel` and `partial
  def` opt-outs.
- **A6 Inhabitance vs conditional schema** — FIRE on every fixture theorem
  `(h : Step …) ⊢ C`: hypothesis-supplied steps are not machine-checked
  inhabitable; compile the concrete `∃`-witness.
- **A7 Behavior-drop mutants** (precision-vs-safety direction) — FIRE on every
  classification/exact-cases master theorem: it classifies the steps that
  EXIST and cannot catch a hidden premise NARROWING when a constructor fires;
  premise-STRENGTHENING is a mutation direction the weakening-only operator
  catalog misses. Each distinguishable trigger of a channel needs its own
  reachability witness (or one quantifying over all).

### B — Unpinned surface: it holds, but binds nothing

- **B1 The `_iff` pin family** (★12/★14/★17 — the highest-yield family in the
  catalog) — three FIRE-ONs:
  - every new erasure/refinement RELATION on the mutable surface: prove
    `R ↔ <explicit content>`, every conjunct spelled out; forward simulation
    survives both too-weak and too-restrictive relations, the iff kills both;
  - every new wrapper `def P : Prop` or `structure … : Prop` consumed
    downstream: prove `P_iff : P ↔ <body>`; witnesses+projections leave the
    hidden-extra-condition, drop-a-field, hidden-`False`, and field-weakening
    mutants alive; only the iff kills all four; demote witnesses/projections
    to corollaries;
  - every laws-carrying record FIELD (instance honesty, Fable): a dishonest
    instance (`spec := decode's own graph`) leaves every named-predicate
    theorem green — require the defeq lock `C.field ≡ named-predicate` by
    `Iff.rfl` as a headline, verified to redden under the graph-swap mutant.
    Per-definition mutation structurally cannot catch instance wiring.
- **B2 Producer pin** (★15) — FIRE on every type-level
  injectivity/no-collision headline: it is about the TYPE; require
  `produced-token-with-full-identity ∈ ruleOutput` and make the headline
  producer-grounded (read events off the rule's output, then assert
  distinctness). Pin a kept lossy projection's non-degeneracy.
- **B3 Cardinality conjunct** [exactly-once; multiplicity] — FIRE on every
  universal value claim over a linear/unique resource: `∀ x, P x → x = good`
  is forged by TWO copies of the right thing; `count = 1` is first-class,
  never derived; the `∃ rest` form needs an explicit not-in-rest clause. The
  most-recurring forgery class in the harvest.
- **B4 Read-back at full strength** — FIRE on every certificate/wrapper field
  and every label: some theorem must consume the field AT the strength the
  docs claim ("ready for X and Y" needs a consumer per half — a one-line
  corollary is "redundant as a lemma, NOT redundant as a spec guard").
- **B5 Existential subject & coupling** (graduated half + Pro's
  blind-reproduced half) — FIRE on every `∃`-headline: (a) conclusion TYPE
  couples returned witnesses to the hidden ones (indexed relation or
  `∃ t, Reads t ∧ P t`) — the decoy-witness separating state decides it;
  (b) every conjunct's grammatical SUBJECT is the bound variable, not a
  ground helper term that happens to equal the witness — both forms are defeq
  at the witness, so mutation cannot distinguish them; the fix is syntactic.
- **B6 Quantified-object coverage** (Codex) — FIRE on every run-coupled/`∀`
  headline: every conjunct must mention the quantified run objects; a conjunct
  over a static canonical fixture binds no run (transport canonical facts onto
  the run via perm-invariant properties).
- **B7 rfl-headline symbolic passthrough** (Fable) — FIRE on every exact `_eq`
  headline proved by `rfl` whose RHS names a helper: it pins the CALLER's
  shape; mutating the helper moves both sides. The helper needs its own
  `_iff`/`_eq` pin; calibrate any "reddens mutant X" claim to the caller
  locus.
- **B8 Law-basis minimality + widening distinguisher** (Pro + Fable) — store
  the value-exact law, DERIVE bit-exactness (converse admits fabricators);
  prove minimality adversary-binding by compiling an inhabitant of the WEAKER
  law that fails the stored one, committed as an in-module negative guard.
  State-space WIDENING of a law field is a mutation-operator class that
  delete/weaken operators cannot generate — add it to the catalog.
- **B9 Exact-object conclusions** — prefer whole-state equality over
  field-wise conjunction (a future field leaves the conjunction
  compiling-but-silent; exact equality fails loudly); state change-factoring
  as an exact-delta disjunction, never a `≠`-triggered hypothesis.
- **B10 Multiplicity-parametric positives** (★11) — FIRE on every list-level
  mutable function: a singleton demo survives `_::_::_ ↦ []`; require exact
  nil/cons/`_eq_map` characterization or a multiplicity-parametric witness.
- **B11 Halt-arm full-field pin** — FIRE on every failure/halt rule: "records
  nothing" still carries every prior field; the `∃`-free full-config matcher
  pins each carried field (each is a silent-change candidate).
- **B12 Hygiene exports** — when a semantic headline cannot catch
  inert-residue mutants, keep it semantic and EXPORT the no-leftover facts as
  named basis; validate with a compiled leaky-state witness that passes every
  headline conjunct and reddens an export.
- **B13 Def-level substitution** [pseudo-oracle, Davis–Weyuker; common-mode
  failure (NEW anchors)] (Codex) — FIRE when one shared helper defines the
  net AND the config AND the run: everything can be coherently wrong and every
  theorem still proves. Pin guarantees over literal authored values; make
  witnesses use non-degenerate field values. Self-question: "could all three
  be wrong the SAME way?"

### C — Overclaim: statement weaker than name/prose (the mutation-blind class)

- **C1 Object-referent audit** (★16, Fable's first out-find) — FIRE on every
  theorem whose NAME references a concrete object (`…Trace`, `…authority`,
  `…spend`): does the STATEMENT mention THAT object, or a different one at a
  different representation level? True theorem, every mutant dies — pure
  reception. Siblings: run-independence of a transported `∃` (argument list
  omits the quantified run?); lossy-projection comparison-level check;
  cosmetic-defeq-vs-genuine-read (break the input to prove the `rfl` bridge
  reads it).
- **C2 N-distinct arity audit** (Fable) — FIRE on every `*_distinct` /
  `*_disjoint` name: N distinct objects need exactly C(N,2) pairwise
  inequalities in the statement; `¬(A ∧ B)` does not exclude A alone. Tell: a
  docstring case taxonomy that silently omits a case points at the missing
  conjunct.
- **C3 Name-vs-semantic-load** — FIRE on every named predicate: construct a
  value SATISFYING the predicate that VIOLATES what the name implies (e.g.
  set-level injectivity named as occurrence uniqueness is satisfied by
  `[e, e]`); rename to the literal content.
- **C4 Frame honesty** [small-footprint / tight specifications,
  O'Hearn–Reynolds–Yang (NEW anchor)] — FIRE on every "no X" / "contains no
  X" claim in a `redex ++ frame` system: disambiguate three readings (the
  produced block has no X / no NEW X / the whole output has no X) — the
  whole-bag reading needs a frame-absence hypothesis. Prose for a bare local
  rewrite says "residue block", never "in G′". Includes temporal inversion
  (graduated) and the shadow-difference probe: neutralize the certified
  distinguisher (constant producer) and check whether another already-varying
  field still proves the conclusion — if yes the intended premise is dead.
- **C5 Bystander/frame-generic overclaim** — FIRE on every uniqueness /
  confluence headline over `∀ frame` + named participants: can the named
  parties be instantiated as bystanders while other owners do the real steps?
  Index hypotheses to the actual firing/owner; demote the frame-generic form
  to an internal engine.
- **C6 Exhibits-vs-projects** [angelic vs demonic nondeterminism (NEW
  anchor)] — a direct construction of a witness for one schedule is a
  legitimate weaker theorem than a projection binding the quantified run's own
  state; prose must say "exhibits, for this schedule", never "projects / the
  same run"; blocks only when the headline claims universality.
- **C7 Docs & prose battery** — FIRE per docs pass: the three-leak taxonomy
  (fixture qualifier dropped / claim-changing guard dropped / vocabulary from
  a richer layer = altitude overclaim); two-snapshot drift on promotion (grep
  the whole doc for the old "future" framing; "converged" cannot coexist with
  a named open fork); verb tense sets claim altitude; word-class sweep
  (graduated) + its status-promotion trigger; determinism ≠ cause-uniqueness;
  injectivity hypotheses belong to inversion, never forward projection;
  scope a fixture-valid distinguisher's prose to the class where it survives;
  plumbing-vs-laundering (ban the forbidden DERIVATION by theorem name, not
  the carrier hypothesis; the anti-laundering tell is a lemma arbitrary in the
  index a laundered version would need); after any lossy projection, scope
  every downstream claim to what survives it.
- **C8 Naming vocabulary** — reserve slogan names for the final conjunction
  (★8); name the DANGEROUS half (`selectedSuffixEmbed`, not `prefixEmbed`);
  "consume" only for linear removal, "read" for persistent facts; "support"
  never "provenance" (graduated); tag export twins; boundary-based over
  event-based capstone names.

### D — Mis-scoped: right claim, wrong boundary or rank

- **D1 Enforcement rank + bad-lowerer test** (graduated; keep) — every claim
  gets exactly one rank; run-output properties a bad lowerer can falsify are
  lowering obligations carried in the statement.
- **D2 Reach-past-boundary** (★2 — caught what the whole inner loop missed) —
  FIRE on every success/positive witness: a premise about a step LATER than
  the property pinned is the mechanical tell of overbuild; stop the witness at
  the boundary; the full-run version demotes to an integration corollary. The
  headline fixture is the SMALLEST net reaching the new boundary and stopping
  right after it (never a prefix of a richer fixture — a cross-layer witness
  must be a same-shape machine that terminates).
- **D3 Inductiveness battery** (graduated CTI lens + additions) — phase
  closure / counterexample-to-induction; WF-uniqueness ≠ semantic cleanliness
  (multiplicity bounds admit orphan garbage — a named Clean predicate as a
  consumed phase obligation, never a negative premise); persistent-fact
  re-entry (can history already contain the fact I am about to add?).
- **D4 Channel coverage** — FIRE on every "full X for every Y" claim: does
  the mechanism actually RUN on every channel quantified over? If one channel
  bypasses it, scope-and-rename or fix the mechanism first; classify as model
  boundary, not proof bug.
- **D5 Forward pressure** (★3) — does the theorem's scope survive the NEXT
  planned rung, or plant a landmine (whole-run parametricity a later identity
  layer contradicts)? Deny the right axis: some axes are denied forever
  (authority), some will be affirmed later (identity).
- **D6 Ledger separation** — admission predicate (quantifies over EVERY
  branch — security reading) vs selected-branch accounting (charges the
  branch that ran — correctness reading); "draws no authority" ≠ "costs no
  fuel"; audit that no proof under a boundary imports a theorem from the
  incompatible resource regime.
- **D7 Run-position of an invariant** — "for every emitted X, f(state)" over
  final-trace membership is wrong when the property is about the state
  immediately BEFORE the witnessing step; factor prefix→step→suffix and state
  it over the prefix projection.
- **D8 Assembly discipline** (★13) — before conjoining separately-proven
  facts: one shared run object, compatible quantifiers; structural step-level
  facts need same-object analogues (they do not transfer through erasure the
  way trace facts do); a property may not hold of every prefix; a fixture
  witness never masquerades as a universal predicate.

### E — Identity & causality

- **E1 Collision-kernel audit** [kernel of a map (NEW anchor)] — FIRE on
  every lossy projection at an identity boundary: never ask "is it
  injective?"; compute the kernel — per constructor, mark each distinguishing
  field KEPT / DROPPED / QUOTIENTED; demand the invariant that recovers each
  drop; force the multiplicity case (N=2 equal payloads into a set target =
  the element-vs-container axis: after element injectivity, ask whether the
  container quotients order/multiplicity — `[x]` vs `[x, x]`).
- **E2 Two-schedule test** [Mazurkiewicz traces; happens-before, Lamport (NEW
  anchors)] — FIRE on every "causal / parent / frontier" structure: run the
  update on independent events under A;B and B;A; if either parent set
  contains the other purely from serial order, it is a prefix log wearing a
  causal label. Replay keys must be invariant under topological reorder;
  result order belongs in the payload, never the parent set.
- **E3 Identity coverage** — full-identity injectivity per KIND (cross-kind
  disjointness is necessary, not sufficient); enumerate every constructor the
  identity ranges over and every collision axis the consuming redex matches
  on (occurrence, owner, branch, position — compile the "same request,
  different occurrence" witness); namespace tiers [freshness, nominal logic,
  Gabbay–Pitts (NEW anchor)]: sibling-vs-sibling distinctness is a different
  obligation from new-vs-any-pre-existing; never write "cannot collide"
  unqualified.
- **E4 Identity construction** — fresh identities are deterministic functions
  of local structure (tree addresses with injectivity/disjointness laws),
  never scheduler-dependent counters [content addressing; De Bruijn (NEW
  anchors)]; renderer/canonicalizer version is part of replay identity;
  in-memory truth vs durable projection: persist the structured identity or
  prove it reconstructs — state which.
- **E5 Lossy compatibility layers** — a fix that ISOLATES a collision into a
  lossy projection is sound only for consumers reading the injective truth:
  name exactly which consumers, demote the frozen lossy identity to a
  projection with a truth→projection theorem, and check the recorded trace
  retains every identity axis the NEXT theorem needs after proof witnesses
  are forgotten.

### F — Resources & composition

- **F1 Conservation shape** — the law is an aggregate sum
  (`childL + childR ≤ parent`), never pointwise containment (each-child-≤
  admits finite-authority duplication); in Lean, derive `draws ≤ ceiling`
  BEFORE truncated subtraction (`Nat.sub` silently masks over-draw); read
  "remaining" off the reached state, not a static expression.
- **F2 Split-vs-copy** [linear logic exponentials (NEW anchor)] — a fork-like
  rule may COPY information (causal history) but must SPLIT consumable
  authority; decision rule: split iff the invariant bounds a sum over all
  parties.
- **F3 Control multiplication costs budget** [potential method (NEW anchor)]
  — any rule that creates control (fork/spawn) consumes a budget unit and is
  forbidden at zero, else a zero-budget actor multiplies unboundedly; verify
  with a potential-function measure.
- **F4 Composition battery** — A∪B bridge invariant (graduated — with the 4c
  correction: prove the pure ALGEBRA now, defer only the semantic implication
  discharging its hypotheses); prove-the-run not the store (the
  store-transplant mutant: importing a prior stage's final store into a fresh
  prestate proves nothing about reachability — thread the actual run);
  packaged witness before parallel generalization (same-source+target pairing
  is sound only under forced schedule); physical-execution coupling with
  Pro's scope rule (couple fuel only when the headline EXHIBITS a run; a
  universal over a SUPPLIED run is legitimately fuel-orthogonal); TOCTOU and
  forkability (graduated).
- **F5 Parallel-rung observables** — once independent redexes exist, exact
  chronological trace equality is the wrong observable: target DAG /
  event-set / per-owner-fold invariants with a serial-projection theorem only
  where the spec demands order; name single-strand results "forced-schedule",
  never "deterministic/confluent"; the non-vacuity shape for a schedule-set
  abstraction is a swapped-schedule witness ACCEPTED by the new surface and
  REJECTED by the old exact pin, plus one interior-visiting schedule (the two
  boundary walls do not evidence universality).

### G — Basis curation (headline vs corollary)

- **G1 Declared-universe irredundancy** (★1) — irredundancy is undefined
  until the mutation universe is declared (fixed background vs mutable
  surface); the same headline flips KEEP↔DEMOTE with the declaration. KEEP is
  proven by a unique-kill witness model (satisfies all other headlines,
  falsifies this one — compile it); DEMOTE by derivation. Extends the
  graduated delete-a-headline lens.
- **G2 Keep-side calibration** — "the witness depends on it" proves USED, not
  basis-worthy (the test: does the STATEMENT pin a public surface no other
  headline states?); a cheap proof is not evidence of redundancy; an
  unconsumed bridge kept as a visible vocabulary-agreement pin is legitimate
  basis; reception may beat bare minimality (`= 1` over `≤ 1` when the
  stronger form is the contract readers need) — document the tiebreak.
- **G3 Placement rules** — projection-straddling: state the load-bearing
  theorem in PUBLIC vocabulary, prove by internal induction, demote the
  readable weakening (★7); at most one projection headline; per-element
  relation is the headline, the aggregate view an extensionality corollary;
  when a rung generalizes a landed concept, keep the specific one and add
  `old ≡ new instance` as a bridge theorem (subsumption-as-theorem, never
  churn); a non-vacuity reachability witness EARNS basis status (★6); split
  an assumption-free algebraic core from the invariant-concluding semantic
  theorem and headline the semantic one (Codex).

### H — Evidence mechanics (how to actually decide)

- **H1 Mutation discipline** — operators in BOTH directions: the existing
  weakening catalog plus premise-STRENGTHENING / behavior-drop (A7) and
  law-field WIDENING (B8), which delete/weaken operators cannot generate.
  **False-kill rule** (Fable; Pro: "the biggest harvest"): a kill manifesting
  only as a proof-term break (rfl/simp in a witness; a type mismatch a
  canonical coercion repairs) is scored only after applying the minimal
  semantic repair the weakened statement deserves — classic case: a field
  derivable from a sibling via an iff. **Detector isolation** (★15): mutate
  ONLY the def body, never the detector theorem (a `replace_all` that moves
  both falsely passes). **Differential per-site isolation**: to localize a
  survivor among candidate sites, weaken each site individually and compile
  — never reason about which is load-bearing.
- **H2 Witness discipline** — the NEGATIVE witness carries the identity claim
  (free hypotheses, only the key differing); positives as parametric as
  negatives; per-axis non-degeneracy plus ONE witness non-degenerate in ALL
  axes (interaction mutants); the degenerate-fixture probe as the complement
  (all fields equal, zero fuel — isolates the one intended distinctness
  source and proves no theorem needs undeclared nondegeneracy) [boundary
  value analysis (NEW anchor)]; minimal boundary fixture (D2).
- **H3 Mechanical floor (Lean-native; run FIRST)** (NEW, this plan) —
  `#lint` / mathlib-style environment linters (unused hypotheses, simp-nf,
  doc linters) mechanize several rubric items; `#guard_msgs` elaboration
  snapshots pin statements and error messages byte-stable across refactors
  (mechanizes statement authenticity and the interface-extraction
  "statements byte-stable" question); `pp.all` re-elaboration + `lean4checker`
  (graduated); greps for `@[implemented_by]`/`@[extern]`/`opaque`/`partial`/
  `native_decide` (graduated) promoted to CI ratchets; `plausible`/`#eval`/
  `decide` before proving (graduated); `exact?`/premise-selection as a
  mechanized redundancy probe for delete-a-headline; compile-the-prose rule
  (Fable): discharge every "this lifts to the general case" doc claim by
  compiling the general corollary in a scratch probe.
- **H4 Proof-method skeletons** (reference-only; the constructive duals) —
  the ∀-terminal-refinement skeleton (canonical per-phase bags → control
  uniqueness → local inversion per operational leaf → snoc-induction phase
  classification → concrete next-step witnesses for nonterminal exclusion →
  recouple order-sensitive erasers from terminal membership + uniqueness);
  extensional-agreement-as-transport for lawful interface bundles (bundle the
  law as a field + payoff theorem that every lawful inhabitant agrees with
  the concrete adapter); sealed-handle stateful drive (fail-closed prefix law
  over an arbitrary handle so only the public step op is usable); two-tier
  failure honesty (request divergence fail-closed vs local miss
  advance-and-continue, each pinned).

### I — Confidentiality & information flow (the oracle-hunt's leak lenses)

This class is the original skill's information-flow spine, preserved intact —
the catalog is theorem-adequacy-oriented and barely touched it, so nothing
here is shed. It is the confidentiality half of the hunt: what can the role
observe or distinguish? `references/lean-robustness.md` is its Lean-specific
attack drill.

- **I1 Public-result partition / decision oracle** — FIRE on every public
  result type: its constructors induce a partition of hidden states; derive
  it, then ask whether the role may learn it. Witness = two hidden states
  differing only in the protected fact that produce different observations.
- **I2 Error algebra / diagnostic side channel** [padding-oracle family,
  Vaudenay, Bleichenbacher] — FIRE on every distinct failure reason: each is
  a declassification, compounded by cross-run adaptivity; prove the role may
  learn each distinction, or collapse candidate-facing errors and keep rich
  diagnostics offline.
- **I3 Non-interference** [two-run / low-equivalence; relational Hoare /
  self-composition; unwinding per-step; QIF / min-entropy for measurement] —
  FIRE on every confidentiality claim: prove `LowEq role s₁ s₂ ∧ secrets vary
  → low observations equal ∧ next states LowEq` by self-composition; a
  constructor count is a smell (crude `log₂k`), not a measurement.
- **I4 Chosen-prefix oracle / active automaton learning** [Angluin L*] — FIRE
  when the adversary drives execution to a boundary: each public tag after the
  prefix is a membership query about the hidden trace/policy/state machine;
  generalizes cross-run replay.
- **I5 Representation-seal reality** [ADT abstraction-barrier leak;
  least-authority; contextual equivalence / full abstraction] — FIRE on every
  sealed handle: is the abstraction barrier real, or does a public eliminator
  (recursor, projection, `noConfusion`, deriving instance, coercion) leak the
  hidden field? Store the capability, not the secret; ask whether any context
  could distinguish the sealed handle from a non-leaking ideal. The full drill
  is `lean-robustness.md`.
- **I6 Declassification discipline** [dimensions & principles; robust
  declassification / transparent endorsement] — FIRE on every intended
  release: name what/who/where/when is released and prove the attacker cannot
  influence it (robustness) and attacker data cannot launder into trusted
  evidence (the integrity dual).

### Adjudication additions (SKILL.md)

Keep v2's four rules; add: (5) verify a CLEAN verdict was produced against
HEAD (check cited line numbers); (6) reviews name what to KEEP (positive
confirmation calibrates trust and prevents over-correction); (7)
challenge→authority loop: refute inner-reviewer calibrations you disagree
with (compile the refutation), route genuine splits to the design authority —
never defer, never majority-vote; (8) blind-reproduction validation for any
newly harvested lens before graduating it (filter-safe puzzle at the pre-fix
state, attempt-1 match required); (9) tiering: mechanical floor and builds on
the cheap model, adversarial review on the advanced model — the advanced
reviewer reliably catches the C-class (reception) defects the gate cannot.

---

## 4 `formal-design` blueprint (build-time sibling)

Purpose: run BEFORE authoring a slice/rung of a formal system. Structure:
a small SKILL.md dispatcher ("what kind of slice is this?") + one reference
per slice kind, each a keyed question set with the failure mode each question
pre-empts. Content (all from the catalog):

- **Increment scoping** — one degree of freedom per rung ("is my smallest
  increment a concrete instance of the new freedom, or the freedom itself?");
  split a bundled rung by its hardest dependency; defer the TRANSFORM never
  the DATA (the persistent fact carries the raw datum so the next rung is
  additive); "additive later" requires the extension point NOW (a headline
  can only reject mutants its types can express); add-and-coexist, retire
  only after proven out; the retirement-is-a-mirage 3-part test;
  first-consumer boundary test (which rule first READS the new fact — a
  producer-only rung is storage, name it so); false-dependency-direction
  (is A-before-B logic or an artifact of planning order?).
- **New primitive** — expressiveness-forcing proof (two states agreeing on
  everything the current primitive reads but requiring different outputs —
  the two-state distinguisher as a NECESSITY proof); a forced primitive
  generalizes an existing seam at higher arity, never adds a capability or
  changes what existing structure means; fixture/layout stays out of the
  primitive; first-order closedness (closed data tokens, never a function
  field — defunctionalize).
- **Identity design** — distinct constructors per event kind from day one;
  structured sum-type ids before codecs; the full E-class rubric run at
  design time; route identity through concrete addresses when the abstract
  type lacks structure; stable schedule-independent freshness (origin + site
  + iteration ordinals; static ids break under loops, counters reintroduce
  schedule-dependence).
- **Carrier/representation** — route-vs-encode 8-question procedure (opaque
  carry vs typed branch vs replay-key vs adapter); representation boundary
  vs synonym (semantic boundaries get nominal carriers, never `abbrev`);
  shared-source-with-two-projections (use the source-shaped side with an
  identity projection; invented defaults are worse); dependent-index
  encoding pointer → SSM; carrier-swap bridge 15-question set (inverse-pair
  mutants: round-trip laws tolerate compensating error pairs — pin the
  projection's exact content in the FUTURE carrier's vocabulary).
- **Seam/interface extraction** — Pro's 12-question set (delete-the-
  abstraction reddens a downstream consumer; interface exactly as strong as
  downstream needs; don't-over-generalize: extract the SMALLEST theorem that
  pins the abstraction; concrete instance re-derives the laws; a pointwise
  model must not satisfy the weakened interface while breaking the
  multi-object claim).
- **Effect channels & invariants** — the two 8-question pre-flight sets
  (fate of every returned value / "ignored" is never an allowed answer;
  single-seam theorems over the call site, not the callee's error tag;
  advance-only-on-success; smallest fixture; which mutant dies) — these are
  ★4/★5 operationalized at design time.
- **Recon discipline** — API-recon before build (a mechanism named in a spec
  is a CLAIM; verify it exists; recon-finds-the-machinery-already-exists
  reshapes the agenda); design-round-skip criterion (skip only when every
  DOF is determined by already-reviewed pieces; recon that refutes a roadmap
  hypothesis reinstates the round); one design round adjudicates a fork
  (present both candidates grounded in recons).
- **Proof-engineering forward hygiene** — pre-prove the induction-shape
  lemmas (`_cons`, `.mono`, append) the next rung's fold needs; export the
  transport lemma a planned swap will need; named law over defeq `rfl` at
  module boundaries (representation-fragility); ∃-hidden fields made public
  one consumer ahead, preferring the single witness-pinning equation
  (`cfgF = <concrete>`) once multiple fields are load-bearing.

---

## 5 Routed to other skills (do NOT fold into this suite)

| Technique | Destination |
|---|---|
| split-by-hardest-dependency, one-DOF rungs, cross-cutting fix as own increment, add-and-coexist sequencing | `atomic-changes` / `story-change` (formal-design cross-references) |
| dependent-index encoding, one-determinant-per-fact, per-position cell, nominal-carrier-vs-abbrev encoding half | `state-space-minimization` (+ `-formal`) |
| subagent checkpoint-commit recovery, connector-fetch verification, filter-safe puzzle construction mechanics | `subagent` / `claude-coordination` / `codex-coordination` |
| PR-body/head-SHA sync | `pr-create` |
| reproduce-before-graduating meta-process (as skill-maintenance policy) | `skill-writing` / `self-improvement` (find-and-prove keeps the reviewer-facing form in Adjudication #8) |
| CI ratchets as a general pattern | `surface-hardening` (find-and-prove's H3 names the Lean instances) |

## 6 Catalog reconciliation (for the symbiote instance)

**Ownership rule.** The symbiote catalog
(`~/.claude/projects/-Users-dkubb-workspace-dkubb-symbiote/memory/reference_pro_reasoning_principles.md`)
is owned by the symbiote-substrate agent, which appends to it continuously.
An executor of THIS goal never edits that file: Phase 5 produces the
shed/trim list below as a REPORT (committed under
`docs/plans/formal-review-suite-validation/`) for the operator to hand over.
The reverse-drift fix (A∪B wording) is in THIS repo and is ours to make.

- Shed outright (fully graduated after this build): the inc-4b trio,
  annotation vacuity, word-class sweep, A∪B (after the algebra-now fix),
  resource-or-fact, mint-collision, obligation-transfer, bad-lowerer,
  temporal inversion, existential coupling, plus every entry absorbed above.
- Trim to the delta until built: any entry marked PARTIAL in the 2026-07-06
  audit.
- Reverse-drift fix (skill lags catalog): the A∪B bullet's "prove it with the
  comparison fixture, not before" → "prove the pure algebra now; defer only
  the semantic implication discharging its hypotheses."
- Soundness caveats to carry into graduation: canonical-tags CLAMP only when
  the envelope declares off-contract input in scope (else it violates
  fail-closed); =-only proof-term style graduates ONLY together with the
  false-kill rule (they correct each other); physical-execution coupling
  carries Pro's ∃/∀ scope rule.

## 7 Build plan (atomic, per repo discipline)

### Execution protocol (read this first on every goal run)

- **State lives in the repo, not in memory.** Each phase's DONE condition is
  checkable from the working tree. On every run: check the DONE conditions in
  order and resume at the first unmet one. Never trust a summary or memory
  over the tree.
- **Step 0 every run:** `just ci` on clean `main` must be green before any
  change (repair or surface first, per `atomic-changes`).
- **Commit discipline:** the `atomic-changes` canonical form — one technique
  per commit, closed-verb subjects, each commit passes `just validate-skills`
  (fast md gate) and each phase ends with a full `just ci` at head; branch
  `dkubb/change/<slug>` + PR + merge per phase (a phase is a natural PR).
- **No scope drift:** a technique not in this plan's allocation (§3/§4/§5)
  requires a plan-amendment commit to THIS file first, naming its class,
  trigger, and witness — then it may land.
- **Definition of done per item:** FIRE-ON trigger stated + the deciding
  mutant/witness named + lineage row where a published anchor exists + listed
  in both `references/rubric.md` and its defect-class module + soundness
  caveat attached where §6 requires one.
- **Blocked on operator only for:** renames of either skill, deletion of
  existing skill content (restructure moves, never deletes), or edits outside
  this repo. Everything else proceeds.

### Phases

- **Phase 0 — foundation.** DONE when: this file is merged to `main` and
  `just ci` exits 0. Verify: `git log --oneline main --
  docs/plans/formal-review-suite.md` non-empty; `just ci`.
- **Phase 1 — restructure (Move/Refactor only, no new content).** Split
  current SKILL.md + lean-robustness mutation material into the eight
  defect-class reference modules; SKILL.md becomes the dispatch layer with
  the rubric skeleton (one line per item + FIRE-ON + pointer). DONE when:
  all files in the §2 tree exist; a content audit shows zero dropped
  techniques vs pre-restructure `git show` (diff the technique inventory,
  not the bytes); `just ci` green; skill-audit pass.
- **Phase 2 — graduate tier-1.** One Add commit per item: B1, B2, B3, B5b,
  B7, B8, B13, C1, C2, C4(shadow-difference), C5, A2, A4, A7, D2, E1, E2,
  F1, F4(run-not-store), G1, H1(false-kill + both directions + detector +
  differential isolation). Plus the one reverse-drift Fix commit (A∪B
  algebra-now wording). DONE when: every listed item meets the per-item
  definition of done; `grep` for each item's house name hits its class
  module; `just ci` green.
- **Phase 3 — graduate tier-2 + mechanics.** Remaining §3 items, H3
  mechanical floor, H4 skeletons, adjudication additions 5–9, and
  `references/prompts.md` including the four reproduced winning prompts from
  the catalog (sum-type injectivity, mint-collision, existential-subject,
  annotation-constant — copy them verbatim from the catalog BEFORE §6
  reconciliation sheds them). DONE when: every §3 item not in Phase 2 meets
  the definition of done; prompts.md contains all four prompts; `just ci`
  green.
- **Phase 4 — create `formal-design`.** New skill per §4: SKILL.md
  dispatcher + one reference per slice kind; cross-references both ways
  (find-and-prove "Relationship" section names it and vice versa). DONE
  when: `skills/formal-design/SKILL.md` exists, `just validate-skills`
  passes, every §4 bullet is present with its question set, both skills
  reference each other.
- **Phase 5 — validation + handover.** (a) Blind-reproduction runs for six
  tier-1 lenses (protocol: adjudication #8 — filter-safe puzzle, pre-fix
  state or synthetic equivalent, fresh subagent armed only with the lens,
  attempt-1 match); reports committed under
  `docs/plans/formal-review-suite-validation/`. (b) The cold-LLM test
  (acceptance #5). (c) english-comprehension + skill-audit passes on both
  skills. (d) The §6 reconciliation REPORT (shed/trim lists) committed to
  the same directory for the operator to hand to the symbiote instance —
  never applied to the catalog by this goal's executor. DONE when: all four
  artifacts exist in the validation directory and `just ci` is green.

## 8 Acceptance criteria

1. Every technique in the 2026-07-06 audit is either present in a skill
   (with FIRE-ON trigger + witness/mutant + lineage row where an anchor
   exists) or listed in §5/§6 with its destination — zero silently dropped.
2. Every rubric item names the mutant/witness that decides it; no
   judgment-only items.
3. The three soundness caveats (§6) appear next to their techniques.
4. Blind-reproduction passes for the tier-1 lenses (attempt-1, filter-safe,
   pre-fix state).
5. A cold LLM given only SKILL.md + one reference module can run a review of
   a Lean module it has never seen and produce findings in the calibrated
   format (this is the placeholder-no-more test).
