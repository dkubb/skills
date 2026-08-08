use std::env;
use std::ffi::OsString;
use std::fs;
use std::iter;
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

#[derive(Debug, Eq, PartialEq)]
struct Observed {
    code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

#[expect(
    clippy::default_numeric_fallback,
    reason = "the associated constant types fix every literal to i32"
)]
impl Observed {
    const COMMAND_ERROR: Option<i32> = Some(1);
    const INPUT_ERROR: Option<i32> = Some(4);
    const IO_ERROR: Option<i32> = Some(3);
    const SUCCESS: Option<i32> = Some(0);
}

impl From<Output> for Observed {
    fn from(output: Output) -> Self {
        Self {
            code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}

#[derive(Debug)]
struct TestRepo {
    directory: TempDir,
}

impl TestRepo {
    fn new() -> Self {
        let repo = Self {
            directory: tempfile::tempdir().expect("create temporary Git repository"),
        };
        repo.git(&["init", "--quiet", "--initial-branch=main"]);
        repo.git(&["config", "user.name", "Git Review Tests"]);
        repo.git(&["config", "user.email", "git-review@example.invalid"]);
        repo
    }

    fn commit(&self, message: &str) -> String {
        self.git(&["commit", "--allow-empty", "--quiet", "--message", message]);
        self.output(&["rev-parse", "HEAD"])
    }

    fn commit_at(&self, message: &str, author: i64, committer: i64) -> String {
        let output = Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .arg("-C")
            .arg(self.path())
            .args(["commit", "--allow-empty", "--quiet", "--message", message])
            .env("GIT_AUTHOR_DATE", format!("@{author} +0000"))
            .env("GIT_COMMITTER_DATE", format!("@{committer} +0000"))
            .output()
            .expect("run dated test Git commit");
        assert!(output.status.success(), "dated test Git commit failed");
        self.output(&["rev-parse", "HEAD"])
    }

    fn empty_message_commit(&self) -> String {
        self.git(&[
            "commit",
            "--allow-empty",
            "--allow-empty-message",
            "--quiet",
            "--message",
            "",
        ]);
        self.output(&["rev-parse", "HEAD"])
    }

    fn git(&self, args: &[&str]) {
        let output = Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .arg("-C")
            .arg(self.path())
            .args(args)
            .output()
            .expect("run test Git command");
        assert!(output.status.success(), "test Git command failed: {args:?}");
    }

    fn output(&self, args: &[&str]) -> String {
        let output = Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .arg("-C")
            .arg(self.path())
            .args(args)
            .output()
            .expect("run test Git command");
        assert!(output.status.success(), "test Git command failed: {args:?}");
        String::from_utf8(output.stdout)
            .expect("test Git output must be UTF-8")
            .trim_end()
            .to_owned()
    }

    fn path(&self) -> &Path {
        self.directory.path()
    }

    fn tree(&self, commit: &str) -> String {
        self.output(&["show", "-s", "--format=%T", commit])
    }

    fn write(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.path().join(name);
        fs::write(&path, contents).expect("write test repository file");
        path
    }
}

fn observed(output: Output) -> Observed {
    output.into()
}

fn skill() -> Command {
    Command::new(env!("CARGO_BIN_EXE_skill"))
}

fn command(repo: &TestRepo, subcommand: &str) -> Command {
    let mut command = skill();
    command
        .args(["git-review", subcommand])
        .arg("--repo")
        .arg(repo.path());
    command
}

#[expect(
    clippy::literal_string_with_formatting_args,
    reason = "shell parameter expansion is not Rust formatting"
)]
fn install_fake_gh(directory: &Path) {
    let script = directory.join("gh");
    fs::write(
        &script,
        "#!/bin/sh\n\
         set -eu\n\
         kind=$1\n\
         if [ \"${GH_TEST_INVALID_UTF8:-}\" = \"$kind\" ]; then printf '\\377'; exit 0; fi\n\
         if [ \"${GH_TEST_SILENT_FAIL:-}\" = \"$kind\" ]; then exit 7; fi\n\
         if [ \"${GH_TEST_FAIL:-}\" = \"$kind\" ]; then\n\
           printf 'forced gh failure\\n' >&2\n\
           exit 7\n\
         fi\n\
         case \"$kind\" in\n\
           pr) printf '%s' \"${GH_TEST_PR_BASE:-}\" ;;\n\
           repo) printf '%s' \"${GH_TEST_REPO_BASE:-}\" ;;\n\
           *) exit 9 ;;\n\
         esac\n",
    )
    .expect("write fake gh executable");
    let mut permissions = fs::metadata(&script)
        .expect("read fake gh metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(script, permissions).expect("make fake gh executable");
}

fn install_git_link(directory: &Path) {
    symlink("/usr/bin/git", directory.join("git")).expect("link Git into fake PATH");
}

#[expect(
    clippy::literal_string_with_formatting_args,
    reason = "shell parameter expansion is not Rust formatting"
)]
fn install_fake_git(directory: &Path) {
    let script = directory.join("git");
    fs::write(
        &script,
        "#!/bin/sh\n\
         set -eu\n\
         mode=${GIT_TEST_MODE:-dates}\n\
         if [ \"$mode\" = fail ]; then exit 7; fi\n\
         if [ \"$mode\" = stderr ]; then printf 'forced git failure\\n' >&2; exit 7; fi\n\
         if [ \"$mode\" = signal ]; then kill -TERM $$; fi\n\
         if [ \"$mode\" = invalid-utf8 ]; then printf '\\377'; exit 0; fi\n\
         fail_at=${GIT_TEST_FAIL_AT:-none}\n\
         case \"$*\" in\n\
           *rev-list*main..HEAD*) [ \"$fail_at\" != revisions ] || exit 7; printf 'child\\n' ;;\n\
           *rev-list*) [ \"$fail_at\" != resolve ] || exit 7; printf 'child\\n' ;;\n\
           *--format=%P*) [ \"$fail_at\" != parents ] || exit 7; printf '%s' \"${GIT_TEST_PARENTS-parent}\" ;;\n\
           *'--format=%at %ct'*) [ \"$fail_at\" != dates ] || exit 7; printf '%s' \"${GIT_TEST_DATES-101 101}\" ;;\n\
           *--format=%ct*) [ \"$fail_at\" != parent ] || exit 7; printf '%s' \"${GIT_TEST_PARENT-100}\" ;;\n\
           *--format=%s*) [ \"$fail_at\" != subject ] || exit 7; printf 'Add message checker' ;;\n\
           *--format=%B*) [ \"$fail_at\" != body ] || exit 7; printf 'Add message checker' ;;\n\
           *--format=%T*) [ \"$fail_at\" != tree ] || exit 7; printf 'tree' ;;\n\
           *) exit 9 ;;\n\
         esac\n",
    )
    .expect("write fake git executable");
    let mut permissions = fs::metadata(&script)
        .expect("read fake git metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(script, permissions).expect("make fake git executable");
}

fn path_with(directory: &Path) -> OsString {
    let current = env::var_os("PATH").expect("test process must define PATH");
    env::join_paths(iter::once(directory.to_path_buf()).chain(env::split_paths(&current)))
        .expect("prepend fake executable directory to PATH")
}

#[expect(
    clippy::inline_modules,
    reason = "the code-review contract requires tests grouped by public behavior"
)]
mod help {
    use super::{Observed, observed, skill};
    use pretty_assertions::assert_eq;

