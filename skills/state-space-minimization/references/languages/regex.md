# Regex

Every regex defines an accepted-input state space: the set of strings the
engine will admit. A loose regex is the same defect as a weak test matcher
or an unbranded primitive — it accepts states the contract did not promise
to accept, and Hyrum's Law guarantees a downstream consumer will eventually
depend on that incidental admission. Tightening a regex is state-space
minimization at the byte level: each anchor, each bounded quantifier, and
each character class shrinks the accepted set to match the *valid* set.
The review checklist for these rules lives in the `code-review`
skill's `languages/regex.md` (and in a dedicated regex-review skill
when one is available); this file frames the rules in the
vocabulary of `../principles.md` and `../primitive-obsession.md`
and shows how regexes pair with branded / newtype'd carriers in
the language modules.

## Anchor unless you can prove the match is bounded

An unanchored regex accepts the pattern *anywhere* in the input. The
accepted state space is "every string that contains a match", not "every
string that *is* a match". The two differ by a factor of input length
times the alphabet — five or six orders of magnitude even on short
inputs. Anchor every full-match validator at both ends:

- `\A...\z` in Rust, Ruby, PCRE — true input bounds, immune to embedded
  newlines
- `^...$` in JavaScript without `/m` — also true input bounds in JS
  because `$` matches end-of-input by default
- `\A...\Z` in PostgreSQL `~` operator — accepts a trailing newline; use
  `\z` if even that is forbidden

The single most common defect in domain regexes is `^[A-Z_]+$` or
`/^[a-z0-9-]{1,64}/` (no `$`). Both let `___`, `FOO__BAR`, or any
arbitrary trailing garbage through. The reusable module-level pattern:

```ts
export const providerSlugPattern = /^[a-z][a-z\d]*(?:-[a-z\d]+)*$/;
export const networkIdPattern = /^network:[1-9]\d{0,31}$/;
```

Both are anchored at both ends, and the slug pattern uses the canonical
`<atom>+(?:<sep><atom>+)*` shape so leading, trailing, and consecutive
separators are unrepresentable by construction (see
`code-review/references/languages/regex.md` § "Last line of defense").

If a regex is genuinely a *partial-match scanner* (e.g. extracting URLs
from a log line), the looseness is intentional — but record it in a
comment and pair the scanner with a strict validator that re-parses each
hit before any downstream consumer sees it. The scanner produces
candidates; the validator produces typed values.

This is the only scanner exception. It does not weaken the rule for
full-match validators; it splits one broad regex into a loose candidate
finder plus a strict parser.

## Bounded quantifiers always

`+` and `*` are unbounded. The accepted state space includes strings of
any length the host platform permits — gigabytes for a `String` in most
languages, the entire file for a stream parser. That violates the same
"bound both ends" rule from `principles.md` § "Bound ranges and
cardinality": every length-bearing field has an upper bound, even when
the design has not named it.

Use `{min,max}` quantifiers for every repeating subpattern in a domain
regex:

```ts
// EVM addresses are exactly 40 hex digits
const evmAddressHexPattern = /^0x[\da-fA-F]{40}$/;

// hash256 is exactly 64 hex digits
const evmHash256HexPattern = /^0x[\da-fA-F]{64}$/;

// EIP-155 chain ID: at least one digit, no leading zero, capped at 32
const eip155NetworkPattern = /^eip155:[1-9]\d{0,31}$/;
```

Each upper bound is a state-space cap. The `{0,31}` on `eip155Network`
is not arbitrary — it is the operational hard limit the principles file
demands when no protocol-published cap exists for the host's native
integer width.

When the regex genuinely is a one-or-more identifier (`providerSlug`),
pair the regex with a bounded carrier so the upper bound is enforced
twice — once at the byte level by the regex, once at the byte-length
level by the brand factory. The regex bounds the *grammar*; the brand
bounds the *length*. Drop neither.

The `regex-review` checker already flags `+`, `*`, and `{n,}` as
warnings (errors under PCRE/JS where backtracking makes them
weapons). Treat every flag as a finding, not a suggestion.

## Character classes over wildcards

`.` accepts every code point the engine considers "any character" — in
Unicode mode that is approximately 1.1 million accepted code points per
position. `.{1,64}` accepts roughly `1,114,112^64` distinct strings; a
character class like `[A-Za-z0-9_-]` accepts `64^64`, which is roughly
`10^96` smaller. Each `.` is the regex analogue of `String` — a typed
admission that the position carries no information.

Replace `.` with the narrowest class the position actually accepts:

