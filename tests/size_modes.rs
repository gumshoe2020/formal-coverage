//! v1.5 acceptance A-C: compiled-files, linked-lines and fail-closed fallbacks.
use formal_coverage::{analyze_with, Options, SizeMode};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const DEP_LIB: &str = r#"// Fixture dependency: one cfg-disabled module and one Verus-style spec function.
#[cfg(windows)]
mod win;
mod used;
pub use used::called;

macro_rules! verus {
    ($($t:tt)*) => {};
}

verus! {
    pub open spec fn spec_only(x: u64) -> bool {
        x > 0
    }
}

pub fn uncalled(x: u64) -> u64 {
    let a = x.wrapping_add(11);
    let b = a.rotate_left(5);
    let c = b ^ 0x5555;
    c.wrapping_mul(13)
}
"#;
const DEP_USED: &str = "// Called by the fixture binary.\n#[inline(never)]\npub fn called(x: u64) -> u64 {\n    x.wrapping_mul(7).rotate_left(3)\n}\n";

fn write(path: PathBuf, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

/// A binary package with two path dependencies, `dep` (the A/B crate) and `other`.
fn fixture(name: &str) -> PathBuf {
    let root = PathBuf::from(format!("logs/size-modes-{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    write(root.join("Cargo.toml"), "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\nexclude = [\"vendor/dep\", \"vendor/other\"]\n[dependencies]\ndep = { path = \"vendor/dep\" }\nother = { path = \"vendor/other\" }\n[profile.release]\ndebug = true\n[profile.nodebug]\ninherits = \"release\"\ndebug = false\nstrip = \"debuginfo\"\n");
    write(root.join("src/main.rs"), "fn main() {\n    let x = std::hint::black_box(3);\n    println!(\"{} {}\", dep::called(x), other::other(x));\n}\n");
    let dep = root.join("vendor/dep");
    write(
        dep.join("Cargo.toml"),
        "[package]\nname = \"dep\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n",
    );
    write(dep.join("src/lib.rs"), DEP_LIB);
    write(dep.join("src/used.rs"), DEP_USED);
    write(
        dep.join("src/win.rs"),
        "pub fn windows_only() -> u32 {\n    1\n}\n",
    );
    write(
        dep.join("tests/t.rs"),
        "#[test]\nfn t() {\n    assert_eq!(dep::called(1), dep::called(1));\n}\n",
    );
    write(
        dep.join("benches/b.rs"),
        "fn main() {\n    let _ = dep::called(2);\n}\n",
    );
    write(
        dep.join("examples/e.rs"),
        "fn main() {\n    println!(\"{}\", dep::called(3));\n}\n",
    );
    let other = root.join("vendor/other");
    write(
        other.join("Cargo.toml"),
        "[package]\nname = \"other\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n",
    );
    write(
        other.join("src/lib.rs"),
        "pub fn other(x: u64) -> u64 {\n    x + 1\n}\n",
    );
    let status = Command::new("cargo")
        .args(["generate-lockfile", "--offline"])
        .current_dir(&root)
        .status()
        .unwrap();
    assert!(status.success());
    root
}

fn options(mode: SizeMode) -> Options {
    Options {
        size_mode: mode,
        ..Options::default()
    }
}

fn dependency<'a>(r: &'a formal_coverage::Report, name: &str) -> &'a formal_coverage::Dependency {
    r.dependencies.iter().find(|d| d.name == name).unwrap()
}

// Whole-crate: lib.rs 18 (spec function included), used.rs 4, win.rs 3, tests 4, benches 3,
// examples 3. Compiled-files: lib.rs 15 (spec function's 3 lines removed) + used.rs 4.
const WHOLE: usize = 35;
const COMPILED: usize = 19;

#[test]
fn compiled_files_counts_only_compiled_non_test_files_without_spec_code() {
    let root = fixture("a");
    let whole = analyze_with(&root, &options(SizeMode::WholeCrate)).unwrap();
    assert_eq!(whole.size_mode, "whole-crate");
    assert_eq!(dependency(&whole, "dep").sloc, WHOLE);
    assert_eq!(dependency(&whole, "dep").size_mode, "whole-crate");
    let compiled = analyze_with(&root, &options(SizeMode::CompiledFiles)).unwrap();
    assert_eq!(compiled.size_mode, "compiled-files");
    assert_eq!(compiled.size_profile, "dev (cargo check)");
    assert!(
        compiled.size_fallbacks.is_empty(),
        "{:?}",
        compiled.size_fallbacks
    );
    let dep = dependency(&compiled, "dep");
    assert_eq!((dep.sloc, dep.whole_crate_sloc), (COMPILED, WHOLE));
    assert_eq!(dep.size_mode, "compiled-files");
    assert_eq!(dependency(&compiled, "other").sloc, 3);
    let md = formal_coverage::markdown(&compiled);
    assert!(md.starts_with("Size mode: `compiled-files`; target: `"));
    assert!(
        md.contains("## Dependency sizes")
            && md.contains("| dep@0.1.0 | compiled-files | 19 | 35 |")
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn missing_dep_info_falls_back_to_whole_crate_for_that_dependency_only() {
    let root = fixture("c");
    let first = analyze_with(&root, &options(SizeMode::CompiledFiles)).unwrap();
    assert!(first.size_fallbacks.is_empty());
    let deps = fs::read_dir(root.join("target/formal-coverage"))
        .unwrap()
        .map(|e| e.unwrap().path().join("debug/deps"))
        .find(|p| p.is_dir())
        .unwrap();
    let removed: Vec<_> = fs::read_dir(&deps)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "d")
                && p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("other-")
        })
        .collect();
    assert_eq!(removed.len(), 1);
    fs::remove_file(&removed[0]).unwrap();
    let second = analyze_with(&root, &options(SizeMode::CompiledFiles)).unwrap();
    let other = dependency(&second, "other");
    assert_eq!(other.size_mode, "whole-crate (fallback)");
    assert!(other
        .size_reason
        .as_deref()
        .unwrap()
        .starts_with("dep-info file missing"));
    assert_eq!(other.sloc, other.whole_crate_sloc);
    assert_eq!(dependency(&second, "dep").size_mode, "compiled-files");
    assert_eq!(dependency(&second, "dep").sloc, COMPILED);
    assert_eq!(second.size_fallbacks.len(), 1);
    assert_eq!(second.size_fallbacks[0].name, "other");
    let md = formal_coverage::markdown(&second);
    let banner = md.lines().nth(2).unwrap();
    assert!(
        banner.starts_with("**SIZE FALLBACKS: 1 runtime dependencies")
            && banner.contains("other@0.1.0 (dep-info file missing")
    );
    fs::remove_dir_all(root).unwrap();
}

