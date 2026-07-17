# Blind-reproduction validation — record-field retention rule (post-suite graduation)

Protocol: find-and-prove adjudication rule 9, run 2026-07-13 before
graduating the catalog's record-field retention entry (harvested at the
symbiote-substrate provenance-capstone rung, PR #85, where Pro's
CODE-round out-find ended a 52-rung clean streak — all three inner review
tiers had missed it). A fresh subagent received a worktree pinned at the
pre-fix state `d43ad3a` and ONLY the technique text: classify every
hypothesis the earning theorem consumes as STORED (a) / DERIVABLE (b) /
OUTSIDE (c) / DEFECT. Target module:
`Symbiote/Substrate/Ports/DrivenReplyReplayProvenance.lean`.

## Result: PASS (attempt 1), matching the Pro finding exactly

- Built the full per-hypothesis classification table for the earning
  theorem `decodedDrivenReply_replay_provenance` over the record
  `DecodedReplyReplayProvenanceAt`: five hypotheses classified cleanly
  ((a) via field/index identity, (b) via one projection or one landed
  iff), isolating exactly one residue.
- Found the Pro finding: `hreplyUnique : replyKeyCount G₀ k ≤ 1` is
  consumed to EARN the record — it is precisely what identifies the
  consumed reply with the producer-injected sealed cell, purchasing the
  word "provenance" — and is neither stored nor derivable (the
  `BuiltFrom` field's caller `base` is existential and arbitrary).
- Proved non-derivability by inhabiting the record in the excluded state:
  a two-same-key-replies construction (caller-forged duplicate in `base`,
  producer-generated copy) satisfies every stored field at
  `replyKeyCount G₀ k = 2` through the record's public constructor.
- Ran the two-state distinguisher independently: graph A (sealed cell
  exclusive) vs graph B (competing caller-forged same-key reply); the
  record's four surfaces bind the same `raw` in both, so a consumer
  holding only the record cannot tell source-binding from
  value-coincidence — the name's reading is TRUE in A, FALSE in B.
- Applied the disclaimer test at case (c): the docstring's disclaimer is
  honest but phrased about the HEADLINE (inviting the checked-upstream
  misreading), and the disclaimed axis is the load-bearer of the name
  "provenance" itself — so (c) is not honestly available and the
  hypothesis lands as a DEFECT with the two prescribed fixes (store the
  uniqueness fact as a fifth field, or descend the name to a shared-raw
  binding record).

## Disposition

Graduated as the retention half of B4 ("Read-back & retention at full
strength", B4b) in `references/pins.md`, with the rubric.md entry and
the SKILL.md skeleton line retitled to match — inside B4 rather than as
a new letter, since it is the converse audit on B4's own surface (B5b is
the lettered-half precedent). Full evidence:
`symbiote-design-rounds/blind-repro-retention-rule.md`. The catalog's
provenance-capstone entry may now be struck (the symbiote agent's lane).
