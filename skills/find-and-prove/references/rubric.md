# The binding rubric — full item catalog

Format per item: **FIRE ON** → check → the mutant/witness that decides it.
[Anchors] are the latent-space terms; ★n = origin in the symbiote catalog's
Pro-rubric item n; (NEW) = a new published anchor. Each class has a reference
module with the full treatments (witnesses, mutants, worked examples) where
they exist; the entry here is the binding one-screen form.

| Class | Module |
|---|---|
| A — Vacuity | `references/vacuity.md` |
| B — Unpinned surface | `references/pins.md` |
| C — Overclaim / reception | `references/reception.md` |
| D — Mis-scoped | `references/scope.md` |
| E — Identity & causality | `references/identity.md` |
| F — Resources & composition | `references/resources.md` |
| G — Basis curation | `references/basis.md` |
| H — Evidence mechanics | `references/evidence.md` |
| I — Information flow | `references/information-flow.md` |

## A — Vacuity: does the suite prove anything?

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
- **A8 New-kind activation test** (Pro's own design-rubric addition, W3.6a) —
  FIRE on every theorem advertised as introducing support for a new
  constructor/kind: instantiate at a canonical POSITIVE value of the new
  kind; evaluate every premise and every conjunct on both sides; require a
  positively-inhabited witness (never an equivalence of two false props) and
  name the conclusion a new-kind wiring mutant reddens. Conjunction headlines
  get the activation matrix (component × old/new kind: premise inhabitable?
  each side can be true? wiring mutant reddens?); a new-kind column with "no"
  on either side being true forbids the new-kind-reception description —
  demote to transport corollary. Self-question: "show me one canonical value
  of the new constructor for which the FULL advertised theorem is positively
  inhabited." Design-time-first: run it on the sketch (formal-design
  `references/recon.md`).

## B — Unpinned surface: it holds, but binds nothing

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
- **B4 Read-back & retention at full strength** (read-back half + Pro's
  blind-reproduced retention half) — FIRE on every certificate/wrapper field
  and every label: (a) some theorem must consume the field AT the strength
  the docs claim ("ready for X and Y" needs a consumer per half — a one-line
  corollary is "redundant as a lemma, NOT redundant as a spec guard");
  (b) the converse on every record earned by a theorem: every consumed
  hypothesis is STORED, DERIVABLE from stored fields, or honestly OUTSIDE
  the record's name — a disclaimer covers only the third case, and only
  when the name survives it; decide by inhabiting the record in the state
  the dropped hypothesis excluded.
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

## C — Overclaim: statement weaker than name/prose (the mutation-blind class)

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

- **C9 Deferral honesty** [parametricity] (Fable's `genPreserves`; blind-
  reproduced) — FIRE on every deferred global property of a helper and
  before accepting any mutation survivor as by-design: re-derive every
  headline with the deferred helper replaced by a universally-quantified
  variable, premises-only; compiles ⇒ honest deferral; needs a
  concrete-map fact ⇒ secretly load-bearing — name the premise or split.

## D — Mis-scoped: right claim, wrong boundary or rank

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

## E — Identity & causality

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

## F — Resources & composition

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

## G — Basis curation (headline vs corollary)

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

## H — Evidence mechanics (how to actually decide)

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

## I — Confidentiality & information flow (the oracle-hunt's leak lenses)

The confidentiality half of the hunt: what can the role observe or
distinguish? `references/lean-robustness.md` is its Lean-specific attack
drill.

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