```ts
// loose: any 1..64 characters, including newlines under /s
const lazy = /^.{1,64}$/;

// tight: ASCII letters, digits, hyphen, underscore — and that is the
// closed set the domain accepts, audited
const tight = /^[A-Za-z0-9_-]{1,64}$/;
```

Prefer named character classes when they fit the domain *exactly*: `\d`
not `[0-9]` for ASCII digits, `\s` not `[ \t\n\r\f]` for whitespace,
`[[:xdigit:]]` or `\p{Hex_Digit}` for hex digits when the engine
documents one and the spec accepts mixed case. When the spec restricts
to lowercase hex (EVM private keys, signatures), keep the explicit
`[\da-f]` — `\p{Hex_Digit}` admits uppercase and is a state-space leak.

Beware Unicode classes silently widening the accepted set: `\d` in PCRE
and JavaScript with `/u` matches Devanagari, Bengali, and several dozen
other digit scripts. If the domain is "ASCII decimal digit", write
`[0-9]` and audit any `\d` site for mismatch.

## Closed alternation, exhaustive sums

Alternation between literal strings is a sum type spelled with regex
syntax. `^(queued|running|done)$` admits exactly three values. That is
not a regex domain — it is a closed enum dressed up. Use the language's
sum-type machinery instead, and the regex disappears:

```ts
// regex form: parser-only sum, no exhaustiveness check downstream
export const statusPattern = /^(?:queued|running|done)$/;

// better: Schema.Literal closes the sum at the type system
const Status = Schema.Literal("queued", "running", "done");
```

