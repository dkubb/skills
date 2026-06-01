# agents

Authoritative source for a small set of agent **skills** — durable, reusable
guidance for AI coding agents — plus a Rust workspace that exposes the
executable parts through a single `skill` CLI.

This repository is the authoritative source for the skills below. Where they
are installed for an agent runtime, the install location symlinks to the
checked-in `skills/` tree rather than copying it, so refinements made here take
effect without drift.

## Skills

| Skill | Kind | Purpose |
|---|---|---|
| [`code-review`](skills/code-review/SKILL.md) | guidance + CLI | Run a clear, cross-language code review and change the review rules. Ships a bundled lint suite runnable via `skill code-review lint`. |
| [`state-space-minimization`](skills/state-space-minimization/SKILL.md) | guidance | Make invalid states unrepresentable by minimizing the program's state space. |
| [`state-space-minimization-formal`](skills/state-space-minimization-formal/SKILL.md) | guidance | The same discipline as a compact formal calculus: domains, ranges, invariants, morphisms, normal forms, proof-preserving boundaries. |
| [`atomic-changes`](skills/atomic-changes/SKILL.md) | guidance | Break work into the smallest atomic steps, verify the foundation first (step 0), and order steps so partial progress always leaves the system valid or better. |

Each skill is a directory with a `SKILL.md`. Guidance-only skills are just that
document plus reference material; `code-review` additionally has a Rust crate
that implements its CLI.

## The `skill` CLI

The `skill` binary dispatches to the skills that have executable behavior. With
no subcommand it prints help; subcommands follow a consistent contract (help on
stdout, errors on stderr, distinct non-zero exit codes per failure class).

```sh
# from the repo root (the cargo workspace is rooted here)
just build                 # build the workspace (uses the pinned toolchain)
cargo run -p skill         # print help
cargo run -p skill code-review lint   # lint this workspace with the bundled lints
```

`code-review lint` runs a bundled [Dylint](https://github.com/trailofbits/dylint)
suite. On first run it bootstraps Dylint (clones the pinned upstream, builds the
driver, compiles the lint crates) under `skills/code-review/lints/target/`;
later runs reuse that cache.

## Layout

```
AGENTS.md                     # operating conventions for agents working here
Cargo.toml                    # virtual workspace (members + shared deps/lints)
rust-toolchain.toml           # pinned nightly
justfile                      # build / lint / test / coverage / CI recipes
.cargo/                       # cargo aliases, deny + clippy config
skill-cli/                    # the Rust crates
  src/main.rs                 # the `skill` binary (clap dispatch)
  core/                       # skill-core: shared error + command-outcome types
  tests/                      # CLI integration tests
skills/                       # one directory per skill (no loose files)
  code-review/                # code-review skill: SKILL.md, references, CLI crate
    lints/                    # vendored Dylint lint workspace (excluded from the root workspace)
  state-space-minimization/         # guidance-only skill
  state-space-minimization-formal/  # guidance-only skill
  atomic-changes/                   # guidance-only skill
```

`code-review` is both a skill and a workspace member: its crate lives in
`skills/code-review/` (so the skill stays self-contained) and is referenced by
the workspace at the repo root.

## Development

All commands run from the repo root.

```sh
just ci      # full gate: fmt-check, lint, test, coverage, deny, validate-skills
just lint    # clippy (strict) + the bundled Dylint lints over this workspace
just test    # test suite + doctests
just coverage  # llvm-cov ratchet (fails if uncovered lines/regions/functions regress)
just fix     # auto-format + clippy --fix
```

**Toolchain.** The workspace is pinned to a nightly in `rust-toolchain.toml`.
On machines where Homebrew's Rust shadows the `rustup` proxies, a bare `cargo`
is Homebrew stable and silently ignores the pin — so the `justfile` invokes
cargo through `rustup run <pinned-toolchain>`. Prefer the `just` recipes over
bare `cargo` to stay on the pinned toolchain.

## Conventions

[`AGENTS.md`](AGENTS.md) is the operating guide for agents (and humans) working
in this repo: define success criteria first, minimize the state space, write
atomic conventional commits documenting the why/what/how, work in dependency
order, and ask before destructive or ambiguous actions.
