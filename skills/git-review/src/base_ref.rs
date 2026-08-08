//! Resolve the base ref for a branch.

use std::io::{self, Write};
use std::process::{Command, Stdio};

use clap::Parser;
use skill_core::SkillError;

/// resolve the base ref for commit review.
#[derive(Clone, Debug, Parser)]
pub(crate) struct ResolveBaseArgs {
    /// optional explicit base ref to use.
    #[arg(value_name = "BASE_REF")]
    base_ref: Option<String>,

    /// path to the Git repository.
    #[arg(long, default_value = ".")]
    repo: String,
}

/// Decode command output without accepting malformed UTF-8.
fn decode_stdout(stdout: Vec<u8>) -> Result<String, SkillError> {
    String::from_utf8(stdout)
        .map_err(|error| SkillError::Io(io::Error::new(io::ErrorKind::InvalidData, error)))
}

/// Run a GitHub CLI program and capture non-empty standard output.
fn run_gh_program(program: &str, repo: &str, args: &[&str]) -> Result<Option<String>, SkillError> {
    let output = Command::new(program)
        .args(args)
        .current_dir(repo)
        .stdin(Stdio::null())
        .output()?;
    interpret_output(output.status.success(), output.stdout)
}

/// Interpret GitHub CLI status and standard output.
fn interpret_output(success: bool, bytes: Vec<u8>) -> Result<Option<String>, SkillError> {
    if !success {
        return Ok(None);
    }

    let stdout = decode_stdout(bytes)?;
    let value = stdout.trim_end();
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_owned()))
    }
}

/// Query GitHub CLI for one value.
fn run_gh(repo: &str, args: &[&str]) -> Result<Option<String>, SkillError> {
    run_gh_program("gh", repo, args)
}

/// GitHub CLI query boundary used to test resolution policy independently.
type Query<'query> = dyn FnMut(&str, &[&str]) -> Result<Option<String>, SkillError> + 'query;

/// Resolve the base using explicit, PR, repository, and fallback precedence.
fn resolve(args: &ResolveBaseArgs, query: &mut Query<'_>) -> Result<String, SkillError> {
    if let Some(base_ref) = args.base_ref.as_ref() {
        return Ok(base_ref.to_owned());
    }

    if let Some(base_ref) = query(
        &args.repo,
        &[
            "pr",
            "view",
            "--json",
            "baseRefName",
            "--jq",
            ".baseRefName",
        ],
    )? {
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
    let line = format!("{base_ref}\n");
    out.write_all(line.as_bytes())?;
    Ok(())
}

/// Resolve and print the base ref.
///
/// # Errors
///
/// Returns `SkillError` when GitHub CLI or output fails.
pub(crate) fn run(args: &ResolveBaseArgs) -> Result<(), SkillError> {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    run_with_writer(args, &mut out, &mut run_gh)
}

#[cfg(test)]
mod tests;
