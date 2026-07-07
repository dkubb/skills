# Identity design — before collisions exist

Run when introducing any identity, key, or event kind that matching rules or
replay will consume. Design-time twin of find-and-prove's class E — run the
whole E-class rubric (collision kernel, two-schedule test, identity
coverage) on the PROPOSED design, then these construction rules:

1. **Distinct constructors per event kind from day one.** A shared
   constructor with a discriminating field is a collision waiting for the
   field to be dropped by a projection.
2. **Structured sum-type ids before codecs.** Design the identity as a
   structured type with injectivity/disjointness laws; encoding to
   bytes/hashes is a later, lawful projection — never the design.
3. **Stable, schedule-independent freshness.** Fresh identities are
   deterministic functions of local structure: origin + site + iteration
   ordinals. Static ids break under loops; counters reintroduce
   schedule-dependence. (Pre-empts: replay keys that differ across
   equivalent schedules — the two-schedule test's failure mode.)
4. **Route identity through concrete addresses when the abstract type lacks
   structure.** Tree addresses with injectivity/disjointness laws give the
   namespace laws for free; an unstructured `Nat` makes every distinctness
   claim a bespoke proof.
5. **Enumerate the collision axes the consuming redex matches on**
   (occurrence, owner, branch, position) and give each axis a
   distinguishing field NOW — the "same request, different occurrence"
   witness must be expressible in the type, or the identity cannot separate
   the cases any theorem will later need. Sibling-vs-sibling distinctness
   and new-vs-pre-existing freshness are DIFFERENT obligations; design for
   both, name which is which.
6. **Namespace manufacture at lowering boundaries.** When two owner-scoped
   facts lower into one global store, the lowering MANUFACTURES the
   namespace (derived disjoint addresses, e.g. parity-split slots) — never
   assumes separation, never demands a source-level distinctness field the
   source doesn't need.
