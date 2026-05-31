# TypeScript

TypeScript idioms for state-space minimization. Read together with the
language-agnostic principles files; this file gives the concrete shapes
and library names. The Effect ecosystem gets dedicated treatment because
it pushes proof preservation, capabilities, and effect tracking further
than anything else mainstream TypeScript offers.

That dedicated treatment is not an instruction to introduce Effect into
every TypeScript project. Use Effect-specific rules when the codebase
already uses Effect, when the user asks to choose an Effect stack, or
when the concrete task is about preserving typed effects.

## Branded / nominal types

TypeScript is structurally typed by default, so `type UserId = string`
collapses with every other string. Branded types add a phantom property
to recover nominal distinctness:

```ts
declare const Brand: unique symbol;
export type Brand<T, B> = T & { readonly [Brand]: B };

export type UserId    = Brand<string, "UserId">;
export type EmailAddr = Brand<string, "EmailAddr">;

// The brand is only trusted if this module is the sole allowed cast site.
export function userId(s: string): UserId {
  if (!/^[a-z0-9-]{1,64}$/.test(s)) throw new ValidationError(s);
  return s as UserId;
}
```

The cast inside `userId` is the trusted-boundary moment. Ban raw brand
assertions outside constructors with a lint rule
(`@typescript-eslint/consistent-type-assertions` with
`assertionStyle: "never"` plus an allowlist) and require code review on
the few sites that remain. Or skip the hand-rolled brand entirely and
use `Schema.brand` (see "Effect-TS" below) so construction and parsing
share one boundary.

The `ts-brand` package provides a smaller helper if you want zero
boilerplate, but it is still a discipline contract — not a type-system
guarantee.

### Brand factories that enforce bounds at every site

A common ingress mistake with `Schema.brand` is to brand the wide parent
type and re-add bounds inconsistently per domain. Hoist the lower bound,
upper bound, and grammar into a brand *factory* so every domain type is
bounded by construction:

```ts
const PrintableAsciiString = Schema.String.pipe(
  Schema.pattern(/^[\x20-\x7E]+$/),
);

export const boundedPrintableString = <
  const BrandName extends string,
  const Min extends number,
  const Max extends number,
>(
  brand: BrandName,
  min: Min,
  max: Max,
) =>
  PrintableAsciiString.pipe(
    Schema.minLength(min),
    Schema.maxLength(max),
    Schema.brand(brand),
  );

export const ProviderName = boundedPrintableString("ProviderName", 1, 64).pipe(
  Schema.pattern(providerSlugPattern),
);
```

The factory makes "unbounded brand" a compile error: the only entry
point requires both bounds. Apply the same pattern to byte arrays,
integer ranges, bigint ranges, and `Uint8Array` payloads.

### Layer brands to capture role distinct from shape

When two domain values share a wire shape but mean different things
(payment destination vs payee wallet, asset contract vs operator
allowlist entry), brand the role on top of the shape:

```ts
export const EvmAddressHex = Schema.String.pipe(
  Schema.pattern(/^0x[0-9a-fA-F]{40}$/),
  Schema.brand("EvmAddressHex"),
);

export const AssetSymbol         = EvmAddressHex.pipe(Schema.brand("AssetSymbol"));
export const DestinationAddress  = EvmAddressHex.pipe(Schema.brand("DestinationAddress"));
export const SpendPolicyAsset    = EvmAddressHex.pipe(Schema.brand("SpendPolicyAsset"));
```

Both still parse as 0x-prefixed 40-hex strings, but `transfer(asset,
destination)` cannot accidentally swap them at the call site.

### Name every downcast to a third-party type

When a third-party API requires a wider type than your branded domain
type (`viem` wants `0x${string}`, AWS SDK wants raw string ARNs), expose
one named conversion per FFI seam rather than letting cast assertions
spread:

```ts
export type EvmPrefixedHex = `0x${string}`;

export const evmHash256ForViem = (v: EvmHash256Hex): EvmPrefixedHex =>
  v as EvmPrefixedHex;

export const evmPrivateKeyForViem = (v: EvmPrivateKeyHex): EvmPrefixedHex =>
  v as EvmPrefixedHex;
```

The lint allowlist now contains a finite set of audited functions, and
"who downcasts `EvmHash256Hex` to viem's hex" is one grep.

## Discriminated unions

The TypeScript workhorse for closed sums. The convention is a literal
`_tag` (or `kind`, `type`) field; `never`-checking in the default arm
forces exhaustiveness:

