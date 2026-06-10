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
| Refactor | Restructure without behavioral change |
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
3. **Move** / **Rename** — preserve shape; mechanical, compiler-checked.
4. **Refactor** — preserves behavior, reshapes structure.
5. **Change** — shifts observable behavior; callers may break.
6. **Add** — expands the state space; the largest source of new bugs.
7. **Upgrade** / **Downgrade** — isolate a dependency bump in its own commit.

## Anti-patterns

- Compound "and" / "or" subjects — split.
- Vague subjects ("misc", "cleanup", "stuff", "various") — name the
  transformation.
- Past-tense or gerund subjects ("fixed", "adding") — use the imperative.
- A subject verb that does not match the diff's effect.
- WIP commits on a shipped branch — squash or rewrite before merge.
- `--no-verify`, or `--amend` / rebase of commits already shared.
- Mixing a refactor with a behavior change in one commit — the
  behavior-preservation guarantee no longer holds.
