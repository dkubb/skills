---
name: progressive-refinement
description: >-
  Keep only commits that their gates accept. Make the work better with reviews
  at applicable scopes. While the remaining work continues, continuously move
  slices that help the user without the subsequent work into a correct Git
  history that is easy to read.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-10-v1"
triggers:
  - "progressive refinement"
  - "striping"
  - "refine passing batches"
  - "extract standalone slices"
  - "construct atomic history"
---

# Progressive refinement

This skill is a draft for Dan Kubb to examine, and its name can change.
It records the approved workflow intent from the voice conversations of October 2 and October 3.
It also identifies methods that did not have a test and decisions that are open.
At this time, the tool goal is a standalone prototype for extraction and for the pipeline.
Symbiote is a possible subsequent host. This draft is not installed.

## When to use this skill

Use this skill to make batches of work better and to divide them into commits that are easy to examine.
This skill is most applicable when slices that help the user can go to delivery before the larger work is completed.
For a small change that is easy, use the usual development procedure and review procedure.

For trials and updates to this draft, use the [trial guidance](references/field-trials.md).

## Terms

The [terminology reference](references/terminology.md) gives the terms of this skill and their definitions.
Use each term only for the item that the reference shows.

## Inputs and outputs

Before you start, record these inputs:

- The goal and the initial intent
- The necessary behavior, invariants, and constraints
- The accepted base and the selected unmerged work
- The gates and rubrics for each maturity stage
- The repository preferences and user approval for publication.

Before a mechanical extraction from a batch, keep that batch at a checkpoint.
The full feature does not have to be completed.
If important intent is missing, record that it is missing. Do not make an intent that the user did not give.
Tell the user that a decision is necessary. Independent work can continue while the decision is open.

Keep these outputs:

- Kept changes that their gates accept
- A remainder that becomes smaller
- Recorded placements that their gates accepted
- Review results with their scopes
- A last history that is easy to read.

Identify local checkpoints and work that is prepared for publication as different items.
Write a report about open dependencies, open judgments, and missing evidence.

## Conditions for each kept commit

Before you keep a commit in the local history, make sure that the gates of its maturity stage accept it.
Apply this condition to initial batches, repairs, fixups, atomic commits from extractions, and new commits between them.
In the Rust example, these gates are formatting, Clippy, and the affected tests without instrumentation.
The last coverage, mutation testing, and publication checks are different. They are stronger, and they are necessary at subsequent maturity stages.

If a gate rejects an attempt, keep the failure in that attempt. Do not keep it as a commit in the kept history.
Temporary Git objects can be necessary to make an isolated candidate for a gate. These objects are not kept checkpoints.

## Conditions for a standalone slice

A standalone slice agrees with all these conditions in relation to the selected base:

- Each prerequisite is in the base or in the slice.
- The gates accept each kept atomic commit of the slice in the sequence of its dependencies.
- If you deploy all of the slice without the subsequent work, no regression occurs.
- If the user rejects, cancels, or reverts the remaining work, the user keeps the slice.

Internal dependencies are permitted. Apply the cancellation test to all of the slice and its prerequisites.
Also apply the cancellation test to each smaller group of changes that you think about for delivery.

An accepted placement shows that the change is correct in the conditions of the test.
The cancellation test shows that the slice helps the user without the subsequent work.
Delivery before the remaining work must have the two results.
Before you give a candidate the label "Qualified", it must have its necessary review and publication assurance.
This condition also applies to a candidate that the fast gates accept.

Apply these conditions to all change classes, which include "change" and "add".
A refactor that prepares for a subsequent feature can help the user only with that feature.
That refactor can be in the same slice as that feature, but keep it in a different atomic commit.
Delivery groups and the presentation sequence do not make dependencies.

## Principles

These principles come from the voice conversations and from [DAAS](../daas/SKILL.md).
Dan Kubb can change them. They apply only to this workflow. They are not new policy for all work.

### P1. Measure progress with results that help the user

Record the result that helps the user and its acceptance criteria. Measure progress to that result.
When a result helps the user without other work and agrees with its criteria, prepare it for delivery.
Supply it immediately when its delivery has approval.

### P2. Keep the intent when you change the structure of the work

When you change the structure of the work, keep the governing intent and the necessary behavior.
Record each change that you make to them.
Before you change the governing intent, make sure that you have approval for that change.

### P3. Make each claim agree with its evidence

