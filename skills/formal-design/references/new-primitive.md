# New primitive — when it is forced and what shape it takes

Run before adding any primitive/instruction to a core language or IR.

1. **Prove the primitive is FORCED by an expressiveness argument, not a
   vibe.** Pick two states that AGREE on everything the current primitive
   can read but DIFFER elsewhere; if the current primitive must produce the
   same output while the desired behaviour needs different ones, the
   primitive is genuinely forced. The two-state distinguisher is a NECESSITY
   proof — this is how "defer until forced" is discharged rigorously.
   (Pre-empts: speculative primitives that solve no expressiveness gap.)
2. **Choose the smallest change that expresses the distinction.** Try
   generalizing an existing operation first. List the invariants its current
   callers depend on. For example, allowing a store of observed values to
   hold computed values introduces a provenance distinction. If the new
   operation genuinely introduces authority, specify that contract
   explicitly.
3. **Keep fixture layout outside the primitive.** Derive parameters from
   the fixture at its boundary. For example, an operation can consume an
   ordered vector of slots while its caller chooses their positions.
4. **Preserve first-order closedness where the language promises it.**
   Represent a new instruction as data interpreted by its evaluator. Check
   every exhaustive match. A callback supplied by the caller changes which
   computations the evaluator admits.
5. **Pin the identity the consumer uses.** Name each field used to match
   or authorize the operation. In a replay system this may include the
   operation, arguments, authority and parents. State the identity law at
   that level before deriving properties of an encoded form.

The review-time counterparts: find-and-prove B2 (producer pin) and E-class
(identity) attack what a mis-designed primitive produces.
