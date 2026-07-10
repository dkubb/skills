# Bash Review Guidance (Language-Specific)

- Use simple English.
- Use short bullets.
- Do not repeat core principles.

## Principle: strict globally, exceptions scoped

Start every script at maximum strictness. When a strict setting blocks
legitimate behavior, do not weaken the global setting. Either decide the
behavior is wrong, or allow it in the smallest possible scope with the
exception visible at the use site. This is the bash form of
`#[expect(..., reason = "...")]`: the default ratchets tight and every
loosening is local, explicit, and reviewable.

## Tooling

- Run `shellcheck --enable=all --external-sources --severity=style` on all
  Bash scripts and require zero findings. Optional checks are off by
  default; `--enable=all` turns on the strictest set.
- Require `# shellcheck shell=bash` as the first directive in every script.
  The hermetic `env -S` shebang is unrecognized by shellcheck (SC1008)
  without it.
- Scope `# shellcheck disable=SCnnnn` to the single line it silences and
  give a reason comment. No file-level blanket disables.
- Run `shfmt -w -i 2 -ci -bn -sr -s -ln bash` on all Bash scripts. Use `-d`
  in place of `-w` as the check-only review gate. Pass `-ln bash`
  explicitly; the `env -S` shebang defeats dialect detection.

## Shebang: environment hygiene

Require the hermetic shebang. It strips environment state that changes
bash behavior before bash starts:

<!-- a shebang is a single line and cannot wrap -->
<!-- rumdl-disable MD013 -->

```bash
#!/usr/bin/env -S -u BASHOPTS -u BASH_ENV -u CDPATH -u GLOBIGNORE -u SHELLOPTS bash --noprofile --norc
```

<!-- rumdl-enable MD013 -->

- `-u BASHOPTS -u SHELLOPTS`: bash imports option lists from these
  environment variables at startup (verified: `SHELLOPTS=xtrace` in the
  environment enables xtrace). Unset them or a caller can toggle options.
- `-u BASH_ENV`: bash sources `$BASH_ENV` in non-interactive shells even
  under `--norc` (verified). Unset it or a caller can inject code.
- `-u CDPATH -u GLOBIGNORE`: both silently change `cd` and glob behavior.
- `--noprofile --norc`: no user rc contamination.
- The shebang carries no `set`/`shopt` options. A shebang applies only on
  direct execution — `bash script.sh` and `source` skip it — and
  duplicating options in both layers gives the same setting two
  determinants. The hygiene flags guarantee nothing executes before the
  prologue's first line, so the prologue below is the single authority for
  all options. The shebang holds only what must act before bash reads any
  input.

## Prologue: strict mode

Require this prologue at the top of every script body:

```bash
set -o errexit -o errtrace -o nounset -o pipefail -o noclobber
(( BASH_VERSINFO[0] >= 5 )) || { echo 'bash >= 5 required' >&2; exit 1; }
shopt -s inherit_errexit failglob shift_verbose varredir_close
shopt -u sourcepath patsub_replacement
IFS=$'\n\t'
umask 077
export LC_ALL=C
```

Why each line, beyond the classic four `-o` flags:

- `noclobber`: `>` refuses to overwrite an existing file. Intentional
  clobbering uses `>|`, which marks the exception at the redirect.
- Version guard: assert the interpreter version instead of writing
  version-tolerant code. It runs before the `shopt` lines so an old bash
  fails with one clear message, not an unknown-option error.
- `inherit_errexit` (bash 4.4+): command substitutions inherit `errexit`.
  Without it, `x=$(false; echo ok)` succeeds and the failure is swallowed
  (verified).
- `failglob`: an unmatched glob is an error, not a literal pattern and not
  a silent empty list. Fail closed; nullglob is the scoped exception where
  an empty match is a legal state.
- `shift_verbose`: `shift` past the end of the arguments is an error.
- `varredir_close` (bash 5.2+): `{fd}>` file descriptors close
  automatically instead of leaking.
- `sourcepath` off: `source name` stops searching `$PATH`; sourcing takes
  explicit paths only.
- `patsub_replacement` off (bash 5.2+, default on): `&` in a
  `${var//pat/rep}` replacement expands to the matched text, so data
  containing `&` silently rewrites itself (verified). Disable it; the
  replacement string means what it says.
- `IFS=$'\n\t'`: word splitting stops splitting on spaces. Bash resets
  `IFS` at startup regardless of the environment (verified), so this is a
  domain tightening, not env hygiene.
- `umask 077`: every created file starts private. Widening is per-file
  with an explicit `chmod`, never by loosening the global umask.
- `LC_ALL=C`: deterministic collation, sorting, and character classes.
- Bash 5.3 evaluates array subscripts once by default (verified), closing
  the `a[$key]` injection hole. On bash 5.0–5.2 add
  `shopt -s assoc_expand_once`.

## Scoped exceptions

The only approved forms of loosening. Each marks the exception at the use
site and leaves the global settings intact:

- Tolerated failure: `cmd || true`, or `rc=0; cmd || rc=$?` when the code
  branches on the result. Never `set +o errexit` around a region.
- Intentional overwrite: `>|` on that redirect only.
- Legal empty glob: toggle in a subshell — `( shopt -u failglob; shopt -s
  nullglob; ... )` — with a comment saying why empty is a valid state.
- Legal absence: `${var:-default}` only where absence is a domain state;
  otherwise `${var:?reason}` so missing input fails with a message.
- Wider permissions: explicit `chmod` on the specific file after creation.
- Intentional word splitting: `read -ra parts <<< "$value"` or a per-command
  `IFS=... read`, never a global `IFS` reset.

## Review focus

- Prefer `$(...)` over backticks and `[[ ... ]]` over `[ ... ]`.
- Use arrays for lists instead of word-splitting.
- Send errors to stderr.
- Use long-form options when available and order options alphabetically when
  multiple options are present.
- Prefer single quotes for strings that do not use variable interpolation.
- Prefer compound conditions with `&&`/`||` instead of nested `if` blocks for
  simple checks.
- Sort `apt-get install` package lists alphabetically.
- Format scripts for human audit: add section headers, use blank lines, and
  break long pipelines or commands across lines.
