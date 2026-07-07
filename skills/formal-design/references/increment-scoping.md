# Increment scoping — sizing and splitting rungs

Run before committing to the next slice's scope. Each question pre-empts a
named failure mode.

1. **Is my smallest increment a concrete INSTANCE of the new freedom, or the
   freedom itself?** (Pre-empts: shipping the general mechanism before any
   instance forces its shape.) One degree of freedom per rung; the instance
   comes first.
2. **Split a bundled rung by its HARDEST dependency.** Pull the hardest piece
   (the one depending on knowing every rule) into its own rung — and land a
   tiny per-step helper as each new rule lands so the later fold doesn't
   force a rewrite of all step definitions. Ship the smallest PROOF-FORCING
   piece first (the one that removes an artificial fixture and forces the
   condition later rungs depend on).
3. **Defer the TRANSFORM, never the DATA.** The persistent fact carries the
   raw datum so the next rung is additive; deferring the datum forces a
   carrier change later (a rewrite, not an extension).
4. **"Additive later" requires the extension point NOW.** A claim
   "constructor X is added later, non-breaking" is FALSE for an inductive
   whose current shape can't express X: adding a constructor changes the
   eliminator and breaks exhaustive matches. To claim both "X is additive"
   and "the current case doesn't use X", the type must already carry the
   skeleton (a closed kind tag + children; current case a childless leaf) OR
   include the recursive constructor now and PROVE the fixture never builds
   it. A headline can only reject mutants its types can express — widen the
   type or drop the rejection claim.
5. **Add-and-coexist; retire only after proven out.** New and old shapes
   coexist until the new one carries the proofs; retirement is a separate
   rung.
6. **The retirement-is-a-mirage test** (run before booking any
   retirement/minimal-basis cleanup rung):
   1. Is the DEFINITION layer already de-duplicated (one unified def + `rfl`
      equalities per instance)? Then there is no definitional debt to
      retire.
   2. Is the STATEMENT layer already the quantified surface with the proof
      dispatching (`cases i <;> exact <landed>`)? Then the interface
      factoring is done.
   3. Can the dispatch be replaced by a direct proof that does NOT `cases`
      the index? For a dependent large-elimination index the answer is
      almost always NO — the dispatch is forced, the per-branch proofs are
      irreducible, and retirement is a MIRAGE. Book it as "coexistence is
      thin by construction", not deferred cleanup.
7. **First-consumer boundary test.** Which rule INSIDE the system first
   READS the new fact? If "none yet", the rung is STORAGE/BINDING, not
   routing/consumption — name it honestly ("await-to-SSA binding", not
   "value routing").
8. **False-dependency-direction.** Is "A before B" logic, or an artifact of
   planning order? A dependency that only exists because the plan was
   written in that order is a free reordering opportunity — and an invented
   constraint if left unquestioned.

Cross-references: the general increment discipline is `atomic-changes` /
`story-change`; this module is its formal-system specialization. The
review-time counterpart is find-and-prove's class D (mis-scoping).
