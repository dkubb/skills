# Make real dependencies explicit

Encode genuine coupling in types, signatures, interfaces, and known dependency
relationships. Minimize unnecessary dependencies, not evidence of dependencies
that actually exist.

A schema or dependency does not disappear because it is implicit. Explicit
relationships let tools identify which assumptions a change breaks.

Genuine dependencies can justify tight coupling. A deliberate trait boundary can
change what a consumer depends on without pretending it has no dependency.

Similar-looking types in two domains are not automatically one concept. Examine
callers, names, tests, and data flow before sharing them; without evidence of
sameness, retain the distinction. Separate implementations can share a
consistent shape for comprehension without sharing code or domain identity.

Make the dependency graph truthful. A foreign key represents an existing
dependency rather than inventing coupling; align identifier representations so
the database can enforce the relationship. Put necessarily coupled writes at the
owning database boundary. Prefer an explicit database operation whose body shows
both writes when callers can use it; a trigger can enforce the coupling in a
legacy system whose callers cannot yet be migrated. Explicitness must not leave
a required invariant dependent on application convention alone.

Push an operation toward the lowest capable layer that owns the data. Prefer a
coherent relational operation for database reads, transformations, and writes,
with explicit inputs and outputs that can be tested directly. Keep input
admission and external I/O in the application where they belong. Treat database
calls as RPCs: batch meaningful work instead of repeatedly fetching, stitching,
and writing one item at a time. Minimize round trips and unnecessary transaction
or lock duration, while preserving the actual transaction contract; a round-trip
count alone is not evidence of atomicity. External I/O followed by persisting
its result can legitimately require separate database interactions.

Known dependencies should remain visible in planning and documentation as well
as code. Implementation will expose missing prerequisites; pause the dependent
work, complete the prerequisite, then resume from a valid state. An incomplete
plan is revisable knowledge, not automatically a catastrophic failure. Keep
unfinished work only while its likely value justifies its actual carrying cost.
