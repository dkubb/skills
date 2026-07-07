# Proof-engineering forward hygiene — build for the next rung

1. **Pre-prove the induction-shape lemmas the next rung's fold needs** —
   `_cons`, `.mono`, append/`snoc` forms — while the definitions are fresh;
   retrofitting them mid-fold costs a detour per lemma.
2. **Export the transport lemma a planned swap will need NOW** (the
   map/commute seam, stated explicitly and kept general) — never inline
   `cases h; rfl` constant-family shortcuts that a later carrier change
   cannot reuse.
3. **Named law over defeq `rfl` at module boundaries.** A boundary proof
   that holds by `rfl` is representation-fragile: the next representation
   change silently breaks every client that relied on the definitional
   coincidence. State the law with a name; prove it by `rfl` today if you
   like — the NAME is the seam.
4. **Make `∃`-hidden fields public one consumer ahead.** When a second
   consumer of a hidden field is on the roadmap, expose it before that rung
   — preferring the single witness-pinning equation (`cfgF = <concrete>`)
   over accumulating per-field conjuncts once multiple fields are
   load-bearing (find-and-prove B9).
5. **Prune to fixpoint.** Run-construction proofs that land syntactically
   on their prefix definitions accumulate redundant unfolding rewrites in
   LAYERS — any simp/rw-prune pass must iterate per file until a full pass
   deletes nothing.
