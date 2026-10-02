# Keep the engineering system small and evolve it explicitly

Start with faithful prose and the minimum useful structure. A rule needs a name,
what it says, and why. Add required metadata when use demonstrates its value,
then backfill existing records.

A speculative process schema creates the same liabilities as speculative
application code.

Make changes to a governing contract explicit even when its representation stays
the same. Changes to intent, expectations, interpretation, ranking, or assurance
can change the model's meaning without changing configuration syntax. Version
that changed meaning so consumers can identify the governing contract. Pure
editorial work or a repair preserving the intended contract is different.

Pursue formal modeling, deterministic tooling, and an executable SDLC as they
earn their cost. This direction requires implementation and verification;
describing machinery does not establish that it exists.

Where the team has authority to migrate all consumers, prefer a bounded
compatibility window and convergence on the current version. Give deprecation
feedback at compilation when practical, otherwise at runtime, and provide the
migration before retiring the old line. Convergence reduces fragmented support
and concentrates real usage feedback on the version being improved. An internal
ecosystem and a public framework have different authority and obligations. One
final release of an old version can be a bounded support choice; it is not a
universal deadline.

Consumers may depend on observable behavior even when it was not promised.
Characterize that behavior so a change is deliberate and visible; this empirical
risk does not itself create an indefinite support obligation. Honor the
compatibility and support promises actually made. An incidental observable
behavior is not automatically a promised contract; a consumer's assumption does
not silently expand that promise. Voluntary accommodation is a separate choice.
When all consumers are under your control, prefer convergence over compatibility
machinery unless its temporary value justifies it. Public and commercial
obligations follow the commitments made to those consumers.