```ts
type Availability =
  | { _tag: "Available" }
  | { _tag: "Offline" }
  | { _tag: "Busy"; until: Date }
  | { _tag: "Banned"; since: Date; reason: BanReason };

function describe(a: Availability): string {
  switch (a._tag) {
    case "Available": return "online";
    case "Offline":   return "offline";
    case "Busy":      return `busy until ${a.until.toISOString()}`;
    case "Banned":    return `banned since ${a.since.toISOString()}`;
    default: {
      const _exhaustive: never = a;
      return _exhaustive;
    }
  }
}
```

`ts-pattern` provides exhaustive `match` with the same `never` check
done structurally; preferred for nested or wide unions.

## Effect's `Match` module

When the project already uses Effect, prefer the `Match` module over a
hand-rolled `switch` plus `never` arm. `Match` returns a value (no
expression-vs-statement mismatch in `Effect.gen`), composes with
pipelines, and reports missing cases at the call site instead of at a
sentinel default.

Two equivalent styles, picked by what reads cleanest:

```ts
import { Match } from "effect";

// 1. Record style for sums on `_tag`. Omit a key and the call fails to compile.
const describe = (a: Availability): string =>
  Match.valueTags(a, {
    Available: () => "online",
    Offline:   () => "offline",
    Busy:      ({ until }) => `busy until ${until.toISOString()}`,
    Banned:    ({ since }) => `banned since ${since.toISOString()}`,
  });

// 2. Pipe style for literal sets or mixed predicates; close with Match.exhaustive.
const finishForReason = (reason: FinishReason): StreamFinish =>
  Match.value(reason).pipe(
    Match.when("stop",   () => ({ _tag: "Finished", reason: "stop" }   as const)),
    Match.when("length", () => ({ _tag: "Finished", reason: "length" } as const)),
    Match.when("filter", () => ({ _tag: "Failed",   message: "filtered" } as const)),
    Match.exhaustive,
  );

// 3. Tags inside a pipe when you want record-style readability but composable:
Match.value(decoded).pipe(
  Match.tags({
    Deltas: (d) => emitDeltas(d.values),
    Done:   ()  => done(),
  }),
  Match.exhaustive,
);
```

Use `Match.tag("Foo", ...)` plus `Match.orElse(...)` only when one or
two cases need special handling and a default branch is acceptable;
prefer the exhaustive forms above for state machines, error-shape
narrowing, and any closed sum where adding a variant should fail to
compile.

## `as const` and template literal types

For closed sets of strings, `as const` narrows the inferred type from
`string` to the union of literals. Template literal types push grammar
into the type system:

```ts
const ROLES = ["admin", "editor", "viewer"] as const;
type Role = (typeof ROLES)[number]; // "admin" | "editor" | "viewer"

type Hex = `#${string}`;
type IsoDate = `${number}-${number}-${number}`;
type Email = `${string}@${string}.${string}`;
```

Template literal types are lightweight; do not stretch them to model
full grammars (regex, JSON, etc.) — at that point use a parser.

## Schema-first parsing

The cleanest realization of *parse, don't validate* in TypeScript: a
schema declaration is both the runtime parser and the static type. Pick
one library and use it at every ingress.

| Library      | Strength                              | When to choose                   |
|--------------|---------------------------------------|----------------------------------|
| **Zod**      | de facto standard, large ecosystem    | most apps; quick to start        |
| **Effect Schema** | bidirectional codecs + Effect integration | apps already using Effect-TS |
| **ArkType**  | TS syntax as the schema DSL           | DX-focused; smaller bundle       |
| **Valibot**  | tree-shakeable, minimal               | bundle size sensitive            |
| **io-ts**    | fp-ts ecosystem, codec pairs          | codebases already on fp-ts       |
| **runtypes** | minimal, predates Zod                 | legacy or minimal-deps           |
| **Yup**      | older form-validation focus           | inherited code only              |

Skeleton (Zod):

```ts
import { z } from "zod";

const CreateUserSchema = z.object({
  email: z.string().email().max(254),
  age:   z.number().int().min(18).max(150),
  role:  z.enum(["admin", "editor", "viewer"]),
});

export type CreateUser = z.infer<typeof CreateUserSchema>;

