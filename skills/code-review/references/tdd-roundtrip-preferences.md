# TDD + Round-trip Debug Preferences

- For bug fixes, reproduce the exact failure with a test before changing code.
- Fix only after the failure is reproducible; keep all tests passing.
- Refactor either the test or the fix, but not both at once; keep tests passing.
- Commit atomically after the fix.
- After the commit, re-run the harness with the same seed and fast-forward to
  the same iteration to confirm deterministic reproduction is fixed.
- Assume failures are non-deterministic until verified by re-running the seed
  before investigating.
- Use fixed unit tests for known-bad inputs and boundary crossing around fixed
  points; reserve property tests/fuzzing for exploration.