fn build(root: &Path, profile: &str) -> PathBuf {
    let log = fs::File::create(root.join(format!("build-{profile}.log"))).unwrap();
    let status = Command::new("cargo")
        .args(["build", "--frozen", "--profile", profile])
        .current_dir(root)
        .stdout(log.try_clone().unwrap())
        .stderr(log)
        .status()
        .unwrap();
    assert!(status.success());
    root.join("target").join(profile).join("app")
}

fn span(text: &str, needle: &str) -> std::ops::RangeInclusive<u64> {
    let lines: Vec<_> = text.lines().collect();
    let start = lines.iter().position(|l| l.contains(needle)).unwrap();
    let end = start + lines[start..].iter().position(|l| *l == "}").unwrap();
    (start as u64 + 1)..=(end as u64 + 1)
}

#[test]
fn linked_lines_count_called_code_only_and_fail_closed_without_debug_info() {
    let root = fixture("b");
    let canonical = root.canonicalize().unwrap();
    let binary = build(&root, "release");
    let table = formal_coverage::dwarf_lines(&binary).unwrap();
    let lines = |file: &str| {
        table
            .lines
            .iter()
            .filter(|(path, _)| Path::new(path) == canonical.join(file))
            .flat_map(|(_, lines)| lines.iter().copied())
            .collect::<Vec<_>>()
    };
    let called = span(DEP_USED, "pub fn called");
    let uncalled = span(DEP_LIB, "pub fn uncalled");
    assert!(
        lines("vendor/dep/src/used.rs")
            .iter()
            .any(|l| called.contains(l)),
        "{:?}",
        table.debug_sources
    );
    assert!(!lines("vendor/dep/src/lib.rs")
        .iter()
        .any(|l| uncalled.contains(l)));
    let report = analyze_with(
        &root,
        &Options {
            size_mode: SizeMode::LinkedLines,
            binary: Some(binary.clone()),
            ..Options::default()
        },
    )
    .unwrap();
    assert!(
        report.size_fallbacks.is_empty(),
        "{:?}",
        report.size_fallbacks
    );
    assert_eq!(report.binary_sha256.as_deref(), Some(table.sha256.as_str()));
    let dep = dependency(&report, "dep");
    assert_eq!(dep.size_mode, "linked-lines");
    assert!(
        dep.sloc > 0 && dep.sloc <= called.clone().count(),
        "{}",
        dep.sloc
    );
    assert!(dep.sloc < COMPILED);
    let md = formal_coverage::markdown(&report);
    assert!(md
        .lines()
        .next()
        .unwrap()
        .contains("linked lines (distinct DWARF file:line pairs), not SLoC"));
    assert!(md.contains("Third-party (runtime)  ") && md.contains("measured linked lines"));

    // No debug info: every dependency falls back, sized exactly as whole-crate.
    let stripped = build(&root, "nodebug");
    let report = analyze_with(
        &root,
        &Options {
            size_mode: SizeMode::LinkedLines,
            binary: Some(stripped),
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(report.size_fallbacks.len(), 2);
    for d in &report.dependencies {
        assert_eq!(d.size_mode, "whole-crate (fallback)");
        assert_eq!(d.sloc, d.whole_crate_sloc);
    }
    assert_eq!(dependency(&report, "dep").sloc, WHOLE);
    assert!(report.size_fallbacks[0]
        .reason
        .contains("no DWARF line rows for first-party sources"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn size_mode_parses_and_rejects_unknown_names() {
    for mode in ["whole-crate", "compiled-files", "linked-lines"] {
        assert_eq!(mode.parse::<SizeMode>().unwrap().as_str(), mode);
    }
    assert!("linked".parse::<SizeMode>().is_err());
    let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args([
            "report",
            "fixtures/inventory",
            "--size-mode",
            "linked-lines",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--binary"));
}