export function parseCreateUser(input: unknown): CreateUser {
  return CreateUserSchema.parse(input); // throws on failure
}
```

Use `.safeParse(input)` when failure is expected; it returns a
discriminated `{ success: true | false }` and avoids exception flow.

The principle: every ingress (HTTP body, env var, query string,
localStorage value, message-bus payload, third-party API response, file
read) goes through the schema. The domain type is the one inferred from
the schema, never a hand-written interface that drifts from the parser.

### Cross-field invariants via `Schema.filter`

Effect Schema lets you attach a predicate to an entire `Struct`, not
just individual fields. Use this for cross-field constraints that
cannot be expressed at field level:

```ts
const RegisteredModel = Schema.Struct({
  contextWindowTokens: ModelTokenCount,
  maxOutputTokens:     ModelTokenCount,
}).pipe(Schema.filter((m) => m.maxOutputTokens <= m.contextWindowTokens));

const FreshBalanceProofFields = Schema.Struct({
  amount:       AssetAmount,
  ageMillis:    DurationMillis,
  maxAgeMillis: DurationMillis,
}).pipe(Schema.filter((p) => p.ageMillis <= p.maxAgeMillis));
```

Prefer restructuring (a single derived field — see
`ingress-and-boundaries.md` § "Restructure data to remove the
constraint") when possible. Reach for struct-level filter when the wire
format forces both fields to appear separately.

### Bounded, unique, canonical-equality collections

Pair `Schema.minItems` / `Schema.maxItems` with a `Schema.filter` that
checks uniqueness under the *canonical* form for the element type, not
the raw form (EVM addresses compared lowercase, IDs compared after
normalization, URLs compared after canonicalization):

```ts
const uniqueEvmAddresses = (xs: readonly EvmAddressHex[]) =>
  new Set(xs.map(canonicalEvmAddressHex)).size === xs.length;

const SpendPolicyNetworks = Schema.Array(EvmAddressHex).pipe(
  Schema.minItems(1),
  Schema.maxItems(128),
  Schema.filter(uniqueEvmAddresses),
);
```

A non-canonical equality check accepts duplicates that round-trip into
the same canonical form, which is the same defect a weak matcher has in
tests: too many states accepted.

## Effect-TS

Effect (`effect` package) is a runtime + type system layered on
TypeScript that brings effect tracking, structured errors, and
capability/dependency injection to mainstream JS/TS. For state-space
minimization it carries three channels in the type signature:

```ts
Effect<Success, Error, Requirements>
```

- **Success**: the value the effect resolves to (return-value state space)
- **Error**: typed errors the effect can fail with (failure state space)
- **Requirements**: capabilities/services the effect needs (authority
  state space)

Each channel is independently narrowable. Together they make the
function signature carry far more proof than `Promise<T>` plus thrown
exceptions.

### Tagged errors instead of thrown exceptions

Throwing turns a failure into an untyped `unknown`. `Data.TaggedError`
is the Effect-TS replacement: errors are typed, exhaustively
matchable, and travel in the `Error` channel of `Effect`.

```ts
import { Data, Effect } from "effect";

class UserNotFound extends Data.TaggedError("UserNotFound")<{
  readonly id: UserId;
}> {}

class DbUnavailable extends Data.TaggedError("DbUnavailable")<{
  readonly cause: unknown;
}> {}

const findUser = (id: UserId): Effect.Effect<User, UserNotFound | DbUnavailable, Database> =>
  Effect.gen(function* () {
    const db = yield* Database;
    const row = yield* db.queryOne(id);
    if (row === null) return yield* new UserNotFound({ id });
    return User.fromRow(row);
  });
```

The signature now lists exactly which failures are possible and exactly
which capabilities the function needs. The compiler refuses to call
`findUser` without supplying a `Database` and refuses to ignore either
error case.

### Bundle `TaggedError` constructors into a namespace

When a module owns a closed family of tagged errors, declare the union
type and a constructor object under the same exported name:

```ts
export type AppError = ParseError | ConfigError | PersistenceError;

export const AppError = {
  Parse:       (p: ParseErrorPayload)       => new ParseError(p),
  Config:      (p: ConfigErrorPayload)      => new ConfigError(p),
  Persistence: (p: PersistenceErrorPayload) => new PersistenceError(p),
} as const;

export const isAppError = (v: unknown): v is AppError =>
  v instanceof ParseError || v instanceof ConfigError || v instanceof PersistenceError;
