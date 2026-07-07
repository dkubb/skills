# Catalog reconciliation report (for the symbiote instance)

Produced by the formal-review-suite build (Phases 1–4, merged 2026-07-07:
PRs #6–#9). Per the plan's section 6 ownership rule, this is a REPORT for
the operator to hand to the symbiote-substrate agent — the build never
edited the catalog
(`~/.claude/projects/-Users-dkubb-workspace-dkubb-symbiote/memory/reference_pro_reasoning_principles.md`).

## Shed outright (fully graduated into the suite by this build)

Each entry is now covered by the named skill location; the catalog may drop
its copy.

| Catalog entry | Graduated to |
|---|---|
| Pro rubric #1 (declared-universe irredundancy) | find-and-prove `references/basis.md` G1 |
| Pro rubric #2 (scope-to-boundary) | `references/scope.md` D2 |
| Pro rubric #3 (forward pressure) | `references/scope.md` D5 |
| Pro rubric #4/#5 (fate of returned values; single seam) | formal-design `references/effect-channels.md` |
| Pro rubric #6 (non-vacuity reachability witness) | `references/basis.md` G3 |
| Pro rubric #7 (public vs internal surface) | `references/basis.md` G3 |
| Pro rubric #8 (name vs statement) | `references/reception.md` C8 |
| Pro rubric #9 (two-snapshot drift) | `references/reception.md` C7 |
| Pro rubric #10 (new-predicate negative witness / conclusion position) | `references/vacuity.md` A2 |
| Pro rubric #11 (multiplicity-parametric positives) | `references/pins.md` B10 |
| Pro rubric #12 (relation `_iff`) | `references/pins.md` B1 |
| Pro rubric #13 (assembly discipline) | `references/scope.md` D8 |
| Pro rubric #14 (wrapper-Prop `_iff`, structure form) | `references/pins.md` B1 |
| Pro rubric #15 (producer pin + detector isolation) | `references/pins.md` B2; `references/evidence.md` H1 |
| Pro rubric #16 (object-referent audit + 3 siblings) | `references/reception.md` C1 |
| Pro rubric #17 (instance-honesty defeq lock) | `references/pins.md` B1 |
| The inc-4b trio (mint-collision, resource-or-fact, who-owns-construction) | `references/scope.md` D3 / `references/resources.md` / formal-design `references/carrier-representation.md` |
| Annotation vacuity (all-constant sweep) | `references/vacuity.md` A3 |
| Word-class doc sweep | `references/reception.md` C7 |
| A∪B composition trap (with the algebra-now fix) | `references/resources.md` F4 |
| Resource-or-fact | `references/resources.md` |
| Mint-collision | `references/scope.md` D3 (persistent-fact re-entry) + prompts |
| Obligation transfer | `references/reception.md` |
| Bad-lowerer test | `references/scope.md` D1 |
| Temporal inversion | `references/reception.md` C4 |
| Existential coupling / existential subject | `references/pins.md` B5/B5b |
| Predicate-collapse lattice (D0 descent) | `references/vacuity.md` A2 |
| Behavior-drop / step_exact_cases limitation | `references/vacuity.md` A7 |
| Frame-pollution audit | `references/reception.md` C4 |
| Shadow-difference elimination | `references/reception.md` C4 |
| Two-step phase closure + persistent-fact re-entry | `references/scope.md` D3 |
| False-kill rule, both-directions operators, differential isolation | `references/evidence.md` H1 |
| The four reproduced winning prompts | `references/prompts.md` |
| W3.5 forced-primitive moves | formal-design `references/new-primitive.md` |
| Route-vs-encode; carrier-swap 15-question set | formal-design `references/carrier-representation.md` |
| Seam-extraction 12 questions + don't-over-generalize | formal-design `references/seam-extraction.md` |
| The two 8-question pre-flight sets (N5B) | formal-design `references/effect-channels.md` |
| Recon discipline, design-round-skip, retirement-is-a-mirage | formal-design `references/recon.md`, `references/increment-scoping.md` |
| Split-by-hardest-dependency, defer-the-transform, extension-point-now, first-consumer, false-dependency | formal-design `references/increment-scoping.md` |
| Proof forward hygiene (induction-shape lemmas, transport export, named-law-over-rfl, witness-pinning cfgF) | formal-design `references/proof-hygiene.md` |

## Trim to the delta

Entries marked PARTIAL in the 2026-07-06 audit are now graduated in full;
the catalog copy can shrink to whatever the symbiote agent has appended
since 2026-07-06 (the delta this build could not see).

## Soundness caveats carried into graduation (verify they rode along)

1. Canonical-tags CLAMP only when the envelope declares off-contract input
   in scope — attached in formal-design `references/carrier-representation.md`.
2. =-only proof-term style graduates only together with the false-kill rule
   — attached in find-and-prove `references/evidence.md` H1.
3. Physical-execution coupling carries the ∃/∀ scope rule (couple fuel only
   when the headline exhibits a run) — attached in find-and-prove
   `references/resources.md` F4.

## Reverse-drift fix applied in the skills repo

The A∪B bullet's "prove it with the comparison fixture, not before" became
"prove the pure algebra now; defer only the semantic implication
discharging its hypotheses" (`references/resources.md` F4, PR #7). The
catalog's own copy already had the correct wording; no catalog change
needed.
