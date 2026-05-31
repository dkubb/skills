# Deterministic Simulation Preference

- Prefer deterministic simulation testing across the codebase (e.g.,
  madsim-style).
- Preserve the "world" isolation pattern; avoid tying core logic directly to
  runtime-specific APIs.
- Be cautious with tokio integration; keep async boundaries testable and
  consider simulation-friendly abstractions.
- Treat DST as a hard constraint: all dependencies must be injectable (Env
  traits, executors, deterministic seeds).
