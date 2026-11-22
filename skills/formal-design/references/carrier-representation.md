# Carrier / representation — where does this value live?

Run when choosing how a value is carried through a system: opaque routed
payload, typed branch, replay key, or adapter-owned encoding.

## Route-vs-encode (the 8-question procedure)

1. Who owns the semantics of construction? Trace one value from producer
   to consumer. Concatenation, normalization and serialization construct a
   representation. Passing a typed value unchanged to an adapter delegates
   construction to that adapter.
2. Is the routed value part of request IDENTITY? The typed payload lives IN
   the request's canonical args and IS part of replay equality — never a
   side-store reference (the intent stops being self-contained; replay can
   match an id whose dereferenced value changed), never a model-only side
   field (two parallel identities that age badly).
3. Does a content hash sneak the canonical-encoding problem back in? A hash
   repairs id-stability but reintroduces in-core canonicalization — the
   thing routing was chosen to avoid.
4. Who renders payload→wire? Exactly one place (the adapter); the reducer
   never serializes bytes.
5. Does the new payload kind disturb a universal theorem stated over a
   fixed type? Generalize the kernel ONCE rather than fork or special-case
   it.
6. Does an exact-routing headline need its PARAMETRICITY sibling? "One
   witness routes the right value" is satisfiable by a reducer that
   inspects the value; pair it with "two configs differing only in the
   payload ⇒ everything but the carried payload invariant".
7. Is the boundary a representation BOUNDARY or a synonym? Semantic
   boundaries get NOMINAL carriers, never `abbrev` (a reducible synonym
   makes the representation definitional and unfixable later).
8. Shared source with two projections: when admission and lowering both
   need the datum and neither carrier reconstructs the other, use the
   SOURCE-SHAPED side with an identity projection — invented defaults on
   either projection are worse than carrying the source.

Dependent-index encoding questions route to `state-space-minimization`
(+ `-formal`).

## Canonical tags + total clamp (bounded-count decoders)

Identify which external spellings the contract accepts for each value
before choosing a decoder. For example, an empty list can be the unique
encoding of zero, with `[j]` encoding a count between one and a positive
bound `K`. Decide whether to reject an out-of-range singleton or clamp
it. Clamping changes which inputs count as valid. An explicit acceptance
policy is required even when the result always fits the internal type.

## Carrier-swap bridge (the 15-question set)

Run before changing the representation of a value consumed by an existing
interface.

1. What are the old and new carriers, and which invariants does each
   enforce?
2. What maps an individual old value into the new carrier? If conversion
   can fail, specify the accepted inputs and the failure result.
3. What exact content must the new consumer receive, including field
   identity, order and multiplicity where observable?
4. What information survives the projection back to the old carrier?
5. For collections, does the conversion agree with applying the individual
   map to every element?
6. Does the reverse projection preserve empty and multi-element inputs?
7. Could an incorrect forward map and a compensating reverse map pass the
   round-trip test? State a law on the new carrier that rejects that pair.
8. Do the laws hold for arbitrary valid inputs? Use a singleton as an
   example of the law.
9. Which outputs *and* state changes must each successful operation
   preserve? For a queue read, check both the item and the remaining queue.
10. What happens on rejection? Identify any existing law that already pins
    the error and successor state.
11. What covers the empty case? Check whether it follows from a collection
    law and the existing interface.
12. Which examples distinguish a broken map, and which claims require a
    quantified proof?
13. At which operation does the bridge stop? Keep later consumers outside
    its hypotheses unless the guarantee explicitly reaches them.
14. Does the theorem name specify the direction and observations it
    preserves? An erasure law alone says nothing about delivery to a client.
15. Which transport or erasure lemma will the next consumer use? State
    that lemma at the boundary to keep representation details private.
