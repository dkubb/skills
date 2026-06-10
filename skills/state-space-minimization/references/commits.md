# Commits as state-space transitions

A commit is a function from a parent repository state to a child
repository state. Same vocabulary from `principles.md` (domain,
codomain, range, preimage), same six operations, same audit
questions — applied at the level of repository state transitions
rather than at types or runtime behavior. The commit message is
the type signature; the diff is the function body.

The reader of a commit infers a *codomain* from the diff — the set of
plausible intents the diff could have realized. The message's job is
to narrow that codomain to the *range* — the specific transformation
the author actually intended. Codomain ≈ range is the goal.

The canonical commit form — the type and verb sets, transformation
priority, subject / body / action-line rules, size bounds, and
anti-patterns — is `atomic-changes` `references/commits.md`. This
module is determinant for the state-space reading of that form and
for the gate-trailer machinery built on it; where the two state the
same fact, the `atomic-changes` file is the determinant.

## Vocabulary applied to commits

| Term | Meaning in the commit context |
|---|---|
| Domain | the parent state(s) a commit applies cleanly to |
| Codomain | the set of states the diff could be inferred to produce, before reading the message |
| Range | the state the diff actually produces, against a specific parent |
| Preimage of failure | the set of changes that could be the cause when a regression appears |
| Type signature | the commit message |
| Function body | the diff |

## Closed sets, read as narrowings

The canonical sets — eleven commit types and nine action verbs —
are in `atomic-changes` `references/commits.md`. Each is
exhaustive: every real transformation maps to exactly one type and
one action verb. The type is the outermost narrowing of the
codomain; picking the wrong type widens the codomain to
"unspecified transformation." The verbs partition the
transformation space into disjoint regions; a commit whose action
cannot be expressed with one verb is trying to do more than one
transformation — split it.

## Transformation priority

The canonical shipping order is in `atomic-changes`
`references/commits.md` § "Ordering (transformation priority)";
this section derives it. The ordering is a state-space gradient:
each tier reduces or preserves the state space before the next tier
expands it. Within a tier, order by disruption — mechanical,
compiler-checked changes before changes whose behavior preservation
must be reasoned about.

1. **Remove** — directly reduces the state space of the codebase.
   Deleted code cannot have bugs.
2. **Fix** — eliminates invalid states. A bug is a system sitting in
   a representable-but-invalid state; the fix removes that state from
   the reachable set.
3. **Move** — preserves the valid state space; only the location
   changes. Mechanical and compiler-checked, and the identifier
   stays fixed, so every reference reads the same after the move —
   the least disruptive of the preserving changes.
4. **Rename** — preserves the valid state space; only the label
   changes. Equally mechanical and compiler-checked, but it shifts
   the codebase's vocabulary at every use site, so it is the more
   disruptive of the two mechanical moves.
5. **Refactor** — preserves the valid state space, reshapes the
   structure. Informational equivalence on inputs and outputs; only
   the implementation changes. More disruptive than move or rename
   because behavior preservation must be reasoned about rather than
   compiler-checked. The proof of preservation is the untouched
   half: a code refactor is verified against unchanged tests, a test
   refactor against unchanged code. Editing both in one commit adds
   a second construction path to "the gates pass" and voids the
   proof (`proof-preservation.md`) — such a diff is a Change. Sets
   up the next tier safely.
6. **Change** — modifies the valid state space. Observable behavior
   shifts. Higher risk than refactor because callers may break.
7. **Add** — expands the state space. Highest risk for new bugs
   because new code is the largest preimage source for the next
   regression.
8. **Upgrade** — a dependency-level state-space change in the
   forward direction. Isolate in its own commit (often its own PR)
   so the diff is purely the bump plus any required adapter changes.
9. **Downgrade** — the corrective direction, usually evidence-driven.
   Same isolation rule as upgrade.

To see where a candidate new verb would sit, ask: does it reduce,
preserve, or expand the state space? The verb set itself stays
closed — extending it is a deliberate change to the canonical
table, not an ad hoc choice in one commit.

