# Terminology for progressive refinement

This reference gives the ASD-STE100 Issue 9 category of each technical noun and technical verb in the skill.
This reference is the source of the definitions.
The skill uses each term only for the item that its definition shows.

Rule 1.5 gives the categories of technical nouns. Rule 1.12 gives the categories of technical verbs.
Rule 1.6 lets you use a word that is not approved in the dictionary when that word is part of a technical noun.
This condition applies to "review", "evidence", "attempt", "base", "delivery", "failure", and "waiver".
The terms of ASD-STE100, for example "technical noun", "rule", and "quoted text", are technical nouns in category 15.

## Definitions

- **Goal:** The result that all the selected work must supply
- **Intent:** The purpose of the work (different from the goal)
- **Initial intent:** The intent before the work started
- **Governing intent:** The intent, invariants, and constraints from the user, the repository, and the applicable skills that control the work
- **Invariant:** A condition that must stay correct in all of the work
- **Constraint:** A limit on the work that the user can accept
- **System constraint:** The condition that sets the limit on accepted progress of the system
- **Gate:** A check with specified inputs and acceptance criteria
- **Gate contract:** The gates of a repository and their sequence for each maturity stage
- **Maturity stage:** A step in the procedure to get assurance, for example development or publication
- **Checkpoint:** Recorded work that its gates accept for its maturity stage and that you can put back into use
- **Batch:** Work that you keep together at one checkpoint
- **Piece:** A selected change for extraction
- **Candidate:** Work that you prepare for a gate or a review
- **Prerequisite:** A change or input that must be available before a different change or task
- **Dependent:** A change that has a prerequisite
- **Dependency:** The relation between a prerequisite and its dependent
- **Independent work:** Work that has no dependency on the other selected work
- **Shared prerequisite:** A prerequisite of two or more pieces
- **Placement:** A piece together with its selected prerequisites in one sequence of commits
- **Placement test:** The gates that you do on one placement
- **Attempt:** One extraction that you try, with its recorded inputs
- **Extraction:** The removal of a piece from the remainder into a different commit
- **Mechanical extraction:** An extraction that keeps the target the same
- **Remainder (D):** The changes that are available for extraction
- **Target:** The result of all the work together that a mechanical extraction must keep the same
- **Conservation:** The condition in which the pieces and the remainder together give the target
- **Integrated result:** The result of all the selected commits together for a review or a delivery
- **Slice:** A group of changes that you can merge together
- **Review slice:** A slice that includes each of its prerequisites that are not in the base
- **Standalone slice:** A slice that agrees with all the conditions in "Conditions for a standalone slice"
- **Delivery:** An accepted result that you supply to the user
- **Cancellation test:** The decision if the user keeps a slice when the user cancels or reverts the subsequent work
- **Evidence:** Recorded data and results that show a specified claim
- **Claim:** Information about the work that has a specified scope
- **Judgment:** A decision or an estimate that the evidence does not fully show
- **Scope:** The work and the conditions that a claim or a review includes
- **Review focus:** The work that a review must examine fully
- **Rubric:** The criteria for a specified review
- **Assurance:** The evidence that you must have before you accept work at a maturity stage
- **Qualification:** The procedure that gets the assurance that is necessary for publication
- **Publication:** A push, a pull request (PR), or a merge that makes work available to other persons
- **Admission bound:** A temporary limit on the quantity of work in the pipeline that is not completed
- **Pipeline:** The sequence of tasks that work goes through to acceptance and delivery
- **Owning change:** The unmerged change that caused the behavior that a repair corrects
- **Target repair:** A repair that changes the target, with a record of the change
- **Waiver:** A decision to accept specified work without one applicable instruction, with a recorded cause and evidence for its facts
- **Normalization:** The last change of the history before publication that puts fixups into their owning changes
- **Boundary of validity:** The smallest set of changes that keeps the necessary behavior and gate results of a change
- **Minimality:** The condition that no smaller related extraction agrees with the applicable criteria
- **Indivisibility:** The condition that no smaller related part of a change can be a commit that its gates accept.

## Categories of the technical nouns in the definitions

