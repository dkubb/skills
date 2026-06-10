# CLI Review Guidance

- Streaming output should use JSONL.
- Single-result output should use JSON.
- Use clear exit codes so automation can detect failures reliably.
- Prefer machine-readable output so `jq` can parse results.
- For JSONL streams, emit one JSON object per line and avoid interleaving
  human-readable text with machine output.
- When running `gh` or similar CLIs behind a proxy, prefer unsetting proxy
  env vars (for example `env -u HTTP_PROXY -u HTTPS_PROXY -u ALL_PROXY`)
  to avoid proxy interference.
- Clap help/doc strings should start with a lowercase letter and end with a period.
- Prefer long-form CLI options in docs and examples; avoid single-letter flags
  (reserve them for repeated interactive use).

## Subcommand output contract

- Assume any program may be embedded in another command; structure output to
  communicate maximally.
- Every subcommand either prints context-specific help (when given no
  actionable command), or executes and produces success output on stdout,
  warnings and errors on stderr, exit 0 on success, and non-zero on error.
- Give errors that callers must handle differently their own distinct exit
  codes, so an embedding script can branch on the failure mode.