```

Call sites read `AppError.Parse({ source, message })` and the original
classes remain available for `Effect.catchTag("Parse", ...)`. Pair with
the `isAppError` guard for FFI / unknown input.

### `Effect.fn` for traced generator functions

`Effect.fn(name)(generatorFn)` is the modern Effect 3.x replacement for
hand-rolled `(args) => Effect.gen(function* () { ... })` followed by
`Effect.withSpan(name)`. The function carries a tracing span and infers
the full `Effect<A, E, R>` from the generator's `yield*` sites.

```ts
export const writeDurably = Effect.fn("writeDurably")(function* (
  deps: DurableStoreDependencies,
  path: string,
  content: string,
) {
  yield* mkdirRecursive(deps, dirname(path));
  // ... rest of the body
});
```

Use `Effect.fnUntraced` for hot paths where the span overhead is not
worth the tracing benefit (numerical inner loops, parsers called per
byte). The signature is the same; only the runtime tracing differs.

Why this matters for state-space minimization: every named `Effect.fn`
declares its full `<A, E, R>` shape at the declaration site, including
`R`. The `R` channel is the *authority state space*; naming the
function pins what authority it requires.

### Multi-step parsing with `Either.gen` / `Effect.gen`

For a parse-then-authorize-then-bind workflow, write each step as a
function returning the narrowed type, then chain with `Either.gen` or
`Effect.gen`:

```ts
import { Either } from "effect";

export const parseRetryProofForRequest = (rawId: unknown, raw: unknown) =>
  Either.gen(function* () {
    const id    = yield* parseLogicalRequestId(rawId);   // narrow rawId  -> id
    const proof = yield* parseRetryProof(raw);           // narrow raw    -> proof
    if (proof.logicalRequestId !== id) {
      return yield* Either.left(retryError("proof does not bind to request"));
    }
    return proof;                                        // returns the bound proof
  });
```

Each `yield*` narrows the value one rung at a time, short-circuits on
the first failure, and the final `return` carries the strongest proof
reachable. The generator block is the TypeScript analogue of Rust's `?`
operator over `Either` / `Effect`. Pair with the schema-union
transition-table pattern (below) when the workflow is a state machine:
parse the `(state, event)` pair, then dispatch with `Match.valueTags`.

### Schema as parser AND static type

Effect Schema is the schema module in the main `effect` package
(Effect 3.x merged the standalone `@effect/schema` into `effect`).
For the patterns in this file use Effect ≥ 3.12 — `Schema.decodeUnknown*`
is documented since 3.10, `Effect.fn` since 3.11, `Effect.fnUntraced`
since 3.12. For earlier 3.x projects, check the installed minor before
applying these examples.

Schema produces *bidirectional* codecs — decode (parse) and encode
(serialize) — so the boundary type and the wire type can both be
derived from one declaration:

```ts
import { Schema } from "effect";

const Email = Schema.String.pipe(
  Schema.pattern(/^[^@\s]+@[^@\s]+\.[^@\s]+$/),
  Schema.maxLength(254),
  Schema.brand("Email"),
);
type Email = Schema.Schema.Type<typeof Email>; // string & Brand<"Email">

const CreateUser = Schema.Struct({
  email: Email,
  age:   Schema.Number.pipe(Schema.int(), Schema.between(18, 150)),
  role:  Schema.Literal("admin", "editor", "viewer"),
});

const decode = Schema.decodeUnknown(CreateUser);
// decode :: unknown -> Effect<CreateUser, ParseError>
```

Notes for proper Effect-TS usage:

- prefer `Schema.brand(...)` for domain types (parse-and-brand in one
  step) instead of a separate brand helper
- prefer `Schema.decodeUnknown` (Effect-returning) over `decodeSync`
  (throws); the Effect form composes with the rest of the pipeline
- expose the domain type via `Schema.Schema.Type<typeof X>`; do not
  re-declare an `interface` that duplicates the schema
- prefer `Schema.decodeUnknownEither(...)` (or `Schema.decodeEither` for
  already-typed input) when the call site wants `Either`-flavored
  short-circuiting in `Either.gen`

### `Schema.parseJson` composition for JSON ingress

Compose `Schema.parseJson(T)` to parse a `string` directly into `T` in
one pass — never `JSON.parse(...)` followed by `Schema.decodeUnknown`.
The composed codec rejects malformed JSON and structurally invalid
payloads with one error path, and `onExcessProperty: "error"` rejects
unknown keys instead of silently widening the domain:

```ts
const decodeWireMessage = (data: string) =>
  Schema.decodeUnknownEither(
    Schema.parseJson(WireMessage),
    { onExcessProperty: "error" },
  )(data);
```

Pair with `Schema.encodeUnknown(Schema.parseJson(T))` for outbound
serialization so the same schema is the only encoder.

### `Schema.transform` for wire DTO ↔ domain conversion

When the on-the-wire shape and the in-memory domain shape differ,
declare both schemas and link them with `Schema.transform`. Encode goes
domain → wire; decode goes wire → domain. The transform is the single
trusted boundary and the schema pair guarantees round-trip safety.

```ts
const PersistedRecordPayload = Schema.Struct({ /* ... */ });

