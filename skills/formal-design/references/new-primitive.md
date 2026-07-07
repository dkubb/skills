# New primitive — when it is forced and what shape it takes

Run before adding any primitive/instruction to a core language or IR.

1. **Prove the primitive is FORCED by an expressiveness argument, not a
   vibe.** Pick two states that AGREE on everything the current primitive
   can read but DIFFER elsewhere; if the current primitive must produce the
   same output while the desired behaviour needs different ones, the
   primitive is genuinely forced. The two-state distinguisher is a NECESSITY
   proof — this is how "defer until forced" is discharged rigorously.
   (Pre-empts: speculative primitives that solve no expressiveness gap.)
2. **A forced primitive GENERALIZES an existing seam at higher arity —
   never adds a capability or changes what existing structure means.**
   Reject the option that makes an existing store mix two meanings
   (observations vs internally computed values), forcing provenance
   distinctions, overwrite rules, and stale-aggregate lemmas. Smallest
   honest increment = widen the existing seam; biggest smell = a new
   capability that breaks an existing invariant ("atoms written only by
   await").
3. **Keep the fixture/layout concern OUT of the primitive.** The primitive
   reads a generic finite ordered vector of slots; a FIXTURE HELPER says
   which slots. If the layout changes, the helper changes, the IR doesn't —
   and count-awareness becomes structural (the construction cannot read
   stale slots the vector doesn't contain).
4. **First-order closedness.** A new syntactic token stays first-order:
   closed data + a trusted fixed evaluator, NEVER a function field (a
   `Vector Atom n → Req` field breaks candidate-closedness). Growing a
   closed language = add a closed token and extend the closedness machinery
   over every match — defunctionalize; no callbacks.
5. **Pin the FULL identity, not the convenient projection.** Request
   construction pins the canonical request identity (cap / adapter / op /
   args / authority / parents) because replay keys on it; a bytes-equality
   corollary is never the headline.

The review-time counterparts: find-and-prove B2 (producer pin) and E-class
(identity) attack what a mis-designed primitive produces.
