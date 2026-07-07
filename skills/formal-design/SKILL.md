---
name: formal-design
description: >-
  Pre-flight decision procedures run BEFORE authoring a slice/rung of a formal
  (Lean or similar) system: identity design, carrier/representation choice,
  primitive forcing, increment scoping, seam/interface extraction, effect
  channels, recon discipline, and proof-engineering forward hygiene. Each slice
  kind has a keyed question set with the failure mode each question pre-empts —
  answering them before building prevents the defects the review-time sibling
  (find-and-prove) would otherwise catch after the fact. Use when planning the
  next increment of a formally-proven system, choosing between design
  candidates, or deciding whether a design round is needed at all.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-07-v1"
  type: design-methodology
---

# Formal-design — pre-flight decision procedures for formal slices

Attacking an artifact (review-time) and choosing a design (build-time) are
different activities with different triggers. This skill is the build-time
sibling of `find-and-prove`: run the matching question set BEFORE authoring a
slice, so the mutants the review hunts for are never born. Every question is
keyed to the failure mode it pre-empts.

## Dispatch: what kind of slice is this?

| Slice kind | Trigger | Module |
|---|---|---|
| New increment / rung | any next-slice planning | `references/increment-scoping.md` |
| New primitive / instruction | "the IR needs a new operation" | `references/new-primitive.md` |
| New identity / key / event kind | anything consumed by matching or replay | `references/identity-design.md` |
| Carrier / representation choice | "where does this value live?" | `references/carrier-representation.md` |
| Interface / seam extraction | factoring duplicated theorems | `references/seam-extraction.md` |
| Effect channel / invariant theorem | replay-checking, returned values | `references/effect-channels.md` |
| Before any of the above | verify the machinery a spec names | `references/recon.md` |
| While building | lemmas the NEXT rung will need | `references/proof-hygiene.md` |

Multiple kinds can apply to one slice — run every matching set. A design
round may be SKIPPED only under the criterion in `references/recon.md`.

## Shared principles

- **One degree of freedom per rung.** The smallest honest increment exercises
  exactly one new freedom; everything else is a later rung.
- **Defer until forced — and discharge "forced" rigorously.** A primitive or
  layer is forced only by an expressiveness proof
  (`references/new-primitive.md`), never by a vibe.
- **A headline can only reject mutants its types can express.** "Additive
  later" claims need the extension point now
  (`references/increment-scoping.md`).
- **Answers are witnesses.** Each question set asks for a concrete artifact
  (a two-state distinguisher, a named consumer, a compiled countermodel) —
  a prose answer is not an answer.

## References (load on demand)

- `references/increment-scoping.md` — sizing and splitting rungs.
- `references/new-primitive.md` — when a new primitive is forced and what
  shape it takes.
- `references/identity-design.md` — designing identities before collisions
  exist.
- `references/carrier-representation.md` — route-vs-encode, representation
  boundaries, carrier-swap bridges.
- `references/seam-extraction.md` — extracting interfaces without changing
  the theorems they abstract.
- `references/effect-channels.md` — the pre-flight sets for effect channels
  and substrate invariants.
- `references/recon.md` — API recon before build; when to skip a design
  round.
- `references/proof-hygiene.md` — forward-looking proof engineering.

## Relationship to other skills

- `find-and-prove` — the review-time sibling: it attacks what this skill
  helps you author; its binding rubric names the defect classes these
  question sets pre-empt. Run it on the built slice afterwards.
- `state-space-minimization` (+ `-formal`) — encoding order, constructive
  encoding, normalization, dependent-index encoding; carrier questions that
  are really state-space questions route there.
- `atomic-changes` / `story-change` — the general increment-sequencing
  discipline; increment-scoping here is its formal-system specialization.