The literal sum gets exhaustiveness checking from `Match.valueTags` /
`switch` + `never`; the regex form gets a parser failure at runtime and
no static cross-check that every branch is handled. See
`principles.md` § "Encode invariants into types" rung 3 ("Enums for
closed sets") and `constructive-vs-predicative.md` for why the
constructive form wins.

Example: a pattern such as
`valuationRuleIdPattern = /^currency-(?:1to1|base-units)$/` is a
closed two-value set. Replace it with
`Schema.Literal("currency-1to1", "currency-base-units")` so the type
system, not the regex engine, owns exhaustiveness.

When alternation is the right tool — open sets where the value space
genuinely is "one of these regex shapes" — order alternatives so the
most-specific or most-common comes first (PCRE / JavaScript backtrackers
short-circuit on the first match), and audit for *overlap*: two
alternatives that admit the same string are a duplicated state and a
mutation-test escape hatch. The `regex-review` checker flags
`(foo|foobar)`-style overlaps as a warning; treat them as design errors.

## Catastrophic backtracking and ReDoS as state-space explosion

ReDoS is state-space explosion in the *engine's* state, not the
language's: nested unbounded quantifiers (`(a+)+`), alternation with
overlapping prefixes (`(a|a)*`), and unbounded lookahead inside a loop
all let an attacker drive the backtracker into exponential paths. The
attack is real; the defect is the same loose-quantifier defect this
file already names.

Defenses, in order:

1. **Anchor and bound** — most ReDoS-vulnerable patterns are also
   unanchored or unbounded. Apply the rules above and the attack
   surface usually evaporates.
2. **Possessive quantifiers / atomic groups** — `(?>...)`, `a++`,
   `a*+` in PCRE / Java / Ruby refuse to backtrack into the group.
   Use them on every loop whose contents are unambiguous. JavaScript
   gained `(?>...)` and possessive quantifiers in ES2024; Rust's
   `regex` crate is backtrack-free by construction.
3. **Switch engines** when the input is hostile. `regex` (Rust) and
   `re2` (C++, Go, Python via `pyre2`) guarantee linear time. Use
   `hyperscan` (Intel, BSD-3) for high-throughput multi-pattern
   matching at packet rates. Treat any PCRE/JavaScript regex on
   user-controlled input as a denial-of-service primitive until proven
   otherwise.

Rust gets the third defense for free: `regex::Regex` is the default.
JavaScript and Ruby do not; audit every `RegExp` and every Ruby `Regexp`
on attacker-controlled input and prefer `re2`-bound bindings (Node's
`re2` package, Ruby's `re2` gem) where possible.

## Pair the regex with the brand

A regex is a *predicate*. Running the predicate produces a boolean and
discards the proof — the same parse-vs-validate failure
`principles.md` warns against. Pair every domain regex with a typed
carrier so the proof of validity travels with the value:

- TypeScript: `Schema.pattern(...)` followed by `Schema.brand(...)`,
  hoisted into a `boundedPrintableString` factory so length and
  grammar are bounded together (see `./typescript.md` § "Brand
  factories that enforce bounds at every site")
- Rust: `nutype` with `validate(regex = PATTERN, len_char_max = N)` so
  the regex and the length cap are part of one type declaration (see
  `./rust.md` § "`nutype` for the full predicative package")
- SQL: `CREATE DOMAIN ... AS text CHECK (value ~ '\A[a-z][a-z0-9_]{0,62}\z')`
  promotes the regex from a query-time check to a column-type
  invariant; every `INSERT` or `UPDATE` that violates it fails at the
  storage boundary, not after the row is read

The carrier makes "this string passed the regex" a fact of the type
system, not a fact a downstream consumer has to take on faith. Without
the carrier, every consumer must re-run the predicate or trust the
caller — primitive obsession dressed in regex syntax.

## Test the rejections, not the acceptances

A regex that accepts every valid string but also accepts one invalid
string is the same defect as a smart constructor with a missing branch:
the trusted boundary admits states the contract forbids. Test the
rejections.

For every regex, the test suite asserts:

- at least one near-miss for **each anchor**: a string that matches the
  body but adds a leading newline, trailing newline, or surrounding
  garbage (a passing test here is a missing `\A` / `\z`)
- at least one near-miss for **each character-class boundary**: the
  character one position outside each `[a-z]`, each `\d`, each
  `[A-Za-z0-9_-]` (a passing test here is a class that drifted from
  the spec)
- at least one near-miss for **each quantifier bound**: `min - 1` and
  `max + 1` characters (a passing test here is an unbounded or
  off-by-one quantifier)
- at least one near-miss for **each alternation branch**: a string
  that matches all branches (overlap), and a string that matches none
  (gap)

This is the regex form of the side-effect rule from `testing.md`: every
test asserts the full Cartesian product of (acceptance × rejection ×
boundary). A test that only checks "the parser accepts these five
strings" leaves the rejection axis unconstrained, and Hyrum's Law fills
the gap.

### Mutation-sensitivity for regex generators

Property-based tests for a regex-backed validator must be
**mutation-sensitive to the regex itself**: deleting any single
character from the regex, or changing any single character in it,
should make at least one generated case fail. If the property suite
survives a one-character mutation, the suite is the same defect class
as a weak matcher — accepts more than the contract.

The standard discipline for regex-backed validators:

1. Generate `*_valid_broad`, `*_valid_biased`, `*_invalid_broad`,
   `*_invalid_biased` strategies independent of the regex under test
   — never use the regex itself in a `prop_filter` to construct
   invalids.
2. Bias invalid generators toward each *anchor*, each *character
   class boundary*, each *quantifier bound*, and each *alternation
   branch*. These are the surfaces a mutation will expose.
3. Run a regex-mutation pass (manually or via a tool) and confirm
   every surviving mutant is either a semantically equivalent rewrite
   or a generator gap to fill. Record any tolerated survivor in the
   test file with the reason.

Stryker (TypeScript) and `cargo-mutants` (Rust) are the language-level
analogues; both will mutate the regex literal embedded in the source
and re-run the suite. A passing suite against a mutated regex is the
same defect a weak `expect(...).toEqual(...)` is in `testing.md` §
"Strict matchers".

## Compile once

A regex constructed from a string literal at every call site re-parses
and re-compiles the pattern on every call. The compiled form is a DFA
or NFA — building it is non-trivial work — and the literal is the same
string each time. The accepted state space does not vary per call;
neither should the compilation.

```rust
// loose: re-compiles on every call; also the regex is constructed from
// a runtime string and the compiler cannot prove it is the same shape
fn is_slug(s: &str) -> bool {
    Regex::new(r"\A[a-z][a-z0-9-]{0,62}\z").unwrap().is_match(s)
}

// tight: compile once, share by reference; lazy_static / OnceLock
// equivalents work the same
static SLUG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[a-z][a-z0-9-]{0,62}\z").expect("static"));
```

```ts
// loose: literal in a function body — V8 caches but the source is still
// duplicated and the cache is per-realm
const isSlug = (s: string) => /^[a-z][a-z0-9-]{0,62}$/.test(s);

// tight: hoist the pattern to a module-level constant so the brand
// factory and the validator share one definition
export const slugPattern = /^[a-z][a-z0-9-]{0,62}$/;
export const Slug = Schema.String.pipe(Schema.pattern(slugPattern), Schema.brand("Slug"));
```

The hoisted constant is also the only place the regex needs to be
audited, mutation-tested, or bumped — not "every grep hit for `^[a-z`".
Production patterns should follow this discipline: every pattern is a
module-level `export const ...Pattern = ...` so the audit surface is one
location, not every call site.

Never construct a regex from interpolated user input. That is the
regex form of SQL injection: an attacker who controls part of the
pattern controls the accepted state space.

## Unicode awareness

Regex semantics shift under Unicode flags:

- `\d` matches Devanagari, Arabic-Indic, Bengali, and several dozen
  other digit scripts in PCRE-Unicode and JavaScript with `/u`. Rust's
  `regex` crate matches `\d` to `\p{decimal_number}` by default
  (Unicode-aware); use `(?-u:\d)` to restrict to ASCII when the
  domain is ASCII-only
- `\w` matches a much wider class under Unicode than under ASCII. If
  the domain is ASCII identifiers, write the class explicitly:
  `[A-Za-z0-9_]`
- case-insensitivity (`/i`) under Unicode involves Turkish dotted I,
  German sharp s, and other locale-dependent foldings. If the input
  is normalized upstream, drop `/i` and require the canonical case
- normalization is *not* part of regex matching. `café` (NFC) and
  `café` (NFD) are different inputs to the regex engine even
  though they render identically. Normalize before matching, or
  reject non-canonical forms at the parse boundary
- escape sequences differ across engines: `\A` and `\z` exist in
  Rust, Ruby, PCRE, Python, but not JavaScript pre-ES2018 named
  groups. Verify the engine before assuming an anchor exists; the
  `regex-review` skill's `--engine` flag is the gate

When in doubt, restrict to ASCII and document the restriction. A
domain regex that accidentally matches Devanagari digits is the same
defect class as a `String` parameter that accidentally accepts every
Unicode sequence — the type lied about its accepted set.

## Rejection-only regexes (denylists)

Almost every domain regex should be an allowlist: enumerate what is
valid, reject the rest. Denylists are an unbounded problem — there is
always one more invalid input the list forgot. `principles.md` §
"Shrink the domain" makes this rule explicit: prefer allowlists over
denylists.

The few cases where a denylist is the right tool:

- **Sanitizers paired with an allowlist** — strip control characters
  before applying the strict validator. The denylist is a
  pre-processing step, not the trust boundary
- **Defense-in-depth WAFs** — reject obvious SQL-injection or XSS
  shapes *in addition to* parameterized queries and contextual
  escaping, never *instead of*. The allowlist (parameterization)
  remains the contract; the denylist catches the residue
- **Logging redactors** — match known secret shapes (AWS keys, JWTs,
  PEM blocks) and redact. The cost of a false negative (one secret
  leaks) is bounded; the cost of a false positive (one log line
  mangled) is recoverable. Document both

When a denylist regex exists, pair it with a test suite that asserts
the denylist *and* the matching allowlist agree on every string in a
shared corpus. Drift between the two is the failure mode: the
allowlist tightens, the denylist does not, and the gap is silent.

## Cross-references

- The `regex-review` skill — the canonical regex rule catalogue and
  the static checker that enforces anchoring, bounded quantifiers,
  and overlap detection.
- The `code-review` skill (`references/languages/regex.md` within
  it) — the review-level rule statements (anchors, classes,
  modifiers, iteration, scope) that this file frames in state-space
  vocabulary.
- `../principles.md` — the underlying state-space arithmetic and the
  "bound both ends" rule that makes `{min,max}` non-negotiable.
- `../primitive-obsession.md` — a regex without a branded carrier is
  primitive obsession. The carrier turns the regex from a predicate
  into a parser.
- `../constructive-vs-predicative.md` — closed alternation should
  collapse to a `Schema.Literal` / Rust enum. The regex is the
  predicative form; the sum type is the constructive form.
- `../ingress-and-boundaries.md` — every regex sits at an ingress
  boundary (parse incoming bytes) or an egress boundary (validate
  before serializing). The trusted-boundary audit applies to the
  regex's call sites.
- `../testing.md` — the strict-matcher rule applied to regex test
  suites: assert the rejection axis with the same precision as the
  acceptance axis.
- `./rust.md` § "`nutype` for the full predicative package" — the
  Rust idiom for pairing a regex with a branded carrier.
- `./typescript.md` § "Brand factories that enforce bounds at every
  site" and § "Schema-first parsing" — the TypeScript / Effect Schema
  idioms.
- `./sql.md` covers `CREATE DOMAIN ... CHECK (VALUE ~ '...')` and
  per-column `CHECK` constraints for PostgreSQL-backed regex
  invariants.
