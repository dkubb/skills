# Deterministic Testing Review Notes

Purpose: Provide review prompts for deterministic behavior and deterministic
simulation testing (DST). Treat determinism as a goal in general. Do not assume
DST is a required project goal unless explicitly stated in project rules or by
the user.

Definition: Deterministic simulation testing (DST) means running the system in
a controlled, repeatable environment where time, randomness, IO, and external
resources are simulated or injected. The same seed and event schedule must
produce the same results.

## Review Prompts

### External Dependencies (DI Considerations)

- Identify calls to external resources (filesystem, network, clock, random,
  process exec, signals, environment variables, temp dirs, database, OS APIs,
  UUID generation, system entropy).
- If these are hard-coded or created internally, consider suggesting dependency
  injection (DI) to make deterministic testing possible.
- Phrase as a suggestion, not a requirement, unless the project rules say DST
  is mandatory.
- Example review prompt: "This hard-codes access to `<resource>`. Would you
  consider injecting it so tests can control it deterministically?"

### Time and Randomness

- Any use of current time or timeouts can cause nondeterminism.
- RNG should be seeded and controlled in tests when outcomes affect logic.
- Use deterministic clocks in simulation tests where possible.

### Concurrency and Scheduling

- Thread scheduling and async task ordering can be nondeterministic.
- Look for races, shared mutable state, and time-based waits.
- Suggest deterministic schedulers or event queues for simulation.

### Data Ordering and Iteration

- Unordered collections (hash maps/sets) can produce unstable iteration order.
- Sorting results before comparison can remove flakiness.

### SQL Query Ordering

- If ORDER BY is omitted, treat the result as nondeterministic.
- LIMIT without ORDER BY is a common hidden determinism bug.
- If no ORDER BY is present, assume the order is effectively random.
- Suggest adding explicit ORDER BY when order matters.

### File System and IO

- File metadata and directory iteration order can vary.
- Temp paths and OS-specific behaviors are often nondeterministic.
- Suggest a deterministic filesystem abstraction or fixed fixtures in tests.

### Floating Point and Locale

- Floating point operations can differ by platform or optimization.
- Locale-dependent parsing/formatting can change across environments.
- Suggest explicit locale or stable formatting when order or output matters.

### Reproducibility and Diagnostics

- When failures are possible, ensure the failure logs capture inputs and seeds
  needed to reproduce the issue.
- Prefer small repro artifacts on failure; avoid logging on success.

## Review Guidance

- Determinism is a quality goal; DST is optional unless specified.
- Raise determinism risks during review but do not block unless required by
  project rules.
- Encourage reproducible tests with controlled inputs, seeds, and clocks.
