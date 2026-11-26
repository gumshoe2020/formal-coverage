//! Git for tests, isolated from any repository the caller's environment points at.
//!
//! A git hook exports `GIT_DIR` (and possibly `GIT_INDEX_FILE`, `GIT_WORK_TREE`, ...),
//! and those override `git -C <dir>`. Every test git call goes through [`git`], which
//! removes git's own list of repository-locating variables (`git rev-parse
//! --local-env-vars`), ignores global and system config and commits as a fixed identity.

use std::{path::Path, process::Command, sync::OnceLock};

const ISOLATION: [(&str, &str); 6] = [
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_CONFIG_NOSYSTEM", "1"),
    ("GIT_AUTHOR_NAME", "Test"),
    ("GIT_AUTHOR_EMAIL", "test@example.invalid"),
    ("GIT_COMMITTER_NAME", "Test"),
    ("GIT_COMMITTER_EMAIL", "test@example.invalid"),
];

/// Git's own list of variables that locate or redirect a repository.
pub fn local_env_vars() -> &'static [String] {
    static VARS: OnceLock<Vec<String>> = OnceLock::new();
    VARS.get_or_init(|| {
        let out = Command::new("git")
            .args(["rev-parse", "--local-env-vars"])
            .output()
            .expect("run git rev-parse --local-env-vars");
        assert!(
            out.status.success(),
            "git rev-parse --local-env-vars failed"
        );
        let vars: Vec<String> = String::from_utf8(out.stdout)
            .expect("variable names are UTF-8")
            .lines()
            .map(str::to_owned)
            .collect();
        assert!(
            vars.iter().any(|v| v == "GIT_DIR"),
            "unexpected list: {vars:?}"
        );
        vars
    })
}

/// `git -C dir` with the repository-locating variables removed and config isolated.
pub fn git(dir: &Path) -> Command {
    let mut command = Command::new("git");
    for var in local_env_vars() {
        command.env_remove(var);
    }
    command.envs(ISOLATION).arg("-C").arg(dir);
    command
}

/// Runs `git -C dir args` through [`git`] and panics with git's stderr on failure.
pub fn run_git(dir: &Path, args: &[&str]) {
    let out = git(dir).args(args).output().expect("spawn git");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Commits everything under `data` as branch `coverage-data` in a new repository there,
/// then fetches it into a new repository at `root` as `refs/remotes/origin/coverage-data`:
/// the fetched data branch that F* receipt reuse reads.
pub fn seed_data_branch(data: &Path, root: &Path) {
    run_git(data, &["init", "-q", "-b", "coverage-data"]);
    run_git(data, &["add", "."]);
    run_git(data, &["commit", "-qm", "main receipt fixture"]);
    run_git(root, &["init", "-q"]);
    let source = data.canonicalize().expect("data directory");
    let source = source.to_str().expect("UTF-8 fixture path");
    run_git(
        root,
        &[
            "fetch",
            "-q",
            source,
            "coverage-data:refs/remotes/origin/coverage-data",
        ],
    );
}

/// Applies the same scrubbing to this process, for library code under test that spawns
/// git itself (it inherits the process environment). Only call it from a test binary
/// whose other tests do not depend on these variables.
#[allow(dead_code)]
pub fn scrub_process_env() {
    for var in local_env_vars() {
        std::env::remove_var(var);
    }
    for (key, value) in ISOLATION {
        std::env::set_var(key, value);
    }
}