    #[test]
    fn prints_contract() {
        let output = skill()
            .arg("git-review")
            .output()
            .expect("run git-review help");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: concat!(
                    "review Git commits against the canonical atomic changes contract\n",
                    "\n",
                    "Usage: skill-git-review [COMMAND]\n",
                    "\n",
                    "Commands:\n",
                    "  check-dates      check for date ordering violations\n",
                    "  check-messages   check for invalid commit messages\n",
                    "  message          compose a canonical Atomic Changes commit message\n",
                    "  resolve-base     resolve the base ref for review\n",
                    "  resolve-targets  resolve commit targets for review\n",
                    "  tree-hashes      emit tree hashes for idempotence verification\n",
                    "  help             Print this message or the help of the given subcommand(s)\n",
                    "\n",
                    "Options:\n",
                    "  -h, --help  Print help\n",
                )
                .as_bytes()
                .to_vec(),
                stderr: Vec::new(),
            }
        );
    }
}

#[expect(
    clippy::inline_modules,
    reason = "the code-review contract requires tests grouped by public behavior"
)]
mod message {
    use super::{Observed, observed, skill};
    use pretty_assertions::assert_eq;

    fn expected(message: &str) -> Observed {
        Observed {
            code: Observed::SUCCESS,
            stdout: format!("{{\"message\":\"{message}\"}}\n").into_bytes(),
            stderr: Vec::new(),
        }
    }

    #[test]
    fn renders_remove() {
        let output = skill()
            .args(["git-review", "message", "remove", "--summary", "old path"])
            .output()
            .expect("compose Remove message");

        assert_eq!(observed(output), expected("Remove old path"));
    }

    #[test]
    fn renders_fix() {
        let output = skill()
            .args(["git-review", "message", "fix", "--summary", "date order"])
            .output()
            .expect("compose Fix message");

        assert_eq!(observed(output), expected("Fix date order"));
    }

