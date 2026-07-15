# Deterministic Testing Review Notes

Purpose: Provide review prompts for deterministic behavior and deterministic
simulation testing (DST). DST is **OPTIONAL** unless project rules or the user
make it a requirement.

Definition: Deterministic simulation testing (DST) means running the system in
a controlled, repeatable environment where time, randomness, IO, and external
resources are simulated or injected. The same seed and event schedule must
produce the same results.

## Review Prompts

**Advice:** The prompts in this section are non-normative unless the project or
user adopts a specific prompt as a requirement.

### External Dependencies (DI Considerations)

- Identify calls to external resources (filesystem, network, clock, random,
  process exec, signals, environment variables, temp dirs, database, OS APIs,
  UUID generation, system entropy).
- If these are hard-coded or created internally, consider suggesting dependency
  injection (DI) to make deterministic testing possible.
- Example review prompt: "This hard-codes access to `<resource>`. Would you
  consider injecting it so tests can control it deterministically?"

### Time and Randomness

- Any use of current time or timeouts can cause nondeterminism.
- Seed and control RNGs in tests when outcomes affect logic.
- Use deterministic clocks in simulation tests where possible.

### Concurrency and Scheduling

- Thread scheduling and async task ordering can be nondeterministic.
- Look for races, shared mutable state, and time-based waits.
- Suggest deterministic schedulers or event queues for simulation.

### Data Ordering and Iteration

- Unordered collections (hash maps/sets) can produce unstable iteration order.
- Compare the exact observed order by default. Sort or use an order-insensitive
  comparison only when the domain contract explicitly says order is
  unobservable; preserve every remaining distinction, including cardinality
  and duplicates.

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

## When DST is the project goal

When project rules or the user state DST as a requirement, the following are
**MUST** requirements:

- All dependencies must be injectable (Env traits, executors, deterministic
  seeds). Prefer deterministic simulation testing across the codebase
  (madsim-style).
- Preserve the "world" isolation pattern; avoid tying core logic directly to
  runtime-specific APIs.
- Be cautious with tokio integration; keep async boundaries testable and
  prefer simulation-friendly abstractions.

## Review Guidance

- DST is **OPTIONAL** unless specified by project rules or the user.
- **Advice:** When DST is not required, reviewers can identify determinism
  risks or encourage reproducible inputs, seeds, and clocks, but those
  observations are not findings.
