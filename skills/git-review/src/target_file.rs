//! Load explicit commit selections from files.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::Value;
use skill_core::SkillError;

use crate::invalid;

/// Exact commit identities selected for a review command.
pub(crate) struct Targets(BTreeSet<String>);

/// Load strict commit-selection JSONL records from `path`.
pub(crate) fn load(path: &Path) -> Result<Targets, SkillError> {
    let contents = fs::read_to_string(path)?;
    let mut targets = BTreeSet::new();

    for (index, line) in contents.lines().enumerate() {
        let value = serde_json::from_str::<Value>(line).map_err(|error| {
            invalid(format!(
                "invalid commit selection on line {}: {error}",
                index + 1
            ))
        })?;
        let object = value.as_object().filter(|map| map.len() == 1);
        let commit = object
            .and_then(|map| map.get("commit"))
            .and_then(Value::as_str)
            .ok_or_else(|| {
                invalid(format!(
                    "commit selection on line {} must be an object with one string commit field",
                    index + 1
                ))
            })?;
        if !targets.insert(commit.to_owned()) {
            return Err(invalid(format!("duplicate commit selection: {commit}")));
        }
    }

    Ok(Targets(targets))
}

/// Select requested commits and reject identities outside the review range.
pub(crate) fn select<'commits>(
    targets: Option<&Targets>,
    commits: &'commits str,
) -> Result<Vec<&'commits str>, SkillError> {
    let Some(selected_targets) = targets else {
        return Ok(commits.lines().collect());
    };

    let available = commits.lines().collect::<BTreeSet<_>>();
    let unknown = selected_targets
        .0
        .iter()
        .filter(|target| !available.contains(target.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    if !unknown.is_empty() {
        return Err(invalid(format!(
            "commit selections are outside the review range: {}",
            unknown.join(", ")
        )));
    }

    Ok(commits
        .lines()
        .filter(|commit| selected_targets.0.contains(*commit))
        .collect())
}
