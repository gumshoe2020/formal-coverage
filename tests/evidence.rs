use formal_coverage::{analyze, apply_evidence, sha256, Receipt};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
fn fixture(extra: &str) -> PathBuf {
    let path = PathBuf::from(format!(
        "logs/evidence-test-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(path.join("src")).unwrap();
    fs::copy("fixtures/evidence/Cargo.toml", path.join("Cargo.toml")).unwrap();
    fs::write(
        path.join("src/lib.rs"),
        format!(
            "{}\n{extra}",
            fs::read_to_string("fixtures/evidence/src/lib.rs").unwrap()
        ),
    )
    .unwrap();
    path
}
fn receipt(root: &std::path::Path, tool: &str, log: &str) -> PathBuf {
    let path = root.join(format!("{tool}.log"));
    fs::write(&path, log).unwrap();
    let command = if tool == "verus" {
        vec!["verus", "--crate-type", "lib", "src/lib.rs"]
    } else {
        vec![
            "cargo",
            "kani",
            "--harness",
            "evidence::harness",
            "--output-format=regular",
            "--unwind",
            "8",
        ]
    };
    let r = Receipt {
        cfg: None,
        tool: tool.into(),
        tool_version: "test fixture (synthetic, not a verifier execution)".into(),
        command: command.into_iter().map(String::from).collect(),
        exit_code: 0,
        files: BTreeMap::from([(
            "src/lib.rs".into(),
            sha256(&fs::read(root.join("src/lib.rs")).unwrap()),
        )]),
        log_sha256: sha256(log.as_bytes()),
        covered_units: BTreeMap::new(),
        harnesses: if tool == "kani" {
            vec!["evidence::harness".into()]
        } else {
            vec![]
        },
    };
    fs::write(
        format!("{}.evidence.json", path.display()),
        serde_json::to_vec(&r).unwrap(),
    )
    .unwrap();
    path
}
#[test]
fn evidence_is_source_bound_and_scoped() {
    let root = fixture("");
    let verus = receipt(
        &root,
        "verus",
        "verification results:: 1 verified, 0 errors\n",
    );
    let kani = receipt(
        &root,
        "kani",
        "Checking harness evidence::harness...\nVERIFICATION:- SUCCESSFUL\n",
    );
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, Some(&verus), Some(&kani)).unwrap();
    let statuses: BTreeMap<_, _> = r
        .units
        .iter()
        .map(|u| (u.name.as_str(), u.status.as_str()))
        .collect();
    assert_eq!(statuses["contracted"], "proven");
    assert_eq!(statuses["harness"], "proven-bounded");
    assert_eq!(statuses["ordinary"], "tested");
    assert_eq!(statuses["untouched"], "unverified");
    assert!(r
        .units
        .iter()
        .find(|u| u.name == "harness")
        .unwrap()
        .proof_scope
        .as_ref()
        .unwrap()
        .contains("harness"));
    fs::write(
        root.join("src/lib.rs"),
        "verus! { fn stale() requires true, ensures true, {} }",
    )
    .unwrap();
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, Some(&verus), None).unwrap();
    assert!(r.units.iter().all(|u| u.status != "proven"));
}
#[test]
fn missing_failing_partial_and_zero_logs_never_prove() {
    for text in [
        "",
        "verification results:: 0 verified, 0 errors\n",
        "verification results:: 1 verified, 1 errors\n",
        "verification results:: 1 verified, 0 errors (partial verification with `--verify-*`)\n",
        "verification results:: 1 verified, 0 errors\nerror: aborted\n",
    ] {
        let root = fixture("verus! { proof fn proof_case() {} }");
        let log = receipt(&root, "verus", text);
        let mut r = analyze(&root, None).unwrap();
        apply_evidence(&mut r, &root, Some(&log), None).unwrap();
        assert!(
            r.units
                .iter()
                .all(|u| u.status != "proven" && u.status != "verified"),
            "{text}"
        );
    }
    let root = fixture("verus! { proof fn proof_case() {} }");
    let log = root.join("bare.log");
    fs::write(&log, "verification results:: 1 verified, 0 errors\n").unwrap();
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, Some(&log), None).unwrap();
    assert!(r
        .units
        .iter()
        .all(|u| u.status != "proven" && u.status != "verified"));
}
#[test]
fn escapes_cap_their_units_and_trust_is_explicit() {
    for (text, kind) in [
        ("fn escape() { assume(true); }", "assume"),
        ("fn escape() { admit(); }", "admit"),
        ("#[verifier::external] fn escape() {}", "external"),
        ("unsafe fn escape() {}", "unsafe"),
        (
            "#[allow(verus::lint)] fn escape() {}",
            "allow(verification lint)",
        ),
    ] {
        let root = fixture(text);
        let log = receipt(
            &root,
            "verus",
            "verification results:: 1 verified, 0 errors\n",
        );
        let mut r = analyze(&root, None).unwrap();
        apply_evidence(&mut r, &root, Some(&log), None).unwrap();
        assert!(r
            .units
            .iter()
            .find(|u| u.name == "escape")
            .is_some_and(|u| u.status == "hazard"));
        assert!(r
            .units
            .iter()
            .any(|u| u.name == "contracted" && u.status == "proven"));
        assert!(
            r.escape_hatches
                .iter()
                .any(|e| e.kind == kind && e.line > 0),
            "{text}"
        );
    }
}
#[test]
fn upstream_is_not_whole_crate_proof() {
    let root = fixture("");
    fs::write(
        root.join("Cargo.lock"),
        "version = 4\n[[package]]\nname = \"libcrux-sha2\"\nversion = \"0.0.9\"\n",
    )
    .unwrap();
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, None, None).unwrap();
    assert_eq!(r.upstream_claims.len(), 1);
    assert_eq!(r.upstream_claims[0].confirmation, "unconfirmed");
    assert_eq!(r.dependencies[0].status, "unverified");
}
#[test]
fn failed_kani_does_not_prove_called_functions() {
    let root = fixture("");
    let log = receipt(
        &root,
        "kani",
        "Checking harness evidence::harness...\nVERIFICATION:- FAILED\n",
    );
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, None, Some(&log)).unwrap();
    assert!(r.units.iter().all(|u| u.status != "proven"));
}

