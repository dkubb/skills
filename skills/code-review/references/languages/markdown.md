# Markdown Review Guidance (Language-Specific)

- Use simple English.
- Use short bullets.
- Apply `../core-principles.md` first.
- Do not repeat core principles.

## Tooling

- Run the repository's documented Markdown formatter and linter with its
  checked-in configuration. Use its wrapper when one is provided or required.
  Otherwise, use `rumdl` when the repository documents it or the repository's
  configured `markdownlint` command. Apply the core tooling-adoption rules when
  no Markdown gate exists.

## Review focus

- Keep structure clear.
- Keep headings and lists consistent.
- Keep wording clear.
- Check heading hierarchy, list indentation, fenced-code language tags, and
  link targets mechanically where tooling supports them.
- Verify that prose, commands, examples, and referenced code describe the same
  current behavior. Stale but well-formatted documentation is a finding.