const PersistedRecord = Schema.Struct({
  committedAt: Timestamp,                    // branded Date
  payload:     PersistedRecordPayload,
});

const PersistedRecordJson = Schema.Struct({
  committedAt: Schema.transform(Schema.DateTimeUtc, Timestamp, {
    decode: DateTime.toDateUtc,
    encode: DateTime.unsafeFromDate,
  }),
  payload: PersistedRecordPayload,
});
```

Avoid hand-written `toJson` / `fromJson` pairs that drift over time.
The transform's two arrows are the only place a value crosses the wire
boundary.

### State machines as `Schema.Union` of `Schema.TaggedStruct`

For a domain state machine, declare each state and event as a
`Schema.TaggedStruct` and assemble them into a `Schema.Union`. Do the
same for the *transition relation*: list the legal `{ state, event }`
pairs as a union, so an illegal pair fails schema decoding before any
switch runs.

```ts
const Unpaid             = Schema.TaggedStruct("Unpaid", {});
const PaymentRequired    = Schema.TaggedStruct("PaymentRequired", {});
const ReceivePmtRequired = Schema.TaggedStruct("ReceivePaymentRequired", {});
const AuthorizePayment   = Schema.TaggedStruct("AuthorizePayment", {});

// Each row is a legal (state, event) pair; missing rows are unrepresentable.
export const PaymentTransition = Schema.Union(
  Schema.Struct({ state: Unpaid,          event: ReceivePmtRequired }),
  Schema.Struct({ state: PaymentRequired, event: AuthorizePayment }),
  // ...
);
```

Pair with a `Match.valueTags` transition function that produces the
next state, and a `parseAndTransition*` wrapper that decodes the wire
pair through the union before dispatching. This is the constructive
form of a state machine: representable transitions equal valid
transitions. Split states with no outgoing edges into a separate
`TerminalState` union so the transition function's input type excludes
them — the dead `illegalTransition` arms then fail to compile.

### Capabilities as Layers

Authority (database, HTTP client, clock, logger, configuration) lives
in the `Requirements` channel and is provided by `Layer`. This is the
TypeScript realization of the capability-token pattern in
`ingress-and-boundaries.md`:

```ts
import { Context, Effect, Layer } from "effect";

class Database extends Context.Tag("Database")<
  Database,
  {
    readonly queryOne: (id: UserId) => Effect.Effect<UserRow | null, DbUnavailable>;
  }
>() {}

const DatabaseLive = Layer.succeed(Database, {
  queryOne: (id) => Effect.tryPromise({
    try:   () => pgPool.query(...).then(rows => rows[0] ?? null),
    catch: (cause) => new DbUnavailable({ cause }),
  }),
});

// findUser cannot be run without a Database in scope
const program = findUser(myId).pipe(Effect.provide(DatabaseLive));
```

The `Requirements` channel makes "what authority does this code need"
visible in every signature. A function with `R = never` has no ambient
authority — Effect-TS's analogue of an effect-pure function.

### Tag/Service split

Define the service interface as a named `interface`, not inline in the
`Context.Tag` generic. The Tag class re-uses the interface; layer
factories return the interface; tests construct the interface directly
and wrap it in `Layer.succeed`. This keeps the *capability contract*
(the interface) testable without the Effect runtime.

```ts
export interface SignerService {
  readonly sign: (payload: SignableBytes) =>
    Effect.Effect<RedactedValue<SignatureHex>, SigningError>;
}

export class Signer extends Context.Tag("app/Signer")<Signer, SignerService>() {}

export const makeSignerService = (deps: SignerDeps): SignerService => ({
  sign: (payload) => Effect.gen(function* () { /* ... */ }),
});

export const signerLayer = (deps: SignerDeps): Layer.Layer<Signer> =>
  Layer.succeed(Signer, makeSignerService(deps));
```

### Layer factories take a typed input record

Layer factories should accept a single readonly input record, not
positional dependencies. Every external function the layer needs
(including ambient ones like `fetch`, `crypto.randomBytes`, `Date.now`,
`node:fs` operations) goes into the input record. This makes the
layer's own dependencies explicit at the call site and removes ambient
authority leaks at one boundary instead of dozens.

```ts
export interface FetchHttpClientInput {
  readonly fetch: typeof fetch;
  readonly timeoutMillis: PositiveDurationMillis;
  readonly signalForTimeout: (ms: number) => AbortSignal; // not AbortSignal.timeout
}