#[test]
fn match_contract_includes_entire_body() {
    let r = formal_coverage::inventory(std::path::Path::new("fixtures/evidence")).unwrap();
    let u = r.units.iter().find(|u| u.name == "match_contract").unwrap();
    assert!(u.tokens.contains("let value"));
    assert_eq!(u.sloc, 6);
}

#[test]
fn unsafe_unit_needs_explicit_bounded_harness_coverage() {
    let root = fixture(
        "unsafe fn danger() {}\n#[kani::proof] fn cover_danger() { unsafe { danger(); } }\n",
    );
    let log = receipt(
        &root,
        "kani",
        "Checking harness evidence::cover_danger...\nVERIFICATION:- SUCCESSFUL\n",
    );
    let sidecar = format!("{}.evidence.json", log.display());
    let mut receipt: Receipt = serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    receipt.command[3] = "evidence::cover_danger".into();
    receipt.harnesses = vec!["evidence::cover_danger".into()];
    fs::write(&sidecar, serde_json::to_vec(&receipt).unwrap()).unwrap();
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, None, Some(&log)).unwrap();
    assert_eq!(
        r.units.iter().find(|u| u.name == "danger").unwrap().status,
        "hazard"
    );
    assert!(formal_coverage::markdown(&r).contains("SECURITY HAZARD"));
    receipt
        .covered_units
        .insert("evidence::danger".into(), "evidence::cover_danger".into());
    fs::write(&sidecar, serde_json::to_vec(&receipt).unwrap()).unwrap();
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, None, Some(&log)).unwrap();
    let target = r.units.iter().find(|u| u.name == "danger").unwrap();
    assert_eq!(target.status, "proven-bounded");
    assert!(target
        .proof_scope
        .as_ref()
        .unwrap()
        .contains("unwind bound 8"));
    // Bounds never contribute to the unbounded security-proven gate.
    assert!(!formal_coverage::gate(&r, 1., None, false)
        .unwrap()
        .is_empty());
}

#[test]
fn third_party_unsafe_is_counted_separately() {
    let root = fixture("");
    fs::create_dir_all(root.join("vendor/dep/src")).unwrap();
    fs::write(
        root.join("vendor/dep/src/lib.rs"),
        "pub unsafe fn low() {}\nfn block() { unsafe {} }\n",
    )
    .unwrap();
    fs::write(
        root.join("Cargo.lock"),
        "version=4\n[[package]]\nname=\"dep\"\nversion=\"1.0.0\"\n",
    )
    .unwrap();
    fs::write(
        root.join("formal-coverage.toml"),
        "[dependency_sources]\n\"dep@1.0.0\"=\"vendor/dep\"\n",
    )
    .unwrap();
    fs::write(
        root.join("vendor/dep/Cargo.toml"),
        "[package]\nname=\"dep\"\nversion=\"1.0.0\"\n",
    )
    .unwrap();
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, None, None).unwrap();
    assert_eq!(r.dependencies[0].unsafe_sites, Some(2));
    assert_eq!(r.dependencies[0].status, "unverified");
    assert_eq!(
        r.escape_hatches.iter().filter(|e| !e.first_party).count(),
        2
    );
    assert!(formal_coverage::markdown(&r).contains("Third-party unsafe sites"));
    fs::write(
        root.join("vendor/dep/Cargo.toml"),
        "[package]\nname=\"wrong-source\"\nversion=\"1.0.0\"\n",
    )
    .unwrap();
    assert!(analyze(&root, None).is_err());
}

