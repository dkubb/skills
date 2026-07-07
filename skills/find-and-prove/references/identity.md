# Identity & causality — defect class E

Load when the target's guarantee is integrity of identity: bindings preserved
(no forgery, no confusion, no collision), identity decided by
equality/hashing/canonicalization, or causal structure claimed over runs.

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
