# agents

> The agent skills I reach for most — kept sharp, and shared. Fewer, better
> tools for thinking through hard problems, not a process framework to adopt.

A *skill* is durable operating guidance: a way of approaching a problem, written
down once so an agent (or a person) can load it on demand. Most here are pure
guidance — a `SKILL.md` you read and apply. `code-review` also ships a runnable
lint suite.

## Skills

| Skill | Kind | What it gives you |
|---|---|---|
| [`state-space-minimization`](skills/state-space-minimization/SKILL.md) | guidance | Make invalid states unrepresentable. Shrink what a program *can* express until whole classes of bugs have nowhere to live. |
| [`state-space-minimization-formal`](skills/state-space-minimization-formal/SKILL.md) | guidance | The same idea as a compact formal calculus: domains, ranges, invariants, morphisms, normal forms, and proof-preserving boundaries. |
| [`atomic-changes`](skills/atomic-changes/SKILL.md) | guidance | Break work into the smallest steps that each leave the system valid or better — foundation first, ordered so partial progress is never a regression. |
| [`code-review`](skills/code-review/SKILL.md) | guidance + CLI | A clear, cross-language code-review discipline — and the rules for changing the rules. Ships a bundled lint suite you can run with `skill code-review lint`. |
| [`git-review`](skills/git-review/SKILL.md) | guidance + CLI | Review commit ranges against the atomic-changes contract using deterministic Git evidence. |

## Use them

Each skill is a directory with a `SKILL.md` — that document *is* the skill.
Point your agent's skills directory at the ones you want (symlink rather than
copy, so updates flow straight through):

```sh
git clone https://github.com/dkubb/skills
ln -s "$PWD/skills/skills/state-space-minimization" \
  ~/.claude/skills/state-space-minimization    # exact path depends on your agent
```

For the skills with executable behavior, a unified `skill` CLI ties them
together:

```sh
cargo run --package skill                    # see what's available
cargo run --package skill code-review lint   # run the bundled lints here
```

`code-review lint` runs a bundled [Dylint](https://github.com/trailofbits/dylint)
suite; the first run bootstraps Dylint (clone, build the driver, compile the
lints) and later runs reuse the cache. The workspace pins a nightly in
`rust-toolchain.toml`, and the `just` recipes wrap cargo so you stay on it.
