# Blind-reproduction validation — six tier-1 lenses

Protocol (find-and-prove adjudication rule 9): a fresh subagent receives a
filter-safe puzzle with a planted defect (synthetic equivalent of the
pre-fix state) plus ONLY the lens text under test — never the answer — and
must produce the known finding on attempt 1. All six runs below were
attempt-1 matches (run 2026-07-07, one fresh general-purpose subagent per
lens, no filesystem or web access).

## 1. B3 Cardinality conjunct — MATCH

- Puzzle: multiset invariant `∀ t ∈ bag, t = cert(k0,x) → x = v0` documented
  as "exactly one certificate for k0 with value v0"; add-only `issue` rule.
- Planted: the two-copy forgery (`∀`-uniqueness reading).
- Result: found the deciding two-copy witness
  `{cert(k0,v0), cert(k0,v0)}` reachable via double-issue, PLUS the empty-bag
  existence gap; corrected invariant as a first-class multiplicity count
  with the not-in-rest caveat. Exceeded the planted finding.

## 2. B5b Existential subject — MATCH

- Puzzle: `∃ cfg, Reaches P cfg ∧ (build P).atoms slot7 = some 42 ∧
  (build P).halted = true`, proof picks `cfg := build P`.
- Planted: two conjuncts subject-bound to the ground helper.
- Result: identified both dangling conjuncts, gave the consumer-type
  failure (no `cfg = build P` in the type; equivalent to a factored
  statement), noted mutation-blindness (defeq at the witness), and produced
  the exact syntactic fix with the unchanged proof.

## 3. C2 N-distinct arity audit — MATCH

- Puzzle: `four_children_distinct` asserting 4 of C(4,2)=6 inequalities;
  docstring taxonomy "same-side or cross-side".
- Planted: the two missing same-parent sibling pairs (the original Fable
  finding this lens graduated from).
- Result: counted 4 vs 6, named both missing pairs, built the
  `left = right = id` satisfying violator (only two distinct strands), gave
  the six-conjunct correction, and flagged that the sibling conjuncts need
  a per-parent injectivity fact — the same upstream note as the original.

## 4. A3 Annotation-constant sweep — MATCH

- Puzzle: labeled step relation with erasure + lifting + single-valuedness
  + halt-none + determinism; docstring "the event log is faithful".
- Planted: no theorem reads the label back.
- Result: ruled VACUOUS; noticed the all-`none` constant is blocked by the
  halt-none theorem and correctly switched to the constant-`some ε₀`
  countermodel (a stronger version of the planted finding); per-theorem
  check matched the lens's erasure/lifting/single-valuedness analysis;
  repair = the `e = eventOf G G'` read-back law subsuming halt-none.

## 5. E2 Two-schedule test — MATCH

- Puzzle: `parents(e) := H` (serial history) with replay keyed on parents;
  independent events may reorder.
- Planted: prefix log wearing a causal label.
- Result: executed A;B vs B;A, showed both parent sets differ purely by
  serial order, named the replay-key breakage (spurious divergence under
  valid reschedules), verdict "prefix-log", fix = read-from/frontier
  parents with order in the payload — the lens's exact prescription.

## 6. A7 Behavior-drop mutants — MATCH

- Puzzle: emit_ok/emit_halt rules + positive witness + mismatch witness +
  `step_exact_cases`; docstring "every divergence halts".
- Planted: the hidden-premise narrowing (`why = intentMismatch only`).
- Result: constructed exactly that mutant, argued per-theorem survival
  (classification theorems classify the steps that EXIST; the narrowed
  premise makes the master theorem easier), named the lost behavior
  (exhaustion = silent stuck state), fix = the disjunctive/universal
  broadened witness keeping the basis at three theorems — the same
  irredundancy-aware fold as the original N5B fix.

## Verdict

6/6 attempt-1 reproductions. Two runs exceeded the planted finding (B3's
existence gap; A3's constant-`some` refinement), consistent with the lens
texts carrying enough of the technique to generalize, not just replay.
