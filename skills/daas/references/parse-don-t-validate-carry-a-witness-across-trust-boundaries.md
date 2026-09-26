# Parse, don’t validate: carry a witness across trust boundaries

Convert incoming data into a type whose construction establishes the component's
invariants. Treat raw database results and external library output as untrusted
inputs too. Each layer expresses the constraints it can enforce.

Data may have been written under older constraints or produced by a different
component's assumptions. A boolean validation result does not carry the
guarantee as reliably as a constrained value.

Reparsing is unnecessary when the exact trusted type and its applicable
guarantee survive a boundary. If the value becomes a raw string, or the
receiving domain has different requirements, parse or transform it into the
receiving type.

A construction witness does not make a time-dependent fact permanent. Apply
[the authorization-lifetime rules][authorization] when a guarantee must hold
at the time of an effect.

Database and language constraints have different powers. Neither should silently
omit its own enforceable rules because another layer checks something similar.
*Push constraints down* does not mean deleting them from independently
responsible layers above. Application validation gives early, appropriate
feedback; database constraints protect persisted state across historical data,
migrations, other writers, and application rewrites. A validated constraint
supports a claim about the historical rows within its scope; a constraint not
yet validated must not be treated as that evidence. The current application's
constructor alone cannot establish all stored data's history. Treat an
unexpected database constraint failure as evidence to investigate an earlier
boundary, while preserving the working safety net.

Give raw and validated representations different capabilities when the domain
needs an admission boundary, even if their fields have identical shapes. Prefer
**deserialize raw → construct validated → serialize validated**: SQL decoding
and deserialization produce raw values; a checked constructor returns a
validated value or an error. Avoid direct decoding that bypasses the
constructor, and withhold serialization from raw values when publication must
require validation. Property generators can produce valid and invalid raw inputs
without manufacturing invalid trusted values. The domain determines whether this
separation earns its cost.

[authorization]: ./derived-state-must-remain-visibly-derived.md
