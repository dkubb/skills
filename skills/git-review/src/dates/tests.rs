use std::io::{self, Write};

use super::{
    CheckDatesArgs, Violation, find_violations, inspect_commit, inspect_commits, inspect_non_root,
    max_parent_timestamp, parse_author_committer, parse_timestamp, read_author_committer, run,
    run_with_writer,
};
use crate::test_repo::TestRepo;

fn args(repo: &TestRepo, base_ref: &str) -> CheckDatesArgs {
    CheckDatesArgs {
        base_ref: base_ref.to_owned(),
        branch_ref: "HEAD".to_owned(),
        repo: repo.path().to_owned(),
    }
}

#[test]
fn parse_timestamp_accepts_integer() {
    assert_eq!(
        parse_timestamp("42", "author").expect("valid timestamp"),
        42
    );
}

#[test]
fn parse_timestamp_rejects_non_integer() {
    assert_eq!(
        parse_timestamp("later", "author")
            .expect_err("invalid timestamp must fail")
            .to_string(),
        "invalid input: invalid author timestamp: invalid digit found in string"
    );
}

#[test]
fn parse_author_committer_accepts_pair() {
    assert_eq!(
        parse_author_committer("41 42").expect("valid pair"),
        (41, 42)
    );
}

#[test]
fn parse_author_committer_rejects_missing_author() {
    assert_eq!(
        parse_author_committer("")
            .expect_err("missing author must fail")
            .to_string(),
        "invalid input: missing author timestamp"
    );
}

#[test]
fn parse_author_committer_rejects_missing_committer() {
    assert_eq!(
        parse_author_committer("41")
            .expect_err("missing committer must fail")
            .to_string(),
        "invalid input: missing committer timestamp"
    );
}

#[test]
fn parse_author_committer_rejects_invalid_author() {
    assert_eq!(
        parse_author_committer("earlier 42")
            .expect_err("invalid author must fail")
            .to_string(),
        "invalid input: invalid author timestamp: invalid digit found in string"
    );
}

#[test]
fn parse_author_committer_rejects_invalid_committer() {
    assert_eq!(
        parse_author_committer("41 later")
            .expect_err("invalid committer must fail")
            .to_string(),
        "invalid input: invalid committer timestamp: invalid digit found in string"
    );
}

#[test]
fn max_parent_timestamp_selects_latest_parent() {
    let repo = TestRepo::new();
    let first = repo.commit_at("Add first parent", 1_700_000_000, 1_700_000_000);
    let second = repo.commit_at("Add second parent", 1_700_000_002, 1_700_000_002);

    assert_eq!(
        max_parent_timestamp(repo.path(), &format!("{first} {second}"))
            .expect("read parent timestamps"),
        1_700_000_002
    );
}

#[test]
fn max_parent_timestamp_accepts_no_parents() {
    let repo = TestRepo::new();

    assert_eq!(
        max_parent_timestamp(repo.path(), "").expect("empty parent set"),
        0
    );
}

#[test]
fn max_parent_timestamp_reports_missing_parent() {
    let repo = TestRepo::new();

    assert_eq!(
        max_parent_timestamp(repo.path(), "missing")
            .expect_err("missing parent must fail")
            .exit_code(),
        1
    );
}

#[test]
fn read_author_committer_reports_missing_commit() {
    let repo = TestRepo::new();

    assert_eq!(
        read_author_committer(repo.path(), "missing")
            .expect_err("missing commit must fail")
            .exit_code(),
        1
    );
}

#[test]
fn inspect_commit_accepts_root() {
    let repo = TestRepo::new();
    let root = repo.commit_at("Add foundation", 1_700_000_000, 1_700_000_000);

    assert_eq!(
        inspect_commit(repo.path(), &root).expect("inspect root"),
        None
    );
}

#[test]
fn inspect_commit_accepts_ordered_commit() {
    let repo = TestRepo::new();
    let _root = repo.commit_at("Add foundation", 1_700_000_000, 1_700_000_000);
    let commit = repo.commit_at("Add checker", 1_700_000_001, 1_700_000_001);

    assert_eq!(
        inspect_commit(repo.path(), &commit).expect("inspect commit"),
        None
    );
}

#[test]
fn inspect_commit_finds_violation() {
    let repo = TestRepo::new();
    let _root = repo.commit_at("Add foundation", 1_700_000_002, 1_700_000_002);
    let commit = repo.commit_at("Add checker", 1_700_000_001, 1_700_000_001);

    assert_eq!(
        inspect_commit(repo.path(), &commit).expect("inspect commit"),
        Some(Violation {
            commit,
            parent: 1_700_000_002,
            author: 1_700_000_001,
            committer: 1_700_000_001,
        })
    );
}

#[test]
fn inspect_commit_reports_missing_commit() {
    let repo = TestRepo::new();

    assert_eq!(
        inspect_commit(repo.path(), "missing")
            .expect_err("missing commit must fail")
            .exit_code(),
        1
    );
}

