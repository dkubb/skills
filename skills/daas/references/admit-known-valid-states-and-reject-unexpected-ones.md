# Admit known valid states and reject unexpected ones

Fail closed by default: use an allow list, define the allowed input, and reject
unexpected cases. Prefer closed parsing and exhaustive handling over accepting
unknown fields or cases.

An apparently additive field can change meaning: adding currency to a numeric
amount can invalidate a consumer's assumed currency. Unknown stored fields can
also acquire unintended authority later.

Forwarding opaque data is an intentional exception when transport itself is the
contract. *Semantic opacity* still requires a contract: validate applicable
encoding, syntax, size, depth, and resource bounds before admission. After
admission, do not interpret its meaning or make application decisions from its
contents. Keep it separate from understood data and do not log it as though its
meaning had been parsed. Prefer a private payload type exposing transport
operations over unrestricted accessors that invite hidden semantic coupling. If
a consumer needs to interpret it, define that new contract explicitly.

Rejection should fit the receiving layer. An internal operation can return an
error; a user-facing boundary should explain an invalid input intelligibly.
Failing closed does not require turning ordinary validation failures into
internal-server errors. Check each condition when the information needed to
establish it is available.

Strictness is a revisable model of valid reality. A rejected legitimate case
calls for investigation and a corrected model, not a claim that the world must
fit the original assumption.
