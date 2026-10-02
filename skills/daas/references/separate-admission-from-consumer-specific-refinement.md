# Separate admission from consumer-specific refinement

Enforce the conditions for a value's legitimate existence at ingress. Refine it
later when a particular consumer requires a stronger state.

Rejecting genuinely invalid data early keeps the core simpler, while forcing
every consumer's strongest requirements onto all data can reject legitimate
uses.

A sole consumer usually defines the producer's admissible output. With multiple
consumers, move stronger constraints toward shared ingress only when doing so
does not remove valid uses.

Expensive checks depend on criticality, latency, and the operation they protect.
An anti-fraud check might run while a customer browses, with purchasing
unavailable until it succeeds. That models a legitimate intermediate state
rather than treating incomplete assurance as complete. [Carry a validation
witness][trust] governs when a layer may trust a value; this principle
determines which constraints belong at admission or at use.

[trust]: ./parse-don-t-validate-carry-a-witness-across-trust-boundaries.md
