//! Load explicit commit selections from files.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use skill_core::SkillError;

/// Load non-empty newline-delimited commit SHAs from `path`.
pub(crate) fn load(path: &Path) -> Result<HashSet<String>, SkillError> {
    let contents = fs::read_to_string(path)?;
    Ok(contents
        .lines()
        .filter(|line| !line.chars().all(char::is_whitespace))
        .map(ToOwned::to_owned)
        .collect())
}
