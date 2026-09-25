# Make behavior deterministic wherever possible

Given the same complete inputs and state, expect the same result. Investigate
unexpected nondeterminism as a defect. Introduce it deliberately when its
benefit justifies surrendering the known deterministic baseline. For creative
search, constrain and evaluate candidates so useful outliers can be retained and
poor ones rejected; generated novelty is not its own acceptance evidence.

Isolate unavoidable nondeterminism at boundaries. Record external observations
and reuse them as fixed inputs for replay, so the system's own behavior can be
reproduced and debugged. A bug fix may intentionally change the failing
trajectory while preserving unaffected behavior.

Rich recording and replay infrastructure still needs a concrete requirement. Dan
often works in systems where recovery or diagnosis supplies that need; frequency
in his projects does not make the machinery mandatory everywhere.
