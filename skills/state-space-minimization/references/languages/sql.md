# SQL

PostgreSQL idioms for state-space minimization. Read together with the
language-agnostic principles files; this file gives the concrete shapes
and migration patterns.

The database is *the* persistent ingress. Every other ingress reparses
on each request; the database carries its narrowing forward across
restarts, deploys, and language rewrites. `CHECK`, `NOT NULL`, foreign
keys, and constraint triggers are the type system that survives every
code release. A row that the schema accepts is a row every future
consumer must handle, so the schema is the strictest place to spend the
state-space-minimization budget. Accept it back from the world only
through the narrowest column types the boundary can prove, and let the
trusted boundary be the database engine itself, not application code.

## Domains as branded primitives

`CREATE DOMAIN` is the SQL equivalent of `Schema.brand` /
`#[nutype]` / a Rust newtype with a smart constructor. The base type
plus its `CHECK` clauses define exactly which values are inhabitants of
the domain; every column of that type carries the proof.

```sql
-- src/lib/prisma/migrations/20260507000000_domain_label/migration.sql
CREATE DOMAIN
  public.label
AS
  text
CONSTRAINT check_01_not_empty                         CHECK (VALUE <> '')
CONSTRAINT check_02_max_length                        CHECK (char_length(VALUE) <= 256)
CONSTRAINT check_03_only_printable_chars              CHECK (VALUE ~ '\A[[:print:]]*\Z')
CONSTRAINT check_04_no_invisible_chars                CHECK (VALUE !~ '[​-‏ -  -⁯﻿]')
CONSTRAINT check_05_no_leading_or_trailing_whitespace CHECK (VALUE = trim(VALUE))
;
```

Every constraint earns its place against a specific failure mode:

- `check_01_not_empty` — the empty string is a 0-state hole the base
  `text` type allows; rule it out at the type level.
- `check_02_max_length` — bounds the upper end. Without one, a single
  bad row blows up indexes, telemetry, and downstream parsers.
- `check_03_only_printable_chars` — control characters are the SQL
  analogue of Unicode-injection bugs at higher layers.
- `check_04_no_invisible_chars` — zero-width and bidi-format characters
  let two distinct strings render identically and round-trip past
  string equality. Uniqueness leak.
- `check_05_no_leading_or_trailing_whitespace` — collapses the
  representation: `'foo '`, `' foo'`, and `'foo'` fold to one canonical
  inhabitant. *Restructuring to remove the constraint* applied at the
  value level.

Name every constraint. `CONSTRAINT check_NN_<reason>` makes the runtime
violation message point straight at the rule that fired — the SQL
equivalent of typed errors instead of `Result<T, String>`. Each domain
also gets a `COMMENT ON DOMAIN` for the narrowing intent.

## Domains compose

Define the wide type once, then layer specializations on top.

```sql
-- src/lib/prisma/migrations/20260507000020_domain_http_url/migration.sql
CREATE DOMAIN
  public.http_url
AS
  public.label
CHECK
  (VALUE ~ '\Ahttps://[^/:[:cntrl:][:space:]]+(?::[1-9]\d*)?(?:/[^[:cntrl:][:space:]]*)?\Z')
;
```

```sql
-- src/lib/prisma/migrations/20260507000050_domain_reason_text/migration.sql
CREATE DOMAIN
  reason_text
AS
  description
CONSTRAINT check_max_length CHECK (char_length(VALUE) <= 1024)
;
```

`http_url` inherits all five `label` constraints and adds the URL
grammar; `reason_text` inherits all five `description` constraints and
tightens the upper bound. The composition is the SQL equivalent of one
brand factory wrapping another — the same discipline as
`boundedPrintableString(...).pipe(Schema.brand(...))` from
`languages/typescript.md` § "Brand factories that enforce bounds at
every site". One source of truth per invariant; tighter contexts add
more checks rather than re-stating the base ones.

## Bounded primitives

Every domain has both a lower and upper bound. `principles.md` § "When
replacing String or Vec, go directly to the bounded form" is law: a
`text` column that is `not_empty` only is a `NonEmptyString` —
placeholder, not destination. Add the upper bound at the same time you
add the lower bound, or record (in the `COMMENT`) why you cannot.

