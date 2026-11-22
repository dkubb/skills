---
name: formal-design
description: >-
  Plan a formal system increment before implementation. Use when choosing a
  representation, identity, primitive, interface, or proof obligation.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-07-v1"
  type: design-methodology
---

# Formal-design — pre-flight decision procedures for formal slices

## When to Activate

Use this skill while planning a formal model, choosing a representation, or
extracting an interface from proven instances.

## When Not to Use

For adversarial review of an existing artifact, use `find-and-prove`.

## Inputs

Bring the proposed change, its first consumer, the contract it relies on,
and the existing definitions and theorems it would extend.

## Outputs

Record the design choice, the witness that distinguishes it from the other
candidates, and the first increment that can establish the claim.

## Utilities

Use the reference modules in the dispatch table for the matching questions.
Use the project's proof checker to verify declarations and witnesses.

## Process

Start with [recon](references/recon.md). Check the named declarations and
their types. If reviewed components already determine every design choice
and their composition introduces no new correspondence claim, proceed to
implementation. Otherwise run every applicable question set below.

| Slice kind | Trigger | Module |
|---|---|---|
| New increment / rung | any next-slice planning | `references/increment-scoping.md` |
| New primitive / instruction | "the IR needs a new operation" | `references/new-primitive.md` |
| New identity / key / event kind | anything consumed by matching or replay | `references/identity-design.md` |
| Carrier / representation choice | "where does this value live?" | `references/carrier-representation.md` |
| Interface / seam extraction | factoring duplicated theorems | `references/seam-extraction.md` |
| Effect channel / invariant theorem | replay-checking, returned values | `references/effect-channels.md` |
| Resource / authority | splitting, spending, delegating, or composing a bounded resource | [Resources](references/resources.md) |
| Confidentiality / seal | a caller can observe results derived from private state | [Confidentiality](references/confidentiality.md) |
| While building | lemmas the NEXT rung will need | `references/proof-hygiene.md` |

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

## Validation Checklist

- The scoped increment exercises one new degree of freedom.
- Each applicable question has a witness or a named open obligation.
- The first consumer can distinguish the chosen design from its alternatives.

## Relationship to other skills

- `find-and-prove` — the review-time sibling: it attacks what this skill
  helps you author; its binding rubric names the defect classes these
  question sets pre-empt. Run it on the built slice afterwards.
- `state-space-minimization` (+ `-formal`) — encoding order, constructive
  encoding, normalization, dependent-index encoding; carrier questions that
  are really state-space questions route there.
- `atomic-changes` / `story-change` — the general increment-sequencing
  discipline; increment-scoping here is its formal-system specialization.
