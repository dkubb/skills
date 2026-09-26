# Preserve distinct states even when outcomes coincide

## Preserve meaningful distinctions

Make transitions explicit, tracked, and observable. Model independent facts
independently; do not merge semantically distinct states merely because they
currently produce the same outcome.

A failed shipment and a cancelled shipment may trigger the same billing action
while expressing different facts. Losing that distinction corrupts the model and
limits later decisions.

Prefer state-specific data types with only the capabilities available in each
state, and explicit transitions that can record when the change occurred. Avoid
one mutable entity with a status field whose meaning requires scattered
validation and callback conditionals. If a concrete consumer needs stable
identity around changing state, make that wrapper intentional; do not add it by
default.

## Keep intermediate states valid

State and the policy for handling it are separate concerns. A migration may omit
a dedicated legacy/new tag when the transition is bounded, validated,
recoverable, and that distinction cannot affect a consumer’s behavior. It must
still represent the values that actually exist: an `Option<T>` may be necessary
until legacy nulls are backfilled, even when new writes already require `T`.
Tighten the application type only after the stronger invariant holds.

Prefer incremental migrations with valid intermediate states that are *safe to
stop here* and resume later. Safe to pause does not require every step to be
reversible. Preserve information needed for recovery before a destructive step.
For critical services, downtime is a last resort; weigh lost revenue,
criticality, and operational risk against the cost of incremental deployment. A
low-criticality personal system may justify a maintenance window. Decomposition
and deployment cadence are separate decisions: independently verified steps can
run consecutively within one window. Mechanize enforceable constraints and use a
plan for the remaining coordinated transitions.

Apply the same migration discipline to governing rules and processes. Preserve
valid intermediate states, make old and new obligations explicit, and remove
concessions after the stronger invariant holds. Where practical, keep the
incumbent operating while its replacement earns the properties consumers rely
on. Compare them under real workloads and transfer responsibility gradually;
retaining the incumbent as an oracle is a deliberate cost/value decision. Do not
compare the real incumbent with an idealized replacement. Include lost
capability, accumulated knowledge, migration cost, temporary regressions, and
the time needed to rebuild confidence. Past expenditure alone is sunk; the
present capabilities and evidence it produced still have value.

## Record observations, not inferred history

For consequential external or distributed work, **Intent → I/O → Result** makes
the lifecycle explicit: persist the intended operation and inputs, perform the
effect, then record the result separately. Use it when knowing what was intended
and subsequently observed matters, even when retry is unavailable. Ordinary
local I/O, such as reading a file, does not need this machinery merely because
it is I/O. Pending work and recorded results can compose into observable state
machines.

An intent without a result means outcome unknown; it establishes neither success
nor failure, nor whether an attempt occurred. In the non-idempotent notification
case, Dan would retain the pattern and avoid automatic retry. Critical uncertain
effects stop unsafe dependent progression while reconciliation seeks external
evidence. Record or backfill only the result that evidence establishes: “no
corresponding record observed” must not become “never attempted.” Reconciliation
itself can use the same pattern when its activity must be observable, so failure
of the recovery mechanism does not look like absence of work. Prefer idempotent
effects where possible; recording a lifecycle does not itself make retries safe.
Claiming, locking, and provider-specific recovery belong in skills.
