# Reflection Improvement Iterations

## Iteration 1

Change: `Boundary.b` now returns a representable-state subtype
`{x : State // x in A.S}` instead of returning a raw `State` with a
separate `lands_in_S` proof field.

Why: this moves the `R(b) subset S(A)` obligation from an extrinsic
side condition into the boundary result shape. Per the calculus, the
ordinary boundary now makes impossible the invalid encoding where a
boundary value exists without evidence that it is representable.

Build result: `PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 2

Change: `TrustedBoundary` now returns a contract-state subtype
`{x : State // x in A.C}` directly and derives its ordinary
`Boundary` view through `TrustedBoundary.toBoundary`.

Why: this removes the extrinsic `lands_in_C` side-condition field.
Trusted construction now makes the proof of `b(u,p) in C(A)` part of
the result shape, so a trusted boundary value without contract evidence
is not representable in the Lean encoding.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 3

Change: `BehaviorOK` now requires `ContractPinned A (m.apply A)` in
addition to behavior preservation on `C(A)`.

Why: the strictness rule forbids shrinking invalidity by changing the
contract. This closes the encoding gap where a mechanism could satisfy
the behavior predicate while silently widening or replacing `C(A)`.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 4

Change: added `EligibleMechanisms` as the single determinant for
candidate membership, sufficiency, and `BehaviorOK`, then rewrote
`EarliestSufficient` and `CostMinimalAmongTies` to quantify over that
set.

Why: eligibility is one fact in the calculus, not three independent
premises to repeat. This normalization removes disagreement states where
one selector ranges over all candidates while another ranges over only
sufficient, contract-preserving candidates.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 5

Change: introduced `Rank` with a positive `index` proof and changed
`Mechanism.rank` from `Nat` to `Rank`; rank comparisons now use
`rank.index`.

Why: `Nat` admitted rank `0`, but the encoding order starts at one.
The positive subtype narrows the codomain without freezing the
architecture-derived rank table into a closed enum.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 6

Change: `ConstructiveDominance` now carries
`contract_pinned : Ap.C = Ac.C`.

Why: constructive dominance compares representations of the same
contract. Without this field, the Lean witness could relate artifacts
whose behavior happened to agree on `Ac.C` while the predicative artifact
used a different contract.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.
