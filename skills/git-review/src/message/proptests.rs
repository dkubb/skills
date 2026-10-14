//! Bounded generated checks for the canonical trailer exception.

#[expect(
    clippy::inline_modules,
    reason = "group properties by their owning message contract"
)]
mod has_valid_format {
    use proptest::prelude::*;

    use crate::message::has_valid_format;

    proptest! {
        #[test]
        fn accepts_long_final_trailers(
            key in "[A-Za-z0-9-]{1,24}",
            spacing in "[ \t]{0,8}",
            indent in "[ \t]{1,8}",
            value in r"[^\r\n]{73,160}[^\s]",
        ) {
            let message = format!("Fix gate evidence\n\nGate-{key}{spacing}: {value}\n{indent}{value}");

            let accepted = has_valid_format(&message);

            prop_assert!(accepted);
        }

        #[test]
        fn rejects_long_body_before_trailers(
            body in "[A-Za-z0-9]{73,160}",
            value in "[A-Za-z0-9]{73,160}",
        ) {
            let message = format!("Fix gate evidence\n\n{body}\n\nGate-test: {value}");

            let accepted = has_valid_format(&message);

            prop_assert!(!accepted);
        }
    }
}

#[expect(
    clippy::inline_modules,
    reason = "group properties by their owning message contract"
)]
mod has_valid_action_lines {
    use proptest::prelude::*;

    use crate::message::has_valid_action_lines;

    proptest! {
        #[test]
        fn accepts_metadata_bullets(
            indent in "[ \t]{1,8}",
            marker in prop_oneof![Just('-'), Just('*')],
            value in "[A-Za-z0-9]{1,160}",
        ) {
            let message = format!("Fix gate evidence\n\nGate-test: accepted\n{indent}{marker} {value}");

            let accepted = has_valid_action_lines(&message);

            prop_assert!(accepted);
        }
    }
}

#[expect(
    clippy::inline_modules,
    reason = "group properties by their owning message contract"
)]
mod render {
    use proptest::prelude::*;

    use crate::message::{MessageArgs, Verb, has_valid_action_lines, has_valid_format, render};

    proptest! {
        #[test]
        fn preserves_long_trailers_for_the_checker(
            key in "[A-Za-z0-9-]{1,24}",
            value in "[A-Za-z0-9]{73,160}",
        ) {
            let body = format!("Keep exact evidence.\n\nGate-{key}: {value}");
            let expected = format!("Fix gate evidence\n\n{body}");
            let args = MessageArgs {
                verb: Verb::Fix,
                summary: "gate evidence".to_owned(),
                body: Some(body),
                action: Vec::new(),
            };

            let message = render(&args).expect("compose valid trailer metadata");

            prop_assert_eq!(&message, &expected);
            prop_assert!(has_valid_format(&message));
            prop_assert!(has_valid_action_lines(&message));
        }
    }
}