Make the scope of each claim agree with its evidence. Do not make a claim more sure than its evidence.
If evidence shows a fact at a specified scope, you can write that fact at that scope.
You can also write a judgment. Identify it as a judgment.

Keep only the records necessary to support the claim. Use existing records before you make new ones.

If evidence does not continue to show the claim in the applicable conditions, do not use it again.
If the results of checks do not agree, do an investigation. Record each judgment that is open.

### P4. Try to find the smallest result that helps the user without other work

Try to find the smallest result that the user keeps without the subsequent work.
Include its necessary prerequisites. Record the dependencies.

### P5. Keep progress that checks accept

Keep only work that the checks for its maturity stage accept.
Use the fastest checks that are sufficient for the current stage.
While you try a change, keep the data necessary to put the last accepted work back into use.
If new evidence shows a possible error in previous assurance, examine that assurance again.

### P6. Let dependencies set the sequence

If capacity is available, do independent work at the same time. Complete prerequisites before their dependents.

### P7. Make the system better at its system constraint

Select changes to the system that help at the system constraint.
Control the quantity of work that is not completed.
Keep sufficient prepared work at the system constraint.
Do not let its queue become longer than necessary.
After important results, examine the system constraint again.

### P8. Keep the decisions of the workflow different from its methods

Record the decisions, the results that the user can accept, and their effects.
Use a method that does the task without parts or steps that are not necessary.
Make the method easy to read and examine.
Before you make a method more general, get evidence of different tasks that occur again.

### P9. Supply sufficient information when it is necessary

Give each task sufficient information to do that task.
Keep the intent and the important constraints available.
When more instructions are necessary, supply them.
Use automatic checks that stop work when it does not obey an applicable instruction.

## Procedure

The steps that follow apply the principles to Git work.
Each piece can go to its next task while the remaining work continues.

### S1. Record the scope and keep batches that their gates accept — P1, P2, P3, P5, P9

1. Record the selected base and the unmerged work.
2. Record the initial intent, the goal, the invariants, and the constraints.
3. Record the gates for each maturity stage.
4. Examine the status of the repository at this time.
5. Keep other work safe.
6. Select a batch that helps the user.
7. Give the task sufficient information for that batch.
8. When it is necessary, give the task access to the adjacent code, architecture, and constraints.
9. Write the code for that batch.
10. Compare the result with the intent and the necessary behavior.
11. Repair each difference, and compare the result again.
12. Do the specified gates on the same work that you will keep.
13. If previous gate evidence has the same inputs and conditions, you can use that evidence again.
14. If the gates accept the batch, make its commit before a checkpoint review or an extraction.
15. For each extraction, record the version of its source checkpoint and its target version.

Before the gates accept a batch, you can examine its intent and architecture.

The first checkpoints do not have to show the last sequence of atomic commits.
That commit records progress in development. The larger feature does not have to be completed.
You must get publication assurance at a subsequent time. This step does not cancel it.

#### Keep only necessary evidence

Use existing gate output, CI records, and Git metadata. Do not duplicate their data.
Do not add evidence files to the repository unless no suitable alternative exists.
A workflow that permits checkpoints only after their gates accept can supply that assurance.
Do not make another record only to repeat that assurance.

For this workflow, use this logical format for gate trailers:

```text
Gate-<slug>: <command-sha1> <tree-hash>^{tree}
```

Use one physical line when the repository's message rules and the message checker permit it.
Otherwise, fold the value to meet their line limit.
For a 72-byte limit, use this layout:

```text
Gate-<slug>: <command-sha1>
 <tree-hash>^{tree}
```

If the key and command hash exceed the active line limit, put the command hash on an indented continuation line too.
Keep the trailer block separate from preceding message text with a blank line.
Use `git interpret-trailers --parse` to read the logical values.
Its output must preserve the command hash and the typed tree expression above.

Use the repository's gate identifier for `<slug>`.
Calculate `<command-sha1>` from the exact UTF-8 command text, including its arguments, without a trailing newline.
For `cargo test`, calculate the SHA-1 hash of `cargo test`.
Use the checked candidate's tree hash for `<tree-hash>`.
Append `^{tree}` to identify it as a tree. Keep the full hash.
Before a comparison, resolve the expression with `git rev-parse --verify`.
Gate readers must support this notation before they can use the record again.
GitHub can still shorten the hash and link it as a commit despite the suffix.
Add a trailer only for a gate result that was observed on the named tree.
If S4 permits reuse on another tree, preserve the initial trailer.
Do not substitute the new tree hash without an accepted execution on that new tree.
Keep the reuse judgment with existing gate output or metadata, as S4 tells you.

