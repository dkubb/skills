# Information flow — defect class I (confidentiality: the leak lenses)

Load when the target's guarantee is confidentiality — what can the role
observe or distinguish? These are the oracle-hunt's leak lenses (the
operational core of decision-oracle extraction) plus the representation-seal
lenses. The Lean-specific attack drill for this class is
`references/lean-robustness.md`.

## I1 Public-result partition / decision oracle

A public result type with constructors `c1…cn` induces a *partition* of
hidden states; derive it, then ask whether the role may learn it. Witness =
two hidden states differing only in the protected fact that produce different
observations. (Replay's `ok / intentMismatch / recordExhausted` is not "an
error type" — it computes `head.intent = actual` and `trace_nonempty`.)

## I2 Error algebra / diagnostic side channel (the padding-oracle family)

Every distinct failure reason is a declassification, and cross-run adaptivity
compounds bits-per-query into full recovery (Vaudenay's padding oracle;
Bleichenbacher). Prove the role may learn each distinction, or collapse
candidate-facing errors and keep rich diagnostics offline. Attack `not-found`
vs `forbidden`, `mismatch` vs `exhausted`, timeout vs denial, parse-error vs
auth-error, stale-version vs nonexistent.

## I3 Non-interference (two-run / low-equivalence)

**Non-interference** as the two-run / **low-equivalence** theorem
(`LowEq role s1 s2` ∧ secrets vary → low observations equal ∧ next states
`LowEq`), proved by **relational Hoare / self-composition / product programs**;
**unwinding** is the per-step form. **QIF / min-entropy** (Smith): `k`
distinguishable outputs is a *crude* `log2 k` upper bound, distribution-
dependent — constructor count is a smell, not a measurement.

## I4 Chosen-prefix oracle / active automaton learning

If the adversary drives execution to a boundary, each public tag after that
prefix is a membership query about the hidden trace/policy/state machine.
Generalizes cross-run replay; catches trace-length, branch-shape,
hidden-policy probing.

## I5 Representation-seal reality — is the seal real?

- **ADT abstraction-barrier leak / public elimination-surface exposure** (the
  precise name for the `casesOn` bug — the type was never abstract). Audit
  recursors, projections (`.1`, parent `toParent`), `noConfusion`, **deriving**
  (`Repr`/`BEq`/`DecidableEq`/`Ord`/`Hashable`/`SizeOf` — each an observer),
  instances + coercions, **`import all`**, reducible aliases, `autoImplicit`.
  Drill in `references/lean-robustness.md`.
- **Least-authority representation** — store the **capability** (a `step`
  closure), not the secret, so a total representation leak yields only the
  capability. Pair with terminality + affine use.
- **Contextual equivalence / refinement** — could *any* context distinguish the
  sealed handle from a non-leaking ideal? (Refinement/simulation by default; full
  abstraction only for hostile linking.)

## I6 Declassification & endorsement discipline

**Evaluator / provenance poisoning** (record-now-judge-later) — can a candidate
produce a trace that's *safe as execution* but *misleading as evidence*
(labels, region boundaries, version mapping, divergence spans, check
identities, corpus, evaluator prompts) while staying within hard stops?
Canonical home: **specification gaming / reward hacking** and the Goodhart
taxonomy — optimize the *measure* (the trace as evidence) while respecting
the *metric*'s hard stops. Endorsement discipline (transparent endorsement,
the integrity dual of robust declassification): attacker-influenced data
must not launder into trusted evidence.
