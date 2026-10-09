---
name: gh-review
description: >-
  Write and publish GitHub pull-request feedback with Conventional Comments.
  Set comment intent, place inline ranges, avoid duplicate feedback, submit
  reviews, verify posted comments, and resolve addressed threads. The caller
  supplies the review rubric.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-10-v1"
triggers:
  - "GitHub review"
  - "leave PR comments"
  - "post review comments"
  - "publish review findings"
  - "Conventional Comments"
---

# GitHub review comments

This skill covers how to comment on a pull request, not what to look for.
The caller supplies the rubric, scope, and blocking decisions.

## Comment prefixes

Use the caller's or repository's conventions when given. Otherwise:

| Prefix | Intent |
|---|---|
| `__suggestion:__` | A judged blocking correction |
| `__suggestion: (nitpick)__` | A nonblocking improvement |
| `__comment:__` | An observation |
| `__question:__` | A question or uncertainty |

- Use these prefixes exactly.
- Do not make every finding blocking. A question is not automatically a blocker.
- One concern per comment. Explain the consequence when it is not obvious.
- Ask a question when the concern is not established. Do not state uncertainty
  as a finding.

## Placement

Use the smallest scope that covers the concern:

1. Line: one affected line.
2. Line range: the full contiguous affected range.
3. File: a per-file comment for a concern that applies to the entire file.
4. PR: a PR-wide comment or review body for concerns that span files or the
   overall change.

Do not invent an inline location to make the API accept a comment.

## Process

1. If the PR has no assignee, ask the user whether to assign the author.
   If the PR is a draft, ask whether to mark it ready for review.
   Change neither without an answer.
2. Use `.github/CONTRIBUTING.md` and `.github/pull_request_template.md` when
   they exist to check the PR title and body against repository guidance.
   Cross-check the PR summary against the actual diff. Comment on unsupported
   claims and changes the summary omits.
3. Read existing inline threads, review bodies, and PR comments first. Do not
   duplicate a concern that is already flagged; reply to its thread instead.
4. Honor publication authority. A request to post comments authorizes them;
   do not ask again. Without it, draft locally. If asked to show drafts first,
   do so.
5. Recheck the PR head before posting. If it moved, revalidate findings and
   coordinates.
6. Submit `REQUEST_CHANGES` when the review has a judged blocking
   `__suggestion:__`, and `COMMENT` otherwise. `APPROVE` needs explicit
   authority.
7. Read back what GitHub actually published. If a request fails or times out,
   inspect existing reviews and comments before retrying.
8. Report the comment URLs, the reviewed revision, and anything not posted.

## Later passes

On a subsequent review pass, resolve an older thread once the current code
shows its concern is addressed. Do not ask for confirmation first. Leave
disputed or unfixed threads open. A merge alone is not evidence of a fix.