After a rebase, compare the trailer with the new command and tree.
Also compare other inputs that the gate uses, as S4 tells you.
For a diff check, the tree hash alone does not identify its base or selection.
Keep those inputs in existing gate output or metadata.
Do not use a trailer again when its evidence no longer supports the claim.

Use gate trailers at this time. A change to git-meta is deferred.

#### Gate repair with an agent

If the harness can make a context fork, you can give a gate failure to a repair agent.

1. Give the repair agent the context of this task.
2. Until the repair is completed, give write access for the files of the repair to that agent only.
3. Keep the repair in the governing intent, invariants, and constraints.
4. If the repair must change the governing intent, stop the repair and get the necessary decision.
5. When the gates accept the batch, make a commit of the repaired batch.
6. Send a short report to the primary conversation.

That report must include:

- The changed code and the cause of each change
- The commit ID of the checkpoint
- The gate evidence
- The open decisions.

During the repair, the primary agent can continue other work that does not use those files.
If there is no different agent, the same limits on the repair and the same checkpoint conditions apply.

To use a completed repair agent again, start its next task with the applicable harness command.
In the Codex collaboration harness, `collaboration.followup_task` starts that task.
`collaboration.send_message` does not start a new task for a completed agent.

#### Example gates for Rust

In the Rust example from the voice conversations, the initial fast gates are these:

- Formatting
- Clippy
- The affected unit tests and property tests without instrumentation, with RTI or the test runner of the repository.

The gate contract of the repository can add other necessary gates.
Keep the feedback loop fast during development, review, and decomposition.
While the code, commit boundaries, or commit sequence change, use the fast gates.
Do not do coverage or mutation testing at that time.
Start coverage and mutation testing only after final review approval of the selected slice.
First finalize its code, atomic boundaries, and sequence through S9 and S10.

The selected qualification sequence can be one standalone slice.
The remainder can continue with fast gates while the slice gets qualification.
For qualification, obey the gate contract of the repository.

Before you make a commit in the local history, do its gates on the same work that you will keep.
If you have evidence for that work with the same inputs, you can use it again.
When you divide commits or do repairs, keep the gate contract.
Change the scope of affected tests only from dependencies that you found.
Do not change that scope to accept a change that a gate rejects.

If the gate contract accepts a source checkpoint, that checkpoint is a known correct start.
That result does not show that a smaller set of commits is correct.

Use these labels for different claims:

- "Retained" for a kept checkpoint that has the necessary conservation checks
- "Passed ⟨rubric⟩" for a review result in the recorded rubric and scope
- "Qualified" for work that has the necessary assurance for publication
- "Merged" for work in the base.

#### RTI selection

