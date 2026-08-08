# Coordination

The coordinator (integration) is the only role with cross-lane sight.
Its job is routing, judgment, git orchestration, and the true branch.
Git orchestration on coordinator-owned refs and worktrees —
conflict-free folds on `candidate`, running candidate gates,
bisecting, advancing `candidate`, `<true>`, and retired banks — is
coordinator work. The coordinator owns
decisions: which side of a conflict wins, what gets applied, when the
true branch moves. What it never performs is the content work those
decisions call for: feature implementation, conflict-resolution edits
inside a lane's range (a conflicting fold included), and failure
forensics are dispatched to specialist lanes so their context never
enters the coordinator's. The dividing line has two axes: WHAT the
operation is, and WHOSE refs it touches. Mechanical git on refs and
worktrees the coordinator owns (`candidate`, `<true>`, retired banks)
is coordinator work; the moment an operation requires writing or
choosing code, a specialist lane owns it; and any operation on a
lane's branch or worktree — however mechanical — belongs to that
lane, on coordinator instruction (a reconciliation landing is
mechanical git AND owner-only). These are the
operating rules and the failure modes observed in practice, each with
its countermeasure.

## Dispatch

- Keep the fast lane loaded: dispatch the next feature batch the
  moment the previous one lands. An idle trailblazer is the only idle
  that costs anything; idle slow lanes just mean the pipeline drained.
- Every dispatch carries: the contract (unchanged parts by reference),
  the task list, files or areas the lane must NOT touch (anything
  another lane holds), and the reporting format.
- Deliver conventions, corrections, and mid-task redirects as
  messages; lanes fold them in at their next tool round.
- When operator input changes a decision, spawn a change-applier lane
  to fix it at the earliest relevant commit and re-prove downstream
  with the exec loop — do not divert the fast lane.

## Judgment routing

- Convergence raises priority, not authority: a finding reported by
  independent lanes goes to the front of the adjudication queue, and
  the coordinator accepts it — dispatching the change to the owning
  lane — only when it contradicts no settled decision, operator
  instruction, or repository rule.
- Findings contradicting a settled decision: surface to the operator
  with the tension named; never silently apply or discard.
- Lane deviations from instructions arrive with justifications; the
  justification is evidence to read before overriding, not authority
  in itself.
- Escalated conflicts (a lane aborted a judgment rebase): the
  coordinator adjudicates which side wins — that is a decision — then
  spawns a reconciliation lane with the author's notes to perform the
  edits. The coordinator never performs the in-range edits itself.

## Lane failure modes and countermeasures

- **Passive-wait stall**: a lane spawns long work and stops "waiting
  to be woken" — in the tested harness, detached child processes
  cannot wake an idle lane (verify what your harness does before
  relying on anything else). Countermeasure: lanes drive
  their own children foreground with bounded timeouts; the coordinator
  verifies any claimed background work with process checks before
  trusting a "waiting" report, and installs its own harness-tracked
  watcher when a long operation genuinely must run detached.
- **Watchdog kill mid-gate**: an interrupted exec is consumed but
  unproven (see git-mechanics); re-run the gate manually before
  continuing.
- **Worktree collision**: two lanes pointed at one branch corrupt each
  other's assumptions. Countermeasure is prevention (one worktree per
  lane instance); on detection, the writing lane banks and stops, the
  reading lane reports without mutating.
- **Dropped corrections**: fixups not folded before the fast lane
  rewrites the same code vanish, and a `git reset --hard` starting a
  fresh verification pass deletes any prior-pass fixups still
  unconsumed. Countermeasure: prompt true-branch advances; never reset
  `verify` until its fixups are consumed or banked to an immutable
  ref; verification re-checks load-bearing invariants per commit (the
  safety-stop pattern) so a dropped correction is caught, not assumed.
- **Banked-work amnesia**: killed or countermanded lanes must commit
  what exists to a branch with notes; successor lanes start from the
  bank. Re-derivation is the failure.
- **Infrastructure interruptions** (API drops, crashes): resume the
  lane from its transcript with a state-assessment-first instruction —
  reconcile the worktree against what was actually committed before
  continuing the plan.

## True-branch policy

- Advance on proof, promptly, and partially when needed: if the tip is
  red, bisect to the last green boundary and advance there — the
  frontier moves even when the newest work is not ready.
- Unfolded-but-proven work never reaches the true branch. It waits on
  the staging frontier (`candidate` in the topology), with the fold
  and the first-commit full gate re-run tracked as a paired
  obligation; the true branch moves only to the folded, proven
  result.
- The true branch never holds unproven history; that discipline is
  what makes every other stumble in the pipeline cost-free.

## Measurement

Collect per-iteration timings (write, lint, test) and fix episodes
from the fast lane, verification lag and fixup counts from the mid
lane, and red-candidate causes from integration. These decide the
balance questions locally: whether lint belongs in the fast loop (in
the one measured Rust project: yes — about a second per iteration and
rarely firing, with hand-fix episodes on wrong-suggestion lints the
only real cost), where gate timeouts sit per era, and whether the
fast lane's convention priming is working (fixup volume is the
metric). Measure your own project; these numbers are observations,
not constants.

## Lane report schema

Every lane report carries, at minimum:

- lane name, worktree path, branch, and the range as OIDs
  (base..head);
- status: complete, partial (with what remains), blocked (on what),
  or banked (branch name and notes location);
- per-commit gate results for the range, naming the exact gate
  command run;
- fixups created: OID, target OID, one-line reason each;
- findings for adjudication, each with evidence;
- the tree OID at the report's head (for identity checks downstream);
- the next action the lane believes is authorized, so the coordinator
  confirms rather than reconstructs.