#[test]
fn cfg_disabled_contracts_are_not_proven_by_a_file_summary() {
    let root = fixture("verus! { #[cfg(any())] fn excluded() requires true, ensures true, {} }\n");
    let log = receipt(
        &root,
        "verus",
        "verification results:: 1 verified, 0 errors\n",
    );
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, Some(&log), None).unwrap();
    assert_eq!(
        r.units
            .iter()
            .find(|u| u.name == "excluded")
            .unwrap()
            .status,
        "unverified"
    );
    assert!(r
        .units
        .iter()
        .any(|u| u.name == "contracted" && u.status == "proven"));
}

#[test]
fn only_confirmed_whole_crate_registry_claim_promotes() {
    // Synthetic registry entry exercises policy; this is not a real upstream proof claim.
    let mut r = analyze(std::path::Path::new("fixtures/classification"), None).unwrap();
    let claim = formal_coverage::Claim {
        crate_name: "external-parser".into(),
        version: "=1.0.0".into(),
        operations: "synthetic whole-crate fixture".into(),
        property: "synthetic parser policy test".into(),
        tool: "synthetic fixture".into(),
        source: "https://example.invalid/synthetic-fixture".into(),
        confirmation: "confirmed".into(),
        whole_crate: true,
    };
    formal_coverage::apply_upstream_registry(&mut r, vec![claim]).unwrap();
    assert_eq!(r.dependencies[0].status, "proven-upstream");
    assert!(
        formal_coverage::summarize(&r)
            .third_party
            .security_relevant
            .percent("proven-upstream")
            > 0.
    );
}

#[test]
fn proof_code_syntax_and_evidence_are_separate_from_runtime() {
    use formal_coverage::Category;
    let root = fixture(
        r#"
verus! {
    pub open spec fn public_open() -> bool { true }
    closed spec fn private_closed() -> bool { true }
    pub closed spec fn public_closed() -> bool { true }
    open spec fn private_open() -> bool { true }
    spec fn plain_spec() -> bool { true }
    pub(crate) open spec fn restricted() -> bool { true }
    pub proof fn public_proof() {}
    broadcast proof fn broadcast_lemma() {}
    pub broadcast proof fn public_broadcast() {}
    proof fn plain_proof() {}
    impl Example {
        pub proof fn method_proof() {}
        pub open spec fn method_spec() -> bool { true }
        fn method_exec() requires true, ensures true, { proof { assert(true); } }
    }
    fn spec_something() {}
    fn string_exec() { let text = "proof fn fake() {}"; }
    // proof fn commented() {}
}
"#,
    );
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, None, None).unwrap();
    let proof: Vec<_> = r
        .units
        .iter()
        .filter(|u| u.category == Category::ProofCode)
        .collect();
    assert_eq!(proof.len(), 13);
    assert!(proof.iter().all(|u| u.status == "unverified"));
    for name in ["method_exec", "spec_something", "string_exec"] {
        assert_eq!(
            r.units.iter().find(|u| u.name == name).unwrap().category,
            Category::Runtime
        );
    }
    assert!(!r
        .units
        .iter()
        .any(|u| ["fake", "commented"].contains(&u.name.as_str())));
    let before = formal_coverage::summarize(&r);
    assert_eq!(before.proof_code.total.units, 13);
    assert_eq!(
        before.all.security_relevant.sloc + before.proof_code.total.sloc,
        r.units.iter().map(|u| u.sloc).sum::<usize>()
    );
    let log = receipt(
        &root,
        "verus",
        "verification results:: 12 verified, 0 errors\n",
    );
    // Synthetic parser evidence tests policy, not mathematical verification of the fixture.
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, Some(&log), None).unwrap();
    let summary = formal_coverage::summarize(&r);
    assert_eq!(summary.proof_code.total.verified, 12);
    assert_eq!(summary.proof_code.total.unverified, 1);
    assert_eq!(
        summary.all.security_relevant.sloc,
        before.all.security_relevant.sloc
    );
    assert!(formal_coverage::markdown(&r).contains("## PROOF CODE"));
    fs::write(
        root.join("formal-coverage.toml"),
        "not_security = [\"*::plain_spec\"]\n",
    )
    .unwrap();
    let split = analyze(&root, None).unwrap();
    let totals = formal_coverage::summarize(&split);
    assert_eq!(totals.proof_code.everything_else.units, 1);
    assert_eq!(totals.proof_code.security_relevant.units, 12);
    assert_eq!(
        totals.all.security_relevant.sloc,
        before.all.security_relevant.sloc
    );
    assert_eq!(totals.all.everything_else.sloc, 0);
    // Full-file evidence never promotes a different module.
    fs::write(root.join("src/extra.rs"), "verus! { proof fn extra() {} }").unwrap();
    let sidecar = format!("{}.evidence.json", log.display());
    let mut evidence: Receipt = serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    evidence.files.insert(
        "src/extra.rs".into(),
        sha256(&fs::read(root.join("src/extra.rs")).unwrap()),
    );
    fs::write(&sidecar, serde_json::to_vec(&evidence).unwrap()).unwrap();
    let mut r = analyze(&root, None).unwrap();
    apply_evidence(&mut r, &root, Some(&log), None).unwrap();
    assert_eq!(formal_coverage::summarize(&r).proof_code.total.verified, 12);
    assert_eq!(
        r.units.iter().find(|u| u.name == "extra").unwrap().status,
        "unverified"
    );
}

