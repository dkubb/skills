---
name: trailblazer
description: >-
  Run development as pipelined lanes in parallel git worktrees: a fast
  trailblazer lane outlines feature shapes at inner-loop speed while
  verification, fixup, reconciliation, and fold lanes apply the full
  quality bar behind it, and an integration coordinator advances a true
  branch that only ever receives folded, per-commit-proven history.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-08-v1"
triggers:
  - "trailblazer"
  - "trailblaze"
  - "lane pipeline"
  - "pipelined development"
  - "fast lane"
  - "parallel worktrees"
  - "slow full gate"
  - "verification lane"
---

# Trailblazer

Decouple exploration speed from verification rigor. One fast lane writes
first-pass approximations of each feature and, within the precedence
order below, is not blocked, slowed, or diverted. All rigor —
per-commit gates, coverage, fixups, history folding — runs behind it in
separate lanes and catches up whenever the fast lane pauses. The
published true branch receives only folded, per-commit-proven atomic
history, so the exploration never compromises the product.

Feedback latency per change collapses from full-gate time to inner-loop
time. (Observed once, on a Rust project with a 5-9 minute full gate:
roughly 10x cadence. Measure locally; the numbers here are one
project's data, not expectations.)

## Precedence

When rules conflict, this order decides, highest first:

1. Safety rules and explicit operator decisions.
2. The repository's own contract (its gates, lint policy, commit
   form).
3. Lane ownership boundaries (worktrees, branches, no-touch lists).
4. Fast-lane throughput.

"Nothing slows the fast lane" is a rule at priority 4: it wins against
convenience, never against safety, the repository, or the operator.

## Why It Works

Two separations do the work. First, latency: the fast lane's feedback
loop is decoupled from every slow proof, so exploration runs at
inner-loop speed. Second — and deeper — context: each lane accumulates
focused working memory for exactly one kind of change (writing shapes,
judging commits, injecting faults, adapting eras), like stations on an
assembly line; each pass over the same set of commits leaves the
artifact strictly better. The coordinator stays high-level — routing
reports, dispatching, deciding — without ever absorbing the context
that conflict resolution or failure forensics require. The lane that
has been staring at one problem class is the lane that solves its
hardest instance; the coordinator that never held that context is the
one that stays effective across the whole project.

## When to Activate

- A project with a slow full quality gate (minutes) and a desire for
  fast iteration against it.
- Greenfield or heavy feature phases where design shape matters more
  than immediate completeness.
- Work that decomposes into commits a per-commit gate can prove, on a
  repository with (or willing to adopt) per-commit gate discipline.
- Multiple agents or workers are available to run lanes concurrently.

This skill's tested profile is Rust/Cargo with LLVM coverage, POSIX
tooling, local worktrees sharing one object database, and an agent
harness where detached child processes cannot wake an idle agent.
Outside that profile, keep the lane model and invariants; re-derive
the concrete commands and the process-supervision rules for your
stack and harness.

## When Not to Use

- Single small changes; anything one gated commit covers.
- Repositories that cannot host parallel worktrees or where the full
  gate is already as fast as the inner loop.
- Work that cannot be decomposed into independently valid commits.

## Inputs

- The repository, its full gate (one command), and its mid-tier gates
  (format, lint, tests — each runnable alone).
- A feature queue or plan for the fast lane to draw from.
- Conventions the fast lane must pre-load so its output lands mostly
  clean (lint-wall patterns, test placement, commit style).
- The repository's atomic-commit contract; this skill does not define
  commit form — delegate decomposition, verb choice, and message rules
  to the `atomic-changes` skill.

## Outputs

- The true branch: folded, per-commit-proven atomic history whose tip
  passes the full gate. A named staging frontier (candidate branch)
  may hold unfolded-but-proven work; the true branch itself never
  does.
- Lane reports in the schema of
  [references/coordination.md](references/coordination.md): ranges by
  OID, per-commit gate evidence, fixups with target OIDs, findings,
  banked branches.
- A measurement log (per-iteration write/lint/test timings, fix
  episodes) for tuning the lane balance locally.

## Utilities

- One mandatory script: [scripts/validate-todo](scripts/validate-todo)
  — the fold-routing witness, run as the sequence editor of every
  fold (see git-mechanics). Everything else is plain git.
- [references/lanes.md](references/lanes.md) — per-lane contracts and
  the canonical loops (fast-lane chain, verification loop, fold
  procedure).
- [references/git-mechanics.md](references/git-mechanics.md) — branch
  topology, the exec gate loop, environment hazards, autosquash and
  fold rules, gate hygiene, coverage contract, bug protocol.
- [references/coordination.md](references/coordination.md) —
  coordinator duties, dispatch and judgment routing, lane failure
  modes with countermeasures, true-branch policy, report schema.
