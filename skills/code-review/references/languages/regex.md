# Regex Review Guidance (Language-Specific)

- Use simple English.
- Use short bullets.
- Apply `../core-principles.md` first.
- Do not repeat core principles.
- This file is the review checklist; the state-space theory behind these
  rules is `state-space-minimization` `references/languages/regex.md`.

## Pattern intent

- Prefer tight patterns that match only the intended inputs.
- Reject everything else by default.
- Accept all known valid inputs. When the full domain is unknown, treat the
  pattern as a refutable hypothesis: start with the strictest reasonable
  pattern consistent with the contract and known examples, then loosen only
  the smallest part that a newly observed valid rejection proves necessary.
  Do not start loose merely because the contract is incomplete.
- Apply the same standards in Rust, SQL, templates, and config.

## Anchors

- Whole-value validation **MUST** use the active engine's strict full-input
  semantics. Explicit absolute anchors are the default because most supported
  languages expose search-style matching APIs.
- Use `\A` and `\z` when the language supports them.
- Use the closest equivalent when a language does not support them.
- In PostgreSQL, prefer `\A` and `\Z`. Verify the active engine for every other
  SQL dialect.
- Avoid `^` and `$` when multiline or partial matches can occur.
- Use `^` and `$` only when strict absolute anchors are not supported, and test
  the engine's multiline and trailing-newline behavior explicitly.
- A genuine full-match API **MAY** provide equivalent whole-value semantics
  without anchors in the pattern. Verify that it consumes the entire input;
  treat this as an exception rather than the default recommendation.

## Groups

- Use non-capturing groups (`(?:...)`) whenever the group exists only for
  alternation or quantification and the match is never read, in every flavor
  that supports them (JavaScript, Rust, Ruby, and PostgreSQL AREs all do).
- Capture only when the code consumes the captured value. An unread capture
  misstates intent and pays bookkeeping cost.
- Prefer named groups over numbered ones when a capture is consumed and the
  flavor supports naming.

## Character classes

- Choose the digit class that matches the domain and engine. Use `[0-9]` for
  ASCII digits. Use `\d` only when the engine's full digit semantics are
  intended and tested.
- Use combined classes when it improves clarity (example: `[\da-f]`).

## Iteration

- Start with the strongest pattern you can justify.
- Loosen only when tests or real-world feedback require it.
- Prefer precise literals, classes, and bounded quantifiers before using
  wildcard quantifiers.
- Treat `.+` and `.*` as a last resort.
- Use `.+` when at least one character is required.
- Use `.*` only when empty is valid and intentional.

## Engine checks

- Verify which regex engine is in use before changing anchors or escapes.
- Bound input length before matching. A regex **MUST NOT** be the only limit on
  an otherwise unbounded input stream or allocation.
- Review nested quantifiers, ambiguous alternation, backtracking, and
  attacker-controlled input size for denial-of-service risk. Require bounded
  input or a linear-time engine when worst-case work cannot be bounded.
- Test representative accepts and rejects, boundary lengths, Unicode cases,
  adversarial near-matches, valid text with a prefix or suffix, and terminal
  newline cases.