| Term | Category | Subject field |
|---|---|---|
| Goal, intent, initial intent, governing intent | 7 | Requirements engineering |
| Invariant, constraint | 7 | Software engineering and formal methods |
| System constraint, admission bound, pipeline | 7 | Theory of constraints and flow control |
| Gate, gate contract, maturity stage, checkpoint | 19 | Continuous integration |
| Batch, piece, candidate, placement, placement test | 19 | Version control workflow |
| Prerequisite, dependent, dependency, shared prerequisite, independent work | 7, 15 | Scheduling and dependency graphs. Rule 1.5 lists "prerequisite" in category 15. |
| Attempt, extraction, mechanical extraction, remainder (D), target, conservation | 19 | Refactoring and version control |
| Integrated result, slice, review slice, standalone slice | 19 | Version control workflow |
| Delivery | 20 | Product delivery to the user |
| Cancellation test | 7 | Product engineering |
| Evidence, claim, judgment, assurance, scope | 7 | Assurance cases (ISO/IEC/IEEE 15026) |
| Review focus, rubric | 7, 15 | Software review |
| Qualification, publication | 19 | Software release |
| Owning change, target repair, normalization | 19 | Version control workflow |
| Waiver | 21 | Quality management. Rule 1.5 lists "waiver" in category 21. |
| Boundary of validity, minimality, indivisibility | 7 | Software engineering |

## Other technical nouns from their subject fields

| Terms | Category | Subject field |
|---|---|---|
| Git, commit, commit ID, commit boundary, commit message, tree, parent, ancestor, descendant, branch, base, merge base, merge conflict, diff, hunk, range, metadata, checkout, history, linear history, repository, version, status, push, pull request (PR), merge | 19 | Git |
| Code, code owner, software, behavior, feature, artifact, architecture, configuration, toolchain, command, file, resource, queue, segment, path | 19 | Software engineering. Rule 1.5 lists "configuration" in category 7. |
| Agent, primary agent, repair agent, review agent, context, context fork, conversation, harness, skill, host, prototype, LLM | 19 | AI agents. Rule 1.5 lists "large language model" and "chatbot" in category 19. |
| Defect, failure, regression, reproducer, diagnostic | 7, 19 | Software testing. Rule 1.5 lists "defect" and "failure" in category 7. |
| Affected test, unit test, property test, test runner, coverage, mutation testing, instrumentation, formatting, fast gate | 19 | Software testing |
| Fixup, amend, refactor, change class, contract class, presentation sequence, presentation ranking, review contract | 19 | Version control workflow and the atomic-changes skill |
| Graph, dependency graph, topological sequence, combination, phase, semantic dependency | 7 | Mathematics and software engineering. Rule 1.5 lists "graph", "combination", and "phase" in category 7. |
| Deployment | 20 | Software release to the user |
| User, decision maker | 11, 20 | Roles. Rule 1.5 lists "end user" in category 20. |
| Draft, principle, section, reference, term, definition, policy, label | 3, 15 | Documentation. Rule 1.5 lists "section", "reference", "label", and "language policy". |

## Technical verbs

| Verb | Category | Subject field |
|---|---|---|
| Merge | 2 c) | Git system operation |
| Revert | 2 c) | Git system operation |
| Deploy | 2 c) | Software system operation |

"Merge" is also a technical noun in category 19. Rule 1.5 and rule 1.12 show "update" in the same two conditions.
Use the past participle of a technical verb only as an adjective, for example "merged work" (rule 1.13).

## Names, identifiers, and quoted text

Rule 8.6 tells you to count each of these items as one word:

- Names: Dan Kubb, Symbiote, DAAS, atomic-changes, git-review, code-review, Clippy, RTI, Rust, ASD-STE100
- Dates and identifiers: October 2, P1 thru P9, S1 thru S10, D, `main`
- Quoted labels: "Retained", "Passed ⟨rubric⟩", "Qualified", "Merged"
- Quoted change classes from atomic-changes: "remove", "fix", "refactor", "move", "rename", "change", "add"
- Quoted titles: "Conditions for a standalone slice", "manual runbook", "workflow plan".

## Approved alternatives

In the skill, use the approved alternatives in the right column.

| Do not use | Use |
|---|---|
| pass, fail (a gate) | the gate accepts, the gate rejects |
| run (a gate) | do the gate |
| implement | write the code for |
| extract (verb) | move into a piece, remove from the remainder |
| review (verb) | do a review of, examine |
| commit (verb) | make a commit of |
| useful | that helps the user |
| current, now | this, at this time |
| parallel | at the same time |
| require, need | necessary, must |
| remain | stay, continue |
| establish | make sure that, show |
| meet (criteria) | agree with |
| state (noun) | condition, commit, tree |
| original | initial |
| exception | waiver |
| opinion | judgment |
| affected (outside "affected test") | related, that it has an effect on |