```sql
-- src/lib/prisma/migrations/20260507000030_domain_non_negative_int32/migration.sql
CREATE DOMAIN
  public.non_negative_int32
AS
  integer
CHECK
  (VALUE >= 0)
;
```

```sql
-- src/lib/prisma/migrations/20260507000040_domain_positive_int32/migration.sql
CREATE DOMAIN
  public.positive_int32
AS
  integer
CHECK
  (VALUE > 0)
;
```

The base `integer` type already supplies the upper bound (PostgreSQL's
signed 32-bit ceiling). When the base type has no operational ceiling
(`text`, `bytea`, `numeric`, `jsonb`), pick one and encode it. Do not
ship a `text` column without a `char_length(VALUE) <= N` bound —
`description` caps at 8192, `label` caps at 256, `reason_text` caps at
1024 because reasons are read-by-humans-during-incident-review and 1024
is the headroom past which the field has stopped being a reason.

For columns that further tighten a domain at the table level, attach
the extra `CHECK` to the column definition:

```sql
-- src/lib/prisma/migrations/20260507000070_inference_tables/migration.sql
gpu_provider  label NOT NULL CHECK (gpu_provider ~ '\A[A-Z](?:_?[A-Z]+)*\Z' AND char_length(gpu_provider) <= 32)
provider_name label NOT NULL CHECK (char_length(provider_name) <= 64)
```

Promote the constraint into a domain the moment the same `CHECK`
appears at two columns. A repeated inline `CHECK` is the SQL form of
primitive obsession.

## Cross-field invariants