#[test]
fn proof_hazards_remain_visible_and_fail_cli_gate() {
    use formal_coverage::{gate, markdown, summarize, Category};
    use std::process::Command;
    for extra in [
        "proof fn escape() { assume(false); }",
        "proof fn escape() { admit(); }",
        "#[verifier::external] proof fn escape() {}",
        "proof fn escape() { unsafe {} }",
    ] {
        let root = fixture(&format!("verus! {{ {extra} }}"));
        let mut r = analyze(&root, None).unwrap();
        let baseline = r.clone();
        let log = receipt(
            &root,
            "verus",
            "verification results:: 1 verified, 0 errors\n",
        );
        apply_evidence(&mut r, &root, Some(&log), None).unwrap();
        assert!(r
            .units
            .iter()
            .any(|u| u.name == "contracted" && u.status == "proven"));
        let escape = r.units.iter().find(|u| u.name == "escape").unwrap();
        assert_eq!(escape.category, Category::ProofCode);
        assert_eq!(escape.status, "hazard");
        assert!(r
            .escape_hatches
            .iter()
            .any(|e| e.unit == escape.id && e.security));
        assert!(markdown(&r).contains("SECURITY HAZARD"));
        assert_eq!(summarize(&r).proof_code.total.hazard_units, 1);
        assert!(gate(&r, 0., Some(&baseline), true)
            .unwrap()
            .iter()
            .any(|f| f.contains("new escape")));
        // Removing the informational security label cannot suppress a proof hazard.
        r.units
            .iter_mut()
            .find(|u| u.name == "escape")
            .unwrap()
            .security = false;
        assert!(markdown(&r).contains("PROOF-CODE HAZARD"));
        assert!(!gate(&r, 0., None, false).unwrap().is_empty());
        let baseline_path = root.join("baseline.json");
        fs::write(&baseline_path, serde_json::to_vec(&baseline).unwrap()).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
            .args([
                "gate",
                root.to_str().unwrap(),
                "--min-security-proven",
                "0",
                "--baseline",
                baseline_path.to_str().unwrap(),
                "--no-new-escape-hatches",
            ])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stderr).contains("new escape"));
    }
}

#[test]
fn receipt_rejections_name_the_actual_mismatch() {
    for (field, value, reason) in [
        ("tool", serde_json::json!("kani"), "tool mismatch"),
        (
            "tool_version",
            serde_json::json!("  "),
            "missing tool version",
        ),
        ("exit_code", serde_json::json!(1), "nonzero exit status"),
        (
            "log_sha256",
            serde_json::json!("invalid"),
            "log hash mismatch",
        ),
    ] {
        let root = fixture("");
        let log = receipt(
            &root,
            "verus",
            "verification results:: 1 verified, 0 errors\n",
        );
        let path = format!("{}.evidence.json", log.display());
        let mut data: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        data[field] = value;
        fs::write(path, serde_json::to_vec(&data).unwrap()).unwrap();
        let mut report = analyze(&root, None).unwrap();
        apply_evidence(&mut report, &root, Some(&log), None).unwrap();
        assert!(
            report.warnings.iter().any(|w| w.ends_with(reason)),
            "{:?}",
            report.warnings
        );
        assert!(report.units.iter().all(|u| u.status != "proven"));
    }
}
