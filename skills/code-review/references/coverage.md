# Coverage Review

## Scope

- Use these steps when coverage is in scope.
- Use the repo coverage policy first.

## Review Steps

1. Do not allow coverage to decrease. This is the coverage instance of the
   ratchet — floor semantics in `state-space-minimization`
   `references/ratchet.md` (the floor only tightens; weakening only on
   explicit user request).
2. If coverage increases, lock the new value. Do not block locking in an
   improvement on reaching 100%; ratchet from the current value.
3. Use exact numeric values.
4. Use all metrics the tool reports.
5. Use branch coverage when the tool supports it.
6. When iterating with fuzz or round-trip coverage, address uncovered gaps
   before running longer fuzz sessions.
7. Run doctests, but do not count them toward coverage thresholds.
8. Keep review feedback in the commit notes.
9. If a coverage tool is flaky, note the risk and ask the user.

## Rust Script Coverage

1. Use per file coverage thresholds.
2. Use nightly for branch coverage.
3. Use the JSON output to read values.
4. Only change thresholds when all metrics stay the same or increase.