A domain only sees one value at a time. Cross-field invariants belong
on the table or, better, are designed away by changing the
representation (`ingress-and-boundaries.md` § "Restructure data to
remove the constraint").

Do not add a `CHECK` just because two fields make a rule visible. First
ask whether a different representation removes the redundant fields or
turns the rule into a single value. The table constraint is the fallback
when the wire shape or migration cost forces both fields to remain.

When the wire shape forces both fields to appear separately, encode the
cross-field rule as a named table-level `CHECK`:

```sql
-- src/lib/prisma/migrations/20260507000070_inference_tables/migration.sql
CONSTRAINT model_events_response_within_context CHECK
  (max_response_tokens IS NULL OR max_context_length IS NULL OR max_response_tokens <= max_context_length)
```

The two-`IS NULL` arms are the price of admitting nulls; reach this
form only after deciding nullable cannot be removed.

For non-overlap invariants (no two reservations cover the same room at
the same time, no two leases hold the same key concurrently), use
`EXCLUDE USING gist`:

```sql
CREATE EXTENSION IF NOT EXISTS btree_gist;

EXCLUDE USING gist (room_id WITH =, period WITH &&)
```

`EXCLUDE` is constructive — the constraint is encoded in the index
definition, so the only inhabitants the table accepts are the
non-overlapping ones. There is no application code that can forget the
check. When mixing scalar equality (`room_id WITH =`) with range
overlap (`period WITH &&`) in a GiST exclusion constraint, install
`btree_gist` (or specify the GiST operator class explicitly) — the
default operator classes do not include scalar equality for `integer`,
`bigint`, `text`, `uuid`, etc.

Before either form, ask: can the redundancy that produces the
constraint be removed? Two timestamp columns plus an "`end > start`"
`CHECK` becomes one `tstzrange` column. Two boolean columns plus an
"at most one is true" `CHECK` becomes one enum. The constraint
disappears, and so does the state space that needed guarding.

## Generated columns

`GENERATED ALWAYS AS ... STORED` removes the second copy of any
derivable fact. The slug is a function of the gpu_provider; a separate
nullable column would let it drift the moment a row is updated by code
that forgets to refresh it.

```sql
-- src/lib/prisma/migrations/20260507000070_inference_tables/migration.sql
slug label GENERATED ALWAYS AS (lower(replace(gpu_provider, '_', '-'))) STORED
```

This is the database normalization analogue of the in-memory
restructuring rule: keep the canonical form, derive the rest. The
derived column has the full type system of any other column — it is
typed `label`, indexable, and queryable, but not insertable. Application
code cannot disagree with the projection because application code
cannot write the column at all.

Use `GENERATED ALWAYS AS ... STORED` whenever the derived value will be
read often or indexed. For values cheap to compute and rarely read,
write them as a `STABLE` SQL function and call the function in the
view.

## Constructive sums

Closed value sets become `CREATE TYPE ... AS ENUM`. Every accepted
value is named; misspelled inputs are rejected by the type checker, not
by application code.

```sql
-- src/lib/prisma/migrations/20260507000067_inference_types/migration.sql
CREATE TYPE
  route_hosting_tier
AS ENUM
  ( 'dedicated'
  , 'public'
  )
;

CREATE TYPE
  model_lifecycle_kind
AS ENUM
  ( 'ACTIVE'
  , 'DEPRECATED'
  , 'OFFLINE'
  , 'RETIRED'
  )
;
```

Decision rule:

- **Closed set, fixed at deploy time, no per-row metadata** → enum.
  `route_hosting_tier`, `audience_channel_kind`,
  `reasoning_effort_kind` are all enum-shaped. Adding a value is a
  migration; that is a feature, not friction.
- **Open or growing set, or each value carries its own attributes** →
  lookup table with a foreign key. An enum forces a migration for
  every new value and stores no metadata; a lookup table accepts
  inserts but pays a join and loses the compile-time exhaustiveness.
- **The "set" is per-row repeated** → a child table with a foreign key
  back to the parent and an enum (or lookup-table FK) on the child for
  the kind. `model_capability_events`, `model_tag_events`, and
  `model_audience_events` all follow this pattern: the *set membership*
  is constructive (each row is one element) and the kind itself is a
  closed enum.

Partial indexes encode "this value is meaningful only in some rows"
without changing the column type:

```sql
CREATE INDEX route_config_events_route_slug_idx
  ON route_config_events(route_slug)
  WHERE route_slug IS NOT NULL;
```

That index is smaller and faster than a full index on a sparsely
populated column, and `WHERE route_slug IS NOT NULL` documents that
NULL is a real, distinct state — the rest of the design must agree.

## Foreign keys, ON DELETE, ON UPDATE

Every reference to another row needs a foreign key. Without one,
orphan rows are a representable invalid state — a `bigint` column with
no FK accepts every 64-bit integer, valid or not, and downstream joins
return spurious nothings instead of failing loudly.

```sql
provider_id bigint NOT NULL REFERENCES provider_ids (provider_id) ON DELETE RESTRICT ON UPDATE RESTRICT
```

`ON DELETE`/`ON UPDATE` close the relational state space. Pick the
behavior the domain actually wants:

- **`RESTRICT`** is the default for append-only history: a deleted
  parent would leave events pointing at nothing; refuse the delete.
  This is usually the right default for append-only event schemas.
- **`CASCADE`** when the child has no meaning without the parent (a
  comment on a deleted post). The cascade *is* the constructive shape:
  the child cannot outlive the parent.
- **`SET NULL`** when the relationship is optional and the child
  outlives the parent. Forces the column to be nullable; reach for it
  only after `RESTRICT` and `CASCADE` are both wrong.
- **`NO ACTION`** is PostgreSQL's default. For non-deferrable
  constraints it behaves like an immediate check, but if the FK is
  declared `DEFERRABLE` the violation can be checked at the end of the
  transaction. Use `RESTRICT` when the domain requires a
  non-deferrable, immediate refusal that cannot be deferred.

Never leave the action defaulted by omission. `ON DELETE NO ACTION` is
not the same as "I haven't decided" — it is a specific transition rule
and unwritten in code reviews it reads as carelessness.

## Append-only triggers as typestate

A row's lifecycle is a state machine. Encode the legal transitions
with constraint triggers; nothing else can express "this row may be
inserted, but never updated or deleted".

```sql
-- src/lib/prisma/migrations/20260507000072_inference_triggers/migration.sql
CREATE TRIGGER provider_events_are_append_only
BEFORE UPDATE OR DELETE ON provider_events
FOR EACH ROW
EXECUTE FUNCTION trigger_reject_update_or_delete()
;

CREATE TRIGGER check_provider_event_insert
BEFORE INSERT ON provider_events
FOR EACH ROW
EXECUTE FUNCTION trigger_check_provider_event_insert()
;
```

Three rules, applied to every event table:

1. The `_ids` table is `append_only`: identifiers are issued, never
   reissued, and never freed.
2. The `_events` table is `append_only`: history cannot be edited.
3. The `_events` insert trigger rejects rows that duplicate current
   state, regress `created_at`, or collide on the natural key with a
   different logical id.

These triggers are the row-level realization of typestate
(`principles.md` § "Encode invariants into types", rung 6). The legal
states for an event row are: not yet inserted → inserted (terminal).
There is no "edited" or "deleted" state because no transition leads
there. Application code cannot weaken the invariant by accident; the
constraint lives in the database.

Pair the row-level triggers with a `DEFERRABLE INITIALLY DEFERRED`
constraint trigger when the invariant spans more than one table within
a transaction:

```sql
CREATE CONSTRAINT TRIGGER model_events_require_route
AFTER INSERT ON model_events
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION trigger_assert_model_row_has_route()
;
```

This is the SQL form of "every model has at least one route" — the
`NonEmptyVec<ModelHostRoutingInput>` invariant from the Rust side. The
deferral lets the application insert the model row, then the routes,
then commit; the trigger fires at commit and rejects the whole
transaction if the routes never appeared.

## RAISE EXCEPTION at boundaries

Inside a trigger function, the boundary parser pattern returns nothing
useful — the trigger either lets the row through or stops the whole
transaction. Stop it with a typed error:

```sql
-- src/lib/prisma/migrations/20260507000071_inference_projection_functions/migration.sql
RAISE EXCEPTION 'route config event created_at must be strictly greater than the last event for the same route_config_id'
  USING ERRCODE = '23514',
        CONSTRAINT = 'route_config_events_created_at_monotonic',
        DETAIL = format(
          'New created_at %s is not strictly greater than last %s for route_config_id %s.',
          NEW.created_at, previous.created_at, NEW.route_config_id
        );
```

Rules:

- `ERRCODE` is one of the standard SQLSTATE codes (`23514` for `CHECK`
  violation, `23505` for unique violation, `23P01` for exclusion
  violation). The application catches by SQLSTATE, not by error string.
- `CONSTRAINT =` names the rule the way a column-level `CONSTRAINT`
  would. Application code branches on the name, not the message.
- `DETAIL =` carries the violating values for the operator who reads
  the log. The user-facing message stays stable; the detail carries the
  forensics.
- Never raise with just a string. A string-only `RAISE EXCEPTION` is
  the SQL equivalent of `Err("something broke".to_string())`.

## Schema namespaces as bounded contexts

A `CREATE SCHEMA` namespace can model a bounded context when the
database hosts multiple domain vocabularies. Each context-specific
schema owns the domains, types, tables, and functions whose meaning is
local to that context; shared primitives (a generic `description`,
`http_url`, `non_negative_int32`) live in a separate, explicitly named
shared schema only when several contexts reuse them.

The example below uses `inference` and `public` as the
context-specific and shared schemas — pick names that match your
project's vocabulary; do not treat `public` and `inference` as
canonical.

```sql
-- src/lib/prisma/migrations/20260507000050_domain_reason_text/migration.sql
CREATE SCHEMA inference;

SET LOCAL search_path TO inference, public;

CREATE DOMAIN
  reason_text
AS
  description
CONSTRAINT check_max_length CHECK (char_length(VALUE) <= 1024)
;
```

`reason_text` lives in the context-specific schema because "reason" is
an inference-context concept. `description` lives in the shared schema
because any context may want a multi-line text type. `architectural-scopes.md` §
"Bounded context" is enforced by file boundaries: the inference
schema's domains cannot drift to mean something different in another
schema because they live in a different namespace.

Cross-context flow goes through *views* and *functions*, not direct
table reads:

```sql
-- src/lib/prisma/migrations/20260507000071_inference_projection_functions/migration.sql
CREATE FUNCTION providers (arg_effective_at timestamptz DEFAULT effective_time())
RETURNS TABLE (provider_id bigint, created_at timestamptz, gpu_provider label,
               provider_name label, slug label, settings_payload inference_settings)
STABLE LANGUAGE sql BEGIN ATOMIC
  SELECT DISTINCT ON (provider_id) ...
END;
```

The function is the anti-corruption layer: outside callers see the
projected, narrowed shape, not the append-only event log underneath.
The wire-DTO/domain split appears naturally — `_events` tables are the
DTOs (one row per fact, raw insertable shape), `providers(at)` is the
domain projection (one row per logical provider, latest non-tombstoned
state). Outside callers never `SELECT` from `_events` directly; they
go through the function.

## NULL discipline

`NOT NULL` everywhere, by default. Three-valued logic is a state-space
leak: every nullable column adds the `+ 1` of `Option<T>` and forces
every consumer to handle "I don't know" alongside the real values.

```sql
-- src/lib/prisma/migrations/20260507000070_inference_tables/migration.sql
created_at  timestamptz NOT NULL DEFAULT effective_time()
reason      reason_text NOT NULL
provider_id bigint      NOT NULL REFERENCES provider_ids (provider_id) ON DELETE RESTRICT ON UPDATE RESTRICT
```

A column is nullable only when NULL means something specific that no
sentinel value could carry — "no override configured", "not yet
attempted", "explicitly cleared by the operator". Document it:

```sql
runner route_runner -- nullable: defaults to vllm in code when null
```

For unique constraints over a nullable column, use `NULLS NOT
DISTINCT` so two NULLs collide rather than slipping past the index:

```sql
CREATE UNIQUE INDEX route_config_events_identity_unique
  ON route_config_events (route_config_id, route_slug, url, ...)
  NULLS NOT DISTINCT
;
```

The default `NULLS DISTINCT` would let the table accept two rows that
agree on every column except a NULL one — the unique constraint would
silently not apply. Reach for `NULLS NOT DISTINCT` whenever a nullable
column appears in a `UNIQUE` index.

For partial uniqueness ("at most one *active* row per parent"), use a
partial unique index:

```sql
CREATE UNIQUE INDEX foo_one_active_per_parent
  ON foo(parent_id) WHERE state = 'ACTIVE';
```

The state space for "two active rows under one parent" goes from
application-bug-away to unrepresentable.

Domain `CHECK` constraints do not fire on NULL — three-valued logic
returns UNKNOWN, and only FALSE rejects. Put `NOT NULL` on the column,
not on the domain; domains are reusable, columns own their
nullability.

## Primary keys: surrogate vs natural, IDENTITY

The narrowest type that proves uniqueness. For event-sourced tables,
that is a `bigint GENERATED ALWAYS AS IDENTITY` surrogate key per event
row, plus a separate `_ids` table holding the logical identifier:

```sql
-- src/lib/prisma/migrations/20260507000070_inference_tables/migration.sql
CREATE TABLE provider_ids
  ( provider_id bigint PRIMARY KEY GENERATED BY DEFAULT AS IDENTITY );

CREATE TABLE provider_events
  ( provider_event_id bigint      PRIMARY KEY GENERATED ALWAYS AS IDENTITY
  , provider_id       bigint      NOT NULL REFERENCES provider_ids (provider_id) ...
  , created_at        timestamptz NOT NULL DEFAULT effective_time()
  );
```

Two distinct keys, two invariants:

- `provider_event_id` (`GENERATED ALWAYS`) is the surrogate event
  pointer. `ALWAYS` bans application-supplied values; the database is
  the only issuer.
- `provider_id` (`GENERATED BY DEFAULT`) is the logical identity. `BY
  DEFAULT` admits explicit inserts so seed data and tests can reuse
  stable ids; production lets the database assign.

Use a natural key as `PRIMARY KEY` only when it is genuinely immutable
*and* short. Mutable natural keys are a migration trap; prefer a
surrogate `bigint` and keep the natural key as a `UNIQUE` constraint
with `NULLS NOT DISTINCT`.

Avoid `serial` and `bigserial` — legacy spellings without the
`ALWAYS`/`BY DEFAULT` distinction. UUIDs are the answer when ids must
be issued offline or by independent producers; they are the wrong
answer when nothing in the design needs that property (the cost is
larger indexes and worse cache locality).

## Testing constraints

Test that bad rows are *rejected*, not just that good rows are
accepted. The trusted boundary is the database engine; the test must
feed it adversarial input and prove the engine refuses.

The pattern from `db/test/domains/public/label.test.sql` wraps the cast
in a plpgsql function that catches the failure and returns its
SQLSTATE, then exercises every accept/reject case from the same fixture
function:

```sql
-- db/test/domains/public/label.test.sql
CREATE FUNCTION
  test.try_cast_label(arg_input text)
RETURNS TABLE (ok boolean, sqlstate text, message text)
LANGUAGE plpgsql
AS $$
DECLARE
  cast_value public.label;
BEGIN
  cast_value := arg_input::public.label;
  RETURN QUERY SELECT true, NULL::text, NULL::text;
EXCEPTION
  WHEN OTHERS THEN
    RETURN QUERY SELECT false, SQLSTATE, SQLERRM;
END;
$$;

-- boundary value (max length 256) accepts; just past (257) rejects
SELECT * FROM test.try_cast_label(repeat('a', 256)) \gx /dev/stdout
SELECT * FROM test.try_cast_label(repeat('a', 257)) \gx /dev/stdout

-- every documented invalid class gets its own assertion
SELECT * FROM test.try_cast_label('')         \gx /dev/stdout
SELECT * FROM test.try_cast_label(E'a\tb')    \gx /dev/stdout
SELECT * FROM test.try_cast_label(E'a\nb')    \gx /dev/stdout
SELECT * FROM test.try_cast_label(E'a​b')    \gx /dev/stdout
SELECT * FROM test.try_cast_label(' a')       \gx /dev/stdout

-- NULL is documented because domain CHECKs do not fire on NULL;
-- NOT NULL belongs on the column, not on the domain
SELECT * FROM test.try_cast_label(NULL) \gx /dev/stdout
```

Mirrors `testing.md` § "test the trusted boundary" — every invalid
input class is asserted, the boundary value and boundary + 1 are both
tested, and the NULL case is recorded because it is *not* what naive
readers expect.

Assert the domain shape too, so a future refactor that weakens the
type fails the test:

```sql
SELECT conname AS name, pg_get_constraintdef(oid) AS definition
FROM pg_constraint
WHERE contypid = 'public.label'::regtype AND contype = 'c'
ORDER BY conname
\gx /dev/stdout
```

The `.expected` file is the strict matcher (`testing.md` § "Strict
matchers"). Whole psql output, including SQLSTATE codes and constraint
names, captured byte-for-byte. Any drift — a renamed constraint, a new
accepted character, a missing rejection — diff-fails. No spot-checks,
no `expect(...).toContain(...)` analogue.

For trigger-level invariants (cross-row, cross-table, append-only,
monotonic `created_at`), write the same shape against the `_events`
tables: insert the bad row, expect a specific SQLSTATE and constraint
name.

The two test harnesses are not interchangeable — pick one and commit
to it:

- **psql golden-output (`.expected` byte-for-byte diff)** captures the
  full output including SQLSTATE codes, error messages, and constraint
  names. Any drift fails the diff. Strongest matcher; brittle if
  PostgreSQL version output changes.
- **pgTAP** uses its own assertion idioms (`lives_ok`,
  `throws_ok(query, 'sqlstate', 'message_pattern')`, plus
  `pg_constraint`/`pg_attribute` catalog checks for constraint
  definitions). Looser than the golden-output diff but composable with
  ordinary test runners.

In either case the test asserts the SQLSTATE *and* the constraint
name, not just that *some* error occurred.

## Cross-references

- `principles.md` § "Encode invariants into types" — domains and
  constraint triggers map onto the predicative-newtype and typestate
  rungs.
- `primitive-obsession.md` — `text` and `bigint` columns are SQL
  primitive obsession; `CREATE DOMAIN` is the cure.
- `constructive-vs-predicative.md` — enums and `EXCLUDE` are
  constructive; `CHECK` is predicative; pick by which makes the
  invalid state unrepresentable by shape.
- `ingress-and-boundaries.md` — the database is the longest-lived
  ingress; restructure to remove cross-field constraints before
  reaching for table-level `CHECK`.
- `architectural-scopes.md` — schema namespaces are bounded contexts;
  cross-context flow goes through views and functions, not direct
  table reads.
- `testing.md` — strict-matcher principle realized by `.expected` file
  diffs over full psql output.
- `external-integration` skill — projection functions are the ACL
  between the event log and downstream consumers.
