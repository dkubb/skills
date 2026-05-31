# code-review skill — bundled Dylint lints

This directory hosts the Dylint lint libraries exposed via
`skill code-review lint`. The full set is copied from
[symbiote/tools/lints](https://github.com/dkubb/symbiote/tree/main/tools/lints).

## Layout

```
lints/
├── README.md
├── Cargo.toml                       # workspace root, mirrors symbiote [workspace.lints]
├── clippy.toml                      # mirrors symbiote disallowed-methods
├── rust-toolchain.toml              # pinned nightly + components
├── patches/
│   └── filter-to-owned-method-extra-symbol.patch
├── run-dylint                       # bash entry: builds upstream cargo-dylint,
│                                    # builds every lint cdylib, runs against caller cwd
├── assert_contains/                 # one lint per crate, each a cdylib
├── bare_clone/
├── ...                              # 24 lints total
└── unwrap_or_else/
```

## Why detach from the workspace

The agent-skills workspace `.cargo/config.toml` sets
`rustflags = ["--forbid", "unsafe_code", ...]`. Dylint cdylibs require
`#[unsafe(no_mangle)]` on `register_lints`; the symbiote pattern uses
`#[expect(unsafe_code, reason = "...")]` to opt back in, but `--forbid`
cannot be overridden by `#[expect]`. The lints workspace at `lints/Cargo.toml`
is its own root (parent `members` glob is one level only, so it isn't
picked up automatically); the runner exports
`RUSTFLAGS="--deny unsafe_code --deny warnings"` to replace (not
concatenate) the parent's rustflags during the lint build, and
`CLIPPY_CONF_DIR="$lints_dir"` so clippy finds the bundled
`disallowed-methods` list.

## Pinning

- **Dylint upstream:** `2c20d164e06c446de2cecadff0c55e16a877c006`
  (upstream `master` on 2026-05-07). Built from source because
  `cargo-dylint 5.0.0` on crates.io predates rustc API changes after
  2026-03-18.
- **Toolchain:** `nightly-2026-05-07` with `llvm-tools-preview` and
  `rustc-dev`. Matches `~/workspace/dkubb/symbiote/tools/lints` so
  copying lints in is a file move plus a workspace-table tweak.

## First-run cost

The first `skill code-review lint` call clones the dylint upstream repo,
applies the compatibility patch, and builds `cargo-dylint` and
`dylint-link` from source — ~30–90 seconds. All artifacts land under
`lints/target/dylint/` (gitignored). Subsequent calls reuse the cache
and complete in ~1 second over a small crate.

## Adding a new lint

The fastest path is to copy one from symbiote:

```sh
cp -r ~/workspace/dkubb/symbiote/tools/lints/<lint_name> \
      ~/workspace/dkubb/agent-skills/skills/code-review/lints/<lint_name>
rm -rf <lint_name>/target
# add <lint_name> to lints/Cargo.toml `members`
```

The lint crate's `[lints] workspace = true` resolves against the
bundled `[workspace.lints]` policy, and the runner's
`DYLINT_LIBRARY_PATH` discovers the cdylib after the next workspace
build.

For a hand-written lint, mirror any existing crate's structure:

1. `lints/<lint_name>/Cargo.toml` — single-file `cdylib`, depends only
   on `dylint_linting` (pinned git rev) and inherits workspace lints.
2. `lints/<lint_name>/src/lib.rs` — `#![feature(rustc_private)]` plus
   `dylint_linting::declare_late_lint! { … }` and a `LateLintPass` impl.
3. UI fixture pair under `ui/<lint_name>.rs` + `.stderr` (optional but
   recommended).

See `../LINT-TODO.md` for hand-written lint implementation guides.

## Refreshing the installed `skill` binary

`~/.cargo/bin/skill` is built once and cached. After changes to the
skill code, refresh it:

```sh
cargo install --path /path/to/agent-skills --force
```

Or for local testing, invoke the workspace's debug binary directly:

```sh
/path/to/agent-skills/target/debug/skill code-review lint
```
