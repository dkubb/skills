use std::cell::Cell;
use std::io::{self, Write};

use super::{
    ResolveBaseArgs, decode_stdout, interpret_output, resolve, run, run_gh, run_gh_program,
    run_with_writer,
};
use crate::invalid;
use crate::test_repo::TestRepo;

fn args(repo: &TestRepo, base_ref: Option<&str>) -> ResolveBaseArgs {
    ResolveBaseArgs {
        base_ref: base_ref.map(ToOwned::to_owned),
        repo: repo.path().to_owned(),
    }
}

#[test]
fn resolve_prefers_explicit_base() {
    let repo = TestRepo::new();
    let resolve_args = args(&repo, Some("stable"));
    let calls: Cell<usize> = Cell::new(0);
    let mut query = |_repo: &str, _args: &[&str]| {
        calls.set(calls.get() + 1);
        Ok(Some("ignored".to_owned()))
    };

    assert_eq!(
        resolve(&resolve_args, &mut query).expect("resolve explicit base"),
        "stable"
    );
    assert_eq!(calls.get(), 0);
}

#[test]
fn resolve_prefers_pull_request_base() {
    let repo = TestRepo::new();
    let resolve_args = args(&repo, None);
    let mut query = |_repo: &str, _args: &[&str]| Ok(Some("release".to_owned()));

    assert_eq!(
        resolve(&resolve_args, &mut query).expect("resolve PR base"),
        "release"
    );
}

#[test]
fn resolve_uses_repository_default_after_missing_pull_request() {
    let repo = TestRepo::new();
    let resolve_args = args(&repo, None);
    let calls: Cell<usize> = Cell::new(0);
    let mut query = |_repo: &str, _args: &[&str]| {
        let call = calls.get();
        calls.set(call + 1);
        if call == 0 {
            Ok(None)
        } else {
            Ok(Some("trunk".to_owned()))
        }
    };

    assert_eq!(
        resolve(&resolve_args, &mut query).expect("resolve repository base"),
        "trunk"
    );
    assert_eq!(calls.get(), 2);
}

#[test]
fn resolve_falls_back_to_main() {
    let repo = TestRepo::new();
    let resolve_args = args(&repo, None);
    let mut query = |_repo: &str, _args: &[&str]| Ok(None);

    assert_eq!(
        resolve(&resolve_args, &mut query).expect("resolve fallback base"),
        "main"
    );
}

#[test]
fn resolve_reports_query_error() {
    let repo = TestRepo::new();
    let resolve_args = args(&repo, None);
    let mut query = |_repo: &str, _args: &[&str]| Err(invalid("query failed"));

    assert_eq!(
        resolve(&resolve_args, &mut query)
            .expect_err("query error must fail")
            .to_string(),
        "invalid input: query failed"
    );
}

#[test]
fn resolve_reports_repository_query_error() {
    let repo = TestRepo::new();
    let resolve_args = args(&repo, None);
    let calls: Cell<usize> = Cell::new(0);
    let mut query = |_repo: &str, _args: &[&str]| {
        let call = calls.get();
        calls.set(call + 1);
        if call == 0 {
            Ok(None)
        } else {
            Err(invalid("repository query failed"))
        }
    };

    assert_eq!(
        resolve(&resolve_args, &mut query)
            .expect_err("repository query error must fail")
            .to_string(),
        "invalid input: repository query failed"
    );
}

#[test]
fn run_gh_reports_success() {
    let repo = TestRepo::new();

    let version = run_gh(repo.path(), &["--version"])
        .expect("run GitHub CLI")
        .expect("GitHub CLI version must be present");

    assert!(!version.is_empty());
}

#[test]
fn run_gh_reports_command_failure_as_missing_value() {
    let repo = TestRepo::new();

    assert_eq!(
        run_gh(repo.path(), &["missing-git-review-command"]).expect("run GitHub CLI"),
        None
    );
}

#[test]
fn run_gh_program_reports_empty_output_as_missing_value() {
    let repo = TestRepo::new();

    assert_eq!(
        run_gh_program("/usr/bin/true", repo.path(), &[]).expect("run true"),
        None
    );
}

#[test]
fn run_gh_program_reports_command_start_failure() {
    let repo = TestRepo::new();

    assert_eq!(
        run_gh_program("missing-git-review-command", repo.path(), &[])
            .expect_err("missing command must fail")
            .exit_code(),
        3
    );
}

#[test]
fn decode_stdout_rejects_invalid_utf8() {
    assert_eq!(
        decode_stdout(vec![0xff])
            .expect_err("invalid UTF-8 must fail")
            .exit_code(),
        3
    );
}

#[test]
fn interpret_output_rejects_invalid_utf8() {
    assert_eq!(
        interpret_output(true, vec![0xff])
            .expect_err("invalid UTF-8 must fail")
            .exit_code(),
        3
    );
}

#[test]
fn run_with_writer_emits_base() {
    let repo = TestRepo::new();
    let resolve_args = args(&repo, Some("stable"));
    let mut query = |_repo: &str, _args: &[&str]| Ok(None);
    let mut output = Vec::new();

    assert_eq!(
        run_with_writer(&resolve_args, &mut output, &mut query).expect("emit base"),
        ()
    );
    assert_eq!(output, b"stable\n");
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
    let resolve_args = args(&repo, Some("stable"));
    let mut query = |_repo: &str, _args: &[&str]| Ok(None);

    assert_eq!(
        run_with_writer(&resolve_args, &mut FailingWriter, &mut query)
            .expect_err("output failure must fail")
            .to_string(),
        "IO error: intentional write failure"
    );
}

#[test]
fn run_with_writer_reports_resolution_error() {
    let repo = TestRepo::new();
    let resolve_args = args(&repo, None);
    let mut query = |_repo: &str, _args: &[&str]| Err(invalid("query failed"));

    assert_eq!(
        run_with_writer(&resolve_args, &mut Vec::new(), &mut query)
            .expect_err("resolution failure must fail")
            .to_string(),
        "invalid input: query failed"
    );
}

#[test]
fn run_accepts_explicit_base() {
    let repo = TestRepo::new();
    let resolve_args = args(&repo, Some("stable"));

    assert_eq!(run(&resolve_args).expect("explicit base"), ());
}
