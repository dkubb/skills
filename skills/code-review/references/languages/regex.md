# Regex Review Guidance (Language-Specific)

- Use simple English.
- Use short bullets.
- Do not repeat core principles.

## Pattern intent

- Prefer tight patterns that match only the intended inputs.
- Reject everything else by default.
- Aim to accept all valid inputs. When in doubt, go tighter.
- Apply the same standards in Rust, SQL, templates, and config.

## Anchors

- Use `\A` and `\z` when the language supports them.
- Use the closest equivalent when a language does not support them.
- In PostgreSQL or SQL, prefer `\A` and `\Z`.
- Avoid `^` and `$` when multiline or partial matches can occur.
- Always anchor full-match patterns. Use `^` and `$` only when `\A` and `\z`
  are not supported.

## Character classes

- Use `\d` instead of `0-9`.
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
