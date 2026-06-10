# SQL Code Review Guidelines (Language-Specific)

- Use simple English.
- Use short bullets.
- Do not repeat core principles.

## Data modeling and sync

- Mirror remote schemas 1:1 in tables when ingesting remote data.
- Persist every field returned by a remote API. Reject unknown fields at ingest.
- Use views or functions to reshape data for app needs.
- Prefer fewer DB round-trips. If a query can be composed into one round-trip
  without clarity loss, do it.
- Treat unique constraints as correctness, not performance.
- Add non-unique indexes only when a query needs them.
- Design non-unique indexes from real query predicates and selectivity.
  Do not lead a composite index with a constant or near-constant column unless
  the query pattern still benefits from that order.
- Use CHECK constraints so invalid data fails fast and clearly.
- Prefer native Postgres enum columns for small, closed value sets that are
  unlikely to change often. Avoid text-plus-CHECK when the enum can be modeled
  directly.
- If a constraint (not uniqueness) fails, tighten upstream validation.
- Use CHAR for fixed-length hashes and IDs.
- Prefer constrained UUID or identifier column types over unconstrained text
  when the stored value has a fixed format.
- Add format checks for hashes (e.g., hex length).
  Use \\A and \\Z for anchors and \\d for digits.
- For nullable strings that must be non-empty when present, add a CHECK.
- For optional reason or note fields, prefer printable-character checks when
  control characters are not needed.
- For arrays that must be non-empty, use cardinality(...) > 0.
- Prefer name[] for lists of identifiers.
- Prefer role-based grants over trigger-based write blocks.
- Prefer domain types (including *_not_null variants) over raw primitives.
- Use db_move.identifier and db_move.created_at domains for primary keys and
  created_at columns.
- Use db_move.sha256_hash for SHA-256 hash columns.
- Add foreign keys for local identifier references when the row must exist.
  Do not rely on length or format checks alone when referential integrity is
  required.
- Add ON DELETE RESTRICT and ON UPDATE RESTRICT to foreign keys unless another
  action is required.
- When related timestamps imply an order, encode it with a CHECK.
  For stateful rows, review whether fields such as updated_at, started_at, and
  completed_at should be monotonic or derived from the state machine.
- Move inline SQL into standalone .sql files and format them to match these
  rules.

## Tables and views

- Order constraints and checks so they read top-to-bottom with the columns.
- Use explicit view column lists when the view reshapes or normalizes columns.
- Design views around usage; include only the columns needed at call sites.
- Prefer UNION ALL in aggregate views; keep each SELECT block separated by a
  blank line.
- Place COMMENT ON after object creation and before GRANT when both exist.
- Put GRANT statements immediately after the object they target.
- For new tables, consider physical row layout. Keep related columns together
  when reasonable, but prefer an order that reduces row padding when it does
  not hurt readability.

## Review focus

- Focus on data modeling, correctness, safety, and change impact.
- Do not re-check what SQL tooling can catch. Require running those tools.
- Ignore generated schema dumps such as `db/structure.sql`.

## Formatting

### General

- Use leading commas in lists (columns, parameters, enums, roles, grants).
- Use semicolons on their own line for multi-line statements.
- Keep semicolons on the same line for single-line statements.
- Do not wrap one-line inline subqueries; keep them as one-liners.
- Uppercase SQL keywords.
- Keep boolean literals lowercase (`true`, `false`).
- Use lowercase SQL functions.
- Use lowercase for unquoted identifiers; preserve case for quoted identifiers.
- Decide casing by AST position, not by raw token text; identifiers that match
  keywords stay identifiers.
- When an AST node has multiple keyword variants, keep all siblings in the same
  case.
- Uppercase `INTERVAL` when used as an interval literal keyword (for example,
  `INTERVAL '1s'`); lowercase interval types in casts and type declarations
  (for example, `::interval`).
- Uppercase EXTRACT fields like `EPOCH`.
- Uppercase function attributes and volatility/parallelism markers (for example,
  `IMMUTABLE`, `STABLE`, `STRICT`, `SECURITY DEFINER`, `PARALLEL SAFE`,
  `LEAKPROOF`).
- Uppercase `WITH ORDINALITY` when it is the keyword; keep `ordinality`
  lowercase when it is a column.
- Uppercase context-required keywords: `AT TIME ZONE`, `BEGIN ATOMIC`,
  `ON CONFLICT`, `NULLS FIRST` / `NULLS LAST`, `WITHIN GROUP`,
  `SET CONSTRAINTS ALL IMMEDIATE`, `RESET ROLE`, and `SAVEPOINT`.