    #[test]
    fn renders_move() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "move",
                "--summary",
                "target loader",
            ])
            .output()
            .expect("compose Move message");

        assert_eq!(observed(output), expected("Move target loader"));
    }

    #[test]
    fn renders_rename() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "rename",
                "--summary",
                "review command",
            ])
            .output()
            .expect("compose Rename message");

        assert_eq!(observed(output), expected("Rename review command"));
    }

    #[test]
    fn renders_refactor() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "refactor",
                "--summary",
                "target loading",
            ])
            .output()
            .expect("compose Refactor message");

        assert_eq!(observed(output), expected("Refactor target loading"));
    }

    #[test]
    fn renders_change() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "change",
                "--summary",
                "review output",
            ])
            .output()
            .expect("compose Change message");

        assert_eq!(observed(output), expected("Change review output"));
    }

    #[test]
    fn renders_add() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "add",
                "--summary",
                "message composer",
            ])
            .output()
            .expect("compose Add message");

        assert_eq!(observed(output), expected("Add message composer"));
    }

    #[test]
    fn renders_upgrade() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "upgrade",
                "--summary",
                "JSON dependency",
            ])
            .output()
            .expect("compose Upgrade message");

        assert_eq!(observed(output), expected("Upgrade JSON dependency"));
    }

    #[test]
    fn renders_downgrade() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "downgrade",
                "--summary",
                "JSON dependency",
            ])
            .output()
            .expect("compose Downgrade message");

        assert_eq!(observed(output), expected("Downgrade JSON dependency"));
    }

    #[test]
    fn renders_action_lines() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "fix",
                "--summary",
                "message validation",
                "--action",
                "Fix reject compound subjects",
                "--action",
                "Fix enforce line wrapping",
            ])
            .output()
            .expect("compose action-line message");

        assert_eq!(
            observed(output),
            expected(
                "Fix message validation\\n\\n- Fix reject compound subjects.\\n- Fix enforce line wrapping."
            )
        );
    }

    #[test]
    fn rejects_compound_subject() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "add",
                "--summary",
                "checker and reporter",
            ])
            .output()
            .expect("reject compound message");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: message violates the Atomic Changes form\n"
                    .to_vec(),
            }
        );
    }

    #[test]
    fn renders_prose_body() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "change",
                "--summary",
                "review output",
                "--body",
                "Explain the stable JSON contract.",
            ])
            .output()
            .expect("compose prose-body message");

        assert_eq!(
            observed(output),
            expected("Change review output\\n\\nExplain the stable JSON contract.")
        );
    }

    #[test]
    fn rejects_overlong_body() {
        let output = skill()
            .args([
                "git-review",
                "message",
                "change",
                "--summary",
                "review output",
                "--body",
                "This body line is deliberately longer than seventy-two characters so it must fail validation.",
            ])
            .output()
            .expect("reject overlong message body");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: message violates the Atomic Changes form\n"
                    .to_vec(),
            }
        );
    }
}

#[expect(
    clippy::inline_modules,
    reason = "the code-review contract requires tests grouped by public behavior"
)]
mod check_messages {
    use super::{Observed, TestRepo, command, install_fake_git, observed};
    use pretty_assertions::assert_eq;

    #[test]
    fn accepts_valid_range() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _valid = repo.commit("Add message checker");

