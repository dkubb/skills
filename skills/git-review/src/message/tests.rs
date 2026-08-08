use super::{
    MessageArgs, Verb, has_atomic_subject, has_valid_action_lines, has_valid_format, render,
};

#[test]
fn renders_each_canonical_verb() {
    let actual = [
        Verb::Remove,
        Verb::Fix,
        Verb::Move,
        Verb::Rename,
        Verb::Refactor,
        Verb::Change,
        Verb::Add,
        Verb::Upgrade,
        Verb::Downgrade,
    ]
    .map(Verb::as_str);

    assert_eq!(
        actual,
        [
            "Remove",
            "Fix",
            "Move",
            "Rename",
            "Refactor",
            "Change",
            "Add",
            "Upgrade",
            "Downgrade",
        ]
    );
}

#[test]
fn renders_subject_only() {
    let args = MessageArgs {
        verb: Verb::Add,
        summary: "message composer".to_owned(),
        body: None,
        action: Vec::new(),
    };

    assert_eq!(
        render(&args).expect("valid message"),
        "Add message composer"
    );
}

#[test]
fn renders_action_lines() {
    let args = MessageArgs {
        verb: Verb::Fix,
        summary: "message validation".to_owned(),
        body: None,
        action: vec![
            "Fix reject compound subjects".to_owned(),
            "Fix enforce line wrapping.".to_owned(),
        ],
    };

    assert_eq!(
        render(&args).expect("valid message"),
        "Fix message validation\n\n- Fix reject compound subjects.\n- Fix enforce line wrapping."
    );
}

#[test]
fn renders_prose_body() {
    let args = MessageArgs {
        verb: Verb::Fix,
        summary: "message validation".to_owned(),
        body: Some("Explain why this commit exists.".to_owned()),
        action: Vec::new(),
    };

    assert_eq!(
        render(&args).expect("valid message"),
        "Fix message validation\n\nExplain why this commit exists."
    );
}

#[test]
fn rejects_compound_subjects() {
    let args = MessageArgs {
        verb: Verb::Add,
        summary: "composer and validator".to_owned(),
        body: None,
        action: Vec::new(),
    };

    assert_eq!(
        render(&args)
            .expect_err("compound subject must be rejected")
            .to_string(),
        "invalid input: message violates the Atomic Changes form"
    );
}

#[test]
fn rejects_invalid_action_lines() {
    let args = MessageArgs {
        verb: Verb::Fix,
        summary: "message validation".to_owned(),
        body: None,
        action: vec!["Create the capability".to_owned()],
    };

    assert_eq!(
        render(&args)
            .expect_err("invalid action line must be rejected")
            .to_string(),
        "invalid input: message violates the Atomic Changes form"
    );
}

#[test]
fn rejects_overlong_lines() {
    let args = MessageArgs {
        verb: Verb::Add,
        summary: "x".repeat(70),
        body: None,
        action: Vec::new(),
    };

    assert_eq!(
        render(&args)
            .expect_err("overlong line must be rejected")
            .to_string(),
        "invalid input: message violates the Atomic Changes form"
    );
}

#[test]
fn rejects_subject_without_summary() {
    assert!(!has_atomic_subject("Add"));
}

#[test]
fn rejects_labeled_body_line() {
    assert!(!has_valid_action_lines(
        "Fix message validation\n\nWhat: explain the change"
    ));
}

#[test]
fn accepts_non_action_body_line() {
    assert!(has_valid_action_lines(
        "Fix message validation\n\nExplain the change."
    ));
}

#[test]
fn rejects_empty_action_line() {
    assert!(!has_valid_action_lines("Fix message validation\n\n-"));
}

#[test]
fn rejects_action_without_marker_space() {
    assert!(!has_valid_action_lines(
        "Fix message validation\n\n-Fix explain the change."
    ));
}

#[test]
fn rejects_empty_message() {
    assert!(!has_valid_format(""));
}
