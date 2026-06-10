# Bash Review Guidance (Language-Specific)

- Use simple English.
- Use short bullets.
- Do not repeat core principles.

## Tooling

- Run `shellcheck` on all Bash scripts and address warnings.
- Run `shfmt -w -i 2 -ci` on all Bash scripts.

## Review focus

- Require a Bash shebang and explicit strict mode via `set -Eeuo pipefail`.
- Prefer `$(...)` over backticks and `[[ ... ]]` over `[ ... ]`.
- Use arrays for lists instead of word-splitting.
- Send errors to stderr.
- Use long-form options when available and order options alphabetically when
  multiple options are present.
- Prefer single quotes for strings that do not use variable interpolation.
- Prefer compound conditions with `&&`/`||` instead of nested `if` blocks for
  simple checks.
- Sort `apt-get install` package lists alphabetically.
- Format scripts for human audit: add section headers, use blank lines, and
  break long pipelines or commands across lines.
