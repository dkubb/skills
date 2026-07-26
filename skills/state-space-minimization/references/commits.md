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

The canonical commit form — the action-verb set, transformation
priority, subject / body / action-line rules, size signals, and
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

The canonical closed set of nine action verbs is in `atomic-changes`
`references/commits.md`. Every real transformation maps to exactly one verb.
The verbs partition the transformation space into disjoint regions; a commit
whose action cannot be expressed with one verb is trying to do more than one
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
   half, which serves as the oracle (`principles.md` § "Search
   algorithm"): a code refactor is verified against unchanged tests,
   a test refactor against unchanged code — the fixed frame of
   reference in either direction. Editing both in one commit adds a
   second construction path to "the gates pass" and voids the proof
   (`proof-preservation.md`) — such a diff is a Change. Back-to-back
   refactors of code and tests commute — gates pass in either order,
   since each is verified against an oracle the other never touches.
   This is the confluence invariant applied to commits
   (`normalization.md` § "Formal underpinnings"): order independence
   is evidence of genuine preservation and of independence (no
   dependency edge, so the pair may parallelize); an ordering that
   fails gates exposes structure-coupled tests or a disguised
   Change. Sets up the next tier safely.
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

- The subject's action verb is the outermost narrowing. Its imperative summary
  narrows the object and effect. Together they pin the transformation precisely
  enough that a reader who never opens the diff knows what kind of state
  transition happened.
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

````text
Fix null user references during registration

- Fix null pointer when downstream lookup returns no row.
  ```rust
  let user = lookup(id).ok_or(LookupError::NotFound)?;
  ```

- Remove the legacy fallback path that masked the null with a
  default user record.

````

## Atomicity

An atomic commit is a single bounded state transition: one
indivisible semantic transformation, one applicable parent state,
and a child state that passes every gate relevant to its affected
surfaces and transitive consumers. The transformation may be a
behavior, schema object, configuration surface, documentation
claim, or executable test contract. The whole branch is then a
composition of atomic transitions.

A monolithic commit is the opposite: many entangled transformations
in one diff. The preimage of failure when CI breaks on a monolithic
commit is the entire diff — bisect collapses to "this large commit
caused it" with no further narrowing. Atomic commits give a small
preimage of failure (this specific transformation), which is *why*
`git bisect` works.

Atomicity rules:

- One indivisible semantic transformation per commit. Two
  independently coherent transformations per commit form a
  Cartesian product; the preimage of failure is their union.
- Each commit independently passes its relevant gates.
  Intermediate invalid states (broken applicable tests or lint) in
  the history are representable-but-invalid — eliminate them at
  write time, not by rewriting after the fact.
- Diff size is a superlinear signal about preimage cardinality, not
  a hard atom boundary. The canonical signal bands are in
  `atomic-changes` `references/commits.md`. A larger diff demands a
  more aggressive search for coherent gated splits, but a
  mechanical, generated, or genuinely indivisible transformation
  remains one commit.

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
  fixed oracle, so the behavior-preservation obligation is
  undischarged; the gates passing no longer proves anything about
  the old behavior.

Compound and past-tense subjects are covered in § "The message as
type signature" above.

## Trailers as typed proof of validity

A commit's claim "this passes the gates" is unverified by default.
Without proof, every commit in the history is in an unconstrained
validity state.

Encode gate results as commit-message trailers, one per named gate:

```text
Fix null user references during registration

- Fix null pointer when downstream lookup returns no row.

Gate-fmt: pass
Gate-lint: pass
Gate-typecheck: pass
Gate-test: pass
Gate-mutation: pass
```

Each trailer is a typed proof that a specific named gate has been run and
passed. The set of trailers on a commit
narrows the validity state space from "unknown" to "exactly these
gates passed." A commit with all expected trailers in `pass` state is a
fully validated state transition. A commit missing an applicable trailer has
incomplete evidence. A `fail` result belongs in the candidate's verification
log, not in a commit message, because the failed candidate must not be
committed.

Trailer rules:

- One trailer per named gate. The name is the gate identifier and the value is
  `pass`.
- Trailers go in the footer block (one blank line after the body),
  one per line, in `Token: value` form.
- The set of expected gate names is project-defined and stable
  across a branch.
- Create the commit only after every applicable gate whose result was unknown
  for the exact candidate tree has passed. Reuse reliable evidence for gates
  already known to have passed that exact tree.
- Create the commit with the complete set of applicable `pass` trailers. A
  failed candidate is not a valid commit and **MUST NOT** be created merely to
  record `Gate-*: fail` evidence.
- If a gate fails, stop the candidate, return to the author, fix the tree, and
  invalidate only the evidence affected by that change.

The downstream invariant: any consumer of a commit (CI, review,
merge automation) can read its trailers and determine its validity
state without re-running gates. Re-runs are decided by *missing*
trailers, not by re-executing already-proven gates.

## Incremental gate execution

The trailer model makes expensive verification incremental without admitting
an invalid commit.

Workflow:

1. Freeze the candidate tree and identify its complete applicable gate set.
2. Reuse reliable `pass` evidence only when it proves the exact candidate tree,
   gate configuration, dependencies, and toolchain under review.
3. Run the cheapest unknown decisive checks first. Run independent unknown
   gates in parallel when their resource use permits it.
4. On failure, stop dependent or more expensive work, fix the candidate, and
   rerun only gates whose prior evidence the fix invalidated.
5. Once every applicable gate is known to pass, create the commit with the
   complete `Gate-*: pass` footer block.

A gate runner resuming after interruption can inspect retained exact-tree
evidence and execute only missing gates. After commit creation, do not rerun a
gate merely because the tree received a commit identity; the content did not
change. Re-run only when relevant content, configuration, dependencies,
toolchain, or the completeness of the earlier evidence changed.

Trailers are not substitutes for gates. They are the evidence layer that lets
gates run lazily, in parallel, and in any order while preserving the rule that
only a fully passing candidate becomes a commit.

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
- `ratchet.md` — gate trailers are the commit-level forcing
  function. Diff size is deliberately a review signal rather than a
  ratcheted threshold because line count cannot define semantic
  indivisibility.
