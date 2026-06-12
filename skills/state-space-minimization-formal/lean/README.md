# Lean reflection of the state-space-minimization calculus

A Lean 4 + mathlib rendering of the core calculus in `../SKILL.md`.

## Purpose

The formalization serves the skill in both directions:

- **It tightens the skill.** Lean forces commitments the prose notation
  can leave implicit — universe choices, embeddings, quantifier domains,
  whether a premise is dischargeable at all. Every place the translation
  had to invent, surrender, or disambiguate is a candidate finding
  against `../SKILL.md`. Findings discovered here flow back into the
  skill as ordinary skill-refinement findings.
- **It proves the skill.** Rules that the prose asserts become theorems
  the type checker discharges (for example,
  `trusted_boundary_reaches_valid` and
  `trusted_boundary_no_reachable_invalid` prove that an
  evidence-carrying constructor yields `R(b) ⊆ C(A)` and
  `I_reach = ∅`). A green build is machine-checked evidence that the
  reflected fragment of the calculus is coherent.

## Naming parity

Lean identifiers mirror the calculus's names wherever the calculus
names a thing (`I_repr`, `I_reach`, `R`, `Strict`, `Sufficient`,
`Obl`), so an LLM holding either representation can connect the dots
to the other. Where Lean forces a name the calculus leaves implicit
(for example `ContractPinned`, from the SKILL.md gloss "the contract
is pinned"), prefer a name derived from the skill's own prose — and
treat a good coined name as a candidate to back-port into the
calculus. Renames on either side are vocabulary drift and should land
on both sides or not at all.

## Version parity

The reflection is stamped with the calculus version it was derived
from, in two places: the header comment of `Reflection.lean`
(verbatim, e.g. `2026-06-v2`) and the lakefile `version` (mapped to
semver, e.g. `2026.6.2`). When `../SKILL.md` bumps
`metadata.version`, a non-matching stamp here means the reflection is
stale and must be re-derived and re-stamped. Matching versions are
the signal that the two representations are connected.

## Determinant

`../SKILL.md` owns the calculus. This project is a derived reflection:
it is not normative and is deliberately not linked from the skill. If
the two disagree, the SKILL.md is right and this file is stale —
re-derive the reflection after calculus changes.

## Scope

Reflected: state sets, boundaries and the trust theorems, `Strict`,
the selection objective with `Sufficient` and the fallback,
constructive dominance, and `Obl`. Not reflected: reception semantics,
self-similarity, and the proof-preservation rules
(`Needs` / `Erases` / `Flows`) — classified as schematic or meta-level
as stated in the source.

## Provenance

Produced by a Codex (GPT-class) reflection probe of `../SKILL.md` on
2026-06-12, graded by compilation under Lean 4.30.0 + mathlib. Three
mechanical patches were applied to the probe output, each at a spot
the probe itself flagged. Subsequent improvements are logged in
`ITERATIONS.md` (autonomous Codex improvement loop) and graded against
the build plus the no-weakened-theorems rule.

## Building

This directory is a self-contained lake project:

```sh
lake exe cache get   # once; downloads the mathlib build cache
lake build           # type-checks Reflection.lean
```
