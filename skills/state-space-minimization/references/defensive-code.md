# Defensive code as state-space expansion

Defensive code is the inverse of narrowing. Each operation in
`principles.md` reduces the representable state space; each
defensive check re-introduces a state the narrowing should have
eliminated. Same vocabulary, same six operations applied as their
inverse — at the level of inserted runtime checks rather than at
the type level.

The two operations are equal-and-opposite, so continuous
application of state-space minimization deletes defensive code as
a natural consequence: once an invariant is carried by the type,
the control flow, or the upstream parser, the check that defended
against its negation has nothing left to do.

We do not write just-in-case code. Every defensive check is a
claim that a representable-but-invalid state can reach this point.
That claim is either true — in which case the type or flow is too
wide and the fix is upstream — or false, in which case the check
is dead code and must be deleted.

## The audit question

For every defensive branch, ask:

> **What specific input would trigger this branch?**

The answer forces an action. There are four possible answers:

1. **An input the type forbids.** Delete the check. Type-level
   invariants do not loosen silently; if you worry they might,
   the worry is the bug, not the missing defense.
2. **An input that was already narrowed earlier in this flow.**
   Delete the redundant check. The upstream narrowing produced a
   value the rest of the flow can trust.
3. **An input from an external boundary that has not yet been
   narrowed.** The check belongs at the boundary, not at every
   internal call site that consumes the value. Lift it. Parse,
   don't validate.
4. *"I don't know."* This is the just-in-case case. Find out
   before keeping the check. If the answer turns out to be (1),
   (2), or (3), apply that. If you genuinely cannot determine the
   answer, the type system or the flow is not carrying the
   invariant — that is the bug to fix, not the missing check to
   add.

Every defensive branch must have an answer from this list. A
branch whose author cannot answer is a state-space leak.

## Sources of upstream narrowing

A defensive check is redundant when an upstream narrowing already
guarantees the state the check defends against cannot reach this
point. Three sources of upstream narrowing:

### Type-level narrowing

The type at this position excludes the state being checked.

- `if (x === null)` on a non-nullable type.
- `match (e) { Variant::A => …, Variant::B => …, _ => … }` with
  a wildcard arm that no real variant can reach.
- `unwrap` defended by a runtime check on a value the type
  already proves non-error.
- A bounds check against a length the array's type forbids.

The type is the proof; the runtime check duplicates a proof the
compiler already has.

### Flow-level narrowing

Control flow earlier in the same function or call chain has
already established the predicate the check is testing.

- After `if x is None: return`, no further `if x is None` is
  needed in the same scope — the early return already eliminated
  the `None` path.
- After a guard that returns early on empty input, no further
  empty check is needed.
- After a predicate has been verified at the start of the call
  chain, no re-verification is needed at sites that can only be
  reached by passing the verification.

A re-check downstream is either redundant or evidence that the
flow is not as constrained as the author assumes — in which case
the flow is the bug, not the check that defends against it.

### Boundary-level narrowing

The value was parsed, validated, or filtered at an ingress
boundary upstream, and the result is a narrower type or a value
with a documented post-condition.

- A string matched against a regex at parse time does not need
  re-matching downstream.
- A user verified as authenticated at the request handler does
  not need re-checking at every service method downstream.
- A row whose constraints were validated by the database does not
  need its constraints re-checked in application code.

The narrowing did its job once at the boundary. Downstream
re-checks duplicate that work and widen the function's apparent
contract.

## try / catch as just-in-case

Wrapping a call you don't know can throw is a state-space
expansion. The wrapper says to readers "callers must consider an
exception path here" even when there is none. The function's
codomain widens to include exception flow that does not exist in
the contract.

The audit is the same:

- *What specific exception type can this call throw, on which
  input?* No specific answer → no catch.
- If a specific exception can be thrown, catch that specific
  type and handle it. `catch (...)`, `catch (Exception e)`,
  `except:` are the maximally-wide forms — they admit every
  exception including bugs that should propagate (assertion
  failures, memory errors, contract violations in the library
  being called).

Two anti-patterns specifically:

- **Try/catch around code that demonstrably cannot throw.** A
  pure computation, a `Result`-returning function called via `?`,
  a typed function with a total signature. The catch will never
  fire, and every reader has to consider it.
- **Catch-all that converts every exception to a default value.**
  Hides bugs in the called code, including bugs that would
  otherwise have produced a fast, loud failure.

The narrowing for exception handling is: declare what *can* throw
and catch only those. Checked exceptions, `Result`/`Either`, and
`throws` annotations enforce this at compile time; languages
without them require the discipline manually.

## Optional chaining and null-coalescing on non-nullable values