## The message as type signature

The concrete rules — subject format and length, imperative voice,
body wrap, action-line form — are canonical in `atomic-changes`
`references/commits.md`. Their state-space reading:

- The subject's `type` is the outermost narrowing; the verb in the
  description is the innermost. Together they pin the
  transformation precisely enough that a reader who never opens
  the diff knows what kind of state transition happened.
- "and" / "or" in a subject is a Cartesian product of two
  transformations — the codomain just doubled. Split.
- Past tense and gerund forms admit ambiguity about whether the
  work is done, planned, or hypothetical; the imperative excludes
  those readings.
- A subject whose verb does not match the diff's actual effect is
  a divergent type signature (see `principles.md` § "Types as
  hypotheses"); bisect and review signals degrade across the whole
  branch when messages cannot be trusted.
- The body asserts the *range* the author was targeting so the
  reader can confirm the diff lands on it. Action lines keep each
  observation independently parseable; explicit `What:` / `Why:` /
  `How:` labels are redundant with the role each action-line
  clause already carries.

Example (action lines narrowing a two-observation body):

```text
fix: handle null user references during registration

- Fix null pointer when downstream lookup returns no row.
  ```rust
  let user = lookup(id).ok_or(LookupError::NotFound)?;
  ```
- Remove the legacy fallback path that masked the null with a
  default user record.
```

## Atomicity

An atomic commit is a single bounded state transition: one
transformation, one observable behavior, one applicable parent
state, and a child state that passes every gate. The whole branch
is then a composition of atomic transitions.

A monolithic commit is the opposite: many entangled transformations
in one diff. The preimage of failure when CI breaks on a monolithic
commit is the entire diff — bisect collapses to "this large commit
caused it" with no further narrowing. Atomic commits give a small
preimage of failure (this specific transformation), which is *why*
`git bisect` works.

Atomicity rules:

- One behavior change per commit. Two behaviors per commit is a
  Cartesian product; preimage of failure is the union.
- Each commit independently passes the full gate. Intermediate
  invalid states (broken tests, broken lint) in the history are
  representable-but-invalid — eliminate them at write time, not by
  rewriting after the fact.
- Diff size is a hard upper bound on preimage cardinality. The
  canonical size bounds are in `atomic-changes`
  `references/commits.md`; they include both source and test
  changes. If source and tests for one behavior do not fit the
  bounds, the behavior probably isn't atomic.

## Anti-patterns, read as state-space failures

The canonical anti-pattern list is in `atomic-changes`
`references/commits.md`. What each one is in state-space terms:

- **Vague subjects** ("misc", "cleanup") — inhabited but useless
  types; they describe no specific transformation.
- **WIP commits on shipped branches** — invalid intermediate
  states in the public history.
- **`--no-verify`** — a smart-constructor backdoor: lets an
  invalid commit-state be constructed past the trusted boundary.
- **Content amends** — replace a state transition in place,
  discarding the audit trail of the previous state; a `fixup!`
  commit represents the correction as its own visible transition
  until autosquash folds it in.
- **`--amend` on shared commits** — rewrites a state other callers
  may already depend on.
- **Mixing refactor with behavior change** — the refactor's
  information-preservation guarantee no longer holds, and the
  preimage of any post-merge regression is the union of both
  transformations.
- **Refactor editing code and tests together** — neither half is a
  fixed frame of reference, so the behavior-preservation obligation
  is undischarged; the gates passing no longer proves anything about
  the old behavior.

Compound and past-tense subjects are covered in § "The message as
type signature" above.

## Trailers as typed proof of validity

A commit's claim "this passes the gates" is unverified by default.
Without proof, every commit in the history is in an unconstrained
validity state.

Encode gate results as commit-message trailers, one per named gate:

```text
fix: handle null user references during registration

- Fix null pointer when downstream lookup returns no row.

Gate-fmt: pass
Gate-lint: pass
Gate-typecheck: pass
Gate-test: pass
Gate-mutation: pass
```

Each trailer is a typed proof that a specific named gate has been
run and produced a specific result. The set of trailers on a commit
narrows the validity state space from "unknown" to "exactly these
gates, with exactly these outcomes." A commit with all expected
trailers in `pass` state is a fully-validated state transition; a
commit missing a trailer is partially validated; a commit with any
`fail` trailer is not a valid state transition and the work is not
yet done.

Trailer rules:

- One trailer per named gate. The name is the gate identifier; the
  value is `pass` or `fail`.
- Trailers go in the footer block (one blank line after the body),
  one per line, in `Token: value` form.
- The set of expected gate names is project-defined and stable
  across a branch.
- A commit must be created with at least one gate-pass trailer. The
  initial trailer represents the fastest gate that establishes the
  commit is worth running the remaining gates against (typically
  fmt or lint).
- Each subsequent gate run amends the commit, adding its trailer.
- On gate failure, the failing trailer is recorded and no further
  gates are attempted on that commit. The work returns to the
  author.

The downstream invariant: any consumer of a commit (CI, review,
merge automation) can read its trailers and determine its validity
state without re-running gates. Re-runs are decided by *missing*
trailers, not by re-executing already-proven gates.

## Incremental gate execution

The trailer model enables incremental execution of expensive gates
without blocking the author.

Workflow:

1. Author stages hunks.
2. Commit handler runs the fastest gate (fmt). On pass, creates
   the commit with `Gate-fmt: pass` as the initial trailer. On
   fail, no commit is created and the failure is surfaced.
3. The commit's tree (from `git write-tree`) is mounted on a
   throwaway worktree. Build caches (`./target` for Rust, `node_modules`
   for Node, etc.) are copied from the main worktree.
4. Each remaining gate runs in the worktree in the project-defined
   order. On each pass, the commit is amended to add the
   corresponding trailer. On failure, the failure is recorded as a
   trailer and the remaining gates are skipped.
5. While the worktree runs gates, the author continues editing the
   main worktree. Successful gate passes accrue on the prior commit
   in the background.

The advantage: agents and humans both keep working "as if" the
previous commit is going to pass, with recovery via stash and
rebase only when a gate failure forces it. Wall-clock time for the
total branch is bounded by the slowest gate of the slowest commit,
not by the sum across the whole branch.

A gate runner that re-runs after an interruption can read each
commit's trailers and skip every gate that already shows `pass`.
The state space of "work to do" is bounded by the set of missing or
failed trailers, not by the size of the branch.

When a gate fails, the recovery sequence is:

1. The author/agent fixes the issue against the main worktree.
2. The fix is either added to the failing commit (via `--fixup`
   targeting that commit, applied later with autosquash) or, if
   the commit has already been shared, applied as a new commit on
   top.
3. The gate runner re-runs the failed gate against the corrected
   commit. On pass, the trailer is updated.

Trailers are not a substitute for the gates themselves. They are
the evidence layer that lets the gates be run lazily, in parallel,
and in any order without losing the proof of validity.

## Cross-references

- `atomic-changes` `references/commits.md` — the canonical commit
  form this module reads in state-space terms.
- `principles.md` — domain, codomain, range, preimage; the
  bilateral goal; the six operations; dictates / stipulates /
  eliminates as roles of a function.
- `proof-preservation.md` — typed proofs that survive across
  conversions; trailers are the same idea applied to commits.
- `testing.md` — gates are the test side of state-space
  minimization; trailers record their outcomes.
- `documentation.md` § "The dominant failure mode is drift" —
  drift is the symmetric failure when claims and code diverge over
  time; commits are the state-transition record those claims
  describe.
- `ratchet.md` — per-commit diff-size thresholds are one
  instance of the ratchet pattern applied to the
  size-of-a-state-transition metric; gate trailers are the
  commit-level forcing function.
