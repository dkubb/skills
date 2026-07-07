# Cold-LLM test — acceptance criterion 5 (Phase 5b)

Protocol: a fresh subagent with NO prior context was given exactly two
files — `skills/find-and-prove/SKILL.md` and
`skills/find-and-prove/references/vacuity.md` — plus a small Lean module
it had never seen (`QueueMachine`: a job queue with three planted defects
and one sound theorem), and asked to review it using the skill's method.

## Planted vs found

| Planted defect | Found? | Evidence produced |
|---|---|---|
| `qstep` silently drops the dequeued job; the log is never written | YES (Finding 1, critical) | kernel-checked witness `(run 1 ⟨[7],[]⟩).log = []`, plus a compiled proof of the NEGATION of the intended headline |
| `Completed s := ∀ j ∈ s.log, j ∈ s.log` is a tautology | YES (Finding 2, critical) | `∀ s, Completed s` compiled with no hypotheses; trivial-model sweep (A1) with a log-erasing step and the no-op step both satisfying the suite |
| `eventually_completed` uses the `∃ fuel` opt-out (A5) | YES (Finding 3, high), correctly kept DISTINCT from Finding 2 | `∃ fuel, Completed (f fuel s)` compiled for an ARBITRARY `f`; demanded the `∀ fuel ≥ bound(input)` form with `s.queue.length` as the bound |
| (control) `queue_shrinks` is sound and binding | KEPT | the stalling mutant compiled and shown killed |

## Format conformance (the calibrated format)

The report ran the mechanical floor FIRST (axiom check, TCB greps, the
compiler's own unused-variable tells), built a ranked oracle/target table,
gave every finding all five calibration parts (claim / role / reachable
observation / minimal compiled witness / impact), classified each by
enforcement rank including a rank-mismatch call ("worse than honest-doc"),
produced the per-headline bad-mutant table with the *if no mutant fails,
it's vacuous* rule applied, named what to KEEP (adjudication rule 7),
separated three nitpicks that fail the five-part test (including an
E1-class multiplicity note), deduped findings 1 and 2 by
(target, hidden predicate, witness shape), and stated the stop condition.

Every witness compiles under Lean 4.30.0 (`propext` only; no `sorry`, no
`native_decide`); the witness file is committed alongside this report as
`cold-llm-witnesses.lean`.

## Verdict

PASS — a cold LLM given only SKILL.md + one reference module produced a
correct, witness-backed review of an unseen Lean module in the calibrated
format, finding all planted defects and over-flagging nothing (the sound
theorem was kept). This is the placeholder-no-more test of acceptance
criterion 5.
