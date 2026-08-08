//! Check commit timestamps for monotonic order.

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use serde_json::Value;
use skill_core::SkillError;

use crate::git::run as run_git;
use crate::invalid;
use crate::output::{json_object, write_json_line};

/// check commit timestamps for monotonic order.
#[derive(Clone, Debug, Parser)]
pub(crate) struct CheckDatesArgs {
    /// base ref for the revision range.
    #[arg(value_name = "BASE_REF")]
    base_ref: String,

    /// branch ref for the revision range.
    #[arg(value_name = "BRANCH_REF", default_value = "HEAD")]
    branch_ref: String,

    /// path to the Git repository.
    #[arg(long, default_value = ".")]
    repo: PathBuf,
}

/// One commit whose timestamp does not strictly follow its parents.
#[derive(Debug, Eq, PartialEq)]
struct Violation {
    /// Violating commit SHA.
    commit: String,
    /// Latest parent committer timestamp.
    parent: i64,
    /// Commit author timestamp.
    author: i64,
    /// Commit committer timestamp.
    committer: i64,
}

/// Parse one timestamp with a field-specific diagnostic.
fn parse_timestamp(value: &str, field: &str) -> Result<i64, SkillError> {
    value
        .parse::<i64>()
        .map_err(|error| invalid(format!("invalid {field} timestamp: {error}")))
}

/// Parse the author and committer timestamp pair.
fn parse_author_committer(line: &str) -> Result<(i64, i64), SkillError> {
    let mut values = line.split_whitespace();
    let author = values
        .next()
        .ok_or_else(|| invalid("missing author timestamp"))?;
    let committer = values
        .next()
        .ok_or_else(|| invalid("missing committer timestamp"))?;
    Ok((
        parse_timestamp(author, "author")?,
        parse_timestamp(committer, "committer")?,
    ))
}

/// Find the newest committer timestamp among `parents`.
fn max_parent_timestamp(repo: &Path, parents: &str) -> Result<i64, SkillError> {
    let mut maximum: i64 = 0;
    for parent in parents.split_whitespace() {
        maximum = maximum.max(parent_timestamp(repo, parent)?);
    }
    Ok(maximum)
}

/// Read one parent committer timestamp.
fn parent_timestamp(repo: &Path, parent: &str) -> Result<i64, SkillError> {
    let value = run_git(repo, &["show", "-s", "--format=%ct", parent])?;
    parse_timestamp(&value, "parent")
}

/// Read one commit's author and committer timestamps.
fn read_author_committer(repo: &Path, commit: &str) -> Result<(i64, i64), SkillError> {
    let value = run_git(repo, &["show", "-s", "--format=%at %ct", commit])?;
    parse_author_committer(&value)
}

/// Inspect one commit for a date-order violation.
fn inspect_commit(repo: &Path, commit: &str) -> Result<Option<Violation>, SkillError> {
    let parents = run_git(repo, &["show", "-s", "--format=%P", commit])?;
    if parents.is_empty() {
        return Ok(None);
    }

    inspect_non_root(repo, commit, &parents)
}

/// Inspect a commit after its non-empty parent list is known.
fn inspect_non_root(
    repo: &Path,
    commit: &str,
    parents: &str,
) -> Result<Option<Violation>, SkillError> {
    let parent = max_parent_timestamp(repo, parents)?;
    let (author, committer) = read_author_committer(repo, commit)?;
    if author > parent && committer > parent {
        Ok(None)
    } else {
        Ok(Some(Violation {
            commit: commit.to_owned(),
            parent,
            author,
            committer,
        }))
    }
}

/// Inspect a pre-resolved commit list.
fn inspect_commits(repo: &Path, commits: &str) -> Result<Vec<Violation>, SkillError> {
    let mut violations = Vec::new();
    for commit in commits.lines() {
        if let Some(violation) = inspect_commit(repo, commit)? {
            violations.push(violation);
        }
    }
    Ok(violations)
}

/// Find violations in the selected revision range.
fn find_violations(args: &CheckDatesArgs) -> Result<Vec<Violation>, SkillError> {
    let range = format!("{}..{}", args.base_ref, args.branch_ref);
    let commits = run_git(
        &args.repo,
        &["rev-list", "--reverse", "--topo-order", &range],
    )?;
    inspect_commits(&args.repo, &commits)
}

/// Render the date-order report.
fn run_with_writer(args: &CheckDatesArgs, out: &mut dyn Write) -> Result<(), SkillError> {
    let violations = find_violations(args)?;
    if violations.is_empty() {
        write_json_line(
            out,
            &json_object([
                ("status", Value::String("ok".to_owned())),
                ("violations", Value::Array(Vec::new())),
            ]),
        )?;
        return Ok(());
    }

    let violations_json = violations
        .iter()
        .map(|violation| {
            json_object([
                ("author", Value::from(violation.author)),
                ("commit", Value::String(violation.commit.clone())),
                ("committer", Value::from(violation.committer)),
                ("parent", Value::from(violation.parent)),
            ])
        })
        .collect::<Vec<_>>();
    write_json_line(
        out,
        &json_object([
            ("status", Value::String("violations".to_owned())),
            ("violations", Value::Array(violations_json)),
        ]),
    )?;

    Err(invalid(format!(
        "date order violations detected ({})",
        violations.len()
    )))
}

/// Check author and committer timestamps for strictly increasing order.
///
/// # Errors
///
/// Returns `SkillError` when Git or output fails, or a violation is found.
pub(crate) fn run(args: &CheckDatesArgs) -> Result<(), SkillError> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    run_with_writer(args, &mut out)
}