- For one-line CHECK constraints, do not add spaces after `(` or before `)`.
- When contiguous lines are sibling AST nodes at the same level (for example
  list items), align their indentation consistently rather than drifting per
  line.
- Quote identifiers only when mixed case or reserved words require it.
- Leave psql meta commands and variable references unchanged; format only SQL
  portions (for example, the query inside `\\copy (...)` and SQL keywords like
  FROM/TO/WITH/FORMAT/CSV/HEADER in `\\copy` arguments).
  Do not case-normalize file paths or psql variables.
- In GRANT statements, uppercase the PUBLIC role when it is the special role
  (keep `public` lowercase for schema names).
- Use `AS` for CTE definitions, not for table aliases.
- Omit `AS` for table and column aliases unless required by syntax.
- Use short aliases only for derived tables, set-returning functions, or when
  disambiguation forces it.
- Prefer schema-qualified relation names for cross-schema or shared objects.
- Avoid `public.` unless search_path ambiguity requires it.
- In EXISTS, omit the select list (`SELECT FROM ...`); avoid `SELECT 1`.
- Keep `EXISTS (` / `NOT EXISTS (` with the opening `(` on the same line as the
  EXISTS keyword.
- In EXISTS subqueries, avoid qualifying columns from the subquery table when
  unambiguous.
- When qualifying columns, use the table name only; avoid schema-qualified
  column references.
- When using `IF NOT EXISTS`, put the object name on its own line.
- Align column types and constraint keywords (including PRIMARY KEY, NOT NULL)
  in CREATE TABLE blocks.
- Name table-level CHECK constraints (e.g., CONSTRAINT check_key_cardinality).
- Sort enum values in declaration order.

### CREATE statements

- Put the object type on its own line, followed by the qualified name on the
  next line.
- Use leading commas and aligned columns in CREATE TABLE and CREATE VIEW column
  lists.
- Keep CONSTRAINT and CHECK clauses aligned with columns.
- Order aligned column specs as: name, type, `NOT NULL`, `DEFAULT`, then
  other specs (`UNIQUE` / `PRIMARY KEY` / `CHECK`), in that order.
- For enum types, list values on separate lines with leading commas.
- For enum types, keep `AS ENUM` on its own line and put the opening `(` on
  the next line.
- For domains, keep AS and NOT NULL on their own lines.

### Functions

- Use `RETURNS TABLE` with aligned columns when returning a table shape.
- Put LANGUAGE and volatility/parallelism attributes on their own lines.
- Lowercase LANGUAGE values (e.g., `LANGUAGE sql`).
- Use `AS $$` for single-statement SQL bodies.
- Use `BEGIN ATOMIC ... END` for multi-statement SQL bodies.
- Keep plpgsql blocks indented one level; align DECLARE entries.
- For plpgsql one-liners, keep the semicolon on the same line.

### SELECT and CTEs

- Keep SELECT, FROM, WHERE, GROUP BY, ORDER BY, LIMIT, and RETURNING on their
  own lines.
- Align SELECT lists and expression aliases.
- Lowercase function names unless quoted.
- Use WITH (or WITH RECURSIVE) with CTEs separated by leading commas.
- Format multi-line CTEs as: `WITH` on its own line, the CTE name (and
  optional MATERIALIZED) on the next line, `AS` aligned with `WITH`, the
  opening `(` aligned with the CTE name's indentation, and a fully formatted
  inner query.
- Name CTE legs with concise nouns (for example `new_resource`,
  `new_provider`, `resource`, `provider`).
- Use SELECT DISTINCT ON with the distinct list on the next line.
- When a LEFT JOIN is only used for exclusion, prefer NOT EXISTS.

### JOINs and predicates

- Put JOIN type on its own line; keep the joined relation on the next line.
- Put ON and USING on their own lines; put USING columns in a parenthesized
  list.
- Use USING when join predicates compare same-named columns.
- Use CROSS JOIN LATERAL explicitly for lateral joins.
- Put AND/OR on their own lines with the predicate on the next line.
- Format CASE with WHEN/THEN/ELSE/END each on their own line.

### DML and COPY

- Use explicit column lists for INSERT.
- Put VALUES rows on separate lines with leading commas.
- Put UPDATE/SET/FROM/WHERE/RETURNING on their own lines.
- Put ON CONFLICT on its own line; put the action on the next line.
- Format COPY as `COPY ( SELECT ... ) TO ... WITH ( ... )` with each clause on
  its own line.
- Keep COPY options as `WITH (FORMAT CSV, HEADER)`.
