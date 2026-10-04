//! Compose commit messages in the canonical Atomic Changes form.

use std::io;

use clap::{Parser, ValueEnum};
use serde_json::Value;
use skill_core::SkillError;

use crate::invalid;
use crate::output::{json_object, write_json_line};

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
pub(crate) fn has_atomic_subject(subject: &str) -> bool {
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
pub(crate) fn has_valid_action_lines(message: &str) -> bool {
    let lines: Vec<&str> = message.lines().collect();
    let trailer_start = trailer_start(&lines);

    lines.iter().enumerate().skip(2).all(|(position, line)| {
        let trimmed = line.trim_start();
        if ["What:", "Why:", "How:"]
            .iter()
            .any(|label| trimmed.starts_with(label))
        {
            return false;
        }
        if position >= trailer_start {
            return true;
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

/// Check wrapping, separators, and trailing whitespace, excluding trailer widths.
pub(crate) fn has_valid_format(message: &str) -> bool {
    let lines: Vec<&str> = message.lines().collect();
    let Some(subject) = lines.first() else {
        return false;
    };

    let trailer_start = trailer_start(&lines);

    !subject.is_empty()
        && lines.get(1).is_none_or(|line| line.is_empty())
        && lines.iter().enumerate().all(|(position, line)| {
            (line.len() <= 72 || position >= trailer_start) && !line.ends_with(char::is_whitespace)
        })
}

/// Locate a complete final trailer block, or return the end of the message.
fn trailer_start(lines: &[&str]) -> usize {
    let content_end = lines
        .iter()
        .rposition(|line| !line.is_empty())
        .map_or(0, |position| position + 1);
    let trailer_start = lines
        .iter()
        .take(content_end)
        .rposition(|line| line.is_empty())
        .map_or(content_end, |position| position + 1);
    let is_trailer = |line: &str| {
        line.split_once(':').is_some_and(|(key, _)| {
            let token = key.trim_end_matches([' ', '\t']);
            !token.is_empty()
                && token
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
    };
    let has_trailers = trailer_start >= 2
        && lines
            .get(trailer_start)
            .is_some_and(|line| is_trailer(line))
        && lines
            .iter()
            .take(content_end)
            .skip(trailer_start + 1)
            .all(|line| line.starts_with([' ', '\t']) || is_trailer(line));

    if has_trailers {
        trailer_start
    } else {
        lines.len()
    }
}

/// Parse the arguments for composing a canonical commit message.
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

/// Compose and emit a canonical commit message as JSON.
///
/// # Errors
///
/// Returns `SkillError` when the requested message is invalid or output
/// cannot be written.
pub(crate) fn run(args: &MessageArgs) -> Result<(), SkillError> {
    let message = render(args)?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    write_json_line(
        &mut out,
        &json_object([("message", Value::String(message))]),
    )?;
    Ok(())
}

#[cfg(test)]
mod proptests;

#[cfg(test)]
#[expect(
    clippy::inline_modules,
    reason = "group tests by their owning message contract"
)]
mod tests {
    mod has_valid_format {
        use crate::message::has_valid_format;

        #[test]
        fn accepts_long_trailer_values() {
            let hash = "a".repeat(40);
            let message = format!(
                "Fix gate evidence\n\nKeep exact command and tree identities.\n\n\
                 Gate-rti-default: {hash} {hash}\n\
                 Reviewed-by: {}\n\n",
                "Reviewer".repeat(20)
            );

            assert!(has_valid_format(&message));
        }

        #[test]
        fn accepts_folded_trailer_values() {
            let value = "a".repeat(80);
            let message = format!("Fix gate evidence\n\nGate-1 \t: {value}\n {value}\n\t{value}");

            assert!(has_valid_format(&message));
        }

        #[test]
        fn accepts_71_byte_subject_with_trailers() {
            let trailer = format!("\n\nGate-test: {}", "a".repeat(80));
            let message = format!("Fix {}{trailer}", "a".repeat(67));

            let accepted = has_valid_format(&message);

            assert!(accepted);
        }

        #[test]
        fn accepts_72_byte_subject_with_trailers() {
            let trailer = format!("\n\nGate-test: {}", "a".repeat(80));
            let message = format!("Fix {}{trailer}", "a".repeat(68));

            let accepted = has_valid_format(&message);

            assert!(accepted);
        }

        #[test]
        fn rejects_73_byte_subject_with_trailers() {
            let trailer = format!("\n\nGate-test: {}", "a".repeat(80));
            let message = format!("Fix {}{trailer}", "a".repeat(69));

            let accepted = has_valid_format(&message);

            assert!(!accepted);
        }

        #[test]
        fn accepts_71_byte_body_with_trailers() {
            let trailer = format!("\n\nGate-test: {}", "a".repeat(80));
            let message = format!("Fix gate evidence\n\n{}{trailer}", "a".repeat(71));

            let accepted = has_valid_format(&message);

            assert!(accepted);
        }

        #[test]
        fn accepts_72_byte_body_with_trailers() {
            let trailer = format!("\n\nGate-test: {}", "a".repeat(80));
            let message = format!("Fix gate evidence\n\n{}{trailer}", "a".repeat(72));

            let accepted = has_valid_format(&message);

            assert!(accepted);
        }

        #[test]
        fn rejects_73_byte_body_with_trailers() {
            let trailer = format!("\n\nGate-test: {}", "a".repeat(80));
            let message = format!("Fix gate evidence\n\n{}{trailer}", "a".repeat(73));

            let accepted = has_valid_format(&message);

            assert!(!accepted);
        }

        #[test]
        fn rejects_trailers_followed_by_prose() {
            let value = "a".repeat(80);
            let message = format!("Fix gate evidence\n\nGate-test: {value}\nOrdinary body.");

            assert!(!has_valid_format(&message));
        }

        #[test]
        fn rejects_trailers_without_a_separating_blank_line() {
            let message = format!(
                "Fix gate evidence\n\nOrdinary body.\nGate-test: {}",
                "a".repeat(80)
            );

            assert!(!has_valid_format(&message));
        }

        #[test]
        fn rejects_orphan_continuations() {
            let value = "a".repeat(80);
            let message = format!("Fix gate evidence\n\n {value}\nGate-test: {value}");

            assert!(!has_valid_format(&message));
        }

        #[test]
        fn rejects_underscore_in_trailer_keys() {
            let message = format!("Fix gate evidence\n\nBad_key: {}", "a".repeat(80));

            assert!(!has_valid_format(&message));
        }

        #[test]
        fn rejects_spaces_inside_trailer_keys() {
            let message = format!("Fix gate evidence\n\nBad key: {}", "a".repeat(80));

            assert!(!has_valid_format(&message));
        }

        #[test]
        fn rejects_empty_trailer_keys() {
            let message = format!("Fix gate evidence\n\n: {}", "a".repeat(80));

            assert!(!has_valid_format(&message));
        }

        #[test]
        fn rejects_non_ascii_trailer_keys() {
            let message = format!("Fix gate evidence\n\nTökén: {}", "a".repeat(80));

            assert!(!has_valid_format(&message));
        }

        #[test]
        fn rejects_trailing_whitespace_in_trailers() {
            let message = format!("Fix gate evidence\n\nGate-test: {} ", "a".repeat(80));

            assert!(!has_valid_format(&message));
        }
    }

    mod has_valid_action_lines {
        use crate::message::has_valid_action_lines;

        #[test]
        fn accepts_metadata_continuations() {
            let message = "Fix gate evidence\n\nGate-test: accepted\n - opaque metadata\n\t* opaque metadata\n-Token: accepted";

            assert!(has_valid_action_lines(message));
        }

        #[test]
        fn rejects_body_bullets_before_trailers() {
            let message = "Fix gate evidence\n\n- opaque body\n\nGate-test: accepted";

            assert!(!has_valid_action_lines(message));
        }

        #[test]
        fn rejects_body_labels_in_trailers() {
            let message = "Fix gate evidence\n\nWhy: explain the change";

            assert!(!has_valid_action_lines(message));
        }
    }
}
