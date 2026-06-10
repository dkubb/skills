---
name: atomic-changes
description: >-
  Break work into the smallest atomic steps. Verify the foundation first
  (step 0), then order steps so partial progress always leaves the system
  valid or better. Owns the canonical commit form: conventional-commit
  structure, the closed action-verb set, and transformation-priority
  ordering.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-05-v1"
triggers:
  - "atomic changes"
  - "atomic steps"
  - "smallest step"
  - "break down the problem"
  - "break it into steps"
  - "decompose the task"
  - "one change at a time"
  - "step 0"
  - "foundation check"
  - "verify the foundation"
  - "are we green"
  - "known good state"
  - "partial progress"
  - "leave it better"
  - "monotonic progress"
  - "safe ordering"
  - "order the steps"
  - "reversible steps"
  - "incremental change"
  - "small preimage of failure"
  - "conventional commit"
  - "commit structure"
  - "commit message"
  - "atomic commit"
  - "atomic commits"
  - "fixup commit"
  - "transformation priority"
  - "commit ordering"
  - "action lines"
---

# Atomic Changes

Make the smallest change that keeps the system whole. Split as small as
possible, but no smaller than stays functional: every step must pass all
gates and be merge-ready and deployable on its own. That functional floor —
not line count — bounds how small a step can be. Start from a verified
foundation, and order steps so that stopping or failing at any point leaves
the system valid or better than the start.

This is the same discipline as programming itself: move the system from one
valid state to the next through a pipeline of transformations, one at a
time, each as simple as possible. A simple transformation can be measured
and verified on its own, so a flaw surfaces inside that one small step —
easy to see, easy to debug. Solid small transformations compose into a
reliable pipeline; large ones hide where they went wrong.

## When to Activate

- The task needs more than one change to reach the goal.
- A change is risky, uncertain, or may not fully succeed.
- You are about to modify a system you have not confirmed is working now.
- A plan mixes several changes into one experiment.
- You must sequence steps so a partial failure is recoverable.
- Independent parts of the work could fan out and run in parallel.
- A long serial task would be easier to time and debug if split into
  segments.

## When Not to Use

- A single, trivial, fully reversible change.
- A pure information request with no change.

## Inputs

- The goal state.
- The full gate set that defines merge-ready and deployable (build, lint,
  test, format, coverage, run) and the single command that runs it. This is
  the verify command referenced below.
- The candidate changes and the dependencies between them.

## Outputs

Emit in this order:

1. Step 0 result: the foundation command and whether it passed or was
   repaired.
2. A dependency DAG of atomic steps: nodes are steps; edges run from each
   dependency to its dependent. Mark independent branches for parallel
   fan-out and chains as serial.
3. The DAG recorded in the harness task or plan tracker: one entry per
   step, dependency edges where supported or a topologically sorted plan
   otherwise; statuses updated as steps verify.
4. The valid/better state each completed step leaves behind.
5. The revert point for each step.

## Utilities

- No mandatory scripts. This is guidance only.
- Use whatever task or plan tracker the harness provides to record the DAG
  and drive execution. If it supports dependency edges (for example
  blockedBy/blocks), encode the DAG directly. If it only offers a flat,
  ordered plan of steps with statuses, topologically sort the DAG into the
  plan order and name each step's blockers in its text. Either way the
  tracker is the single source of execution order, updated as each step is
  verified.
- `references/commits.md` — the canonical commit structure: the closed type
  and verb sets, transformation-priority ordering, and the subject, body, and
  action-line rules used to classify, order, and write each step.
- Related skills in this repo:
  - `state-space-minimization` (`references/normalization.md`) — decompose
    into atoms, then recompose along use.
  - `code-review` — verify each step against the review rules.

## Process

1. **Step 0 — verify the foundation.** Before any change, run the verify
   command on the current state. Confirm it passes. If it is unexpectedly
   broken, stop and repair or surface that first. Never build on an
   unverified base. Cached success is not a verified foundation; force the
   real path when in doubt.
2. Build the dependency DAG. If you do not yet know the right steps,
   explore first: run a cheap, reversible experiment (a throwaway WIP or
   spike), adding whatever it takes until the gates pass — the green state
   tells you the necessary set is complete. The integrity floor applies
   only to what you keep, not to this scratch work. Then read the order
   back out of the working whole — what must come first, then second — and
   keep pulling pieces out and reordering until every step is atomic,
   gated, and the set is ready to deploy. The DAG is often recovered from a
   working result, not planned up front. Independent branches fan out and
   run in parallel; a chain runs in series; a topological sort gives a
   valid serial order. Do not ship the spike; extract from it.
3. Split as small as possible, bounded by functional integrity. Each step
   changes one thing AND leaves the system whole — it passes the full gate
   set and is merge-ready and deployable on its own. That floor is what
   stops a split: a cut that would leave the tree red, a feature
   half-wired, or any gate failing is too small — fold it into the change
   that makes it whole. Within that floor, smaller is better: each segment
   gets its own timing and verify, so a failure isolates to one segment
   instead of one large step.
