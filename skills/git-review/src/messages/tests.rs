use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser as _;
use clap::error::ErrorKind;

use super::{
    CheckMessagesArgs, find_invalid, inspect_commits, load_targets, parse_message, read_message,
    revisions, run, run_with_writer,
};
use crate::test_repo::TestRepo;

fn range_args(repo: &TestRepo, base_ref: &str) -> CheckMessagesArgs {
    CheckMessagesArgs {
        root: false,
        base_ref: Some(base_ref.to_owned()),
        branch_ref: "HEAD".to_owned(),
        commits_file: None,
        repo: repo.path().to_owned(),
    }
}

#[test]
fn finds_invalid_message_in_range() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let _valid = repo.commit("Add message checker\n\n- Add direct validation tests.");
    let invalid = repo.commit("feat: add conventional subject");
    let args = range_args(&repo, &base);

    assert_eq!(
        find_invalid(&args).expect("inspect messages"),
        vec![(invalid, "feat: add conventional subject".to_owned())]
    );
}

#[test]
fn filters_commits_with_target_file() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let _excluded = repo.commit("feat: add excluded subject");
    let included = repo.commit("feat: add included subject");
    let targets = repo.write("targets", &format!("\n{included}\n"));
    let mut args = range_args(&repo, &base);
    args.commits_file = Some(targets);

    assert_eq!(
        find_invalid(&args).expect("inspect selected messages"),
        vec![(included, "feat: add included subject".to_owned())]
    );
}

#[test]
fn root_mode_uses_single_positional_ref() {
    let repo = TestRepo::new();
    let invalid = repo.commit("feat: add conventional subject");
    let args = CheckMessagesArgs {
        root: true,
        base_ref: Some("HEAD".to_owned()),
        branch_ref: "HEAD".to_owned(),
        commits_file: None,
        repo: repo.path().to_owned(),
    };

    assert_eq!(
        find_invalid(&args).expect("inspect root history"),
        vec![(invalid, "feat: add conventional subject".to_owned())]
    );
}

#[test]
fn root_mode_uses_branch_ref_without_positional_ref() {
    let repo = TestRepo::new();
    let commit = repo.commit("Add foundation");
    let args = CheckMessagesArgs {
        root: true,
        base_ref: None,
        branch_ref: "HEAD".to_owned(),
        commits_file: None,
        repo: repo.path().to_owned(),
    };

    assert_eq!(revisions(&args).expect("resolve root history"), commit);
}

#[test]
fn root_mode_rejects_two_positional_refs() {
    let repo = TestRepo::new();
    let args = CheckMessagesArgs {
        root: true,
        base_ref: Some("main".to_owned()),
        branch_ref: "HEAD~1".to_owned(),
        commits_file: None,
        repo: repo.path().to_owned(),
    };

    assert_eq!(
        revisions(&args)
            .expect_err("two root refs must be rejected")
            .to_string(),
        "invalid input: when --root is set, provide at most one ref (for example, --root HEAD)"
    );
}

#[test]
fn range_mode_requires_base_ref() {
    let repo = TestRepo::new();
    let args = CheckMessagesArgs {
        root: false,
        base_ref: None,
        branch_ref: "HEAD".to_owned(),
        commits_file: None,
        repo: repo.path().to_owned(),
    };

    assert_eq!(
        revisions(&args)
            .expect_err("missing base ref must be rejected")
            .to_string(),
        "invalid input: BASE_REF is required unless --root is provided"
    );
}

#[test]
fn load_targets_reports_missing_file() {
    let missing = PathBuf::from("missing-git-review-targets");

    let error = load_targets(&missing).expect_err("missing target file must fail");

    assert_eq!(
        error.to_string(),
        "IO error: No such file or directory (os error 2)"
    );
    assert_eq!(error.exit_code(), 3);
}

#[test]
fn find_invalid_reports_target_file_error() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let mut args = range_args(&repo, &base);
    args.commits_file = Some(PathBuf::from("missing-git-review-targets"));

    assert_eq!(
        find_invalid(&args)
            .expect_err("missing target file must fail")
            .exit_code(),
        3
    );
}

#[test]
fn find_invalid_reports_revision_error() {
    let repo = TestRepo::new();
    let args = range_args(&repo, "missing");

    assert_eq!(
        find_invalid(&args)
            .expect_err("missing base revision must fail")
            .exit_code(),
        1
    );
}

#[test]
fn inspect_commits_reports_missing_commit() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let args = range_args(&repo, &base);

    assert_eq!(
        inspect_commits(&args, None, "missing")
            .expect_err("missing commit must fail")
            .exit_code(),
        1
    );
}

#[test]
fn read_message_reports_missing_commit() {
    let repo = TestRepo::new();

    assert_eq!(
        read_message(repo.path(), "missing")
            .expect_err("missing commit must fail")
            .exit_code(),
        1
    );
}

#[test]
fn parse_message_rejects_missing_separator() {
    assert_eq!(
        parse_message("Add message checker")
            .expect_err("missing separator must fail")
            .to_string(),
        "invalid input: Git returned a message without a subject separator"
    );
}

#[test]
fn run_accepts_valid_range() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let _valid = repo.commit("Add message checker");
    let args = range_args(&repo, &base);

    assert_eq!(run(&args).expect("valid range"), ());
}

#[test]
fn run_rejects_invalid_range() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let invalid = repo.commit("feat: add conventional subject");
    let args = range_args(&repo, &base);

    let mut output = Vec::new();
    let error = run_with_writer(&args, &mut output).expect_err("invalid range must fail");

    assert_eq!(
        String::from_utf8(output).expect("message output must be UTF-8"),
        format!("{invalid} feat: add conventional subject\n")
    );
    assert_eq!(
        error.to_string(),
        "invalid input: found commit messages that violate the Atomic Changes form"
    );
}

struct FailingWriter;

#[expect(
    clippy::missing_trait_methods,
    reason = "the default Write methods delegate to write for this failure fixture"
)]
impl Write for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("intentional write failure"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn run_with_writer_reports_output_error() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let _invalid = repo.commit("feat: add conventional subject");
    let args = range_args(&repo, &base);

    assert_eq!(
        run_with_writer(&args, &mut FailingWriter)
            .expect_err("output failure must fail")
            .to_string(),
        "IO error: intentional write failure"
    );
}

#[test]
fn run_with_writer_reports_inspection_error() {
    let repo = TestRepo::new();
    let args = range_args(&repo, "missing");

    assert_eq!(
        run_with_writer(&args, &mut Vec::new())
            .expect_err("missing revision must fail")
            .exit_code(),
        1
    );
}

#[test]
fn clap_requires_base_without_root() {
    let error = CheckMessagesArgs::try_parse_from(["check-messages"])
        .expect_err("base ref must be required");

    assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
}
