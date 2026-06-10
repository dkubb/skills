# Commit structure

The canonical form for commits in this repo. A commit is one bounded change:
one transformation, a message that names it, and a tree that passes every gate.
Smaller commits give a smaller preimage of failure — when CI breaks, `git
bisect` lands on a single transformation instead of a tangle.

## Atomic

- One transformation per commit — one type, one action verb. If you need "and"
  to describe it, it is two commits.
- Each commit passes the full gate on its own. No broken intermediate states in
  history; eliminate them at write time, not by rewriting after.
- Smaller is better, bounded by functional integrity: the tree stays green and
  deployable. Rough diff-size guide (source + tests): ≤30 lines ideal, 30–50
  the max for confident review, 50–300 needs justification, >300 split.

## Subject — `type(scope): summary`

- `type` from the closed set: `feat`, `fix`, `refactor`, `perf`, `style`,
  `test`, `docs`, `build`, `ci`, `chore`, `revert`. A `!` before the colon
  marks a breaking change to the public contract.
- ≤70 characters including the type prefix.
- Lowercase, imperative present ("add", not "added" or "adds"), no trailing
  period.
- No "and" / "or" — a compound subject is two transformations. Split.
- The subject verb must match what the diff actually does.

## Body

- One blank line after the subject; wrap at 72 columns.
- Explain the *why* — the constraint or intent. The diff already shows the
  *what*.
- No `What:` / `Why:` / `How:` labels. When the body makes more than one point,
  use action lines instead of prose bullets.

## Action lines

When the body has multiple observations, write each as one action line:

```text
- <Verb> <reason>.
  <how — optional, indented, only when the implementation is load-bearing>
- <Verb> <reason>.
```

The verb is from the closed set below; the reason is the *why* and ends with a
period. A single-observation body can stay prose.

## Action verbs (closed set)

| Verb | Use for |
|---|---|
| Remove | Delete unused code, dependencies, or features |
| Fix | Correct a regression, bug, or constraint violation |
| Move | Relocate code (a refactor that changes path, not shape) |
| Rename | Change identifiers (a refactor that changes name, not shape) |
| Refactor | Restructure without behavioral change; touches code or tests, not both |
| Change | Modify existing observable behavior |
| Add | Introduce new public API or capability |
| Upgrade | Bump a dependency to a newer version |
| Downgrade | Revert a dependency to an older version |

A change that needs a verb outside this set is more than one transformation —
split it.

## Ordering (transformation priority)

Absent a functional dependency forcing another sequence, ship in this order.
Each tier reduces or preserves the state space before the next expands it;
within a tier, mechanical and compiler-checked changes come before ones whose
behavior preservation must be reasoned about.

1. **Remove** — reduces the state space; deleted code cannot have bugs.
2. **Fix** — eliminates an invalid (representable-but-wrong) state.
3. **Move** — relocates code; the path changes, name and shape stay.
   Mechanical, compiler-checked.
4. **Rename** — changes identifiers; the name changes, location and shape
   stay. Mechanical and compiler-checked, but shifts the codebase's
   vocabulary at every use site.
5. **Refactor** — preserves behavior, reshapes structure; preservation
   must be reasoned about, not compiler-checked. Touches code or
   tests, never both: the untouched half is the fixed frame of
   reference proving the behavior survived. Code and tests may each
   be refactored back-to-back — separate commits, either order, gates
   passing between — but a single diff editing both has no frame, and
   is a Change.
6. **Change** — shifts observable behavior; callers may break.
7. **Add** — expands the state space; the largest source of new bugs.
8. **Upgrade** — a dependency bump, isolated in its own commit.
9. **Downgrade** — a dependency rollback, isolated in its own commit;
   usually an evidence-driven correction.

## Anti-patterns

- Compound "and" / "or" subjects — split.
- Vague subjects ("misc", "cleanup", "stuff", "various") — name the
  transformation.
- Past-tense or gerund subjects ("fixed", "adding") — use the imperative.
- A subject verb that does not match the diff's effect.
- WIP commits on a shipped branch — squash or rewrite before merge.
- `--no-verify` — bypasses the gates unconditionally; never use it.
- `--amend` for content changes — the change is invisible until already
  folded into history, can quietly turn one commit into two
  transformations, and can land on a commit when an earlier one was the
  right target. Put the correction in a `fixup!` commit aimed at the
  right commit; it stays auditable until autosquash folds it in on
  request. Gate-trailer amends on unshared commits are metadata-only and
  exempt.
- `--amend` or rebase of commits already shared — rewrites a state
  others may depend on.
- Mixing a refactor with a behavior change in one commit — the
  behavior-preservation guarantee no longer holds.
- A refactor commit that edits code and tests together — with neither
  half held fixed there is no frame of reference, so no proof the
  behavior survived. Reclassify as Change, or split into back-to-back
  refactor commits (either order) with gates passing between.
