# Commit structure

The canonical form for commits in this repo. A commit is one bounded change:
one indivisible transformation, a message that names it, and a tree that
passes every gate relevant to its affected surfaces and transitive consumers.
Smaller commits give a smaller preimage of failure — when CI breaks, `git
bisect` lands on a single transformation instead of a tangle.

## Atomic

- One transformation per commit — one semantic action verb. If you need "and"
  to describe it, it is two commits.
- Each commit passes its relevant gates on its own. Select gates from the
  affected surfaces and their transitive consumers, not from unrelated file
  types. No broken intermediate states in history; eliminate them at write
  time, not by rewriting after.
- Smaller is better, bounded by semantic and functional integrity: the commit
  is one coherent transformation, and the tree stays green and deployable.
  Line count never defines the atom, but it is a superlinear review-risk
  signal:

  | Changed lines | Review signal |
  |---:|---|
  | 0–50 | Normal |
  | 51–100 | Noticeable |
  | 101–250 | Strong |
  | 251–500 | Very strong |
  | 501–1,000 | Exceptional |
  | More than 1,000 | Extreme |

  At each higher band, search more aggressively for coherent gated splits and
  require stronger evidence that the transformation is indivisible. Doubling
  a diff more than doubles its review burden because interactions and context
  grow with it. These are signals, never hard limits: a large mechanical,
  generated, or genuinely indivisible commit remains valid when its reason
  and verification are clear.
- For new code, the default atom is one public symbol plus its tests; N
  independent public symbols are N commits, never one. (Other verbs keep
  their own grain: a `Refactor` splits code from tests so the untouched half
  is the oracle; `Move`/`Rename` keep a symbol with its mechanical reference
  updates.) Splitting never needs justification — combining does, and the
  justification is that the candidate is one indivisible transformation or
  splitting would turn a relevant gate red: symbols that pass the gate only
  together (a trait and its only impl, a type and its smart constructor,
  mutually recursive functions, a public contract deployable only with its
  counterpart) are one atom. A large symbol still prompts another search for
  coherent behavioral or helper seams, but size alone cannot create one. A
  commit that bundles two coherent stand-alone changes is as defective as one
  that breaks the build.
- Operate as if full mutation testing were always on, whatever gates the repo
  configures: you cannot add functionality no test would kill a mutant of.
  Every behavior is exercised either directly by a public symbol's own tests,
  or through a private function reachable from a public tested symbol whose
  tests assert the result. So each public atom carries direct tests, not
  incidental coverage (which rarely kills mutants), and a private function —
  uncoverable with no caller — is never a standalone atom; it ships inside the
  atom that introduces its first tested caller.

### TDD contract/capability exception

The default new-behavior atom contains one public symbol, its implementation,
and its direct tests. A TDD workflow may split that atom into an adjacent,
dependency-linked pair only when the first commit is independently valid:

1. `Add <behavior> contract` — the final behavioral test executes under an
   exact expected-failure contract, and any declaration required for static
   compilation fails closed without providing the capability.
2. `Add <behavior>` — the minimum implementation adds the capability and
   removes only the expected-failure accommodation.

Red means current behavior is not expected to satisfy the final test
expectations. It does not mean the project test command fails. The Red commit
must pass every relevant gate, and no existing production path may invoke its
new failing declaration. The mechanism must reject a different failure, a
skipped or unexecuted test, and an unexpected pass.

This exception decomposes an executable contract from the capability it
specifies. It does not permit a broken intermediate tree, a general-purpose
stub commit, or a partially wired feature. Green depends on Red, so the pair
stays adjacent unless another functional dependency requires otherwise.

## Subject — `<Verb> <imperative summary>`

- Start with one semantic action verb from the
  [closed set](#action-verbs-closed-set). The verb is capitalized and names
  the transformation: `Remove`, `Fix`, `Move`,
  `Rename`, `Refactor`, `Change`, `Add`, `Upgrade`, or `Downgrade`.
- Do not use conventional commit prefixes in git commit subjects. Reserve
  `type(scope): summary` syntax for pull request titles or repo-specific
  tooling that explicitly asks for it.
- Keep the subject concise, ideally around 50 characters and no more than 72.
- Imperative present ("Add", not "Added" or "Adds"), no trailing period.
- No "and" / "or" — a compound subject is two transformations. Split.
- The subject verb must match what the diff actually does.

## Body

- One blank line after the subject; wrap at 72 columns.
- Keep it brief. Explain what changed and why the commit exists. Include how
  only when the implementation is novel, complex, or not self-evident from
  the diff.
- No `What:` / `Why:` / `How:` labels. When the body makes more than one point,
  use action lines instead of prose bullets.

## Action lines

When the body has multiple observations, write each as one action line:

```text
- <Verb> <reason>.
  <how — optional, indented, only when the implementation is load-bearing>
- <Verb> <reason>.
```

The verb is from the [closed set](#action-verbs-closed-set); the reason is the
*why* and ends with a period. A single-observation body can stay prose.

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
   tests, never both: the untouched half is the oracle — the fixed
   frame of reference that proves the behavior survived. Code and
   tests may each be refactored back-to-back — separate commits,
   either order, gates passing between — but a single diff editing
   both has no oracle, and is a Change. A true refactor pair
   commutes: gates pass for both orderings, because each commit is
   verified against an oracle the other does not touch. An ordering
   that fails exposes structure-coupled tests or a disguised Change.
6. **Change** — shifts observable behavior; callers may break.
7. **Add** — expands the state space; the largest source of new bugs.
8. **Upgrade** — a dependency bump, isolated in its own commit.
9. **Downgrade** — a dependency rollback, isolated in its own commit;
   usually an evidence-driven correction.

## Worked example — one module, three functions

A module exposes three public functions where `B` calls `A` and `C` calls `B`
(dependencies: A ← B ← C). That is three commits, not one:

1. `Add A` — the function and its tests. A leaf with no internal caller yet is
   still whole: its relevant gates pass, and it is deployable on its own.
2. `Add B` — depends on A existing, so it follows (dependencies before
   dependents).
3. `Add C` — depends on B, so it comes last.

One commit holding all three is too large even though it is green. A, B, and C
are separate public symbols, and each can stand as a gated commit once its
dependencies exist, so each is its own `Add`. Bundling them means a failing
gate cannot say whether A, B, or C broke it, and `git bisect` lands on three
transformations at once. "They are one feature" orders the three commits
adjacent; it does not merge them.

Each `Add` carries its own tests. If B cannot be honestly tested until C
exists, B is not yet a stand-alone public atom: fold it into C, or keep B
private until C introduces the tested public contract.

## Anti-patterns

- Compound "and" / "or" subjects — split.
- Bundling independent public symbols in one commit because they are "one
  feature" or "related" — relatedness orders adjacent commits, it does not
  merge them; each stand-alone symbol is its own commit.
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
- `reset --soft <base>` + recommit (or a file-splitting amend) without
  staging first — Edit/Write changes land in the working tree *unstaged*;
  `reset --soft` moves HEAD but leaves the index at the pre-edit tree, so the
  recommit captures the old content and silently drops the edits. It hides
  because gates run on the working tree: `check`/lint/test all pass while the
  commits omit the change. Stage (`git add -A`) before the reset; after
  recommitting verify `git diff HEAD` is empty.
- Mixing a refactor with a behavior change in one commit — the
  behavior-preservation guarantee no longer holds.
- A refactor commit that edits code and tests together — with neither
  half held fixed there is no oracle, so no proof the behavior
  survived. Reclassify as Change, or split into back-to-back refactor
  commits (either order) with gates passing between.