        let output = command(&repo, "check-messages")
            .args([&base, "HEAD"])
            .output()
            .expect("check valid messages");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: Vec::new(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn defaults_branch_ref_to_head() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _valid = repo.commit("Add message checker");

        let output = command(&repo, "check-messages")
            .arg(&base)
            .output()
            .expect("check messages through HEAD");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: Vec::new(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn reports_invalid_message_as_jsonl() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let invalid = repo.commit("feat: add conventional subject");

        let output = command(&repo, "check-messages")
            .args([&base, "HEAD"])
            .output()
            .expect("check invalid messages");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: format!(
                    "{{\"commit\":\"{invalid}\",\"subject\":\"feat: add conventional subject\"}}\n"
                )
                .into_bytes(),
                stderr: b"Error: invalid input: found commit messages that violate the Atomic Changes form\n".to_vec(),
            }
        );
    }

    #[test]
    fn filters_with_resolver_jsonl() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _excluded = repo.commit("feat: add excluded subject");
        let included = repo.commit("feat: add included subject");
        let selections = repo.write("targets.jsonl", &format!("{{\"commit\":\"{included}\"}}\n"));

        let output = command(&repo, "check-messages")
            .args([&base, "HEAD", "--commits-file"])
            .arg(selections)
            .output()
            .expect("check selected message");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: format!(
                    "{{\"commit\":\"{included}\",\"subject\":\"feat: add included subject\"}}\n"
                )
                .into_bytes(),
                stderr: b"Error: invalid input: found commit messages that violate the Atomic Changes form\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_selection_outside_range() {
        let repo = TestRepo::new();
        let outside = repo.commit("Add base commit");
        let base = repo.commit("Add range base");
        let _inside = repo.commit("Add message checker");
        let selections = repo.write("targets.jsonl", &format!("{{\"commit\":\"{outside}\"}}\n"));

        let output = command(&repo, "check-messages")
            .args([&base, "HEAD", "--commits-file"])
            .arg(selections)
            .output()
            .expect("reject outside selection");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: format!(
                    "Error: invalid input: commit selections are outside the review range: {outside}\n"
                )
                .into_bytes(),
            }
        );
    }

    #[test]
    fn rejects_duplicate_selection() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let commit = repo.commit("Add message checker");
        let selections = repo.write(
            "targets.jsonl",
            &format!("{{\"commit\":\"{commit}\"}}\n{{\"commit\":\"{commit}\"}}\n"),
        );

        let output = command(&repo, "check-messages")
            .args([&base, "HEAD", "--commits-file"])
            .arg(selections)
            .output()
            .expect("reject duplicate selection");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: format!("Error: invalid input: duplicate commit selection: {commit}\n")
                    .into_bytes(),
            }
        );
    }

    #[test]
    fn checks_all_reachable_commits_from_explicit_root() {
        let repo = TestRepo::new();
        let _valid = repo.commit("Add base commit");
        let invalid = repo.commit("feat: add conventional subject");

        let output = command(&repo, "check-messages")
            .args(["--root", "HEAD"])
            .output()
            .expect("check explicit root history");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: format!(
                    "{{\"commit\":\"{invalid}\",\"subject\":\"feat: add conventional subject\"}}\n"
                )
                .into_bytes(),
                stderr: b"Error: invalid input: found commit messages that violate the Atomic Changes form\n".to_vec(),
            }
        );
    }

    #[test]
    fn checks_head_when_root_ref_is_omitted() {
        let repo = TestRepo::new();
        let invalid = repo.commit("feat: add conventional subject");

        let output = command(&repo, "check-messages")
            .arg("--root")
            .output()
            .expect("check default root history");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: format!(
                    "{{\"commit\":\"{invalid}\",\"subject\":\"feat: add conventional subject\"}}\n"
                )
                .into_bytes(),
                stderr: b"Error: invalid input: found commit messages that violate the Atomic Changes form\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_two_refs_with_root() {
        let repo = TestRepo::new();

        let output = command(&repo, "check-messages")
            .args(["--root", "main", "HEAD"])
            .output()
            .expect("reject two root refs");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: when --root is set, provide at most one ref (for example, --root HEAD)\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_missing_base_without_root() {
        let repo = TestRepo::new();

        let output = command(&repo, "check-messages")
            .output()
            .expect("reject missing range base");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: BASE_REF is required unless --root is provided\n"
                    .to_vec(),
            }
        );
    }

    #[test]
    fn rejects_every_malformed_message_shape() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let no_summary = repo.commit("Add");
        let labeled = repo.commit("Add labeled body\n\nWhat: explain the change");
        let empty_action = repo.commit("Add empty action\n\n-");
        let unspaced_action = repo.commit("Add unspaced action\n\n-Fix explain the change.");
        let unfinished_action = repo.commit("Add unfinished action\n\n- Add explain the change");
        let empty = repo.empty_message_commit();

        let output = command(&repo, "check-messages")
            .args([&base, "HEAD"])
            .output()
            .expect("check malformed message shapes");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: format!(
                    "{{\"commit\":\"{no_summary}\",\"subject\":\"Add\"}}\n\
                     {{\"commit\":\"{labeled}\",\"subject\":\"Add labeled body\"}}\n\
                     {{\"commit\":\"{empty_action}\",\"subject\":\"Add empty action\"}}\n\
                     {{\"commit\":\"{unspaced_action}\",\"subject\":\"Add unspaced action\"}}\n\
                     {{\"commit\":\"{unfinished_action}\",\"subject\":\"Add unfinished action\"}}\n\
                     {{\"commit\":\"{empty}\",\"subject\":\"\"}}\n"
                )
                .into_bytes(),
                stderr: b"Error: invalid input: found commit messages that violate the Atomic Changes form\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_malformed_selection_json() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _commit = repo.commit("Add message checker");
        let selections = repo.write("targets.jsonl", "not-json\n");

        let output = command(&repo, "check-messages")
            .args([&base, "HEAD", "--commits-file"])
            .arg(selections)
            .output()
            .expect("reject malformed selection JSON");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: invalid commit selection on line 1: expected ident at line 1 column 2\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_selection_with_extra_field() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let commit = repo.commit("Add message checker");
        let selections = repo.write(
            "targets.jsonl",
            &format!("{{\"commit\":\"{commit}\",\"extra\":true}}\n"),
        );

        let output = command(&repo, "check-messages")
            .args([&base, "HEAD", "--commits-file"])
            .arg(selections)
            .output()
            .expect("reject wide selection object");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: commit selection on line 1 must be an object with one string commit field\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_three_range_refs() {
        let repo = TestRepo::new();

        let output = command(&repo, "check-messages")
            .args(["main", "HEAD", "extra"])
            .output()
            .expect("reject three range refs");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: provide a base ref and at most one branch ref (for example, main HEAD)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_missing_selection_file() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _commit = repo.commit("Add message checker");

        let output = command(&repo, "check-messages")
            .args([&base, "HEAD", "--commits-file", "missing.jsonl"])
            .output()
            .expect("report missing selection file");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: No such file or directory (os error 2)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_subject_query_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-messages")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_FAIL_AT", "subject")
            .output()
            .expect("report subject query failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git show -s --format=%s child` exited with code 7\n"
                    .to_vec(),
            }
        );
    }

    #[test]
    fn reports_body_query_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-messages")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_FAIL_AT", "body")
            .output()
            .expect("report body query failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git show -s --format=%B child` exited with code 7\n"
                    .to_vec(),
            }
        );
    }
}

