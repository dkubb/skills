# Published lineage — every technique to its canonical name and source

Each entry is a latent-space activator: the precise term pulls its literature
into a reviewer's context. Use the term, not a paraphrase. Format:
**canonical name** — *source* — the idea.

## The unifying frame

- **PER model of secure information flow** — *Sabelfeld & Sands 2001* —
  info flow = preserving an equivalence.
- **Dependency Core Calculus (DCC)** — *Abadi, Banerjee, Heintze, Riecke,
  POPL'99* — one notion under NI / slicing / binding-time / parametricity.
- **hyperproperties / k-safety** — *Clarkson & Schneider 2010* — security = a
  property of *trace sets*.
- **abstract interpretation / Galois connection** — *Cousot & Cousot 1977* —
  the precision lattice of sound abstractions.
- **safety / liveness** — *Alpern & Schneider 1985* — the two property classes.

## Group 1 — claim / spec truth

- **vacuity / antecedent failure** — *Beer–Ben-David–Eisner–Rodeh;
  Kupferman–Vardi* — passes trivially / weakest theorem.
- **mutation analysis** — *mCoq; "Proof Process Evaluation with Mutation
  Analysis"* — spec/proof too weak (kill the mutant).
- **adequacy of encodings** — *Harper–Honsell–Plotkin (LF)* — model faithful
  to the real object.
- **conservative extension** — *Kunčar–Popescu* — a new def didn't enlarge
  the kernel.
- **proof engineering / TCB** — *Ringer et al., QED at Large* — TCB / axiom
  hygiene.

## Group 2 — representation & API surface

- **contextual equivalence / full abstraction** — *Plotkin 1977; Patrignani
  (secure-compilation survey)* — seal sound in every context.
- **representation independence / parametricity** — *Reynolds; Wadler, Theorems
  for Free! 1989* — two impls indistinguishable by ops.
- **ADT abstraction-barrier leak / public eliminator exposure** — *(Lean
  elaboration; ML module signatures)* — the recursor breaks the seal.
- **object-capability / least authority** — *Miller, Robust Composition 2006* —
  store the capability, not the secret.
- **parse-don't-validate; make illegal states unrepresentable; typestate** —
  *King; Minsky; Rust* — constructive encoding.

## Group 3 — authority & resources

- **separation logic / resource algebra** — *Reynolds; O'Hearn; Iris* — owned
  resources, aliasing, frame rule.
- **confused deputy / ambient authority** — *Hardy 1988* — a privileged op
  tricked by the caller.
- **complete mediation** (+ fail-safe defaults, least privilege) —
  *Saltzer–Schroeder* — check every path, no cached bypass.
- **membranes / attenuation** — *Miller; ocap* — recursive seal + revoke.
- **affine / linear types** — *(Wadler; Rust ownership)* — single-use.
- **best correct approximation / optimal abstract transformer / strongest
  postcondition** — *Cousot; Dijkstra predicate transformers* — sound but not
  tightest.
- **frame conditions / frame problem** — *(classical)* — what does not change.

## Group 4 — information flow

- **non-interference** — *Goguen–Meseguer 1982* — no secret-dependent
  observation.
- **relational Hoare logic / self-composition / product programs** — *Benton;
  Barthe et al.; Sousa–Dillig (Cartesian HL)* — prove 2-safety via a product
  run.
- **unwinding conditions** — *Goguen–Meseguer; Rushby* — local per-step NI
  proof.
- **declassification: dimensions & principles** — *Sabelfeld–Sands 2007* —
  controlled release (what/who/where/when).
- **quantitative information flow / min-entropy** — *Geoffrey Smith* — measure
  leak in bits (guess-in-one-try).
- **termination-/timing-sensitive NI** — *Askarov et al.; Stefan et al.* —
  divergence/timing as outputs.
- **observational determinism** — *Zdancewic–Myers* — scheduler-independent
  low view.
- **robust declassification / endorsement** — *Zdancewic–Myers* — attacker
  can't influence release.
- **intransitive non-interference / ipurge** — *Rushby* — authorized relays,
  no transitive flow.
- **adaptive/chosen-query adversary** (Dolev–Yao *only* for symbolic
  protocols) — *Dolev–Yao 1983* — explicit attacker capabilities.
- **active automaton learning (L\*)** — *Angluin 1987; Vaandrager, Model
  Learning, CACM 2017* — each public tag after a chosen prefix is a
  membership query; the adversary learns the hidden state machine.

## Group 5 — dynamics & composition

- **safety / liveness decomposition** — *Alpern–Schneider* — bad-never vs
  good-eventually + fairness.
- **refinement mapping / simulation / bisimulation** (history/prophecy) —
  *Abadi–Lamport 1991* — prove an implementation/bridge.
- **subset-closure / robust hyperproperty preservation** — *Clarkson–Schneider;
  secure-compilation survey* — refinement preserves only subset-closed
  hyperproperties.
- **assume-guarantee** — *Pnueli tradition* — module assumptions vs guarantees.
- **linearizability** — *Herlihy–Wing* — the commit/visibility point.
- **injective agreement** (aliveness/weak/non-injective/injective) — *Lowe* —
  fresh/unique anti-replay.
- **codec / canonical laws** — *(classical)* — round-trip, idempotence,
  normalization soundness/completeness, canonical uniqueness.

## Group 6 — evidence generation

- **small-scope hypothesis (Alloy); property-based testing (QuickChick)** —
  *Jackson; Hughes/Paraskevopoulou* — small counterexamples first.
- **metamorphic testing** — *T.Y. Chen 1998* — relations without an oracle.
- **differential testing** — *(classical)* — compare two implementations.
- **property taxonomy** (invariant/postcondition/metamorphic/model-based/
  inductive) — *Hughes, How to Specify It! 2019* — which property kinds to
  write.
- **CEGAR** — *Clarke–Grumberg–Jha–Lu–Veith* — "impossible" needs an
  invariant.
