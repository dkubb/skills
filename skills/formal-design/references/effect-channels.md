# Effect channels & invariants — the two pre-flight sets

Run set A before replay-checking any effect channel; run set B before
designing ANY substrate invariant theorem. Each question pre-empts the
review finding named in parentheses.

## A — replay-checking an effect channel (8 questions)

1. Where exactly is the boundary — which step consults the supply?
   (Pre-empts boundary-vs-event naming overclaims.)
2. What is the success criterion at that boundary?
3. What is the FATE of every returned observation/field — delivered,
   decoded, stored, or explicitly refused? **"Ignored" is never an allowed
   answer**; any dropped field is a candidate silent channel — pin it with
   a constructor + witness.
4. Does refusal halt OPAQUELY (no diagnostic side channel — find-and-prove
   I2)?
5. Does the supply advance ONLY on success? (Advance-on-refusal
   double-consumes under retry.)
6. Does the trace say exactly what was ACCEPTED (never the refused
   attempt)?
7. What is the SMALLEST boundary fixture (reaches the boundary, stops right
   after — find-and-prove D2)?
8. Which mutant does each new theorem kill? (No named mutant → not done.)

When a refusal lives on the SUCCESS constructor of a sub-call (a match with
the wrong shape), state the single-seam theorem over the SEAM (the call
site), not the call's error tag.

## B — any substrate invariant theorem (8 questions)

1. What is mutable design surface vs fixed background? (Undeclared →
   irredundancy judgments are undefined — find-and-prove G1.)
2. Is this a master theorem, a corollary, or a witness?
3. What UNIQUE mutant does it kill?
4. Which existing headline would it derive from?
5. Does it stop at its boundary (no premise about later steps — D2)?
6. Is it stated on the public or internal surface (G3)?
7. Does the name match the statement (C-class)?
8. Does it fail LOUDLY when a new field is added (B9 — exact-object
   conclusions)?