#[expect(
    clippy::inline_modules,
    reason = "the code-review contract requires tests grouped by public behavior"
)]
mod check_dates {
    use super::{Observed, TestRepo, command, install_fake_git, observed};
    use pretty_assertions::assert_eq;

    #[test]
    fn reports_ordered_range_as_json() {
        let repo = TestRepo::new();
        let base = repo.commit_at("Add base commit", 100, 100);
        let _ordered = repo.commit_at("Add date checker", 101, 101);

        let output = command(&repo, "check-dates")
            .args([&base, "HEAD"])
            .output()
            .expect("check ordered dates");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: b"{\"status\":\"ok\",\"violations\":[]}\n".to_vec(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn reports_violation_as_json() {
        let repo = TestRepo::new();
        let base = repo.commit_at("Add base commit", 100, 100);
        let invalid = repo.commit_at("Add date checker", 100, 100);

        let output = command(&repo, "check-dates")
            .args([&base, "HEAD"])
            .output()
            .expect("check violating dates");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: format!(
                    "{{\"status\":\"violations\",\"violations\":[{{\"author\":100,\"commit\":\"{invalid}\",\"committer\":100,\"parent\":100}}]}}\n"
                )
                .into_bytes(),
                stderr: b"Error: invalid input: date order violations detected (1)\n".to_vec(),
            }
        );
    }

    #[test]
    fn accepts_root_commit() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_PARENTS", "")
            .output()
            .expect("check root commit dates");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: b"{\"status\":\"ok\",\"violations\":[]}\n".to_vec(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn rejects_missing_author_timestamp() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_DATES", "")
            .output()
            .expect("report missing author timestamp");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: missing author timestamp\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_missing_committer_timestamp() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_DATES", "101")
            .output()
            .expect("report missing committer timestamp");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: missing committer timestamp\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_invalid_author_timestamp() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_DATES", "bad 101")
            .output()
            .expect("report invalid author timestamp");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: invalid author timestamp: invalid digit found in string\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_invalid_committer_timestamp() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_DATES", "101 bad")
            .output()
            .expect("report invalid committer timestamp");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: invalid committer timestamp: invalid digit found in string\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_invalid_parent_timestamp() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_PARENT", "bad")
            .output()
            .expect("report invalid parent timestamp");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: invalid input: invalid parent timestamp: invalid digit found in string\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_git_failure_without_stderr() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_MODE", "fail")
            .output()
            .expect("report silent Git failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git rev-list --reverse --topo-order main..HEAD` exited with code 7\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_signaled_git_process() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_MODE", "signal")
            .output()
            .expect("report signaled Git process");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git rev-list --reverse --topo-order main..HEAD` terminated by signal\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_invalid_utf8_git_output() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_MODE", "invalid-utf8")
            .output()
            .expect("report invalid UTF-8 Git output");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: invalid utf-8 sequence of 1 bytes from index 0\n"
                    .to_vec(),
            }
        );
    }

    #[test]
    fn reports_git_failure_with_stderr() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_MODE", "stderr")
            .output()
            .expect("report Git failure with stderr");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git rev-list --reverse --topo-order main..HEAD: forced git failure` exited with code 7\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_parent_list_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_FAIL_AT", "parents")
            .output()
            .expect("report parent-list failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git show -s --format=%P child` exited with code 7\n"
                    .to_vec(),
            }
        );
    }

    #[test]
    fn reports_parent_timestamp_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_FAIL_AT", "parent")
            .output()
            .expect("report parent timestamp failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git show -s --format=%ct parent` exited with code 7\n"
                    .to_vec(),
            }
        );
    }

    #[test]
    fn reports_commit_timestamp_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_FAIL_AT", "dates")
            .output()
            .expect("report commit timestamp failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git show -s --format=%at %ct child` exited with code 7\n"
                    .to_vec(),
            }
        );
    }

    #[test]
    fn reports_missing_git_executable() {
        let repo = TestRepo::new();
        let empty_path = tempfile::tempdir().expect("create empty executable directory");

        let output = command(&repo, "check-dates")
            .args(["main", "HEAD"])
            .env("PATH", empty_path.path())
            .output()
            .expect("report missing Git executable");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: No such file or directory (os error 2)\n".to_vec(),
            }
        );
    }
}