`x?.foo` when `x` cannot be null, `x ?? default` when `x` cannot
be undefined, `x !== null && x.foo` after a check that already
narrowed `x` to non-null — same audit, same forced action. The
operators exist because some values can be null; using them where
the value cannot be widens the reader's state space because the
reader has to consider a null path that does not exist.

## Where defensive code survives the audit

The cases below are not just-in-case code; the check is
load-bearing because the alternative is a real failure mode the
type system cannot exclude.

### Trust boundaries

Input from outside the program — HTTP requests, file system
reads, database rows, environment variables, CLI args, FFI
returns, network responses, IPC messages, untrusted serialized
data — is wide by default. Narrow it once at the boundary. The
check there is not "defensive" — it is the parser, the smart
constructor, the validator that produces the narrow type. Beyond
the boundary, no further checks are needed for the same
invariant.

### Invariants the type system cannot express

When the invariant is real but unexpressible in the type system —
time-varying properties (a lease that may have expired since the
last check), capability tokens (a user banned since the last
check), cross-system invariants (a row in a different database) —
the runtime check is the only mechanism that can enforce it.
Place it where the invariant is needed, and add a one-line
comment naming the invariant being checked. See
`ingress-and-boundaries.md` for capability-token patterns that
lift many of these into types.

### Platform-level failure modes

The language and runtime cannot promise that memory allocation
succeeds, that a network call returns, that a file descriptor is
still valid, that a thread is not interrupted, that an FFI call
returns sane data, that an `unsafe` block honors its contract.
The type system stops at the platform boundary; defensive code
beyond it is necessary.

Even here, the check should be specific. `try { allocate(...) }
catch (OutOfMemoryError e) { … }` is specific to the platform
failure it expects. `try { … } catch (Throwable t) { … }` is not.

## Examples of just-in-case code to delete

```text
# Nil check after a type that excludes nil
fn process(user: &User) {
    if user == nil { return; }    // delete: type forbids nil
    …
}

# Regex re-match after an upstream parser already matched
fn handle(email: ValidatedEmail) {
    if !EMAIL_REGEX.is_match(email.as_str()) { return Err(…); }
                                       // delete: ValidatedEmail
                                       // proves the regex match
    …
}

# Predicate re-verification after a control-flow guarantee
fn after_login(session: ActiveSession) {
    if !session.is_active() { return; }   // delete: the type IS
                                          // the active session
    …
}

# Re-check inside a same-scope guard already taken
fn process(maybe: Option<User>) {
    if maybe.is_none() { return; }
    let user = maybe.unwrap();
    if user.id.is_none() { return; }  // if user.id is non-nullable
                                      //   by type, delete this
    …
}

# try / catch with no specific failure mode known
try {
    return pure_function(x);          // pure; cannot throw
} catch (Exception e) {
    log("unexpected"); return default; // delete: there is no
                                       // exception path
}

# Optional chaining on non-optional fields
const len = user?.name?.length;       // if user and user.name
                                      // are required, drop the ?
```

In every case, the right move is *upstream*: fix the type, fix the
flow, or fix the boundary parser. Deleting the local check without
fixing the upstream invariant turns the defensive code into a
silent bug if the upstream is actually wrong.

## Discipline

- For every defensive check in a code review, attach an answer to
  the audit question. No answer is a finding.
- Treat catch-all clauses (`catch (...)`, `catch (Exception)`,
  bare `except:`) as state-space-expansion sites by default.
  Narrow to specific exception types or delete the wrapper.
- When deleting a defensive check, confirm the upstream narrowing
  is real. If the type, flow, or boundary does not yet carry the
  invariant, fix that first, then delete the downstream check.
- Treat the audit as continuous: each time SSM is applied
  upstream, downstream defensive checks become deletable. Trail
  the narrowing with deletions; do not leave them behind.

## Cross-references

- `principles.md` § "Three roles a function can play" — defensive
  code is the negation of "eliminates"; it claims an invalid
  state exists past a boundary that should have removed it
- `testing.md` § "Test corpus as production specification" —
  defensive branches against impossible inputs grow when wrong
  tests bypass the production constructor; the chain that produces
  them and the test-side discipline that prevents them
- `total-functions.md` — defensive code is what a partial function
  turns into when the caller does not narrow the input; lifting
  the narrowing to the type makes the function total and the
  defenses go away
- `least-power.md` — wider input types force more defensive
  branches; tighter types remove them by construction
- `ingress-and-boundaries.md` — where defensive code legitimately
  lives (at the boundary) and capability-token patterns that lift
  runtime checks into types
- `documentation.md` — vague qualifiers like "may throw" or
  "various exceptions" are the documentation analog of catch-all
  defensive code; same audit applies to prose
- `proof-preservation.md` — when a typed proof flows through
  conversions without being re-validated, downstream defensive
  checks become unnecessary
