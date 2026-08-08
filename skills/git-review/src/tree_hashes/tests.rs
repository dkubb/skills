use std::io::{self, Write};
use std::path::PathBuf;

use super::{
    TreeHashesArgs, find_hashes, inspect_commits, read_tree, revisions, run, run_with_writer,
};
use crate::test_repo::TestRepo;

fn args(repo: &TestRepo, base_ref: &str) -> TreeHashesArgs {
    TreeHashesArgs {
        base_ref: base_ref.to_owned(),
        branch_ref: "HEAD".to_owned(),
        commits_file: None,
        repo: repo.path().to_owned(),
    }
}

#[test]
fn finds_hashes_in_commit_order() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let first = repo.commit("Add first change");
    let second = repo.commit("Add second change");
    let tree = repo.tree(&first);
    let hash_args = args(&repo, &base);

    assert_eq!(
        find_hashes(&hash_args).expect("find hashes"),
        vec![(first, tree.clone()), (second, tree)]
    );
}

#[test]
fn filters_hashes_with_target_file() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let _excluded = repo.commit("Add first change");
    let included = repo.commit("Add second change");
    let tree = repo.tree(&included);
    let targets = repo.write("targets", &format!("{included}\n"));
    let mut hash_args = args(&repo, &base);
    hash_args.commits_file = Some(targets);

    assert_eq!(
        find_hashes(&hash_args).expect("find selected hashes"),
        vec![(included, tree)]
    );
}

#[test]
fn find_hashes_reports_target_file_error() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let mut hash_args = args(&repo, &base);
    hash_args.commits_file = Some(PathBuf::from("missing-git-review-targets"));

    assert_eq!(
        find_hashes(&hash_args)
            .expect_err("missing target file must fail")
            .exit_code(),
        3
    );
}

#[test]
fn revisions_reports_invalid_range() {
    let repo = TestRepo::new();
    let hash_args = args(&repo, "missing");

    assert_eq!(
        revisions(&hash_args)
            .expect_err("invalid range must fail")
            .exit_code(),
        1
    );
}

#[test]
fn find_hashes_reports_invalid_range() {
    let repo = TestRepo::new();
    let hash_args = args(&repo, "missing");

    assert_eq!(
        find_hashes(&hash_args)
            .expect_err("invalid range must fail")
            .exit_code(),
        1
    );
}

#[test]
fn read_tree_reports_missing_commit() {
    let repo = TestRepo::new();

    assert_eq!(
        read_tree(repo.path(), "missing")
            .expect_err("missing commit must fail")
            .exit_code(),
        1
    );
}

#[test]
fn inspect_commits_reports_missing_commit() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let hash_args = args(&repo, &base);

    assert_eq!(
        inspect_commits(&hash_args, None, "missing")
            .expect_err("missing commit must fail")
            .exit_code(),
        1
    );
}

#[test]
fn run_with_writer_emits_hashes() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let commit = repo.commit("Add tree hash reporter");
    let tree = repo.tree(&commit);
    let hash_args = args(&repo, &base);
    let mut output = Vec::new();

    assert_eq!(
        run_with_writer(&hash_args, &mut output).expect("emit hashes"),
        ()
    );
    assert_eq!(
        String::from_utf8(output).expect("hash output must be UTF-8"),
        format!("{commit} {tree}\n")
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
    let _commit = repo.commit("Add tree hash reporter");
    let hash_args = args(&repo, &base);

    assert_eq!(
        run_with_writer(&hash_args, &mut FailingWriter)
            .expect_err("output failure must fail")
            .to_string(),
        "IO error: intentional write failure"
    );
}

#[test]
fn run_with_writer_reports_inspection_error() {
    let repo = TestRepo::new();
    let hash_args = args(&repo, "missing");

    assert_eq!(
        run_with_writer(&hash_args, &mut Vec::new())
            .expect_err("inspection failure must fail")
            .exit_code(),
        1
    );
}

#[test]
fn run_accepts_empty_range() {
    let repo = TestRepo::new();
    let base = repo.commit("Add foundation");
    let hash_args = args(&repo, &base);

    assert_eq!(run(&hash_args).expect("empty range"), ());
}