#[expect(
    clippy::inline_modules,
    reason = "the code-review contract requires tests grouped by public behavior"
)]
mod tree_hashes {
    use super::{Observed, TestRepo, command, install_fake_git, observed};
    use pretty_assertions::assert_eq;

    #[test]
    fn emits_ordered_jsonl() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let first = repo.commit("Add first commit");
        let second = repo.commit("Add second commit");
        let first_tree = repo.tree(&first);
        let second_tree = repo.tree(&second);

        let output = command(&repo, "tree-hashes")
            .args([&base, "HEAD"])
            .output()
            .expect("emit tree hashes");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: format!(
                    "{{\"commit\":\"{first}\",\"tree\":\"{first_tree}\"}}\n{{\"commit\":\"{second}\",\"tree\":\"{second_tree}\"}}\n"
                )
                .into_bytes(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn filters_with_resolver_jsonl() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _excluded = repo.commit("Add first commit");
        let included = repo.commit("Add second commit");
        let tree = repo.tree(&included);
        let selections = repo.write("targets.jsonl", &format!("{{\"commit\":\"{included}\"}}\n"));

        let output = command(&repo, "tree-hashes")
            .args([&base, "HEAD", "--commits-file"])
            .arg(selections)
            .output()
            .expect("emit selected tree hash");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: format!("{{\"commit\":\"{included}\",\"tree\":\"{tree}\"}}\n").into_bytes(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn reports_missing_selection_file() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _commit = repo.commit("Add tree hash reporter");

        let output = command(&repo, "tree-hashes")
            .args([&base, "HEAD", "--commits-file", "missing.jsonl"])
            .output()
            .expect("report missing selection file");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: No such file or directory (os error 2)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_revision_query_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "tree-hashes")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_MODE", "fail")
            .output()
            .expect("report revision query failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git rev-list --reverse --topo-order main..HEAD` exited with code 7\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_tree_query_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "tree-hashes")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_FAIL_AT", "tree")
            .output()
            .expect("report tree query failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git show -s --format=%T child` exited with code 7\n"
                    .to_vec(),
            }
        );
    }

    #[test]
    fn rejects_selection_outside_range() {
        let repo = TestRepo::new();
        let outside = repo.commit("Add outside commit");
        let base = repo.commit("Add base commit");
        let _inside = repo.commit("Add tree hash reporter");
        let selections = repo.write("targets.jsonl", &format!("{{\"commit\":\"{outside}\"}}\n"));

        let output = command(&repo, "tree-hashes")
            .args([&base, "HEAD", "--commits-file"])
            .arg(selections)
            .output()
            .expect("reject outside tree selection");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: format!(
                    "Error: invalid input: commit selections are outside the review range: {outside}\n"
                )
                .into_bytes(),
            }
        );
    }
}

#[expect(
    clippy::inline_modules,
    reason = "the code-review contract requires tests grouped by public behavior"
)]
mod resolve_targets {
    use super::{Observed, TestRepo, command, install_fake_git, observed};
    use pretty_assertions::assert_eq;

    #[test]
    fn emits_full_range_as_jsonl() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let first = repo.commit("Add first commit");
        let second = repo.commit("Add second commit");

        let output = command(&repo, "resolve-targets")
            .args([&base, "HEAD"])
            .output()
            .expect("resolve full range");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: format!("{{\"commit\":\"{first}\"}}\n{{\"commit\":\"{second}\"}}\n")
                    .into_bytes(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn emits_requested_commit() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let first = repo.commit("Add first commit");
        let _second = repo.commit("Add second commit");

        let output = command(&repo, "resolve-targets")
            .args([&base, "HEAD", &first])
            .output()
            .expect("resolve requested commit");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: format!("{{\"commit\":\"{first}\"}}\n").into_bytes(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn rejects_commit_outside_range() {
        let repo = TestRepo::new();
        let _root = repo.commit("Add root commit");
        let outside = repo.commit("Add outside commit");
        let base = repo.commit("Add base commit");
        let _inside = repo.commit("Add inside commit");

        let output = command(&repo, "resolve-targets")
            .args([&base, "HEAD", &outside])
            .output()
            .expect("reject requested commit outside range");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::INPUT_ERROR,
                stdout: Vec::new(),
                stderr: format!(
                    "Error: invalid input: requested commits are outside the review range: {outside}\n"
                )
                .into_bytes(),
            }
        );
    }

