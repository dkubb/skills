# Ruby Code Review Guidelines (Language-Specific)

- Use simple English.
- Use short bullets.
- Apply `../core-principles.md` first.
- Do not repeat core principles.

## Tooling

- Use RuboCop when reviewing Ruby changes.
- Configure RuboCop according to the lint configuration policy in
  `../core-principles.md`. Record project-specific cop selection, tuning, and
  reasoned exceptions in `.rubocop.yml`.
- Configure every mechanically enforceable rule below as a RuboCop cop. Review
  a rule manually only when RuboCop cannot decide its semantic applicability.
- Run the repository RuboCop wrapper and require zero findings. If the
  repository has no wrapper, use its locked bundle to run RuboCop against all
  configured paths; do not use an unpinned global executable.

## Formatting

- Use ASCII when possible. Use UTF-8 only when needed.
- Use 2 spaces. No tabs.
- Use Unix line endings.
- Use spaces around operators, after commas, colons, and semicolons.
- Use spaces around `{` and before `}`.
- No spaces after `(` or `[` and before `]` or `)`.
- Align `when` and `else` with `case`.
- Add a blank line before the return value unless the method is one line.
- Add a blank line between method defs.
- Use YARD for API docs. Do not add a blank line between doc and def.
- Use blank lines to split logical sections in long methods.
- Keep lines under 80 chars.
- Strip trailing whitespace.

## Syntax

- Use `def` with parentheses when there are arguments.
- Avoid `for` unless you are sure it is needed.
- Avoid `then` except in case statements.
- Use `when x then ...` for one-line cases.
- Use `&&` and `||` for boolean expressions.
- Use `and` and `or` for control flow only.
- Avoid multiline `?:`. Use `if`.
- Use parentheses when calling methods with arguments.
- Use `{...}` for one-line blocks.
- Use `do...end` for multiline blocks.
- Avoid `return` when not required.
- Avoid `\\` line continuation.
- Use `||=` only when `nil` and `false` have the same domain meaning. Do not use
  it when `false` is a value that must be preserved.
- Prefer `Regexp` objects.
- Avoid `=~`, `$0-9`, `$~`, `$PREMATCH`, and `$POSTMATCH` when possible.

## Naming

- Use `snake_case` for methods.
- Use `CamelCase` for classes and modules.
- Keep acronyms uppercase (HTTP, RFC, XML).
- Use `SCREAMING_SNAKE_CASE` for other constants.
- Avoid single letter names.
- Use consistent names.
- Keep names close to the object class name.
- Prefix unused variables with `_`.
- Use `each_with_object` instead of `inject` when the memo does not change.
- Use `other` for predicate comparisons with same-type arguments.
- Prefer `map` over `collect` and `select` over `find_all`.
- Prefer `detect` over `find` so collection predicate
  search is visually distinct from object lookup. Configure RuboCop's
  collection-method mapping explicitly; this differs from its default.

## Comments

- Capitalize comments and use punctuation for full sentences.
- Use one space after periods.
- Avoid obvious comments.

## Design

- Avoid hash options when they hide multiple responsibilities.
- Avoid long methods and long parameter lists.
- Use `def self.method` for singleton methods.
- Add global methods to `Kernel` only if needed, and make them private.
- Use `alias_method` over `alias` so aliasing remains an
  explicit method call and can use computed names. Configure `Style/Alias`
  explicitly; this differs from RuboCop's default.
- Freeze objects assigned to constants.
- Use `OptionParser` for CLI options. Do not expose options as global variables
  through `ruby -s`.
- Avoid needless metaprogramming.
- One method = one purpose. Split boolean-driven branches.
- If a method needs “AND” or “OR” in its description, split it.
- If blank-line sections are independent, split them into methods.
- Keep methods at ~10 lines max, prefer 5 or fewer.

## General

- Prefer functional style when it makes sense. Avoid mutation.
- Use CQS: a query returns state without changing it; a command changes state
  without returning domain data. A fluent command that returns `self` is a
  deliberate API exception and needs a reason.
- Do not mutate arguments unless that is the method’s purpose.
- Do not monkey patch core classes in libraries.
- Validate untrusted input at boundaries. Inside validated domain code, do not
  add redundant defensive checks that obscure a violated invariant.
- Keep code simple and consistent.
- Avoid overdesign and underdesign.
- Prefer explicit coercion helpers (like `coerce` methods) at boundaries.
- Use immutable value objects when possible by freezing them or using an
  immutability helper. Memoization alone does not make an object immutable.
- Favor small, composable objects and methods that encode domain rules.
- Preserve existing semantics; add targeted tests before changing behavior.

## Review focus

- Focus on Ruby-specific design, API clarity, and style conventions.
- Expect thorough test coverage. When the repository configures mutation
  testing, run its wrapper against the changed contracts and require no
  surviving non-equivalent mutants. Record the reason for every equivalent or
  excluded mutant.