export interface CryptoNonceInput {
  readonly bytes: (byteLength: NonceByteLength) => Uint8Array;
  readonly encodeHex: (bytes: Uint8Array) => string;       // not Buffer.from
  readonly byteLength: NonceByteLength;
}
```

Default arguments that fall back to ambient APIs (`= globalThis.fetch`,
`= process.env`, `= Date.now`) reintroduce the leaks the input record
removed. Force every caller to supply the input record explicitly.

### Test fakes as alternate Layers, not alternate Tags

Production and test code share the *same* `Context.Tag` and `*Service`
interface. Only the `Layer` differs. A `fakes/` directory mirroring
`shell/` (or `live/` / `prod/`) keeps this 1:1 — each shell file has a
fakes counterpart that produces `Layer.Layer<SameTag>`.

If a fake needs a different *signature*, that is a smell: the Tag
contract is leaking implementation. Either fix the Tag interface or
add a state-inspection helper that returns from outside the Effect
system (e.g. an `events: AuditEvent[]` array exposed alongside the
`Layer`, not in it).

### Capability tokens with `unique symbol` and `Schema.declare`

For *issued* proofs (the trusted module produced this value, callers
cannot forge it from JSON), combine three layers: predicative fields
validated by Schema, a non-enumerable `unique symbol` brand only the
trusted module can attach, and a `Schema.declare` that re-checks the
brand at decode time so the proof can appear inside larger schemas
without losing its binding.

```ts
const freshBalanceProofSymbol: unique symbol = Symbol("FreshBalanceProof");

const FreshBalanceProofFields = Schema.Struct({
  amount:           AssetAmount,
  ageMillis:        DurationMillis,
  maxAgeMillis:     DurationMillis,
  nonSpendingProof: Schema.Literal(true),
}).pipe(Schema.filter((p) => p.ageMillis <= p.maxAgeMillis));

export interface FreshBalanceProof
  extends Schema.Schema.Type<typeof FreshBalanceProofFields> {
  readonly [freshBalanceProofSymbol]: true;
}

const isFreshBalanceProof = (v: unknown): v is FreshBalanceProof =>
  typeof v === "object"
  && v !== null
  && (v as Record<typeof freshBalanceProofSymbol, unknown>)[freshBalanceProofSymbol] === true;

export const FreshBalanceProof: Schema.Schema<FreshBalanceProof> =
  Schema.declare(isFreshBalanceProof);

export const makeFreshBalanceProof = (v: unknown) =>
  Either.map(
    Schema.decodeUnknownEither(FreshBalanceProofFields)(v),
    (fields) =>
      Object.defineProperty(
        Object.freeze({ ...fields }),
        freshBalanceProofSymbol,
        { value: true, enumerable: false },
      ) as FreshBalanceProof,
  );
```

This is the value-level realization of capability tokens — distinct
from the `R`-channel realization via `Layer`. Use it when the proof
must travel through `Schema.decodeUnknown` of an aggregate (e.g. a
spend-policy input) without being forgeable from JSON. Use a
*non-enumerable* symbol (`Object.defineProperty` with
`enumerable: false`); ordinary `{ ...fields, [sym]: true }` literals
attach the symbol *enumerably* and object spread will preserve it,
which forges the proof. Object spread of a non-enumerable symbol
drops it; JSON round-trip drops every symbol; only the trusted
constructor re-attaches it.

### Env / CLI parsing with allowlist + Schema decode + scrub

The Zod skeleton above shows the basic shape; in Effect-TS the full
discipline is:

```ts
export const appEnvironmentKeys = [
  "APP_COMPATIBILITY_PROFILE",
  "APP_SIGNING_PRIVATE_KEY",
  "APP_RETRY_LIMIT",
  // ...
] as const;

const appEnvironmentKeySet = new Set<string>(appEnvironmentKeys);

const findUnknownEnvKey = (env: Record<string, string | undefined>) =>
  Object.keys(env).find(
    (key) => key.startsWith("APP_") && !appEnvironmentKeySet.has(key),
  );

export const parseEnvironmentConfig = (
  env: Record<string, string | undefined>,
) =>
  Either.gen(function* () {
    const unknown = findUnknownEnvKey(env);
    if (unknown !== undefined) {
      return yield* Either.left(
        AppError.Config({ key: unknown, message: "unknown" }),
      );
    }
    const profile = yield* parseCompatProfile(env.APP_COMPATIBILITY_PROFILE);
    const retry   = yield* parseRetryCount(env.APP_RETRY_LIMIT);
    const key     = yield* parseRedactedSigningKey(env.APP_SIGNING_PRIVATE_KEY);
    return { profile, retry, key };
  });

