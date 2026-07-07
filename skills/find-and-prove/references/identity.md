# Identity & causality — defect class E

Load when the target's guarantee is integrity of identity: bindings preserved
(no forgery, no confusion, no collision), identity decided by
equality/hashing/canonicalization, or causal structure claimed over runs.

## E1 Collision-kernel audit

FIRE on every lossy projection at an identity boundary: never ask "is it
injective?" — compute the KERNEL of the map. Per constructor, mark each
distinguishing field KEPT / DROPPED / QUOTIENTED, and demand the invariant
that recovers each drop (or the explicit decision that nothing downstream
needs it). Force the multiplicity case: N = 2 equal payloads into a set
target is the element-vs-container axis — after element injectivity, ask
whether the CONTAINER quotients order or multiplicity (`[x]` vs `[x, x]`
collapsing into `{x}`). The deciding witness: two distinct sources in one
kernel class that a downstream consumer must distinguish. [Anchor: kernel of
a map.]

## E2 Two-schedule test

FIRE on every "causal / parent / frontier" structure: run the update on two
INDEPENDENT events under both orders A;B and B;A. If either parent set
contains the other event purely from serial order, the structure is a prefix
log wearing a causal label. Replay keys must be invariant under topological
reorder; result ORDER belongs in the payload, never in the parent set. The
deciding witness is the run pair whose recorded causal structures differ
only by the scheduling of independent events. [Anchors: Mazurkiewicz traces;
happens-before — Lamport.]

## Multi-field coherence / evidence binding

Every payload with id+args+result+proof+version+authority+parents must prove
its fields describe the *same* event. Attack by swapping one field while
preserving the others; require ok-coherence, error-coherence, frame,
provenance-binding, hash/payload agreement.

## Codec / canonicalization lawfulness

Round-trip, idempotence, soundness+completeness, canonical uniqueness. A bad
canonicalizer is a *replay-semantics* bug — a canonicalization law matters
*because* it decides identity equality. **Runtime-bridge laws**: lowering
preserves/narrows authority; "Lean proved it" ≠ "Rust does it".
