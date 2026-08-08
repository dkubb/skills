use clap::Parser as _;

use super::{Args, run};

#[test]
fn run_without_subcommand_returns_unit_success() {
    let args = Args::parse_from(["git-review"]);

    assert_eq!(run(&args).expect("print help"), ());
}

#[test]
fn run_message_returns_unit_success() {
    let args = Args::parse_from([
        "git-review",
        "message",
        "add",
        "--summary",
        "message composer",
    ]);

    assert_eq!(run(&args).expect("compose message"), ());
}

#[test]
fn run_check_messages_returns_unit_success() {
    let repo = super::test_repo::TestRepo::new();
    let base = repo.commit("Add foundation");
    let _valid = repo.commit("Add message checker");
    let args = Args::parse_from([
        "git-review",
        "check-messages",
        &base,
        "HEAD",
        "--repo",
        repo.path(),
    ]);

    assert_eq!(run(&args).expect("check valid messages"), ());
}

#[test]
fn run_check_dates_returns_unit_success() {
    let repo = super::test_repo::TestRepo::new();
    let base = repo.commit_at("Add foundation", 1_700_000_000, 1_700_000_000);
    let _valid = repo.commit_at("Add date checker", 1_700_000_001, 1_700_000_001);
    let args = Args::parse_from([
        "git-review",
        "check-dates",
        &base,
        "HEAD",
        "--repo",
        repo.path(),
    ]);

    assert_eq!(run(&args).expect("check valid dates"), ());
}

#[test]
fn run_tree_hashes_returns_unit_success() {
    let repo = super::test_repo::TestRepo::new();
    let base = repo.commit("Add foundation");
    let _commit = repo.commit("Add tree hash reporter");
    let args = Args::parse_from([
        "git-review",
        "tree-hashes",
        &base,
        "HEAD",
        "--repo",
        repo.path(),
    ]);

    assert_eq!(run(&args).expect("emit tree hashes"), ());
}

#[test]
fn run_resolve_targets_returns_unit_success() {
    let repo = super::test_repo::TestRepo::new();
    let base = repo.commit("Add foundation");
    let _commit = repo.commit("Add target resolver");
    let args = Args::parse_from([
        "git-review",
        "resolve-targets",
        &base,
        "HEAD",
        "--repo",
        repo.path(),
    ]);

    assert_eq!(run(&args).expect("resolve targets"), ());
}