export const parseAndScrubEnvironmentConfig = (
  env: Record<string, string | undefined>,
) => {
  const parsed = parseEnvironmentConfig(env);
  return Either.map(parsed, (config) => {
    for (const key of appEnvironmentKeys) Reflect.deleteProperty(env, key);
    return config;
  });
};
```

Rules:

1. Declare the *closed* allowlist of accepted keys as `as const` and
   build a `Set` for membership.
2. Reject any key that matches your prefix but is not in the allowlist
   — typos fail loudly instead of silently being ignored.
3. Each accepted key gets its own narrowing parser that returns a
   branded domain type, never a raw `string`.
4. Secrets parse into `Redacted.Redacted<Brand>` or a domain alias such
   as `RedactedValue<Brand>` (`import { Redacted } from "effect"`) so
   they cannot be accidentally logged.
5. After parsing, *delete* every consumed key from the env map —
   especially secrets — so later code in the process cannot re-read
   them.

### Avoid these mistakes

- **Do not mix raw `Promise` with `Effect`** in the middle of a
  pipeline. Convert at the boundary with `Effect.tryPromise` so the
  failure type is preserved.
- **Do not throw inside an `Effect.gen` block**. Use `Effect.fail` or
  yield a `TaggedError` so the failure stays in the `Error` channel.
- **`Effect.runPromise` is permitted only at program entry points or
  when bridging to a foreign callback / SDK that does not accept
  `Effect`**. Wrap the run with `Effect.either` and re-throw at the
  bridge so the typed error channel survives across the boundary:

  ```ts
  export const makeOpenAISdkFetchBridge =
    (submit: SdkSubmit): typeof fetch =>
    async (url, init) => {
      const result = await Effect.runPromise(Effect.either(submit(url, init)));
      return Either.match(result, {
        onLeft:  (error) => { throw error; },
        onRight: (response) => response,
      });
    };
  ```

- **Do not rely on `any`-typed services**. Always declare a `Tag` so
  the `Requirements` channel records the dependency.
- **Do not skip schemas at boundaries**. Every `unknown` from the
  outside world goes through `Schema.decodeUnknown`.
- **Do not catch errors with `Effect.catchAll`** when a typed
  `Effect.catchTag` or `Effect.catchTags` would preserve the remaining
  error narrowing.

## HTTP boundary patterns

The HTTP layer is the dominant ingress in most TypeScript services.
Treat `Response` and `RequestInit` as wire-shaped DTOs and never let
them leak past the boundary into domain code.

### HTTP response as a parsed value object

Wrap `fetch` so callers see a parsed `HttpResponse` value, not the
runtime `Response`. Inside the wrapper:

- parse the status as a branded `HttpStatusCode` (100..599) — not
  `number`
- parse body bytes through a byte-bound `ResponseBodyBytes` schema,
  after a `content-length` short-circuit so oversized payloads never
  allocate
- parse each header pair through `Schema.Tuple(HttpHeaderName,
  HttpHeaderValue)`, canonicalize names to lowercase, and reject
  duplicates after canonicalization
- freeze the resulting `Map` so headers cannot be mutated downstream
- expose body via a getter that clones the `Uint8Array` so consumers
  cannot retroactively mutate the parsed value
- the `fetch` failure type is one tagged error
  (`AppError.Protocol({ profile: "http", ... })`); never let
  `Response`, `TypeError`, or `AbortError` escape the wrapper

Pair with a generic `decodeHttpResponseJson<T>(response, schema,
onFailure): Either<T, AppError>` so the *intermediate* `unknown` from a
two-step "parse JSON then validate" never appears as a representable
state in the call graph.

### SDK fetch bridge: reject, do not replace

When wrapping a third-party SDK that calls into your transport, the
wrapper's parse function should *reject* requests that mention headers
your transport owns — never silently overwrite. Forbidden-name `Set` is
checked after canonicalization (lowercase). This makes the SDK-author
contract a parser-enforced fact and prevents a future SDK upgrade from
sneaking an `Authorization` header past you:

```ts
const transportOwnedHeaderNames = new Set([
  "authorization", "x-payment", "payment-required", /* ... */
]);