    #[test]
    fn resolves_requested_revision_range() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let first = repo.commit("Add first commit");
        let second = repo.commit("Add second commit");
        let third = repo.commit("Add third commit");
        let requested = format!("{first}..{third}");

        let output = command(&repo, "resolve-targets")
            .args([&base, "HEAD", &requested])
            .output()
            .expect("resolve requested revision range");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: format!("{{\"commit\":\"{second}\"}}\n{{\"commit\":\"{third}\"}}\n")
                    .into_bytes(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn reports_review_range_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "resolve-targets")
            .args(["main", "HEAD"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_FAIL_AT", "revisions")
            .output()
            .expect("report review range failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git rev-list --reverse --topo-order main..HEAD` exited with code 7\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_requested_ref_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "resolve-targets")
            .args(["main", "HEAD", "child"])
            .env("PATH", fake_bin.path())
            .env("GIT_TEST_FAIL_AT", "resolve")
            .output()
            .expect("report requested ref failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git rev-list --reverse --topo-order child^..child` exited with code 7\n".to_vec(),
            }
        );
    }
}

#[expect(
    clippy::inline_modules,
    reason = "the code-review contract requires tests grouped by public behavior"
)]
mod resolve_base {
    use super::{
        Observed, TestRepo, command, install_fake_gh, install_fake_git, install_git_link, observed,
        path_with,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn emits_explicit_base_as_json() {
        let repo = TestRepo::new();

        let output = command(&repo, "resolve-base")
            .arg("stable")
            .output()
            .expect("resolve explicit base");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: b"{\"base_ref\":\"stable\"}\n".to_vec(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn prefers_pull_request_base() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake gh directory");
        install_fake_gh(fake_bin.path());

        let output = command(&repo, "resolve-base")
            .env("PATH", path_with(fake_bin.path()))
            .env("GH_TEST_PR_BASE", "develop")
            .env("GH_TEST_REPO_BASE", "stable")
            .output()
            .expect("resolve pull request base");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: b"{\"base_ref\":\"develop\"}\n".to_vec(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn uses_repository_default_without_pull_request() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake gh directory");
        install_fake_gh(fake_bin.path());

        let output = command(&repo, "resolve-base")
            .env("PATH", path_with(fake_bin.path()))
            .env("GH_TEST_REPO_BASE", "stable")
            .output()
            .expect("resolve repository default base");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: b"{\"base_ref\":\"stable\"}\n".to_vec(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn rejects_pull_request_query_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake gh directory");
        install_fake_gh(fake_bin.path());

        let output = command(&repo, "resolve-base")
            .env("PATH", path_with(fake_bin.path()))
            .env("GH_TEST_FAIL", "pr")
            .output()
            .expect("report pull request query failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `gh pr list --state all --head main --limit 1 --json baseRefName --jq .[0].baseRefName // empty: forced gh failure` exited with code 7\n".to_vec(),
            }
        );
    }

    #[test]
    fn falls_back_to_main_when_queries_are_empty() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake gh directory");
        install_fake_gh(fake_bin.path());

        let output = command(&repo, "resolve-base")
            .env("PATH", path_with(fake_bin.path()))
            .output()
            .expect("resolve main fallback");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::SUCCESS,
                stdout: b"{\"base_ref\":\"main\"}\n".to_vec(),
                stderr: Vec::new(),
            }
        );
    }

    #[test]
    fn rejects_silent_pull_request_query_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake gh directory");
        install_fake_gh(fake_bin.path());

        let output = command(&repo, "resolve-base")
            .env("PATH", path_with(fake_bin.path()))
            .env("GH_TEST_SILENT_FAIL", "pr")
            .output()
            .expect("report silent pull request query failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `gh pr list --state all --head main --limit 1 --json baseRefName --jq .[0].baseRefName // empty` exited with code 7\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_invalid_utf8_query_output() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake gh directory");
        install_fake_gh(fake_bin.path());

