//! Compose commit messages in the canonical Atomic Changes form.

use std::io::{self, Write as _};

use clap::{Parser, ValueEnum};
use skill_core::SkillError;

use crate::invalid;

/// Atomic Changes verbs in transformation-priority order.
const VERBS: [&str; 9] = [
    "Remove",
    "Fix",
    "Move",
    "Rename",
    "Refactor",
    "Change",
    "Add",
    "Upgrade",
    "Downgrade",
];

/// Canonical Atomic Changes action verbs.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum Verb {
    /// Remove existing capability.
    Remove,
    /// Fix an invalid state.
    Fix,
    /// Move code without changing its shape.
    Move,
    /// Rename identifiers.
    Rename,
    /// Restructure without changing behavior.
    Refactor,
    /// Change observable behavior.
    Change,
    /// Add public capability.
    Add,
    /// Upgrade a dependency.
    Upgrade,
    /// Downgrade a dependency.
    Downgrade,
}

impl Verb {
    /// Render the canonical capitalized verb.
    const fn as_str(self) -> &'static str {
        match self {
            Self::Remove => "Remove",
            Self::Fix => "Fix",
            Self::Move => "Move",
            Self::Rename => "Rename",
            Self::Refactor => "Refactor",
            Self::Change => "Change",
            Self::Add => "Add",
            Self::Upgrade => "Upgrade",
            Self::Downgrade => "Downgrade",
        }
    }
}

/// Determine whether a subject uses one canonical verb and simple summary.
fn has_atomic_subject(subject: &str) -> bool {
    let Some((verb, summary)) = subject.split_once(' ') else {
        return false;
    };

    VERBS.contains(&verb)
        && !summary.chars().all(char::is_whitespace)
        && !summary.ends_with('.')
        && !summary.split_whitespace().any(|word| {
            let bare_word = word.trim_matches(|character: char| !character.is_alphanumeric());
            bare_word.eq_ignore_ascii_case("and") || bare_word.eq_ignore_ascii_case("or")
        })
}

/// Check that every body bullet is a canonical action line.
fn has_valid_action_lines(message: &str) -> bool {
    message.lines().skip(2).all(|line| {
        let trimmed = line.trim_start();
        if ["What:", "Why:", "How:"]
            .iter()
            .any(|label| trimmed.starts_with(label))
        {
            return false;
        }
        let Some(rest) = trimmed.strip_prefix(['-', '*']) else {
            return true;
        };
        let Some(first) = rest.chars().next() else {
            return false;
        };
        if !first.is_whitespace() {
            return false;
        }

        let action = rest.trim_start();
        VERBS.iter().any(|verb| {
            action.starts_with(verb)
                && action.as_bytes().get(verb.len()) == Some(&b' ')
                && action.ends_with('.')
        })
    })
}

/// Check wrapping, separators, and trailing whitespace.
fn has_valid_format(message: &str) -> bool {
    let lines: Vec<&str> = message.lines().collect();
    let Some(subject) = lines.first() else {
        return false;
    };

    !subject.is_empty()
        && lines.get(1).is_none_or(|line| line.is_empty())
        && lines
            .iter()
            .all(|line| line.len() <= 72 && !line.ends_with(char::is_whitespace))
}

/// Compose and print a canonical commit message.
#[derive(Clone, Debug, Parser)]
pub(crate) struct MessageArgs {
    /// Semantic action verb.
    #[arg(value_enum)]
    verb: Verb,

    /// Imperative subject summary without the leading verb.
    #[arg(long)]
    summary: String,

    /// Single-observation prose body.
    #[arg(long, conflicts_with = "action")]
    body: Option<String>,

    /// Body action line with its leading Atomic Changes verb.
    #[arg(long, conflicts_with = "body")]
    action: Vec<String>,
}

/// Render a commit message from parsed arguments.
fn render(args: &MessageArgs) -> Result<String, SkillError> {
    let subject = format!("{} {}", args.verb.as_str(), args.summary.trim());
    if !has_atomic_subject(&subject) {
        return Err(invalid("message violates the Atomic Changes form"));
    }
    let mut message = subject;

    if let Some(body) = args.body.as_deref() {
        message.push_str("\n\n");
        message.push_str(body.trim());
    } else if !args.action.is_empty() {
        message.push_str("\n\n");
        for (index, action) in args.action.iter().enumerate() {
            if index > 0 {
                message.push('\n');
            }
            let reason = action.trim().trim_end_matches('.');
            message.push_str("- ");
            message.push_str(reason);
            message.push('.');
        }
    } else {
        // A subject-only message is valid.
    }

    if !has_valid_action_lines(&message) || !has_valid_format(&message) {
        return Err(invalid("message violates the Atomic Changes form"));
    }

    Ok(message)
}

/// Compose and print a canonical commit message.
///
/// # Errors
///
/// Returns `SkillError` when the requested message is invalid or output
/// cannot be written.
pub(crate) fn run(args: &MessageArgs) -> Result<(), SkillError> {
    let message = render(args)?;
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    writeln!(out, "{message}")?;
    Ok(())
}

#[cfg(test)]
mod tests;
