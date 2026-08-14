# TypeScript Review Guidance (Language-Specific)

- Apply `../core-principles.md` first.
- Use simple English and short bullets.

## Required gates

- Run the repository wrappers for formatting, type-checking, linting, tests,
  and coverage. A transpiler or bundler completing is not proof that `tsc` or
  typed ESLint passed.
- Require `strict: true`. Also evaluate strictness flags outside the `strict`
  family, including `exactOptionalPropertyTypes`, `noUncheckedIndexedAccess`,
  `noImplicitOverride`, `noImplicitReturns`, `noFallthroughCasesInSwitch`,
  `noPropertyAccessFromIndexSignature`, and `noUncheckedSideEffectImports`.
  Evaluate only flags supported by the pinned TypeScript version; upgrading the
  compiler is a separate decision. Disable a supported flag only for a recorded
  incompatibility.
- Keep `skipLibCheck` off by default. If dependency declarations force it on,
  record the concrete failure and keep application code fully checked.
- Choose exactly one deterministic owner for unused-code checks when TypeScript
  and ESLint overlap. Do not disable the check in both tools.
- Use type-aware typescript-eslint rules. Start policy evaluation from every
  available rule, resolve conflicts deliberately, and retain the strictest
  compatible set. The `all` preset is an inventory, not an unexplained
  permanent configuration; its rules can conflict or require project options.
- Scope every `eslint-disable`, `@ts-expect-error`, or equivalent suppression to
  the smallest line or construct and require a reason. Prefer
  `@ts-expect-error` over `@ts-ignore` so a stale suppression fails.

## Formatting

- Use the repository formatter. When a project has none, prefer a locally
  installed, exactly pinned Prettier and its standard output as the baseline.
- Let the formatter own printing and ESLint own semantic quality. Disable
  conflicting formatting rules rather than creating two formatting
  authorities.
- Run the write mode during development and the check mode in CI.

## Types and domain modeling

- Treat explicit or inferred `any` as a blocker. Receive uncertain values as
  `unknown`, narrow or decode them once, and return a domain type.
- Do not use type assertions, non-null assertions, or double casts as runtime
  validation. An unavoidable assertion needs a proved invariant and a durable
  reason at the use site.
- Use discriminated unions for closed alternatives and make switches
  exhaustive. Do not represent mutually exclusive states as independent
  booleans or bags of optional properties.
- Use branded or opaque types and smart constructors for constrained
  primitives. Keep construction paths private enough that callers cannot forge
  a validated value.
- Preserve the distinction between an absent property and a property present
  with `undefined`. Do not weaken exact optional semantics for convenience.
- Exported constants and functions that return cached or otherwise shared
  object graphs must return deeply immutable data by default. Prevent mutation
  of every reachable mutable container, not only the top-level value.
- For immutable literal data, pair the runtime and compile-time contracts:
  write `Object.freeze({ ... } as const)`. `Object.freeze` is shallow, so a
  nested graph also needs recursive freezing and a deeply readonly return type.
- A mutable exported constant or shared object-graph return needs a concrete
  current use case, not speculative flexibility. Its tests must state and
  exercise the required mutability. Tests for an immutable graph must cover
  nested reachable values, not only `Object.isFrozen` on the root.
- When a change characterizes an existing exported constant or shared
  object-graph return before modifying it, the characterization test must
  record whether the current graph is mutable or immutable. A later commit that
  changes that contract must update the assertion deliberately.
- Prefer inference for obvious local values. Give exported boundaries explicit
  contracts when that makes the public API stable and reviewable; avoid
  redundant annotations that can drift from the implementation.
- Prefer `satisfies` when checking a value against a shape while preserving its
  useful inferred type.

## Boundaries and runtime data

- TypeScript types are erased. Treat JSON, environment variables, network and
  database results, persisted data, DOM input, and third-party `any` as
  untrusted even when a declaration file claims a type.
- Decode and validate at ingress and when persisted data crosses back into a
  trusted domain. Do not cast external data into an application type.
- Keep generated clients and transport schemas behind an anti-corruption
  boundary. Translate them into validated domain types rather than spreading
  generated or vendor types through core code.
- Keep runtime validation and static types derived from one schema or prove
  their equivalence with boundary and round-trip tests.

## Errors, promises, and resources

- For new code, expected failures appear in the API as typed
  values or in Effect's error channel when Effect is adopted. Reserve thrown
  exceptions or defects for unexpected failures that cannot be handled
  locally. Preserve established public throwing APIs through explicit adapters
  and migrate incrementally rather than rewriting them opportunistically.
