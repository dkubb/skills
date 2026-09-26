# Derived state must remain visibly derived

Prefer immutable recorded facts where they fit the domain. Introduce caches or
materialized projections only for measured needs, with benchmarks supporting
materialized current-state projections. Mark derived state explicitly, retain
authoritative data, and detect drift through reconstruction or comparison. A
faster view must not become a competing truth whose disagreement cannot be
investigated.

Content-addressable caches key results by their complete inputs. A hit claims
the result belongs to those inputs; a miss permits recomputation.

This is not a mandate for event sourcing everywhere. Distinguish missing data
from contradictory data: a cache miss can be recomputed, but disagreement
between a cached result and recomputation for the same inputs is a fatal
invariant violation. Neither result can be trusted merely because it is newer.
Stop the affected computation rather than silently choosing a winner.
Time-dependent facts such as current authorization need more than a timeless
construction witness. The domain contract determines which time's facts govern a
consequential action; an earlier accepted commitment and a permission that must
remain current are different guarantees. Do not infer a durable entitlement when
the contract has not established one.

For queued work, validate authorization at admission and before the effect.
Default to current authorization and fail closed unless an explicit commitment
makes intent-time authorization binding; then establish authorization against
that earlier state. A conservative denial is visible and can expose a missing
requirement, while an unauthorized effect may be harmful and irreversible.

**Preservation is separate from exposure.** Retain potentially useful evidence
when storage is cheap and deletion would discard future options. That does not
make every historical record suitable for the active working context. Keep the
current model unambiguous; retrieve superseded reasoning deliberately, with
enough context to recognize what replaced it.

An agent's context is a projection for a particular decision. Supply the narrow
relevant view, with progressive access to supporting evidence when needed. The
context constructor is primarily responsible for including necessary facts and
instructions; the agent cannot discover an omission it has no way to know
exists. It should still ask about evident gaps.
