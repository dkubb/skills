//! Resolve commit targets for review workflows.

use std::collections::BTreeSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use serde_json::Value;
use skill_core::SkillError;

use crate::git::run as run_git;
use crate::output::{json_object, write_json_line};

/// resolve commit targets for commit review.
#[derive(Clone, Debug, Parser)]
pub(crate) struct ResolveTargetsArgs {
    /// base ref for the revision range.
    #[arg(value_name = "BASE_REF")]
    base_ref: String,

    /// branch ref for the revision range.
    #[arg(value_name = "BRANCH_REF", default_value = "HEAD")]
    branch_ref: String,

    /// optional ref specs to target.
    #[arg(value_name = "REF", num_args = 0..)]
    refs: Vec<String>,

    /// path to the Git repository.
    #[arg(long, default_value = ".")]
    repo: PathBuf,
}

/// Resolve all commits in the review range.
fn revisions(args: &ResolveTargetsArgs) -> Result<String, SkillError> {
    let range = format!("{}..{}", args.base_ref, args.branch_ref);
    run_git(
        &args.repo,
        &["rev-list", "--reverse", "--topo-order", &range],
    )
}

/// Resolve one requested ref or range.
fn resolve_ref(repo: &Path, spec: &str) -> Result<String, SkillError> {
    let revision = if spec.contains("..") {
        spec.to_owned()
    } else {
        format!("{spec}^..{spec}")
    };
    run_git(repo, &["rev-list", "--reverse", "--topo-order", &revision])
}

/// Resolve the requested subset into a set.
fn requested(args: &ResolveTargetsArgs, commits: &str) -> Result<BTreeSet<String>, SkillError> {
    if args.refs.is_empty() {
        return Ok(commits.lines().map(ToOwned::to_owned).collect());
    }

    let mut selected = BTreeSet::new();
    for spec in &args.refs {
        selected.extend(
            resolve_ref(&args.repo, spec)?
                .lines()
                .map(ToOwned::to_owned),
        );
    }

    let available = commits.lines().collect::<BTreeSet<_>>();
    let unknown = selected
        .iter()
        .filter(|commit| !available.contains(commit.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    if !unknown.is_empty() {
        return Err(crate::invalid(format!(
            "requested commits are outside the review range: {}",
            unknown.join(", ")
        )));
    }

    Ok(selected)
}

/// Compute requested commits in dependency-safe Git order.
fn resolve_commits(args: &ResolveTargetsArgs) -> Result<Vec<String>, SkillError> {
    let commits = revisions(args)?;
    let selected = requested(args, &commits)?;
    Ok(commits
        .lines()
        .filter(|commit| selected.contains(*commit))
        .map(ToOwned::to_owned)
        .collect())
}

/// Write one selected commit per JSONL record.
fn run_with_writer(args: &ResolveTargetsArgs, out: &mut dyn Write) -> Result<(), SkillError> {
    for commit in resolve_commits(args)? {
        write_json_line(out, &json_object([("commit", Value::String(commit))]))?;
    }
    Ok(())
}

/// Resolve and emit target commits as JSONL.
///
/// # Errors
///
/// Returns `SkillError` when Git or output fails.
pub(crate) fn run(args: &ResolveTargetsArgs) -> Result<(), SkillError> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    run_with_writer(args, &mut out)
}
