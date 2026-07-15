# Coverage Review

Absolute uncovered item counts are the canonical signal because they are
directly measurable and identify the remaining gaps. Unlike percentages, they
do not change merely because unrelated covered code changes the denominator.
For each metric, let `U` be the actual uncovered item count and `T` the
configured maximum allowed uncovered count. The gate is `U <= T`; lower `T` is
stricter, and raising `T` is a regression.

## Scope

- Use these steps when coverage is in scope.
- Use the repository's coverage command and scope when they expose absolute
  item counts. A percentage-only policy does not satisfy this rubric; report
  the conflict and obtain absolute counts before approving the coverage gate.

## Review Steps

1. Use only absolute uncovered item counts. Do not use coverage percentages
   for evaluation, enforcement, or reporting.
2. Keep each configured uncovered threshold at the exact current count so the
   gate has no regression slack. This is the coverage instance of the ratchet —
   ceiling semantics in
   `state-space-minimization` `references/ratchet.md` (the ceiling only
   tightens; weakening only on explicit user request).
3. When an actual uncovered count decreases, lower the configured threshold to
   that count. The destination is a zero threshold. Raising an uncovered
   threshold **MUST** be treated as a regression and requires explicit user
   approval, even when the resulting code still passes the looser gate.
4. Use every absolute item metric the tool reports, such as uncovered lines,
   branches, functions, and regions. When a report provides covered and total
   counts, derive uncovered as `total - covered`.
5. A report that exposes only percentages is insufficient evidence. Obtain
   machine-readable item totals from the tool or use an equivalent tool that
   provides them.
6. Use branch coverage when the tool supports it.
7. When iterating with fuzz or round-trip coverage, address uncovered gaps
   before running longer fuzz sessions.
8. Run doctests, but do not count them toward coverage thresholds.
9. Keep review findings in the review output. When the repository records
   commit gate trailers, record a passing coverage gate as
   `Gate-coverage: pass`; do not copy review commentary into the commit
   message.
10. If a coverage tool is flaky, note the risk and ask the user.

## Rust Script Coverage

1. Use per-file absolute uncovered item ceilings.
2. Use nightly for branch coverage.
3. Use the JSON output to read or derive uncovered item counts.
4. Only lower thresholds when every uncovered metric passes its current
   threshold. Never raise one threshold to compensate for improving another.