const forbidTransportHeaders = (entries: readonly (readonly [string, string])[]) =>
  entries.some(([name]) => transportOwnedHeaderNames.has(name.toLowerCase()))
    ? Either.left(bridgeError("SDK forbids transport-owned headers"))
    : Either.right(entries);
```

### Streaming codec discipline

Streaming parse uses three layered sum types so no intermediate state
is implicit:

1. **Frame result** (`DecodedStreamData = Deltas | Done`) — the
   per-line parse outcome.
2. **Loop step** (`DecodedDeltas | Done | DecodeFailed`) — the one-step
   transition.
3. **Loop terminus** (`DoneObserved | DecodeFailed | OpenEnded`) — how
   the loop ended.

The accumulator is itself a sum (`NoTextStarted | TextStarted`) so the
type tells you whether `text_start` has been emitted. A `mergeFinish`
helper keeps the first non-`Open` finish so a late `[DONE]` after
`finish_reason` does not overwrite the real reason.

Caveat: it is tempting to read the whole stream with `new
Response(body).text()` before splitting frames, which sidesteps
partial-frame edge cases by eliminating streaming. The frame-by-frame
discipline above survives a port to incremental SSE / chunked
transfer; the buffered shortcut does not. Decide which trade-off you
want before writing the codec, and document it.

## Closed-world data with `Object.freeze`, `readonly`, and `const`

For value objects, prefer `readonly` fields and `as const` so the type
is genuinely immutable in TypeScript's view:

```ts
type DateRange = Readonly<{
  _tag: "DateRange";
  start: Date;
  end: Date;
}>;

export function dateRange(start: Date, end: Date): DateRange {
  if (end < start) throw new ValidationError("end before start");
  return Object.freeze({ _tag: "DateRange", start, end });
}
```

`readonly` is shallow; deep immutability needs `Readonly<T>` recursion
or a library (`type-fest`'s `ReadonlyDeep`, Effect's `Data.struct`).

## Test matchers in TypeScript

See `references/testing.md` § "Identity contract vs structural
contract" for the language-agnostic principle. The Jest / Vitest
matcher trio maps to it directly:

| Matcher           | Accepted set                                                 | Use for                                                       |
|-------------------|--------------------------------------------------------------|---------------------------------------------------------------|
| `.toBe(x)`        | `Object.is(actual, x)`                                       | primitives; identity contracts (memoization, caching, singletons) |
| `.toStrictEqual(x)` | deep equality + class tags + own properties                | structural contracts                                          |
| `.toEqual(x)`     | deep equality, **ignores class tags and extra `undefined`** | banned                                                        |

`.toEqual` admits a plain object where a `Date` was expected, an
instance of a different class with the same fields, and objects with
extra `undefined` properties the contract did not include. Each is an
accepted state outside the contract's range — the same defect a wide
return type has on the production side. The two legitimate matchers
for object / array comparison are `.toBe` (identity) and
`.toStrictEqual` (structural).

Enforce mechanically: `eslint-plugin-jest`'s `prefer-strict-equal`
(autofixes `.toEqual` → `.toStrictEqual`) plus a
`no-restricted-syntax` rule that rejects any remaining `.toEqual` /
`.toEqual.bind` / aliased call site. Review-only enforcement decays;
lint enforcement does not — see `references/testing.md` § "Lints
carry the matcher-tightening principles".

```ts
// weak: spot-check
expect(user.id).toBe(expectedId);

// strong: full structural equality, class tags and own-props enforced
expect(user).toStrictEqual({
  id: expectedId,
  role: "admin",
  email: expectedEmail,
});
```

```ts
// weak: tag only
expect(result).not.toBeNull();

// strong: full payload, strict structural
expect(result).toStrictEqual({ _tag: "Found", user: expectedUser });
```

For mocks, prefer `vi.fn()` / `jest.fn()` with `mock.calls` asserted
exactly via `.toStrictEqual`, not `toHaveBeenCalledWith` (which is a
single-call subset match). For property tests, use `fast-check` and
assert canonical outputs.

For Effect-TS programs, `@effect/vitest` provides `it.effect` which
runs effectful tests inside the Effect runtime; assert on the typed
`Exit` value rather than awaiting a `Promise`.

## Cross-references

- `principles.md`, `constructive-vs-predicative.md`,
  `proof-preservation.md`, `ingress-and-boundaries.md` — the
  principles these idioms apply.
- `boolean-blindness.md` — discriminated unions with `_tag` are the
  TypeScript answer.
- `testing.md` — strict-matcher principle applied in TypeScript.
- `external-integration` skill — ACL/Gateway construction with
  schema-first parsers.
