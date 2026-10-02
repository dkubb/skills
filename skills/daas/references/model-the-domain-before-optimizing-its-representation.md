# Model the domain before optimizing its representation

Begin with the information the system needs, its valid values, invariants,
relationships, and changes over time. Prefer the smallest data type that
represents all required states and as few other states as possible.

Software cannot fulfill its purpose if its representation loses part of the real
domain. Smaller valid state spaces also reduce reasoning and testing.

Domain meaning comes before machine-friendly layout. A more efficient
representation is acceptable when it preserves the guarantees and serves a
measured requirement. A smart wrapper can carry the validation witness around an
optimized internal representation.

A primitive's raw representable values and the valid values inside a privately
constructed wrapper are different domains. Do not assess the primitive while
ignoring the constructor that constrains it. Unwrapping can discard the witness
and widen what callers must consider.

Separate authoritative storage, use-specific projections, and the domain
representation. Each should express the information and guarantees its consumer
needs rather than imposing one representation on every layer.