        let output = command(&repo, "resolve-base")
            .env("PATH", path_with(fake_bin.path()))
            .env("GH_TEST_INVALID_UTF8", "pr")
            .output()
            .expect("report invalid UTF-8 query output");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: invalid utf-8 sequence of 1 bytes from index 0\n"
                    .to_vec(),
            }
        );
    }

    #[test]
    fn rejects_missing_github_cli() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create Git-only PATH");
        install_git_link(fake_bin.path());

        let output = command(&repo, "resolve-base")
            .env("PATH", fake_bin.path())
            .output()
            .expect("report missing GitHub CLI");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: No such file or directory (os error 2)\n".to_vec(),
            }
        );
    }

    #[test]
    fn rejects_repository_query_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake gh directory");
        install_fake_gh(fake_bin.path());

        let output = command(&repo, "resolve-base")
            .env("PATH", path_with(fake_bin.path()))
            .env("GH_TEST_FAIL", "repo")
            .output()
            .expect("report repository query failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `gh repo view --json defaultBranchRef --jq .defaultBranchRef.name: forced gh failure` exited with code 7\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_branch_query_failure() {
        let repo = TestRepo::new();
        let fake_bin = tempfile::tempdir().expect("create fake git directory");
        install_fake_git(fake_bin.path());

        let output = command(&repo, "resolve-base")
            .env("PATH", fake_bin.path())
            .output()
            .expect("report branch query failure");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::COMMAND_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: command `git branch --show-current` exited with code 9\n".to_vec(),
            }
        );
    }
}

#[expect(
    clippy::inline_modules,
    reason = "the code-review contract requires tests grouped by public behavior"
)]
mod output_failure {
    use std::os::fd::OwnedFd;
    use std::os::unix::net::UnixStream;
    use std::process::Stdio;

    use super::{Observed, TestRepo, command, observed, skill};
    use pretty_assertions::assert_eq;

    fn closed_stdout() -> Stdio {
        let (reader, writer) = UnixStream::pair().expect("create stdout socket pair");
        drop(reader);
        let writer_fd: OwnedFd = writer.into();
        Stdio::from(writer_fd)
    }

    #[test]
    fn reports_closed_stdout() {
        let child = skill()
            .args([
                "git-review",
                "message",
                "add",
                "--summary",
                "message composer",
            ])
            .stdout(closed_stdout())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn message composer");
        let output = child.wait_with_output().expect("wait for message composer");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: Broken pipe (os error 32)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_closed_stdout_for_message_checker() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _invalid = repo.commit("feat: add conventional subject");
        let child = command(&repo, "check-messages")
            .args([&base, "HEAD"])
            .stdout(closed_stdout())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn message checker");
        let output = child.wait_with_output().expect("wait for message checker");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: Broken pipe (os error 32)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_closed_stdout_for_ordered_dates() {
        let repo = TestRepo::new();
        let base = repo.commit_at("Add base commit", 100, 100);
        let _ordered = repo.commit_at("Add date checker", 101, 101);
        let child = command(&repo, "check-dates")
            .args([&base, "HEAD"])
            .stdout(closed_stdout())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn date checker");
        let output = child.wait_with_output().expect("wait for date checker");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: Broken pipe (os error 32)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_closed_stdout_for_date_violation() {
        let repo = TestRepo::new();
        let base = repo.commit_at("Add base commit", 100, 100);
        let _invalid = repo.commit_at("Add date checker", 100, 100);
        let child = command(&repo, "check-dates")
            .args([&base, "HEAD"])
            .stdout(closed_stdout())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn date checker");
        let output = child.wait_with_output().expect("wait for date checker");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: Broken pipe (os error 32)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_closed_stdout_for_tree_hashes() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _commit = repo.commit("Add tree hash reporter");
        let child = command(&repo, "tree-hashes")
            .args([&base, "HEAD"])
            .stdout(closed_stdout())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn tree hash reporter");
        let output = child
            .wait_with_output()
            .expect("wait for tree hash reporter");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: Broken pipe (os error 32)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_closed_stdout_for_target_resolver() {
        let repo = TestRepo::new();
        let base = repo.commit("Add base commit");
        let _commit = repo.commit("Add target resolver");
        let child = command(&repo, "resolve-targets")
            .args([&base, "HEAD"])
            .stdout(closed_stdout())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn target resolver");
        let output = child.wait_with_output().expect("wait for target resolver");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: Broken pipe (os error 32)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_closed_stdout_for_base_resolver() {
        let repo = TestRepo::new();
        let child = command(&repo, "resolve-base")
            .arg("stable")
            .stdout(closed_stdout())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn base resolver");
        let output = child.wait_with_output().expect("wait for base resolver");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: Broken pipe (os error 32)\n".to_vec(),
            }
        );
    }

    #[test]
    fn reports_closed_stdout_for_help() {
        let child = skill()
            .arg("git-review")
            .stdout(closed_stdout())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn git-review help");
        let output = child.wait_with_output().expect("wait for git-review help");

        assert_eq!(
            observed(output),
            Observed {
                code: Observed::IO_ERROR,
                stdout: Vec::new(),
                stderr: b"Error: IO error: Broken pipe (os error 32)\n".to_vec(),
            }
        );
    }
}
