set shell := ["bash", "--noprofile", "--norc", "-o", "errexit", "-o", "errtrace", "-o", "nounset", "-o", "pipefail", "-c"]

# The pinned toolchain (resolved from rust-toolchain.toml). cargo runs through
# `rustup run` because this machine's rustup proxies are broken, so a bare
# `cargo` would be Homebrew stable and silently ignore the pin.
TOOLCHAIN := `rustup show active-toolchain | cut -d' ' -f1`

# Default recipe shows available commands
default:
    just --list

# Build the project
build:
    rustup run {{ TOOLCHAIN }} cargo build --quiet

# Check code compiles
check:
    rustup run {{ TOOLCHAIN }} cargo check --quiet

# Full CI validation pipeline
ci: fmt-check lint test coverage deny validate-skills

# Enforce the coverage ratchet. The command and its missed-line ceiling live
# in the `coverage` cargo alias (.cargo/config.toml); this recipe only supplies
# the pinned toolchain so the bundled llvm-tools-preview is found.
coverage:
    rustup run {{ TOOLCHAIN }} cargo coverage

# Clean build artifacts
clean:
    cargo clean --quiet

# Security + license audit
deny:
    cargo deny check --config .cargo/deny.toml

# Format code (fix in place)
fmt *args:
    rustup run {{ TOOLCHAIN }} cargo fmt-all --quiet {{ args }}

# Check formatting without changes
fmt-check:
    just fmt --check

# Lint with auto-fix
fix: fmt
    just lint-rust --allow-dirty --allow-staged --fix

# Lint the workspace: clippy + the bundled dylint lints (both check only).
lint: lint-rust lint-dylint

# Rust clippy (check only, pass args for --fix).
lint-rust *args:
    rustup run {{ TOOLCHAIN }} cargo clippy-all --quiet {{ args }}

# The bundled code-review dylint lints, against this workspace.
lint-dylint:
    rustup run {{ TOOLCHAIN }} cargo run -p skill --quiet code-review lint

# Run tests: the suite, then doctests (test-all uses --all-targets, which
# skips doctests, so run them explicitly).
test:
    rustup run {{ TOOLCHAIN }} cargo test-all --quiet
    rustup run {{ TOOLCHAIN }} cargo test-doc --quiet

# Validate all skills have required files
validate-skills:
    #!/usr/bin/env bash
    set -euo pipefail
    IFS=$'\n\t'
    failed=0
    for skill_dir in skills/*/; do
        if [[ ! -f "${skill_dir}SKILL.md" ]]; then
            echo "ERROR: ${skill_dir}SKILL.md missing"
            failed=1
        fi
        has_cargo=$([[ -f "${skill_dir}Cargo.toml" ]] && echo 1 || echo 0)
        has_src=$([[ -d "${skill_dir}src" ]] && echo 1 || echo 0)
        if (( has_cargo ^ has_src )); then
            echo "ERROR: ${skill_dir} has Cargo.toml or src/ but not both"
            failed=1
        fi
    done
    if [[ "$failed" -eq 1 ]]; then
        exit 1
    fi
    echo "All skills validated"