#[test]
fn inspect_non_root_reports_missing_parent() {
    let repo = TestRepo::new();

    assert_eq!(
        inspect_non_root(repo.path(), "missing", "missing")
            .expect_err("missing parent must fail")
            .exit_code(),
        1
    );
}

#[test]
fn inspect_non_root_reports_missing_commit() {
    let repo = TestRepo::new();
    let parent = repo.commit_at("Add foundation", 1_700_000_000, 1_700_000_000);

    assert_eq!(
        inspect_non_root(repo.path(), "missing", &parent)
            .expect_err("missing commit must fail")
            .exit_code(),
        1
    );
}

#[test]
fn inspect_commits_reports_invalid_entry() {
    let repo = TestRepo::new();

    assert_eq!(
        inspect_commits(repo.path(), "missing")
            .expect_err("invalid entry must fail")
            .exit_code(),
        1
    );
}

#[test]
fn find_violations_reports_invalid_range() {
    let repo = TestRepo::new();
    let check_args = args(&repo, "missing");

    assert_eq!(
        find_violations(&check_args)
            .expect_err("invalid range must fail")
            .exit_code(),
        1
    );
}

#[test]
fn run_with_writer_reports_ok() {
    let repo = TestRepo::new();
    let base = repo.commit_at("Add foundation", 1_700_000_000, 1_700_000_000);
    let _commit = repo.commit_at("Add checker", 1_700_000_001, 1_700_000_001);
    let check_args = args(&repo, &base);
    let mut output = Vec::new();

    assert_eq!(
        run_with_writer(&check_args, &mut output).expect("ordered range"),
        ()
    );
    assert_eq!(output, b"DATE_ORDER_OK\n");
}

#[test]
fn run_with_writer_reports_violation() {
    let repo = TestRepo::new();
    let base = repo.commit_at("Add foundation", 1_700_000_002, 1_700_000_002);
    let commit = repo.commit_at("Add checker", 1_700_000_001, 1_700_000_001);
    let check_args = args(&repo, &base);
    let mut output = Vec::new();

    let error = run_with_writer(&check_args, &mut output).expect_err("violation must fail");

    assert_eq!(
        String::from_utf8(output).expect("report must be UTF-8"),
        format!(
            "DATE_ORDER_VIOLATIONS\n{commit} parent=1700000002 author=1700000001 committer=1700000001\n"
        )
    );
    assert_eq!(
        error.to_string(),
        "invalid input: date order violations detected (1)"
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

struct FailAfterOneWrite {
    writes: usize,
}

#[expect(
    clippy::missing_trait_methods,
    reason = "the default Write methods delegate to write for this failure fixture"
)]
impl Write for FailAfterOneWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writes += 1;
        if self.writes == 1 {
            Ok(buf.len())
        } else {
            Err(io::Error::other("intentional write failure"))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn run_with_writer_reports_ok_output_error() {
    let repo = TestRepo::new();
    let base = repo.commit_at("Add foundation", 1_700_000_000, 1_700_000_000);
    let check_args = args(&repo, &base);

    assert_eq!(
        run_with_writer(&check_args, &mut FailingWriter)
            .expect_err("output failure must fail")
            .to_string(),
        "IO error: intentional write failure"
    );
}

#[test]
fn run_with_writer_reports_violation_header_error() {
    let repo = TestRepo::new();
    let base = repo.commit_at("Add foundation", 1_700_000_002, 1_700_000_002);
    let _commit = repo.commit_at("Add checker", 1_700_000_001, 1_700_000_001);
    let check_args = args(&repo, &base);

    assert_eq!(
        run_with_writer(&check_args, &mut FailingWriter)
            .expect_err("output failure must fail")
            .to_string(),
        "IO error: intentional write failure"
    );
}

#[test]
fn run_with_writer_reports_violation_detail_error() {
    let repo = TestRepo::new();
    let base = repo.commit_at("Add foundation", 1_700_000_002, 1_700_000_002);
    let _commit = repo.commit_at("Add checker", 1_700_000_001, 1_700_000_001);
    let check_args = args(&repo, &base);
    let mut writer = FailAfterOneWrite { writes: 0 };

    assert_eq!(
        run_with_writer(&check_args, &mut writer)
            .expect_err("detail output failure must fail")
            .to_string(),
        "IO error: intentional write failure"
    );
}

#[test]
fn run_with_writer_reports_inspection_error() {
    let repo = TestRepo::new();
    let check_args = args(&repo, "missing");

    assert_eq!(
        run_with_writer(&check_args, &mut Vec::new())
            .expect_err("inspection failure must fail")
            .exit_code(),
        1
    );
}

#[test]
fn run_accepts_ordered_range() {
    let repo = TestRepo::new();
    let base = repo.commit_at("Add foundation", 1_700_000_000, 1_700_000_000);
    let _commit = repo.commit_at("Add checker", 1_700_000_001, 1_700_000_001);
    let check_args = args(&repo, &base);

    assert_eq!(run(&check_args).expect("ordered range"), ());
}
