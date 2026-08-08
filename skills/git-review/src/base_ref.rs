//! Resolve the base ref for a branch.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use clap::Parser;
use serde_json::Value;
use skill_core::SkillError;

use crate::git::run as run_git;
use crate::output::{json_object, write_json_line};

/// resolve the base ref for commit review.
#[derive(Clone, Debug, Parser)]
pub(crate) struct ResolveBaseArgs {
    /// optional explicit base ref to use.
    #[arg(value_name = "BASE_REF")]
    base_ref: Option<String>,

    /// path to the Git repository.
    #[arg(long, default_value = ".")]
    repo: PathBuf,
}

/// Decode command output without accepting malformed UTF-8.
fn decode_stdout(stdout: Vec<u8>) -> Result<String, SkillError> {
    String::from_utf8(stdout)
        .map_err(|error| SkillError::Io(io::Error::new(io::ErrorKind::InvalidData, error)))
}

/// Run a GitHub CLI program and capture non-empty standard output.
fn run_gh_program(program: &str, repo: &Path, args: &[&str]) -> Result<Option<String>, SkillError> {
    let output = Command::new(program)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .args(args)
        .current_dir(repo)
        .stdin(Stdio::null())
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        let command = if stderr.is_empty() {
            format!("gh {}", args.join(" "))
        } else {
            format!("gh {}: {stderr}", args.join(" "))
        };
        let outcome = output.status.code().map_or(
            skill_core::CommandOutcome::Signaled,
            skill_core::CommandOutcome::Exited,
        );
        return Err(SkillError::Command { command, outcome });
    }

    interpret_output(output.stdout)
}

/// Interpret GitHub CLI status and standard output.
fn interpret_output(bytes: Vec<u8>) -> Result<Option<String>, SkillError> {
    let stdout = decode_stdout(bytes)?;
    let value = stdout.trim_end();
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_owned()))
    }
}

/// Query GitHub CLI for one value.
fn run_gh(repo: &Path, args: &[&str]) -> Result<Option<String>, SkillError> {
    run_gh_program("gh", repo, args)
}

/// GitHub CLI query boundary used to test resolution policy independently.
type Query<'query> = dyn FnMut(&Path, &[&str]) -> Result<Option<String>, SkillError> + 'query;

/// Resolve the base using explicit, PR, repository, and fallback precedence.
fn resolve(args: &ResolveBaseArgs, query: &mut Query<'_>) -> Result<String, SkillError> {
    if let Some(base_ref) = args.base_ref.as_ref() {
        return Ok(base_ref.to_owned());
    }

    let branch = run_git(&args.repo, &["branch", "--show-current"])?;
    if !branch.is_empty()
        && let Some(base_ref) = query(
            &args.repo,
            &[
                "pr",
                "list",
                "--state",
                "all",
                "--head",
                &branch,
                "--limit",
                "1",
                "--json",
                "baseRefName",
                "--jq",
                ".[0].baseRefName // empty",
            ],
        )?
    {
        return Ok(base_ref);
    }

    query(
        &args.repo,
        &[
            "repo",
            "view",
            "--json",
            "defaultBranchRef",
            "--jq",
            ".defaultBranchRef.name",
        ],
    )?
    .map_or_else(|| Ok("main".to_owned()), Ok)
}

/// Resolve and write the base ref.
fn run_with_writer(
    args: &ResolveBaseArgs,
    out: &mut dyn Write,
    query: &mut Query<'_>,
) -> Result<(), SkillError> {
    let base_ref = resolve(args, query)?;
    write_json_line(out, &json_object([("base_ref", Value::String(base_ref))]))
}

/// Resolve and emit the base ref as JSON.
///
/// # Errors
///
/// Returns `SkillError` when GitHub CLI or output fails.
pub(crate) fn run(args: &ResolveBaseArgs) -> Result<(), SkillError> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    run_with_writer(args, &mut out, &mut run_gh)
}
