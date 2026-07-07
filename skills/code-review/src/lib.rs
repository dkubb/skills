//! run a clear code review across languages and change review rules.
//!
//! the skill is primarily guidance (see SKILL.md). the `lint` subcommand
//! runs the bundled Dylint lint library against the caller's project.

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use clap::{CommandFactory as _, Parser, Subcommand};
use skill_core::{CommandOutcome, SkillError};

/// run a clear code review across languages and change review rules.
#[derive(Debug, Parser)]
#[command(name = "code-review")]
pub struct Args {
    /// subcommand to execute; omit to print help.
    #[command(subcommand)]
    command: Option<SkillCommand>,
}

/// code-review subcommands.
#[derive(Clone, Debug, Subcommand)]
enum SkillCommand {
    /// run bundled Dylint lints against the caller's project.
    Lint(LintArgs),
}

/// arguments for the `lint` subcommand.
#[derive(Clone, Debug, Parser)]
struct LintArgs {
    /// working directory to run the lints against. defaults to the current
    /// directory.
    #[arg(long, value_name = "DIR")]
    cwd: Option<PathBuf>,

    /// extra arguments forwarded verbatim to `cargo dylint -- ...`.
    /// anything after `--` lands here.
    #[arg(last = true, value_name = "DYLINT_ARGS")]
    forward: Vec<String>,
}

/// run the code-review skill.
///
/// # Errors
///
/// returns `SkillError` when the `lint` subcommand fails to invoke the
/// bundled runner or the runner exits non-zero.
pub fn run(args: &Args) -> Result<(), SkillError> {
    match &args.command {
        Some(SkillCommand::Lint(lint_args)) => run_lint(lint_args),
        None => {
            Args::command().print_help()?;
            Ok(())
        }
    }
}

/// execute the bundled `lints/run-dylint` against the caller's project.
fn run_lint(args: &LintArgs) -> Result<(), SkillError> {
    let runner = require_runner(&lints_dir())?;
    let caller_cwd = resolve_cwd(args.cwd.as_deref())?;

    let mut cmd = Command::new(&runner);
    cmd.env("DYLINT_CALLER_CWD", &caller_cwd);
    if !args.forward.is_empty() {
        cmd.args(&args.forward);
    }

    let status = cmd.status()?;

    report(status, &caller_cwd)
}

/// resolve the bundled lint runner, requiring it to exist on disk.
fn require_runner(lints_dir: &Path) -> Result<PathBuf, SkillError> {
    let path = lints_dir.join("run-dylint");
    if path.is_file() {
        Ok(path)
    } else {
        Err(SkillError::MissingFile { path })
    }
}

/// resolve the working directory to run the lints against.
fn resolve_cwd(cwd: Option<&Path>) -> Result<PathBuf, SkillError> {
    match cwd {
        Some(dir) => Ok(dir.to_path_buf()),
        None => Ok(env::current_dir()?),
    }
}

/// translate an exit code into a non-success command outcome.
fn failure_outcome(code: Option<i32>) -> CommandOutcome {
    code.map_or(CommandOutcome::Signaled, CommandOutcome::Exited)
}

/// report the lint run result: print success, or describe the failure.
fn report(status: ExitStatus, cwd: &Path) -> Result<(), SkillError> {
    if status.success() {
        println!(
            "code-review lint: bundled dylint checks passed, no findings in {}",
            cwd.display()
        );
        Ok(())
    } else {
        Err(SkillError::Command {
            command: "code-review lint".to_owned(),
            outcome: failure_outcome(status.code()),
        })
    }
}

/// resolve the absolute path to the skill's bundled lints directory.
///
/// uses the compile-time `CARGO_MANIFEST_DIR` on purpose: the lint runner
/// shells out to `lints/run-dylint`, which needs the on-disk lints source
/// tree (it clones and builds dylint there). this skill therefore only
/// works run from its source checkout, not relocated to an arbitrary path.
fn lints_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("lints")
}

#[cfg(test)]
mod tests;