- Every promise has an owner. Await it, return it, or deliberately detach it
  with explicit lifetime, error reporting, and cancellation behavior.
- Propagate `AbortSignal` or the repository's cancellation mechanism through
  cancellable work. Clean up timers, listeners, subscriptions, handles, and
  resources on success, failure, and cancellation.
- Do not catch `unknown` and collapse it into a generic string or success
  value. Preserve the original cause and distinguish expected error variants.

## Modules and APIs

- Match TypeScript module settings to the actual runtime or bundler. Do not
  mix module-resolution models because an editor happens to resolve the import.
- Use type-only imports where the distinction prevents runtime ambiguity.
  Review side-effect imports carefully; a misspelled side-effect import must
  fail the type-check.
- Keep public APIs small and domain-shaped. Do not leak framework, generated,
  or Effect types across a public boundary unless that is the intentional
  contract.

## Testing

- Require runtime boundary tests for decoders and smart constructors, including
  valid, invalid, and boundary values.
- Require property tests for every smart constructor, serializer, emitter,
  output generator, deserializer, and parser. Generators must span the complete
  declared domain, including its boundaries and interior states; every paired
  producer and consumer requires round-trip properties.
- Add compile-time type tests for important public generic behavior and for
  invalid programs that must remain rejected.
- Use deterministic clocks and schedulers for timing, retry, and concurrency
  behavior. Do not make wall-clock sleeps the oracle.

## Effect

Effect is a conditional architecture option, not the TypeScript community
baseline and not a requirement for every TypeScript file.

- Evaluate Effect first for a new application or cohesive subsystem dominated
  by composed asynchronous workflows, typed failures, cancellation or
  concurrency, resource lifetimes, service dependencies, retries, scheduling,
  configuration, or observability.
- Adopt it when those benefits outweigh its learning curve, different
  programming style, API surface, interoperability cost, and measured bundle
  or startup constraints.
- Do not require Effect for a small script, a simple pure library,
  straightforward UI code, or a mature project whose existing abstractions
  already solve the problem coherently.
- Introduce Effect one cohesive architectural slice at a time, preferably at a
  high-complexity boundary. Do not add it opportunistically to an unrelated
  diff or rewrite a working application wholesale.
- Keep `Effect<Success, Error, Requirements>` honest. Expected errors belong in
  the error channel, required services belong in requirements, and defects
  remain distinct from recoverable failures.
- Decode untrusted data with `effect/Schema` when the slice uses Effect. Use
  Services and Layers for actual dependencies, scoped resource management for
  lifetimes, and the platform `runMain` at the outer application boundary when
  applicable.
- Avoid tacit or point-free Effect calls when they can erase inference, weaken
  stack traces, or hide intent. Prefer explicit lambdas.
- Keep Promise, throwing, framework, and public-library interoperability at
  explicit adapters. Inside an Effect-owned slice, do not alternate casually
  between Effect and Promise control flow.
- Expand adoption only after the slice demonstrates clearer error contracts,
  safer resource behavior, better testability, or lower complexity.

## Minimum review pass

1. Identify and run the real format, type, lint, test, and coverage gates.
2. Inspect the effective TypeScript and ESLint configuration, not only the
   checked-in base file.
3. Find suppressions, `any`, assertions, floating promises, external casts,
   and non-exhaustive closed-state handling; adjudicate each match.
4. Trace every changed external boundary from `unknown` input to a validated
   domain value and every changed async operation to its owner and cleanup.
5. If Effect is added or already owns the slice, apply the Effect criteria and
   review it as Effect code. Otherwise, do not demand Effect.
6. For each finding, provide the evidence and improvement argument required by
   the core principles.

## Primary references

- [TypeScript strictness](https://www.typescriptlang.org/docs/handbook/2/basic-types.html#strictness)
- [TypeScript compiler options](https://www.typescriptlang.org/tsconfig/)
- [typescript-eslint typed linting](https://typescript-eslint.io/getting-started/typed-linting/)
- [typescript-eslint shared configurations](https://typescript-eslint.io/users/configs/)
- [Prettier installation and CI checks](https://prettier.io/docs/install.html)
- [Effect rationale](https://effect.website/docs/getting-started/why-effect/)
- [The Effect type](https://effect.website/docs/getting-started/the-effect-type/)
- [Effect code-style guidelines](https://effect.website/docs/code-style/guidelines/)
- [Effect Schema](https://effect.website/docs/schema/introduction/)
