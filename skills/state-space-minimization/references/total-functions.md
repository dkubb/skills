# Total functions

A **total function** returns a valid result for every value of its
declared input type. A **partial function** does not. Partial functions
push the unhandled cases onto every caller, who must either guard inputs
(often forgetting cases), accept a panic, or invent a sentinel value.

Domain narrowing turns partial functions total. This is the single most
mechanical state-space win available, because the type system enforces
the contract once the input type is right.

## Examples

```
partial: negatives have no real square root
  fn sqrt(x: f64) -> f64

total: input type excludes the partial cases
  fn sqrt(x: NonNegativeF64) -> f64
```

```
partial: empty slices have no head
  fn head[T](xs: &[T]) -> T

total: input type excludes the partial cases
  fn head[T](xs: &NonEmpty<T>) -> &T
```

```
partial: index out of range panics
  fn at[T](xs: &[T], i: usize) -> &T

total: index type tied to the slice length
  fn at[T, len](xs: &SizedSlice<T, len>, i: BoundedIndex<len>) -> &T
```

## Why totality matters

Once a function is total:

- callers stop carrying defensive checks before the call
- error paths shrink to where errors are actually possible
- the signature becomes the truth about the contract instead of an
  optimistic summary
- reasoning is local: the caller sees the full set of possible outputs

For each partial domain function, choose exactly one outcome:

- narrow the input type so the function is total, or
- return a typed `Result` / `Option` because failure is a real outcome
  the caller must handle, or
- document why the operation is inherently partial (I/O, network,
  coordination) and confine the partiality to the smallest call site.

Push all input narrowing to the first boundary that sees the raw value.
The boundary parser is the place where partial → total conversion
happens at scale.

## Sentinel returns are partial in disguise

A function that returns `null` / `None` / a magic value to signal "I
couldn't" is still partial — the partiality has just moved into the
return type. The caller still has to handle the failure case.

A `Result` or sum-typed return is a *total function over the failure
case*: the caller is forced to handle each branch. That is good. The
defect is when:

- the failure case can never actually happen given the input — the
  function is total but the type lies (shrink the codomain)
- the failure type is `Result<_, String>` or `Result<_, AnyError>` —
  totality is preserved but the failure shape is unbounded; the caller
  cannot distinguish recoverable from fatal cases
- the function returns `Option` when the failure would be informative —
  `Result` with a typed error preserves more proof

## Totality is preserved by composition

If every function in a pipeline is total over its input type, the
pipeline is total. If even one is partial, the pipeline is partial and
*every* call site must handle the partial case or risk it propagating.
Push narrowing to the entry point so the rest of the system can be total
by construction.

## When totality is the wrong goal

Some operations are inherently partial and forcing totality with sentinel
return types only obscures it:

- I/O can fail at any moment; a `Result` here is honest, not a defect
- network calls can time out, retry, or partially succeed
- user input cannot be statically constrained — totality lives at the
  parser, not the renderer
- coordination across processes (locks, leases) introduces partiality
  that no input type can eliminate

Totality is a property of *pure* functions over *bounded* inputs. Use it
where it applies; do not stretch it past where it earns its keep.

## Cross-references

- `principles.md` § "Shrink the domain" — the input narrowing move.
- `principles.md` § "Shrink the codomain" — narrowing return types so
  total functions stay honest.
- `ingress-and-boundaries.md` — boundary parsing is the place where
  partial input becomes total downstream.
