//! M1: real Cargo dep-info drives scope; synthetic receipts test coverage accounting only.
use formal_coverage::{
    analyze, analyze_with, apply_evidence, markdown, proof_ratio, sha256, summarize, Options,
    Receipt,
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);

fn write(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}
fn fixture() -> PathBuf {
    let root = PathBuf::from(format!(
        "logs/ship-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    write(&root, "Cargo.toml", "[workspace]\nmembers=['app','helper','demo','optional','buildonly','devonly']\nexclude=['vendor']\nresolver='2'\n");
    for name in ["app", "helper", "demo", "optional", "buildonly", "devonly"] {
        write(
            &root,
            &format!("{name}/Cargo.toml"),
            &format!("[package]\nname='{name}'\nversion='0.1.0'\nedition='2021'\n"),
        );
    }
    write(
        &root,
        "vendor/Cargo.toml",
        "[package]\nname='vendor'\nversion='0.1.0'\nedition='2021'\n[workspace]\n",
    );
    write(&root, "vendor/src/lib.rs", "pub fn external() {}\n");
    let app_manifest = root.join("app/Cargo.toml");
    let mut manifest = fs::read_to_string(&app_manifest).unwrap();
    manifest.push_str("[dependencies]\nvendor={path='../vendor'}\nhelper={path='../helper'}\noptional={path='../optional',optional=true}\n[features]\ndefault=['d']\nd=[]\nx=['dep:optional']\n[build-dependencies]\nbuildonly={path='../buildonly'}\n[dev-dependencies]\ndevonly={path='../devonly'}\n");
    fs::write(app_manifest, manifest).unwrap();
    write(&root, "app/src/lib.rs", "#[cfg(feature=\"x\")]\nmod gated;\n#[cfg(feature=\"d\")]\nmod defaults;\npub fn run() -> u64 { helper::value() }\n");
    write(
        &root,
        "app/src/defaults.rs",
        "pub fn default_only() -> u64 { 3 }\n",
    );
    write(&root, "app/src/gated.rs", "macro_rules! verus { ($($t:tt)*) => {}; }\nverus! {\nfn secret() -> (r: u64)\n    ensures r == 7,\n{\n    7\n}\nproof fn lemma() {}\n}\npub fn extra() -> u64 { optional::value() }\n");
    write(
        &root,
        "app/src/bin/local.rs",
        "fn main() { let _ = app::run(); }\n",
    );
    write(
        &root,
        "app/build.rs",
        "fn main() { let _ = buildonly::value(); }\n",
    );
    for name in ["helper", "optional", "buildonly", "devonly"] {
        write(
            &root,
            &format!("{name}/src/lib.rs"),
            "pub fn value() -> u64 { 1 }\n",
        );
    }
    write(&root, "demo/src/main.rs", "fn main() {}\n");
    assert!(Command::new("cargo")
        .args(["generate-lockfile", "--offline"])
        .current_dir(&root)
        .status()
        .unwrap()
        .success());
    root
}
fn config(root: &Path, packages: &str, features: &str) {
    write(root, "formal-coverage.toml", &format!("[[security.exclude]]\npattern='*::extra'\nreason='non-security fixture'\n[ship]\npackages=[{packages}]\nfeatures=[{features}]\nreason='fixture shipped artifact'\n"));
}
fn files(r: &formal_coverage::Report) -> Vec<&str> {
    r.not_shipped.iter().map(|f| f.file.as_str()).collect()
}

#[test]
fn shipped_files_and_normal_closure_only_with_feature_union_and_binary_roots() {
    let root = fixture();
    let before = analyze(&root, None).unwrap();
    config(&root, "'app'", "'default'");
    let r = analyze(&root, None).unwrap();
    assert_eq!(
        serde_json::to_value(&r.dependencies).unwrap(),
        serde_json::to_value(&before.dependencies).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&r.build_dependencies).unwrap(),
        serde_json::to_value(&before.build_dependencies).unwrap()
    );

    assert!(
        !r.warnings.iter().any(|w| w.starts_with("ship:")),
        "{:?}",
        r.warnings
    );
    assert_eq!(
        files(&r),
        [
            "app/build.rs",
            "app/src/gated.rs",
            "buildonly/src/lib.rs",
            "demo/src/main.rs",
            "devonly/src/lib.rs",
            "optional/src/lib.rs"
        ]
    );
    assert!(r.units.iter().any(|u| u.file == "app/src/lib.rs"));
    assert!(r.units.iter().any(|u| u.file == "helper/src/lib.rs"));
    assert!(r.units.iter().any(|u| u.file == "app/src/bin/local.rs"));
    assert!(r.units.iter().any(|u| u.file == "app/src/defaults.rs"));
    assert_eq!(summarize(&r).all.everything_else.sloc, 0);
    assert_eq!(summarize(&r).first_party.security_relevant.sloc, 4);
    assert_eq!(summarize(&r).all.security_relevant.sloc, 5);
    assert_eq!(r.inputs[0].sloc_out_of_security, 0);
    assert_eq!(summarize(&r).proof_code.total.sloc, 0);
    let md = markdown(&r);
    assert!(md.contains("## Not shipped") && md.contains("Not shipped: "));
    for file in &r.not_shipped {
        assert!(md.contains(&format!("| {} | {} |", file.file, file.sloc)));
        assert!(!r.source_files.contains(&file.file));
    }
    let json = serde_json::to_value(&r).unwrap();
    assert_eq!(json["not_shipped"].as_array().unwrap().len(), 6);
    assert_eq!(json["ship"]["reason"], "fixture shipped artifact");
    config(&root, "'app'", "'default','x'");
    let enabled = analyze(&root, None).unwrap();
    assert!(!files(&enabled).contains(&"app/src/gated.rs"));
    assert!(!files(&enabled).contains(&"optional/src/lib.rs"));
    assert!(summarize(&enabled).all.everything_else.sloc > 0);
    assert!(enabled.inputs[0].sloc_out_of_security > 0);
    assert!(summarize(&enabled).proof_code.total.sloc > 0);
    assert!(
        summarize(&enabled).all.security_relevant.sloc > summarize(&r).all.security_relevant.sloc
    );
    // A binary-only package is a valid root; lib and bin artifacts are unioned across roots.
    config(&root, "'app','demo'", "'default'");
    assert!(!files(&analyze(&root, None).unwrap()).contains(&"demo/src/main.rs"));
}

#[test]
fn missing_dep_info_retains_only_affected_package_and_failed_check_retains_inventory() {
    let root = fixture();
    config(&root, "'app'", "'default'");
    let r = analyze(&root, None).unwrap();
    assert!(files(&r).contains(&"app/src/gated.rs"));
    let pattern = format!(
        "{}/target/formal-coverage-ship/**/libapp-*.rmeta",
        root.display()
    );
    let paths: Vec<_> = glob::glob(&pattern).unwrap().map(Result::unwrap).collect();
    assert_eq!(paths.len(), 1);
    let stem = paths[0]
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("lib")
        .unwrap();
    fs::remove_file(paths[0].with_file_name(format!("{stem}.d"))).unwrap();
    let fallback = analyze(&root, None).unwrap();
    assert!(fallback.units.iter().any(|u| u.file == "app/src/gated.rs"));
    assert!(
        fallback
            .warnings
            .iter()
            .any(|w| w.contains("package app") && w.contains("dep-info")),
        "{:?}",
        fallback.warnings
    );
    assert!(files(&fallback).contains(&"demo/src/main.rs"));
    assert!(!files(&fallback).contains(&"helper/src/lib.rs"));
    write(
        &root,
        "app/src/lib.rs",
        "compile_error!(\"intentional ship check failure\");\npub fn run() {}\n",
    );
    let failed = analyze(&root, None).unwrap();
    assert!(failed.not_shipped.is_empty());
    assert!(failed.units.iter().any(|u| u.file == "app/src/gated.rs"));
    assert!(failed
        .warnings
        .iter()
        .any(|w| w.contains("package app") && w.contains("cargo check failed")));
}

#[test]
fn ship_validation_and_absence_are_backward_compatible() {
    let root = fixture();
    let before = analyze(&root, None).unwrap();
    let before_json = serde_json::to_string(&before).unwrap();
    assert!(!before_json.contains("not_shipped") && !before_json.contains("\"ship\""));
    assert_eq!(
        before_json,
        serde_json::to_string(&analyze_with(&root, &Options::default()).unwrap()).unwrap()
    );
    assert_eq!(markdown(&before), markdown(&analyze(&root, None).unwrap()));
    for text in [
        "[ship]\npackages=['app']\n",
        "[ship]\npackages=['app']\nreason='   '\n",
    ] {
        write(&root, "formal-coverage.toml", text);
        assert!(format!("{:#}", analyze(&root, None).unwrap_err()).contains("reason"));
    }
    config(&root, "'unknown'", "'default'");
    assert!(analyze(&root, None)
        .unwrap_err()
        .to_string()
        .contains("unknown workspace package"));
    config(&root, "'app'", "");
    let r = analyze(&root, None).unwrap();
    assert!(files(&r).contains(&"app/src/defaults.rs"));
}

#[test]
fn excluded_proven_runtime_and_proof_code_leave_every_total() {
    let root = fixture();
    let log = "verification results:: 2 verified, 0 errors\n";
    write(&root, "verus.log", log);
    let inventory = formal_coverage::inventory(&root).unwrap();
    let receipt = Receipt {
        tool: "verus".into(),
        tool_version: "synthetic scope policy fixture".into(),
        command: vec![
            "verus".into(),
            "--crate-type".into(),
            "lib".into(),
            "app/src/gated.rs".into(),
        ],
        exit_code: 0,
        cfg: None,
        files: inventory
            .source_files
            .iter()
            .map(|file| (file.clone(), sha256(&fs::read(root.join(file)).unwrap())))
            .collect(),
        log_sha256: sha256(log.as_bytes()),
        covered_units: BTreeMap::new(),
        harnesses: vec![],
    };
    write(
        &root,
        "verus.log.evidence.json",
        &serde_json::to_string(&receipt).unwrap(),
    );
    let mut before = analyze(&root, None).unwrap();
    apply_evidence(&mut before, &root, Some(&root.join("verus.log")), None).unwrap();
    assert!(before
        .units
        .iter()
        .any(|u| u.file == "app/src/gated.rs" && u.status == "proven"));
    config(&root, "'app'", "'default'");
    let mut after = analyze(&root, None).unwrap();
    apply_evidence(&mut after, &root, Some(&root.join("verus.log")), None).unwrap();
    assert!(after
        .not_shipped
        .iter()
        .find(|f| f.file == "app/src/gated.rs")
        .unwrap()
        .units
        .iter()
        .any(|u| u.status == "proven"));
    assert!(markdown(&after).contains(" / proven"));
    let summary = summarize(&after);
    assert_eq!(summary.all.security_relevant.statuses["proven"], 0);
    assert_eq!(summary.first_party.security_relevant.statuses["proven"], 0);
    assert_eq!(summary.proof_code.total.sloc, 0);
    assert_eq!(proof_ratio(&after).security_relevant.proven_runtime_sloc, 0);
    assert_eq!(proof_ratio(&after).security_relevant.proof_sloc, 0);
    assert!(summary.all.security_relevant.sloc < summarize(&before).all.security_relevant.sloc);
    assert!(after.units.iter().all(|u| u.file != "app/src/gated.rs"));
}

// Keep this separate from fixture(): existing scope/count assertions must stay unchanged.
fn evidence_fixture() -> PathBuf {
    let root = fixture();
    let demo_manifest = root.join("demo/Cargo.toml");
    let mut manifest = fs::read_to_string(&demo_manifest).unwrap();
    manifest.push_str("[dev-dependencies]\napp={path='../app'}\n");
    fs::write(demo_manifest, manifest).unwrap();
    assert!(Command::new("cargo")
        .args(["generate-lockfile", "--offline"])
        .current_dir(&root)
        .status()
        .unwrap()
        .success());
    let lib = root.join("app/src/lib.rs");
    let mut source = fs::read_to_string(&lib).unwrap();
    source.push_str("#[cfg(test)]\nmod internal_tests;\npub fn from_member_test() -> u64 { 7 }\npub fn from_cfg_test() -> u64 { 9 }\n");
    fs::write(lib, source).unwrap();
    write(
        &root,
        "demo/tests/api.rs",
        "#[test]\nfn member_test() { assert_eq!(app::from_member_test(), 7); }\n",
    );
    write(
        &root,
        "app/src/internal_tests.rs",
        "#[test]\nfn internal_test() { assert_eq!(crate::from_cfg_test(), 9); }\n",
    );
    root
}

#[test]
fn not_shipped_member_and_cfg_tests_still_confer_tested() {
    let root = evidence_fixture();
    let mut before = analyze(&root, None).unwrap();
    apply_evidence(&mut before, &root, None, None).unwrap();
    config(&root, "'app'", "'default'");
    let mut after = analyze(&root, None).unwrap();
    apply_evidence(&mut after, &root, None, None).unwrap();
    for file in ["demo/tests/api.rs", "app/src/internal_tests.rs"] {
        assert!(files(&after).contains(&file), "{file}");
        assert!(!after.source_files.iter().any(|f| f == file));
    }
    for name in ["from_member_test", "from_cfg_test"] {
        let old = before.units.iter().find(|u| u.name == name).unwrap();
        let new = after.units.iter().find(|u| u.name == name).unwrap();
        assert_eq!(old.status, "tested", "{name}: baseline");
        assert_eq!(new.status, "tested", "{name}: shipped");
        assert_eq!(
            serde_json::to_value(old).unwrap(),
            serde_json::to_value(new).unwrap()
        );
    }
}

#[test]
fn ship_preserves_every_retained_unit_with_full_test_evidence() {
    use formal_coverage::GhostLines;
    let root = evidence_fixture();
    for ghost_lines in [GhostLines::Separate, GhostLines::Runtime] {
        let options = Options {
            ghost_lines,
            ..Options::default()
        };
        let mut before = analyze_with(&root, &options).unwrap();
        apply_evidence(&mut before, &root, None, None).unwrap();
        config(&root, "'app'", "'default'");
        let mut after = analyze_with(&root, &options).unwrap();
        apply_evidence(&mut after, &root, None, None).unwrap();
        assert!(
            !after.warnings.iter().any(|w| w.starts_with("ship:")),
            "{:?}",
            after.warnings
        );
        assert!(!after.units.is_empty());
        let retained: Vec<_> = before
            .units
            .iter()
            .filter(|u| !files(&after).contains(&u.file.as_str()))
            .collect();
        // Full identity includes status/tier, category, SLoC and proof metadata.
        assert_eq!(
            serde_json::to_value(retained).unwrap(),
            serde_json::to_value(&after.units).unwrap()
        );
        fs::remove_file(root.join("formal-coverage.toml")).unwrap();
    }
}

#[test]
fn not_shipped_kani_harness_still_confers_bounded_proof() {
    let root = fixture();
    let lib = root.join("app/src/lib.rs");
    let mut source = fs::read_to_string(&lib).unwrap();
    source.push_str("#[cfg(kani)]\nmod proofs;\npub fn proof_target() -> u64 { 7 }\n");
    fs::write(lib, source).unwrap();
    write(
        &root,
        "app/src/proofs.rs",
        "#[kani::proof]\nfn harness() { assert_eq!(crate::proof_target(), 7); }\n",
    );
    let inventory = formal_coverage::inventory(&root).unwrap();
    let harness = inventory.units.iter().find(|u| u.kani_harness).unwrap();
    let target = inventory
        .units
        .iter()
        .find(|u| u.name == "proof_target")
        .unwrap();
    let log = format!(
        "Checking harness {}...\nVERIFICATION:- SUCCESSFUL\n",
        harness.id
    );
    write(&root, "kani.log", &log);
    let receipt = Receipt {
        tool: "kani".into(),
        tool_version: "synthetic scope policy fixture".into(),
        command: vec![
            "cargo".into(),
            "kani".into(),
            "--harness".into(),
            harness.id.clone(),
        ],
        exit_code: 0,
        cfg: None,
        files: inventory
            .source_files
            .iter()
            .map(|file| (file.clone(), sha256(&fs::read(root.join(file)).unwrap())))
            .collect(),
        log_sha256: sha256(log.as_bytes()),
        covered_units: BTreeMap::from([(target.id.clone(), harness.id.clone())]),
        harnesses: vec![harness.id.clone()],
    };
    write(
        &root,
        "kani.log.evidence.json",
        &serde_json::to_string(&receipt).unwrap(),
    );
    let mut before = analyze(&root, None).unwrap();
    apply_evidence(&mut before, &root, None, Some(&root.join("kani.log"))).unwrap();
    config(&root, "'app'", "'default'");
    let mut after = analyze(&root, None).unwrap();
    apply_evidence(&mut after, &root, None, Some(&root.join("kani.log"))).unwrap();
    assert!(files(&after).contains(&"app/src/proofs.rs"));
    assert!(after.evidence_runs.iter().all(|r| r.accepted));
    let old = before
        .units
        .iter()
        .find(|u| u.name == "proof_target")
        .unwrap();
    let new = after
        .units
        .iter()
        .find(|u| u.name == "proof_target")
        .unwrap();
    assert_eq!(old.status, "proven-bounded");
    assert_eq!(
        serde_json::to_value(old).unwrap(),
        serde_json::to_value(new).unwrap()
    );
    assert_eq!(summarize(&after).proof_code.total.sloc, 0);
}
