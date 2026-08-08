//! Test-only Git repository fixture.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_REPO: AtomicU64 = AtomicU64::new(0);

pub(crate) struct TestRepo {
    path: PathBuf,
}

impl TestRepo {
    pub(crate) fn new() -> Self {
        let sequence = NEXT_REPO.fetch_add(1, Ordering::Relaxed);
        let path = env::temp_dir().join(format!(
            "skill-git-review-test-{}-{sequence}",
            process::id()
        ));
        assert!(!path.exists(), "test repository path must be unique");
        fs::create_dir_all(&path).expect("create unique test repository directory");

        let repo = Self { path };
        repo.git(&["init", "--quiet", "--initial-branch=main"]);
        repo.git(&["config", "user.name", "Git Review Tests"]);
        repo.git(&["config", "user.email", "git-review@example.invalid"]);
        repo
    }

    pub(crate) fn path(&self) -> &str {
        self.path.to_str().expect("test path must be UTF-8")
    }

    pub(crate) fn commit(&self, message: &str) -> String {
        self.git(&["commit", "--allow-empty", "--quiet", "--message", message]);
        self.output(&["rev-parse", "HEAD"])
    }

    pub(crate) fn commit_at(&self, message: &str, author: i64, committer: i64) -> String {
        let author_date = format!("@{author} +0000");
        let committer_date = format!("@{committer} +0000");
        let status = Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .arg("-C")
            .arg(&self.path)
            .args(["commit", "--allow-empty", "--quiet", "--message", message])
            .env("GIT_AUTHOR_DATE", author_date)
            .env("GIT_COMMITTER_DATE", committer_date)
            .status()
            .expect("run dated test Git commit");
        assert!(status.success(), "dated test Git commit failed");
        self.output(&["rev-parse", "HEAD"])
    }

    pub(crate) fn write(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.path.join(name);
        fs::write(&path, contents).expect("write test repository file");
        path
    }

    fn git(&self, args: &[&str]) {
        let status = Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .arg("-C")
            .arg(&self.path)
            .args(args)
            .status()
            .expect("run test Git command");
        assert!(status.success(), "test Git command failed: {args:?}");
    }

    fn output(&self, args: &[&str]) -> String {
        let output = Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .arg("-C")
            .arg(&self.path)
            .args(args)
            .output()
            .expect("run test Git command");
        assert!(output.status.success(), "test Git command failed: {args:?}");
        String::from_utf8(output.stdout)
            .expect("test Git output must be UTF-8")
            .trim_end()
            .to_owned()
    }
}

#[expect(
    clippy::missing_trait_methods,
    reason = "Drop cleanup has no meaningful pin_drop implementation"
)]
impl Drop for TestRepo {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("remove test repository directory");
    }
}
