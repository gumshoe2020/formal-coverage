//! Synthetic receipt-policy fixtures, not verifier executions.
use formal_coverage::{analyze, apply_evidence, sha256, Receipt};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);

fn fixture(attrs: &str, extra: &str, flags: &[&str]) -> (PathBuf, PathBuf) {
    let root = PathBuf::from(format!(
        "logs/kani-features-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::copy("fixtures/evidence/Cargo.toml", root.join("Cargo.toml")).unwrap();
    let proof = if attrs.contains("proof_for_contract") {
        ""
    } else {
        "#[kani::proof]"
    };
    fs::write(root.join("src/lib.rs"), format!("fn target() {{}} fn fake() {{}}\n{attrs}\n{proof} fn harness() {{ target(); }}\n{extra}")).unwrap();
    let log = root.join("run.log");
    fs::write(
        &log,
        "Checking harness evidence::harness...\nVERIFICATION:- SUCCESSFUL\n",
    )
    .unwrap();
    let mut command = vec!["cargo".into(), "kani".into()];
    command.extend(flags.iter().map(|s| s.to_string()));
    command.extend(["--harness", "evidence::harness", "--output-format=terse"].map(String::from));
    let receipt = Receipt {
        cfg: None,
        tool: "kani".into(),
        tool_version: "synthetic policy fixture".into(),
        command,
        exit_code: 0,
        files: BTreeMap::from([(
            "src/lib.rs".into(),
            sha256(&fs::read(root.join("src/lib.rs")).unwrap()),
        )]),
        log_sha256: sha256(&fs::read(&log).unwrap()),
        harnesses: vec!["evidence::harness".into()],
        covered_units: BTreeMap::new(),
    };
    fs::write(
        format!("{}.evidence.json", log.display()),
        serde_json::to_vec(&receipt).unwrap(),
    )
    .unwrap();
    (root, log)
}
fn report(root: &std::path::Path, log: &std::path::Path) -> formal_coverage::Report {
    let mut r = analyze(root, None).unwrap();
    apply_evidence(&mut r, root, None, Some(log)).unwrap();
    r
}
#[test]
fn every_allowlisted_feature_retains_source_assumptions_and_only_bounded_credit() {
    for (feature, attrs, assumption) in [
        (
            "unstable-options",
            "",
            "Kani default: no unwind limit, unwinding assertions on",
        ),
        (
            "stubbing",
            "#[kani::stub(target, fake)]",
            "target replaced by stub fake",
        ),
        (
            "function-contracts",
            "#[kani::proof_for_contract(target)] #[kani::stub_verified(target)]",
            "target's contract (requires/ensures)",
        ),
        (
            "restrict-vtable",
            "",
            "dyn calls restricted to type-compatible vtable targets",
        ),
    ] {
        let (root, log) = fixture(
            attrs,
            "",
            &["-Z", feature, "--harness-timeout", "40m", "-p", "evidence"],
        );
        let r = report(&root, &log);
        assert!(
            r.evidence_runs[0].accepted,
            "{:?}",
            r.evidence_runs[0].reasons
        );
        let unit = r.units.iter().find(|u| u.name == "harness").unwrap();
        assert_eq!(unit.status, "proven-bounded");
        assert!(unit.proof_scope.as_ref().unwrap().contains(assumption));
        if feature == "function-contracts" {
            assert!(unit
                .proof_scope
                .as_ref()
                .unwrap()
                .contains("target assumed to meet its contract"));
        }
        assert!(!r.units.iter().any(|u| u.status == "proven"));
    }
}
#[test]
fn disabled_checks_parallelism_unknown_features_and_ambiguous_filters_fail_closed() {
    for flags in [
        vec!["-Z", "other-feature"],
        vec!["--no-unwinding-checks"],
        vec!["--workspace"],
        vec!["-j"],
        vec!["--jobs", "1"],
        vec!["--cbmc-args", "x"],
        vec!["--default-unwind", "8"],
        vec!["--no-default-checks"],
        vec!["--no-memory-safety-checks"],
        vec!["--no-overflow-checks"],
        vec!["--no-assertion-reachability-checks"],
        vec!["--enable-unstable"],
        vec!["--concrete-playback"],
        vec!["--harness", "evidence::second"],
    ] {
        let (root, log) = fixture("", "#[kani::proof] fn second() {}", &flags);
        let r = report(&root, &log);
        assert!(!r.evidence_runs[0].accepted, "{flags:?}");
    }
    let (root, log) = fixture("", "#[kani::proof] fn harness_more() {}", &[]);
    assert!(!report(&root, &log).evidence_runs[0].accepted);
}
#[test]
fn bounds_follow_command_then_hashed_attribute_then_default() {
    for (flags, want) in [
        (vec!["--unwind", "8"], "unwind bound 8"),
        (vec![], "unwind bound 0"),
    ] {
        let (root, log) = fixture("#[kani::unwind(0)]", "", &flags);
        let r = report(&root, &log);
        assert!(r.evidence_runs[0].accepted);
        assert!(r
            .units
            .iter()
            .find(|u| u.name == "harness")
            .unwrap()
            .proof_scope
            .as_ref()
            .unwrap()
            .contains(want));
    }
    let (root, log) = fixture("", "", &["--unwind", "0"]);
    assert!(!report(&root, &log).evidence_runs[0].accepted);
}
#[test]
fn stub_scope_cannot_be_replaced_by_receipt_claims_or_stale_source() {
    let (root, log) = fixture("#[kani::stub(target, fake)]", "", &["-Z", "stubbing"]);
    let sidecar = format!("{}.evidence.json", log.display());
    let bytes = fs::read(&sidecar).unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["proof_scope"] = "harness with no stub assumptions".into();
    fs::write(&sidecar, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(!report(&root, &log).evidence_runs[0].accepted);
    fs::write(&sidecar, bytes).unwrap();
    let source = root.join("src/lib.rs");
    fs::write(
        &source,
        fs::read_to_string(&source)
            .unwrap()
            .replace("stub(target, fake)", "stub(target, other)"),
    )
    .unwrap();
    assert!(!report(&root, &log).evidence_runs[0].accepted);
}
#[test]
fn crate_less_logged_names_resolve_uniquely_but_expected_panics_never_credit() {
    let (root, log) = fixture("", "", &[]);
    let sidecar = format!("{}.evidence.json", log.display());
    let mut r: Receipt = serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    for arg in &mut r.command {
        if arg == "evidence::harness" {
            *arg = "harness".into();
        }
    }
    r.harnesses = vec!["harness".into()];
    fs::write(
        &log,
        "Checking harness harness...\nVERIFICATION:- SUCCESSFUL\n",
    )
    .unwrap();
    r.log_sha256 = sha256(&fs::read(&log).unwrap());
    fs::write(&sidecar, serde_json::to_vec(&r).unwrap()).unwrap();
    assert!(report(&root, &log).evidence_runs[0]
        .credited_units
        .contains(&"evidence::harness".into()));
    let (root, log) = fixture("#[kani::should_panic]", "", &[]);
    assert!(!report(&root, &log).evidence_runs[0].accepted);
}

#[test]
fn local_helpers_do_not_inherit_harness_identity_or_scope() {
    let (root, log) = fixture("#[kani::stub(target, fake)]", "", &["-Z", "stubbing"]);
    let source = root.join("src/lib.rs");
    fs::write(
        &source,
        fs::read_to_string(&source)
            .unwrap()
            .replace("target();", "fn helper() {} target();"),
    )
    .unwrap();
    let sidecar = format!("{}.evidence.json", log.display());
    let mut receipt: Receipt = serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    receipt
        .files
        .insert("src/lib.rs".into(), sha256(&fs::read(source).unwrap()));
    fs::write(sidecar, serde_json::to_vec(&receipt).unwrap()).unwrap();
    let r = report(&root, &log);
    assert!(
        r.evidence_runs[0].accepted,
        "{:?}",
        r.evidence_runs[0].reasons
    );
    assert_eq!(
        r.units
            .iter()
            .find(|u| u.name == "helper")
            .unwrap()
            .category,
        formal_coverage::Category::ProofCode
    );
}

#[test]
fn workspace_with_two_harness_results_is_rejected() {
    let (root, log) = fixture("", "#[kani::proof] fn second() {}", &["--workspace"]);
    fs::write(&log, "Checking harness evidence::harness...\nVERIFICATION:- SUCCESSFUL\nChecking harness evidence::second...\nVERIFICATION:- SUCCESSFUL\n").unwrap();
    let sidecar = format!("{}.evidence.json", log.display());
    let mut receipt: Receipt = serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    receipt.harnesses.push("evidence::second".into());
    receipt.log_sha256 = sha256(&fs::read(&log).unwrap());
    fs::write(sidecar, serde_json::to_vec(&receipt).unwrap()).unwrap();
    assert!(!report(&root, &log).evidence_runs[0].accepted);
}