- Related skills: `atomic-changes` (commit form; authoritative),
  `code-review` (verification lane's rulebook where present),
  `worktree-dag` (adjacent coordination patterns).

## The Lanes

One worktree per lane instance, always. Full contracts live in
[references/lanes.md](references/lanes.md).

- **Trailblazer** (fast): outlines feature shapes; happy-path plus
  design-defining boundary tests only; never runs slow gates; commits
  small provisional atoms and rebases onto the true branch after every
  commit.
- **Verification** (mid): audits each commit under mid-tier gates and
  proves corrected states — standalone proof of every commit in the
  final history is the folded pass's claim; corrections become fixup
  commits; judgment calls — including non-atomic commits that need
  restructuring — become findings for the coordinator.
- **Fixup / backfill** (on demand): owns slow-gate debt — coverage,
  mutation survivors, cross-cutting corrections.
- **Reconciliation** (on demand): replays the fast lane onto an
  advanced true branch when they diverge; may amend its replays;
  re-gates per commit.
- **Fold** (dispatched when a fold conflicts): works on its own
  `fold/<id>` branch cut from the candidate OID, resolves fold
  conflicts by era-adaptation, and proves every folded commit; the
  final tree must be byte-identical to the pre-fold tree. Conflict-free
  folds are mechanical and stay with the coordinator.
- **Integration** (the coordinator, usually the top-level session):
  dispatches and routes between lanes, arbitrates findings, and owns
  the true branch. Mechanical git orchestration on refs it owns —
  conflict-free folds on candidate, candidate gates, bisecting,
  advancing the true branch — IS coordinator work, and the
  coordinator owns adjudication: which side wins, what gets applied.
  What it never performs is the content work those decisions call for
  (feature implementation, conflict-resolution edits inside a lane's
  range, a conflicting fold, failure forensics — dispatched so their
  context never enters the coordinator's) or any operation on a
  lane's own branch or worktree, however mechanical — those the
  owning lane executes on instruction.

## Core Invariants

1. Within the precedence order, nothing slows the fast lane. New
   non-feature work gets a new lane, never a diversion.
2. One worktree per lane instance; lanes never share a branch.
3. The true branch only advances to proven states, and it must advance
   promptly: a stalled true branch silently sheds corrections, because
   the fast lane keeps rebuilding from a pre-fix base.
4. Fixups target by commit OID (`git commit --fixup=<oid>`), choosing
   the commit the correction semantically belongs to — a regression's
   introducing commit, era-adapting content authored against a later
   layout. Autosquash routes by subject, not OID, so subjects among
   eligible target commits in the unpublished range stay unique
   (repeated `fixup!` subjects for one shared target are normal) and
   the fold validates its generated todo against the recorded targets
   before executing.
5. The true branch is folded-only, without exception: zero fixup
   commits, every commit individually proven, final tree identical to
   the proven tree. Unfolded-but-proven work waits on the staging
   frontier, tracked as an explicit fold obligation.
6. Regressions in unpublished history are fixed at the introducing
   commit via `git rebase --exec <gate> --reschedule-failed-exec` —
   stop, fix or amend there, continue — never absorbed by a tip
   commit. A defect whose introducer is already published gets a
   forward corrective commit carrying its regression test; the true
   branch is never rewritten.
7. Escapes and interruptions are banked, not discarded: an interrupted
   or countermanded lane commits what it has, with notes, to a branch
   the next lane starts from.

## Process

1. Set up: true branch, the branch topology of
   [references/git-mechanics.md](references/git-mechanics.md), one
   worktree + branch per initial lane (trailblazer, verification),
   conventions brief for the fast lane.
2. Run the fast lane continuously against the feature queue; run
   verification passes behind it as commits accumulate.
3. Spawn fixup, reconciliation, or fold lanes when their kind of work
   appears; retire them when it is done.
4. Integration cycles: collect lane reports, fold audited fixups,
   prove candidates, advance the true branch, re-dispatch the fast
   lane.
5. On any bug: reproduction script first, bisect with it if the
   culprit is not obvious (cheap — every commit is gate-green). An
   unpublished introducer gets the fix folded in with the repro
   promoted to a test; an introducer already on the true branch gets
   a forward corrective commit carrying the repro instead.
6. Finish with a fold pass so the published branch carries only clean
   atomic commits, each proven.

## Validation Checklist

- The fast lane never ran a slow gate, wrote a coverage fixup, or
  waited on another lane — and never overrode safety, repository, or
  operator rules to stay fast.
- Every lane had its own worktree; no branch was shared.
- The true branch advanced at least once per verification cycle, or
  the stall was treated as the pipeline's top problem.
- Every fixup targeted its recorded commit by OID; the fold's
  generated todo matched the recorded targets; conflict-free folds
  completed without hand resolution; every conflicting fold was
  dispatched to a fold lane and its result proven.
- Every regression fix in unpublished history landed at its
  introducing commit under a rescheduled exec gate, and the gate
  re-ran green there; published defects landed forward corrective
  commits with their regression tests.
- The final fold left zero fixup commits, a byte-identical tree, and a
  green per-commit proof pass; the true branch never held unfolded
  history.
- Published commits satisfy the repository's atomic-commit contract
  (`atomic-changes`), not just its gates.
- Interrupted work was banked with notes and consumed by a successor
  lane, not re-derived.