4. Split by transformation type. One step does one kind of change, drawn
   from the closed verb set (see `references/commits.md`). A step that spans
   two verbs is doing too much — split it.
5. Order by these criteria, applied in rank order (each one breaks ties in
   the criterion above it):
   1. **Dependencies before dependents** — whatever must exist for the next
      step to work goes first (the DAG's topological order).
   2. **Highest learning opportunity first** — do what teaches the most
      early, so every later decision is better informed. When the
      highest-learning step is also high-risk, take the learning through a
      reversible spike (step 2) and keep the *shipped* sequence on the risk
      order below.
   3. **Lowest risk first** — clear low-risk changes early so they deploy
      and earn feedback while you focus on the riskier ones. This is why
      transformations run Remove → Fix → Move → Rename → Refactor →
      Change → Add → Upgrade → Downgrade (dependency moves isolated in
      their own steps).
   4. **Related items together** — group steps into a narrative that flows
      one into the next; do not jump between unrelated items.
   5. **Tie-break** — make each item a distinct unit and sort
      alphabetically, so the order is deterministic.

   Criterion 3 reduces *system* risk, not task risk. For one task in
   isolation, doing the hard part first can be fine; but a system has to
   ship and run, and a high-risk change laid down first spikes complexity
   and leaves the system fragile — failures get likelier and harder to fix,
   with all the trailing low-risk work piled on a shaky base. So reduce
   complexity first (Remove, Fix, Refactor), then add it (Change, Add):
   ship the preparation and get real-world feedback early, and surface any
   suboptimal prep before the high-risk change is built on it.

   Mechanically this is one stable, lexicographic sort: criterion 1 is the
   only hard constraint and sets what can run in parallel; criteria 2-5 are
   sort keys applied left-to-right to produce one deterministic order. That
   order does two jobs — it decides which ready items to take first when
   capacity is limited, and it is the order finished work is *presented* to
   the outside world. Items independent under criterion 1 are embarrassingly
   parallel: run them all at once when resources allow, do not hold one
   back; even when they finish out of order, present them in this canonical
   order. Git commits are the prime case — capture them as they happen, then
   reorder them to match. Regenerate positional numbering to fit the new
   order; preserve a number only when it is an identifier referenced from
   outside the artifact.
6. Record the DAG in the harness task or plan tracker, one entry per atomic
   step. If the tracker supports dependency edges, encode each dependency
   directly so a step becomes runnable only after its dependencies
   complete. If it only offers a flat plan, topologically sort the DAG into
   the plan order and name each step's blockers in its text. The tracker is
   the single source of execution order.
7. Execute as a loop, re-evaluating the DAG each iteration from the point of
   view of the current largest bottleneck — the runnable step the most work
   depends on. Take that step, mark it in progress, apply the one change,
   run its verify, and mark it complete only when verify passes. The
   bottleneck moves: after a green WIP it may become "extract X from the
   WIP"; do that, then re-evaluate and loop. Independent runnable steps may
   run in parallel where the harness allows.
8. When a step is blocked on an operator decision, treat the open questions
   as their own DAG. Ask the largest blocker first — the bottleneck the most
   other questions and steps depend on — one at a time, never batched. Order
   the rest by theme so related questions stay adjacent (story form) and the
   operator holds one context. Run any already-unblocked steps meanwhile.
9. If a step fails, the cause is that step. Leave its entry in progress with
   a note and revert that step alone; earlier steps and their completed
   entries stay. New sub-steps become new entries with their own
   dependencies.
10. Record what each step verified. Commit atomically: each commit passes
    the full gate set and is merge-ready and deployable on its own. Do not
    amend unless asked: an amend folds a change into history before it can
    be audited, can quietly make one commit carry two transformations, and
    can land on the wrong commit when an earlier one was the right target.
    Put the correction in a `fixup!` commit aimed at the right commit so it
    stays visible for audit; autosquash folds it in only on request.
    Gate-trailer amends on unshared commits are metadata-only and exempt.

## Validation Checklist

- Step 0 ran and the foundation passed before any change.
- The dependency graph is acyclic; dependencies and dependents are named.
- The DAG is recorded in the harness tracker (dependency edges where
  supported, else a topologically sorted plan), and execution follows that
  order; statuses are kept current.
- Independent branches are marked for parallel fan-out.
- Each step changes one thing and has its own verify command.
- Every step passes the full gate set and is merge-ready and deployable on
  its own; functional integrity, not line count, bounds atom size.
- When the right steps were unknown, exploration used a reversible spike;
  only the atomic steps extracted from it were kept and gated.
- The order guarantees partial progress leaves the system valid or better.
- Ordering applied the ranked criteria in order: dependencies, then learning
  opportunity, then risk, then related-grouping, then alphabetical tie-break.
- Independent items ran in parallel; artifacts were presented in the
  canonical criteria order regardless of finish order.
- The largest bottleneck was re-chosen each iteration; the constraint was
  allowed to move (e.g. to an "extract from WIP" step).
- Each step was verified before the next began.
- A failed step reverts without losing earlier steps.
- Work that another skill owns was delegated, not duplicated.
