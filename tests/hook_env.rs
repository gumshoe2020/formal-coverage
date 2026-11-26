//! Test git fixtures stay out of the repository a git hook points at.
//!
//! A hook exports GIT_DIR (and may export GIT_WORK_TREE and GIT_INDEX_FILE); they override
//! `git -C`. In v1.12.1 a pre-push `cargo test` committed the F* reuse fixture onto the real
//! branch. This binary holds a single test because it sets those variables process-wide.
mod common;

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Every file under `dir` (worktree and .git: HEAD, index, refs, objects, config, logs).
fn snapshot(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = BTreeMap::new();
    let mut pending = vec![dir.to_path_buf()];
    while let Some(next) = pending.pop() {
        for entry in fs::read_dir(&next).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                let bytes = fs::read(&path).unwrap();
                files.insert(path.strip_prefix(dir).unwrap().to_path_buf(), bytes);
            }
        }
    }
    files
}

fn changed_paths(
    before: &BTreeMap<PathBuf, Vec<u8>>,
    after: &BTreeMap<PathBuf, Vec<u8>>,
) -> Vec<PathBuf> {
    let mut keys: Vec<&PathBuf> = before.keys().chain(after.keys()).collect();
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .filter(|k| before.get(*k) != after.get(*k))
        .cloned()
        .collect()
}

fn decoy(dir: &Path) -> PathBuf {
    let decoy = dir.join("decoy");
    fs::create_dir_all(&decoy).unwrap();
    fs::write(decoy.join("keep.txt"), "tracked\n").unwrap();
    common::run_git(&decoy, &["init", "-q", "-b", "main"]);
    common::run_git(&decoy, &["add", "."]);
    common::run_git(&decoy, &["commit", "-qm", "decoy"]);
    decoy
}

fn data_dir(dir: &Path, name: &str) -> (PathBuf, PathBuf) {
    let data = dir.join(name).join("data-record");
    fs::create_dir_all(data.join("receipts")).unwrap();
    fs::write(data.join("receipts/Hax_fixture.json"), "{}\n").unwrap();
    let root = dir.join(name).join("root");
    fs::create_dir_all(&root).unwrap();
    (data, root)
}

/// The v1.12.1 fixture code: plain `git -C`, inheriting the process environment (only the
/// user's global config is ignored, so signing or hooks there cannot stop the control).
fn unscrubbed_fixture(data: &Path) {
    for args in [
        &["init", "-q", "-b", "coverage-data"][..],
        &["add", "."],
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@local",
            "commit",
            "-qm",
            "fixture",
        ],
    ] {
        let _ = Command::new("git")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .arg("-C")
            .arg(data)
            .args(args)
            .output()
            .unwrap();
    }
}

fn set_hook_env(decoy: &Path, vars: &[&str]) {
    for var in ["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE"] {
        std::env::remove_var(var);
    }
    for var in vars {
        let value = match *var {
            "GIT_DIR" => decoy.join(".git"),
            "GIT_WORK_TREE" => decoy.to_path_buf(),
            "GIT_INDEX_FILE" => decoy.join(".git/index"),
            other => panic!("unexpected {other}"),
        };
        std::env::set_var(var, value);
    }
}

#[test]
fn fixtures_leave_the_hook_repository_byte_unchanged() {
    let base = std::env::temp_dir().join(format!("fc-hook-env-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    let scenarios: [&[&str]; 2] = [
        &["GIT_DIR"],
        &["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE"],
    ];
    for (n, vars) in scenarios.into_iter().enumerate() {
        let dir = base.join(n.to_string());
        let decoy = decoy(&dir);
        let before = snapshot(&decoy);
        set_hook_env(&decoy, vars);

        let (data, root) = data_dir(&dir, "scrubbed");
        // Check the decoy before reporting any fixture failure: a redirected fixture
        // usually fails too, but the damage to the decoy is the point.
        let seeded = std::panic::catch_unwind(|| common::seed_data_branch(&data, &root));
        let after = snapshot(&decoy);
        assert!(
            before == after,
            "{vars:?}: helper fixture changed the decoy: {:?}",
            changed_paths(&before, &after)
        );
        if let Err(panic) = seeded {
            std::panic::resume_unwind(panic);
        }
        assert!(root
            .join(".git/refs/remotes/origin/coverage-data")
            .is_file());

        // Control: the same environment really does redirect unscrubbed git.
        let (data, _) = data_dir(&dir, "unscrubbed");
        unscrubbed_fixture(&data);
        let changed = changed_paths(&before, &snapshot(&decoy));
        assert!(
            !changed.is_empty(),
            "{vars:?}: control did not reach the decoy; the test proves nothing"
        );
        if vars == ["GIT_DIR"] {
            // As in the incident: the fixture commit lands on the decoy's branch.
            assert!(
                changed.contains(&PathBuf::from(".git/refs/heads/main")),
                "{changed:?}"
            );
        }
        set_hook_env(&decoy, &[]);
    }
    fs::remove_dir_all(&base).unwrap();
}