Use the [RTI skill](https://github.com/dkubb/rust-test-isolator/blob/main/SKILL.md) for its commands and evidence.
With the fast gates, examine RTI's selection for the selected commit or range.
Do this for "move", "remove", and test-only changes too.
Before you use `EMPTY` as evidence, get that result from RTI.

An RTI `EMPTY` result gives no evidence that production functions have correct behavior.
A constant change or a test-only regression can have this result.
Do not use this result to accept a Rust change or a test-only regression.

Use the repository's applicable gates for these changes.
Before you select a production owner, make sure that this owner includes the changed behavior.
This owner must include the necessary test scope.

With the fast gates, compare the test names that RTI selects for that owner with the names of the affected tests.
For each difference, make sure that the gate contract permits it.
Keep this comparison with the gate evidence.
File paths do not show the selected test names.

If qualification must test a public function, make sure that the tests call that function.
Tests that call only a private function do not supply that assurance.

#### Time of reviews

At checkpoints at which a review can help, examine the intent, the architecture, and how the work will continue.
Do these steps before you do work on changes that an architecture change can make incorrect.
A code review can include one batch, some batches, a delivery slice, or one commit.

Select how frequently you do reviews from the quantity and the effects of the work that you did not examine.
At the end, do a review of the completed integrated result.
These phases occur for each piece, and they can overlap.
Extraction can help before the feature is completed and before you have a full dependency graph.

### S2. Select work from the prepared work — P1, P6, P7

1. Find the pieces and the reviews that have their necessary inputs.
2. Find the work that waits and the resources that two or more tasks use.
3. Examine the waits that occur again, the queues that become longer, and the time that work waits for prerequisites.
4. Also examine the time for checks and decisions.
5. From these data, identify a temporary system constraint.
6. Select a prepared task that can help at that system constraint. For example, select an extraction, a placement test, a review, a repair, or a task that prepares inputs.
7. If an extraction only makes the queue before the system constraint longer, do not start that extraction.
8. In that condition, help the task at the system constraint, or prepare its inputs.
9. If you cannot help or prepare, stop and give the capacity to other work.
10. If no prepared work can go to its next task, stop or tell the user about the important open decision.

For the first manual test of this workflow, select a small admission bound and record it.
After important results, examine that bound again.
The bound is temporary. It is not the best value for all conditions.

When you select work, include the time for these tasks:

- Checks that you do again
- Repairs that you do again
- Tasks that prepare context
- Decisions.

Keep sufficient prepared work at the system constraint.
When you select prepared work for the system constraint, include differences in the time for each task.
With the queue length, record the scope of work that is not completed.
Also record your judgment of the time necessary to complete that work.

Extraction can increase the number of commits. When you merge a slice, that work goes out of the unmerged scope.
Select the sequence of work from its effect on accepted progress, not only from the number of its descendants.

At each scope of development or review, find standalone slices in the work that you already have.
Use atomic-changes to identify their change classes.
First look for "remove", "fix", "refactor", "move", and "rename" changes to code already in the accepted base.
Select those changes that agree with the cancellation test.
Give their extraction, review, qualification, and delivery priority over more work on the remainder.

Prepare each such slice for a PR.
Do not wait for the full feature to prepare these PRs.
Merge only with approval and the necessary qualification, as S10 tells you.
Keep a change with the subsequent work if the user would not keep that change without it.
During normalization, put repairs and cleanup of new feature code into their owning changes.

Also apply the cancellation test to "change" and "add".
Put prerequisites before dependents.
Select the sequence of delivery from its effect on the system constraint.

For a defect that is in the base, prepare a standalone slice with a reproducer and a correction.
For a defect that the unmerged work caused, repair its owning change during normalization.
With this step, you add slices that help the user continuously.
Do not wait for all extractions before you do a review of a slice or prepare its PR.

### S3. Select one extraction from the remainder (D) — P1, P2, P4, P8, P9

1. Read atomic-changes and its criteria for commits before you make the piece.
2. Select a piece from the remainder that has only related changes.
3. Select the version of the prerequisites that the piece must have.
4. Record the target version for this attempt.
5. Move each necessary hunk from the remainder into the piece.
6. Make sure that the target stays the same.
7. Use the result of this attempt to select the next attempt.

Examples from history do not tell you the causes of commit boundaries without the criteria of atomic-changes.
A decision maker can tell you which attempts at extraction can help.
A decision maker can also tell you which previous parents are possible for a placement test.

Do one attempt at a time. This limit applies to the scope of each attempt, not to the total number of pieces.
You do not have to make a full dependency graph first. The graph can become larger with each attempt.
If a standalone slice stays standalone with a prepared dependent, you can add that dependent to the slice.

The remainder (D) is a name for the remaining changes, not for one commit. Its commit ID can change.
The diff of the remainder becomes smaller, but its target stays the same.
A new development batch that the gates accept starts a new target version.
Do not use two target versions in one mechanical extraction.

New development can continue on the remainder while pieces from extractions go through the pipeline.
On those paths, do the work for prerequisites before the work for their dependents.
This method does not make it necessary to examine merged work on `main` again.

### S4. Make the attempt and examine it — P3, P4, P5

1. Make the candidate in an isolated checkout with its selected prerequisites and its recorded target version.
2. Make sure that the candidate shows the extraction that you selected.
3. Make checks for the applicable acceptance criteria in the governing instructions at the selected base.
4. Apply those checks to the candidate itself. A match with expected results that you generated does not replace those checks.
5. Do the gates of its maturity stage, or use evidence with the same inputs again as P3 tells you.
6. Record the inputs of each gate result.
7. If the gate changed the candidate or the checkout, make sure that you know about each change.

If a gate rejects a placement and you thought that this result was possible, keep the previous checkpoint.
If you reject the candidate, keep its diagnostic and the previous checkpoint.

For each rejected attempt, record what the failure shows and your next decision.
Include the cause that the evidence shows. If the cause is not known, identify it as open.
Use this record to change the extraction or its prerequisites, or to stop this attempt.
Keep this record even if you stop extraction.

Some failures can occur that you did not think were possible.
Examples are a software failure, a harness failure, and gate results that do not agree.
If such a failure occurs, stop the work that it has an effect on.
Do an investigation of the failure before you continue that work.
Independent work can continue if its evidence stays correct.

Before you use development evidence for a new commit, compare the recorded inputs with the new inputs.
If a gate uses the commit identity or diff selection, compare these inputs too.
Keep the initial commit identity in the evidence.
For reuse on another tree, identify the initial result, new tree, and scope of the reuse claim.
Record which relevant inputs stayed the same.
Use existing gate output or metadata. Do not duplicate the initial result.
If the evidence no longer shows a necessary claim, get evidence for this claim.

With the evidence, keep the inputs that can change its claim. Use references to existing records where possible:

- Tree checks: the tree, command, configuration, toolchain, and conditions in which the gate operates
- Diff checks: the inputs of tree checks, plus the parent, range, selected code owners, and contract classes
- Selection of affected tests: the inputs of diff checks, plus the selected test names
- History checks: the applicable metadata
- Reviews: the intent, the rubric, the governing instructions, and the adjacent context.

Keep the inputs that can change the gate's claim fixed.
Changes to unrelated refs in shared Git storage do not invalidate an attempt.
Include those refs only if the gate contract checks them.

If evidence does not include the necessary claim, do not use it again.
An accepted extraction shows that the extraction is correct only in the conditions of the test.
It does not show mathematical minimality. It does not show that the piece has no semantic dependencies.

One rejected extraction does not show indivisibility.
If two reviews do not agree about a claim, use the review contract to make a decision.
A usual difference between two reviews does not show that two gate results do not agree.

### S5. Keep an extraction with conservation — P2, P3, P5

1. Prepare the piece, the new remainder, and the necessary connections to subsequent work.
2. Compare the tree of the pieces and the remainder together with the target tree.
3. Find the subsequent work that the extraction did not change.
4. Compare the input and output trees of that work with their previous trees.
5. Make sure that the gates accept the candidate.
6. Make sure that the source, the prerequisites, and the target agree with the versions that you recorded.
7. If all these conditions are correct, keep the new checkpoint.

If an input version changed, examine the attempt again.
Do not use an attempt with changed inputs to replace a newer target.
If the inputs and conditions of the piece agree, you can use the evidence of the piece again.
But make the changed target and its combination again, and do the applicable gates on them.
A temporary candidate object is not a kept checkpoint.

In a linear history, the output tree of the remainder stays equal to the target tree.
Its parent includes the pieces from extractions.
If you use independent paths, make sure that their combination gives the target tree again.
Do the applicable gates on each new tree that you will keep or examine.
For a tree that did not change, you can use evidence with the same inputs again.

### S6. Remove sequence that is not necessary and let paths go — P4, P6, P7

1. If it can help, try a previous parent or a smaller group of prerequisites.
2. Make sure that the applicable gates and reviews accept the placement.
3. Make sure that the target stays the same.
4. If the two conditions are correct, keep that placement.
5. Let the kept piece go to its next prepared task.

Some independent prerequisites can be necessary together in one context.
Do not show them as one dependency.
Do not keep ancestors that have no relation to the piece only to make a linear history.
Do not add dependencies between independent paths.

If you do work at the same time, obey these instructions:

- Use isolated checkouts.
- Give write access for each piece that can change to one person or one agent only.
- Apply the same limit to each segment of a path that is not shared.
- For an attempt, use only kept versions of shared prerequisites that do not change.
- Give each repair of a shared prerequisite to the person or agent that has write access to it.
- Before you accept dependent results, make sure that the changes to each shared ancestor and its dependents are synchronized.

### S7. Do a review at the selected review focus with the applicable context — P1, P3, P6, P8, P9

1. Select the review focus.
2. Apply the applicable instructions to that review focus.
3. When it is necessary, read the prerequisites and the adjacent code.
4. Record each review result at the scope that its evidence shows.
5. Identify the owning change of each review result.

The review focus can be all of the selected unmerged branch or one review slice.
It can also be a range of two or more commits or one commit.

The review focus tells you where to look and which work you must examine.
It does not prevent results about the architecture or results in previous prerequisites.
A result about work that is not in the review focus does not show that a full review of that work occurred.
Merged code in the base gives context for the selected changes and how they operate together.
Do not start a review of `main` that has no relation to the selected changes.

Give a review agent that did not write the code these inputs:

- The initial intent and the governing intent
- The applicable invariants and constraints
- The selected commits
- One rubric or a related group of rubrics
- Access to the code and the evidence of ancestors, when it is necessary.

If the harness lets you, keep the review conversation different from the conversation of the repair.
Compare the intent that the user gave with the code.
Compare the commit message with the code and with the initial intent.
Do not use the commit message as the source of the initial intent.
Examine how each atomic commit helps the goal.
In the review of the integrated result, examine all of the goal.

Use code review to make batches better during development.
After extraction, do a review of each atomic commit in its standalone context.
A review of the source batch does not replace that review.
To examine candidate atomic commits that you made, use git-review.
git-review examines commit boundaries, sequence, and correct commits. It also makes sure that intent, diff, and message agree.

Before you write a result as a fact, make sure that evidence at the specified scope shows it.
If evidence does not show a claim about facts, record that claim as open.
You can write a judgment. Identify it as a judgment, and write how sure you are.
Also write the data that it uses and the facts that evidence shows for it.

A decision maker can accept a review result. A decision maker can also tell you to do an investigation or a repair.
A second LLM review of each judgment is not necessary.

If evidence shows that the work does not obey an applicable instruction, do not accept that work.
To continue, do one of these steps:

- Repair the work.
- If the review contract lets you use a waiver, get a decision on the waiver.

Before you accept the waiver, make sure that the decision accepts its cause and the evidence for its facts.
The waiver applies only to the specified work.
Keep the waiver, its cause, and its evidence.

Before changed work goes to its next task, examine it again.
Also examine the related results of previous reviews again.
Before a dependent has a review, make sure that a review with the same rubric accepted its shared prerequisites.

### S8. Add target repairs — P2, P3, P5, P6

1. Record the cause of the repair and how it agrees with the governing intent and the invariants.
2. Before you change the governing intent, make sure that you have approval for that change.
3. Use the previous piece as the three-way merge base for the target and the repaired piece.
4. From these three inputs, make the new target.
5. Give each merge conflict and each related change to one owning piece or repair.
6. Do the gates on the integrated result.

For a repair of an input that two or more persons or agents use, do these steps:

1. At a safe point, stop the persons and agents that change work that uses that input.
2. Keep their work available.
3. Change the context of the dependent work.
4. Examine the evidence of each changed input again.
5. When the necessary checks accept the work, let that work continue.

Keep the data necessary to put the previous checkpoint back into use.
Before you continue, keep the repaired result that its gates accept.
When you can, use fixups during development, because they are easy to examine.
Before you keep a fixup, make sure that its gates accept its tree.

Obey the applicable user, repository, and skill instructions for the repair method.
Approval to change history does not cancel an instruction to use fixups.
Before an amend that changes the tree, make sure that your approval includes this amend.
Keep the data necessary to put the previous checkpoint back into use. Get evidence for the changed tree.
The previous evidence applies only to the inputs that it recorded.

If you have approval to change the last history, put each correction into its owning change during normalization.
If a change helps without other changes, keep it in a different commit.
For a defect that is on `main`, make a new commit that corrects it. Do not change the history of `main`.

A target repair changes the target and records the change.
A mechanical extraction does not change the target.
The target versions and the methods to give work to other agents are prototype methods that Dan Kubb must examine.

### S9. Continue to divide the work and record the cause when you stop — P1, P4, P7

1. While extraction can supply progress that helps the user, continue extraction from the remainder.
2. While extraction can make the necessary artifact better, also continue.
3. If it can help, divide the pieces from extractions again.
4. If you stop an attempt, or stop it until a subsequent time, record the cause.

A judgment that you cannot divide the change can stop the attempt before a placement test.
In that condition, record that no placement test occurred.
That judgment does not show indivisibility.
The extraction work can stop or wait if intent is missing or the attempt limit is used.
It can also stop or wait if more extractions do not help the maturity stage.

At the last artifact review, do these steps:

1. For each change, examine its one cause and its boundary of validity.
2. Identify each smaller related extraction that you have not tried.
3. If the attempt limit lets you, do a placement test for each identified extraction.
4. If the attempt limit stops a necessary placement test, record that qualification is not completed.
5. If the review contract lets you use a waiver, get a decision on it as S7 tells you.

When the commit boundaries of a piece change, examine the related review claims again.
If evidence for a specified scope continues to apply, keep it.
Do not think that all reviews stay correct. Do not think that you must do all reviews again.

### S10. Make the Git artifact and get its qualification — P1, P2, P3, P8

1. Put the necessary prerequisites before their dependents in a topological sequence.
2. Use the presentation ranking of atomic-changes only between alternatives that have no dependency between them.
3. When you make a linear history from the graph, keep that graph and its independent paths available.
4. Do the necessary checks on the tree at each kept commit, or use evidence with the same inputs again.
5. When you make the linear history or a delivery, examine each piece review with changed inputs again.
6. Do a review of the integrated result and of how easy it is to read across commit boundaries.
7. A review of the integrated result does not replace a necessary piece review.
8. In your reports, keep the claims about checkpoints, qualification, and publication different.

Before coverage, make sure that the selected qualification sequence agrees with all these conditions:

- Each commit is one coherent transformation. Complete the minimality checks in S9 for each commit.
- The commit boundaries and sequence agree with atomic-changes.
- During approved normalization, each branch-local correction went into its owning change.
- Formatting, Clippy, and the affected unit tests and property tests accept each final Rust commit.
  Use RTI for the affected tests and its selection checks in S1.
- The selected sequence has final review approval, including its code, standalone commits, and integrated result. No blocking review finding is open.

Then do the applicable coverage checks for each atomic commit in that sequence.
Before mutation testing, complete the applicable coverage checks for all commits in that sequence.
Use this last pass to examine whether the tests exercise and protect the accepted behavior.

If qualification requires a correction, return the affected work to fast gates and review.
Correct its owning change.
Get final review approval again before you continue qualification.
Use S4 to decide which previous results still apply.

While the remainder continues, do these steps for each prepared standalone slice.
When a standalone slice agrees with its conditions, you can tell the user that it is prepared for publication.

Before publication, read the documented repository preferences and the user's instructions for the session.
Identify which actions have approval: a push, a PR, or a merge.
Do each publication action only when it has approval.
Merge only when a documented repository preference or explicit user instruction permits it.
The user's restrictions apply even when the repository permits automatic merging.
If merge approval is missing or unclear, do not merge or enable automatic merging.
This skill, its change classes, and successful gates or reviews do not give merge approval.
You can ask the user for merge approval in the session.
If existing approval includes the action and its conditions, do not ask for it again.

Continue to prepare extraction PRs under the approval that you have.
Without merge approval, keep prepared PRs open and continue independent work.
Extraction, review, and qualification do not have to wait for merge approval.

The slice can include dependent commits and independent paths. Record its dependency graph.
Get the qualification of each necessary commit in the slice and of all of the slice.

An extraction does not show that you merged or supplied the slice.
A merge makes the unmerged scope smaller. It does not show that the user can use the result.
When deployment is necessary for delivery, record its result before you tell the user that delivery is complete.
After a merge, rebase the selected remaining branches on the accepted base at a safe point.
Keep other work safe, and synchronize shared dependents as S8 tells you.
Prepared dependents can then go to their next task.

If merged work overlaps the selected range, record a new base and range at a safe point that you identify.
Before qualification or delivery on a new base, also do this step.

Do not select merged work as the review focus.
During qualification, do not change the base.
A new base starts a new target version.
For this version, examine the evidence with the inputs in S4.
If the evidence still shows the necessary claim, use it again.
If the evidence does not show a necessary claim, do the related qualification again.

Obey the user and repository instructions for gates on unchanged patch-ids.
Work on `main` that has no relation to the selected range does not make it necessary to start an active maturity stage again.

## References

The [code-review skill](../code-review/SKILL.md) uses [git-review](../git-review/SKILL.md) for each target that has a commit.
Before Dan Kubb accepts this draft, make that skill agree with the sequence in S7.

The unmerged git-factor runtime uses `git hash-object --no-filters` for its command hash.
That Git blob hash includes an object header. S1 uses SHA-1 of the command bytes only.
These hashes differ for the same command.
Agree on one hash method before you use gate trailers across these workflows.
Until then, do not use a trailer from one hash method as evidence for the other.
