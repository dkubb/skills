use std::collections::HashSet;
use std::io::{self, Write};

use super::{
    ResolveTargetsArgs, requested, resolve_commits, resolve_ref, revisions, run, run_with_writer,
};
use crate::test_repo::TestRepo;

fn args(repo: &TestRepo, base_ref: &str) -> ResolveTargetsArgs {
    ResolveTargetsArgs {
        base_ref: base_ref.to_owned(),
        branch_ref: "HEAD".to_owned(),
        refs: Vec::new(),
        repo: repo.path().to_owned(),
    }
}

#[test]
fn resolves_full_range_by_default() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let first = repo.commit("Add first change");
    let second = repo.commit("Add second change");
    let target_args = args(&repo, &base);

    assert_eq!(
        resolve_commits(&target_args).expect("resolve commits"),
        vec![first, second]
    );
}

#[test]
fn resolves_single_commit_in_range_order() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let first = repo.commit("Add first change");
    let second = repo.commit("Add second change");
    let mut target_args = args(&repo, &base);
    target_args.refs = vec![second.clone(), first.clone()];

    assert_eq!(
        resolve_commits(&target_args).expect("resolve selected commits"),
        vec![first, second]
    );
}

#[test]
fn resolves_revision_range() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let first = repo.commit("Add first change");
    let second = repo.commit("Add second change");
    let mut target_args = args(&repo, &base);
    target_args.refs = vec![format!("{first}..{second}")];

    assert_eq!(
        resolve_commits(&target_args).expect("resolve selected range"),
        vec![second]
    );
}

#[test]
fn ignores_empty_requested_ref() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let commit = repo.commit("Add first change");
    let mut target_args = args(&repo, &base);
    target_args.refs = vec![String::new()];

    assert_eq!(
        requested(&target_args, &commit).expect("resolve empty selection"),
        HashSet::default()
    );
}

#[test]
fn revisions_reports_invalid_range() {
    let repo = TestRepo::new();
    let target_args = args(&repo, "missing");

    assert_eq!(
        revisions(&target_args)
            .expect_err("invalid range must fail")
            .exit_code(),
        1
    );
}

#[test]
fn resolve_ref_reports_invalid_ref() {
    let repo = TestRepo::new();

    assert_eq!(
        resolve_ref(repo.path(), "missing")
            .expect_err("invalid ref must fail")
            .exit_code(),
        1
    );
}

#[test]
fn requested_reports_invalid_ref() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let mut target_args = args(&repo, &base);
    target_args.refs = vec!["missing".to_owned()];

    assert_eq!(
        requested(&target_args, "")
            .expect_err("invalid ref must fail")
            .exit_code(),
        1
    );
}

#[test]
fn resolve_commits_reports_invalid_range() {
    let repo = TestRepo::new();
    let target_args = args(&repo, "missing");

    assert_eq!(
        resolve_commits(&target_args)
            .expect_err("invalid range must fail")
            .exit_code(),
        1
    );
}

#[test]
fn resolve_commits_reports_invalid_requested_ref() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let _commit = repo.commit("Add first change");
    let mut target_args = args(&repo, &base);
    target_args.refs = vec!["missing".to_owned()];

    assert_eq!(
        resolve_commits(&target_args)
            .expect_err("invalid requested ref must fail")
            .exit_code(),
        1
    );
}

#[test]
fn run_with_writer_emits_selected_commits() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let commit = repo.commit("Add target resolver");
    let target_args = args(&repo, &base);
    let mut output = Vec::new();

    assert_eq!(
        run_with_writer(&target_args, &mut output).expect("emit commits"),
        ()
    );
    assert_eq!(
        String::from_utf8(output).expect("target output must be UTF-8"),
        format!("{commit}\n")
    );
}

struct FailingWriter;

#[expect(
    clippy::missing_trait_methods,
    reason = "the default Write methods delegate to write for this failure fixture"
)]
impl Write for FailingWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
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
    let _commit = repo.commit("Add target resolver");
    let target_args = args(&repo, &base);

    assert_eq!(
        run_with_writer(&target_args, &mut FailingWriter)
            .expect_err("output failure must fail")
            .to_string(),
        "IO error: intentional write failure"
    );
}

#[test]
fn run_with_writer_reports_resolution_error() {
    let repo = TestRepo::new();
    let target_args = args(&repo, "missing");

    assert_eq!(
        run_with_writer(&target_args, &mut Vec::new())
            .expect_err("resolution failure must fail")
            .exit_code(),
        1
    );
}

#[test]
fn run_accepts_empty_range() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let target_args = args(&repo, &base);

    assert_eq!(run(&target_args).expect("empty range"), ());
}
