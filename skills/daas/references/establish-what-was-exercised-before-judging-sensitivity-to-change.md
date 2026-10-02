# Establish what was exercised before judging sensitivity to change

Before changing an insufficiently tested area, use characterization tests to
record its current behavior. Establish the applicable region, line, branch, and
function coverage before mutation testing, plus MC/DC where supported. The
normal local bar is full applicable execution coverage and no live mutants in
the defined scope. Verify unit and property tests as applicable. Dan usually
brings the code he is about to change to full mutation coverage first,
especially in unstable systems. Establish that baseline before the
behavior-changing commit. If code cannot be covered, establish that it is
genuinely unreachable before removing it; a surviving mutant alone does not
establish that fact.

Mutation testing is less useful when ordinary execution gaps already explain why
changes survive. A characterized baseline makes subsequent changes deliberate
and independently assessable.

Characterizing incorrect behavior records what exists without endorsing it.
Mutation sensitivity protects that observed baseline; it does not make the
observed behavior correct. Preserve a visible semantic delta when changing it,
so accidental reliance and deliberate behavior changes can be investigated.

Ordinarily establish that baseline and then make a separate behavior-changing or
removal commit.

If genuinely unreachable code prevents the characterization and removal from
existing as independently valid steps, the smallest coherent atom may include
both. Do not weaken upstream constraints merely to reach dead code. Tool support
and any exclusions must be explicit; verify current tool capabilities.

Expensive testing is a design signal before it is a reason to test less. Look
for mixed effects and pure logic, hidden dependencies, or a unit boundary that
can be decomposed. Design new code for local verification; strengthen inherited
code as work reaches it. Criticality, deadlines, and competing priorities can
justify a visible exception by making the exception intentional and matching
assurance to criticality, rather than silently treating integration evidence as
a substitute.
