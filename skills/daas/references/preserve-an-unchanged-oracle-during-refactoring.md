# Preserve an unchanged oracle during refactoring

A pure refactor changes implementation or tests in one step, with the other side
unchanged. Keep coverage and mutation sensitivity from regressing.

Changing behavior and the expectation that judges it together removes an
important independent check.

Refactor before a feature when the refactor independently makes the change
easier. If it cannot stand alone, make the behavior change first and refactor
the stable result afterward. Alternate code and test refactors in separately
valid commits when needed.

A necessary simultaneous change to code and behavioral expectations is a Change,
not a pure Refactor; its atomic commit may contain both. A Fix should have a
test that fails without the correction and passes with it; establish that
evidence rather than assuming tooling provides it.

A defect witness can also precede its Fix as an explicitly expected-failure
test: first observe the actual failure, retain a truthful green characterization
checkpoint, then change behavior and expectation together in the Fix.
The `atomic-changes` skill owns commit mechanics; the test runner determines
how pending tests execute. This is not permission to hide an unexecuted test
as evidence.

An incumbent oracle is useful evidence, not infallible truth. If replacement
work reveals a defect, repair the still-used incumbent to establish an honest
comparison baseline; if the replacement is already live, a forward fix may be
appropriate. Keep the behavioral fix distinct from the pure refactor. Dan
usually retires the old system after parity and replacement, rather than paying
to maintain two systems indefinitely.
