# Carrier / representation — where does this value live?

Run when choosing how a value is carried through a system: opaque routed
payload, typed branch, replay key, or adapter-owned encoding.

## Route-vs-encode (the 8-question procedure)

1. Who owns the SEMANTICS of construction? If the net concatenates /
   encodes / normalizes to build a request, you rebuilt the rejected
   in-core encoder; if an admitted adapter/template owns it and the net
   only routes a typed value into a SEALED template, it stays routing.
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

For a value-language → bounded-count decoder, raw `min n K` gives a second
spelling of zero and can contradict the base case's policy; canonical tags
(`0 ↦ [], j ↦ [j]`) with a total selector clamping nonempty atoms into
`1..K` keep the reachable language exact and the selector total.
**Soundness caveat (binding):** the CLAMP is sound only when the envelope
DECLARES off-contract input in scope — clamping undeclared out-of-contract
input violates fail-closed. Declare the envelope or reject, never silently
clamp.

## Carrier-swap bridge (the 15-question set)

Run before bridging a typed-carrier supply to an erased-carrier supply
ahead of a carrier change:

1. What is the future proof-carrying carrier, and what is the current
   erased carrier?
2. What is the per-record projection into the future carrier?
3. Is that projection pinned EXACTLY, including intent AND observation
   ORDER (future-carrier-facing, not just via the raw erasure)?
4. What is the per-record erasure back to the current carrier?
5. Is the LIST projection pinned as a canonical map, not merely
   demonstrated on a singleton?
6. Is the LIST erasure pinned as a canonical map, including the empty list
   AND multi-record lists?
7. Does a raw-erasure-facing bridge accidentally HIDE a future-carrier bug?
   Check INVERSE-PAIR mutants: a bad projection followed by a compensating
   erasure cancels under the round-trip but delivers wrong content to the
   new carrier — pin the projection's exact content in the FUTURE carrier's
   vocabulary.
8. Does the supply theorem quantify over ARBITRARY records, not one demo?
9. Does the matched-head `next` theorem pin BOTH the returned observations
   and the returned tail supply?
10. Is the mismatch case genuinely new, or fixed by the kernel replay law
    plus intent preservation?
11. Is the exhausted case genuinely new, or already fixed by the
    list-erasure map plus the kernel nil law?
12. Did a concrete singleton demo sneak into the BASIS? Replace with a
    parametric map theorem or demote to an example.
13. Does the theorem stop at the supply boundary, without dragging in a
    reducer run?
14. Do names say "decoded supply erases to raw supply", not "the reducer
    receives decoded observations"?
15. Which transport/erasure lemma will the NEXT rung use to avoid
    `cases h; rfl` constant-family shortcuts? Export it now.
