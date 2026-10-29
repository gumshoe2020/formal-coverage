//! Synthetic execution receipts exercise attribution policy, not mathematical correctness.
use formal_coverage::{analyze, apply_evidence_many, sha256, Receipt};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
fn fixture(files: &[(&str, &str)]) -> PathBuf {
    let root = PathBuf::from(format!(
        "logs/union-test-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname=\"union_fixture\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[workspace]\n",
    )
    .unwrap();
    for (file, text) in files {
        let path = root.join(file);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    root
}
fn receipt(root: &Path, name: &str, input: &str) -> PathBuf {
    let log = root.join(format!("{name}.log"));
    fs::write(&log, "verification results:: 1 verified, 0 errors\n").unwrap();
    let files = formal_coverage::inventory(root)
        .unwrap()
        .source_files
        .into_iter()
        .map(|file| {
            let hash = sha256(&fs::read(root.join(&file)).unwrap());
            (file, hash)
        })
        .collect();
    let receipt = Receipt {
        cfg: None,
        tool: "verus".into(),
        tool_version: "synthetic fixture".into(),
        command: vec![
            "verus".into(),
            "--crate-type".into(),
            "lib".into(),
            input.into(),
        ],
        exit_code: 0,
        files,
        log_sha256: sha256(&fs::read(&log).unwrap()),
        harnesses: vec![],
        covered_units: BTreeMap::new(),
    };
    fs::write(
        format!("{}.evidence.json", log.display()),
        serde_json::to_vec(&receipt).unwrap(),
    )
    .unwrap();
    log
}
fn change(log: &Path, f: impl FnOnce(&mut Receipt)) {
    let sidecar = format!("{}.evidence.json", log.display());
    let mut r = serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    f(&mut r);
    fs::write(sidecar, serde_json::to_vec(&r).unwrap()).unwrap();
}
fn statuses(root: &Path, logs: &[PathBuf]) -> formal_coverage::Report {
    let mut report = analyze(root, None).unwrap();
    apply_evidence_many(&mut report, root, logs, &[]).unwrap();
    report
}
#[test]
fn union_is_internal_and_rejected_receipts_are_isolated() {
    let root = fixture(&[
        ("src/lib.rs", "verus! { proof fn first() {} }"),
        ("src/other.rs", "verus! { proof fn second() {} }"),
    ]);
    let a = receipt(&root, "a", "src/lib.rs");
    let b = receipt(&root, "b", "src/other.rs");
    let both = statuses(&root, &[a.clone(), b.clone()]);
    assert!(both.units.iter().all(|u| u.status == "verified"));
    assert!(both
        .evidence_runs
        .iter()
        .all(|r| r.accepted && r.credited_units.len() == 1));
    let json = root.join("report.json");
    let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args([
            "report",
            root.to_str().unwrap(),
            "--verus-log",
            a.to_str().unwrap(),
            "--verus-log",
            b.to_str().unwrap(),
            "--json",
            json.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: formal_coverage::Report = serde_json::from_slice(&fs::read(json).unwrap()).unwrap();
    assert!(report.units.iter().all(|u| u.status == "verified"));
    for bad in ["stale", "log", "command", "malformed", "missing"] {
        let b = receipt(&root, "b", "src/other.rs");
        match bad {
            "stale" => change(&b, |r| {
                r.files.insert("src/lib.rs".into(), "stale".into());
            }),
            "log" => change(&b, |r| r.log_sha256 = "mismatch".into()),
            "command" => change(&b, |r| r.command.push("--no-verify".into())),
            "malformed" => fs::write(format!("{}.evidence.json", b.display()), "{").unwrap(),
            _ => fs::remove_file(format!("{}.evidence.json", b.display())).unwrap(),
        }
        let report = statuses(&root, &[a.clone(), b]);
        assert_eq!(
            report
                .units
                .iter()
                .find(|u| u.name == "first")
                .unwrap()
                .status,
            "verified"
        );
        assert_eq!(
            report
                .units
                .iter()
                .find(|u| u.name == "second")
                .unwrap()
                .status,
            "unverified"
        );
        assert!(!report.evidence_runs[1].accepted);
        assert!(report.evidence_runs[1].credited_units.is_empty());
        assert!(!report.evidence_runs[1].reasons.is_empty());
    }
}
#[test]
fn module_file_rules_nested_inline_and_verus_scopes() {
    let root = fixture(&[
        (
            "src/lib.rs",
            "mod a; mod b; mod inline { mod leaf; } verus! { proof fn root() {} }",
        ),
        ("src/a.rs", "mod child; verus! { proof fn a() {} }"),
        ("src/a/child.rs", "verus! { proof fn child() {} }"),
        ("src/b/mod.rs", "mod inner; verus! { proof fn b() {} }"),
        ("src/b/inner.rs", "verus! { proof fn inner() {} }"),
        ("src/inline/leaf.rs", "verus! { proof fn leaf() {} }"),
        ("src/unrelated.rs", "verus! { proof fn unrelated() {} }"),
    ]);
    let log = receipt(&root, "run", "src/lib.rs");
    let report = statuses(&root, &[log]);
    assert_eq!(
        report
            .units
            .iter()
            .filter(|u| u.status == "verified")
            .count(),
        6
    );
    assert_eq!(
        report
            .units
            .iter()
            .find(|u| u.name == "unrelated")
            .unwrap()
            .status,
        "unverified"
    );
}
#[test]
fn missing_hash_and_attributes_never_credit_modules() {
    for attribute in [
        "",
        "#[path=\"a.rs\"]",
        "#[cfg(any())]",
        "#[cfg_attr(mystery(),path=\"a.rs\")]",
    ] {
        let root = fixture(&[
            (
                "src/lib.rs",
                &format!("{attribute} mod a; verus! {{ proof fn root() {{}} }}"),
            ),
            ("src/a.rs", "verus! { proof fn child() {} }"),
        ]);
        let log = receipt(&root, "run", "src/lib.rs");
        if attribute.is_empty() {
            change(&log, |r| {
                r.files.remove("src/a.rs");
            });
        }
        let report = statuses(&root, &[log]);
        assert_eq!(
            report
                .units
                .iter()
                .find(|u| u.name == "child")
                .unwrap()
                .status,
            "unverified"
        );
        assert!(report
            .warnings
            .iter()
            .any(|w| w.contains("a") && (w.contains("missing") || w.contains("unverified"))));
    }
}
#[test]
fn restricted_and_unknown_commands_fail_closed() {
    for flag in [
        "--verify-root",
        "--verify-module",
        "--verify-only-module",
        "--verify-function",
        "--no-verify",
        "--unconfirmed-future-filter",
    ] {
        let root = fixture(&[
            ("src/lib.rs", "mod a; verus! { proof fn root() {} }"),
            ("src/a.rs", "verus! { proof fn child() {} }"),
        ]);
        let log = receipt(&root, "run", "src/lib.rs");
        change(&log, |r| r.command.push(flag.into()));
        let report = statuses(&root, &[log]);
        assert!(report.units.iter().all(|u| u.status == "unverified"));
        assert!(!report.evidence_runs[0].accepted);
    }
}
#[test]
fn skip_flag_cannot_masquerade_as_a_hashed_root() {
    let root = fixture(&[
        ("src/lib.rs", "mod a; verus! { proof fn root() {} }"),
        ("src/a.rs", "verus! { proof fn child() {} }"),
    ]);
    fs::write(root.join("--no-verify"), "mod a;").unwrap();
    let log = receipt(&root, "run", "src/lib.rs");
    change(&log, |r| {
        r.command[3] = "--no-verify".into();
        r.files.insert(
            "--no-verify".into(),
            sha256(&fs::read(root.join("--no-verify")).unwrap()),
        );
    });
    let report = statuses(&root, &[log]);
    assert!(!report.evidence_runs[0].accepted);
    assert!(report.units.iter().all(|u| u.status == "unverified"));
}

#[test]
fn module_hazards_are_local_and_block_the_gate() {
    let escape = "proof fn danger() { assume(false); }";
    let root = fixture(&[
        ("src/lib.rs", "mod a; verus! { proof fn root() {} }"),
        ("src/a.rs", &format!("verus! {{ {escape} }}")),
    ]);
    let baseline = analyze(&root, None).unwrap();
    let log = receipt(&root, "run", "src/lib.rs");
    let report = statuses(&root, std::slice::from_ref(&log));
    assert!(report
        .units
        .iter()
        .any(|u| u.name == "root" && u.status == "verified"));
    assert_eq!(
        report
            .units
            .iter()
            .find(|u| u.name == "danger")
            .unwrap()
            .status,
        "hazard"
    );
    assert_eq!(report.evidence_runs[0].credited_units.len(), 1);
    assert!(!formal_coverage::gate(&report, 0., Some(&baseline), true)
        .unwrap()
        .is_empty());
    let baseline_path = root.join("baseline.json");
    fs::write(&baseline_path, serde_json::to_vec(&baseline).unwrap()).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args([
            "gate",
            root.to_str().unwrap(),
            "--min-security-proven",
            "0",
            "--baseline",
            baseline_path.to_str().unwrap(),
            "--no-new-escape-hatches",
            "--verus-log",
            log.to_str().unwrap(),
            "--verus-log",
            log.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("new escape"));
}

#[test]
fn functionless_module_escape_is_visible_and_fails_gate() {
    let root = fixture(&[
        ("src/lib.rs", "mod a; verus! { proof fn root() {} }"),
        ("src/a.rs", "verus! { assume(false); }"),
    ]);
    let log = receipt(&root, "run", "src/lib.rs");
    let report = statuses(&root, &[log]);
    assert!(report
        .units
        .iter()
        .any(|u| u.name == "root" && u.status == "verified"));
    assert!(report
        .escape_hatches
        .iter()
        .any(|e| e.file == "src/a.rs" && e.kind == "assume"));
    assert!(formal_coverage::markdown(&report).contains("FILE-SCOPE HAZARD"));
    assert!(!formal_coverage::gate(&report, 0., None, false)
        .unwrap()
        .is_empty());
}

#[test]
fn kani_receipts_keep_harness_mapping_and_bounds_independent() {
    let root = fixture(&[(
        "src/lib.rs",
        "#[kani::proof] fn one() { target(); }\n#[kani::proof] fn two() {}\nfn target() {}",
    )]);
    let mut logs = vec![];
    for (name, bound) in [("one", "8"), ("two", "16")] {
        let log = receipt(&root, name, "src/lib.rs");
        let text =
            format!("Checking harness union_fixture::{name}...\nVERIFICATION:- SUCCESSFUL\n");
        fs::write(&log, &text).unwrap();
        change(&log, |r| {
            r.tool = "kani".into();
            r.command = [
                "cargo",
                "kani",
                "--harness",
                &format!("union_fixture::{name}"),
                "--output-format=regular",
                "--unwind",
                bound,
            ]
            .into_iter()
            .map(String::from)
            .collect();
            r.harnesses = vec![format!("union_fixture::{name}")];
            r.log_sha256 = sha256(text.as_bytes());
            // This invalid cross-receipt mapping must not borrow one's successful harness.
            if name == "two" {
                r.covered_units
                    .insert("union_fixture::target".into(), "union_fixture::one".into());
            }
        });
        logs.push(log);
    }
    let mut report = analyze(&root, None).unwrap();
    apply_evidence_many(&mut report, &root, &[], &logs).unwrap();
    assert_eq!(
        report
            .units
            .iter()
            .filter(|u| u.status == "proven-bounded")
            .count(),
        2
    );
    assert_eq!(
        report
            .units
            .iter()
            .find(|u| u.name == "target")
            .unwrap()
            .status,
        "unverified"
    );
    assert!(report
        .units
        .iter()
        .find(|u| u.name == "two")
        .unwrap()
        .proof_scope
        .as_ref()
        .unwrap()
        .contains("bound 16"));
    assert!(report
        .evidence_runs
        .iter()
        .all(|r| r.credited_units.len() == 1));
    assert_eq!(
        formal_coverage::summarize(&report)
            .proof_code
            .total
            .verified,
        0
    );
    assert_eq!(
        formal_coverage::summarize(&report)
            .first_party
            .security_relevant
            .percent("proven"),
        0.
    );
    let json = root.join("kani.json");
    let out = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args([
            "report",
            root.to_str().unwrap(),
            "--kani-log",
            logs[0].to_str().unwrap(),
            "--kani-log",
            logs[1].to_str().unwrap(),
            "--json",
            json.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    let report: formal_coverage::Report = serde_json::from_slice(&fs::read(json).unwrap()).unwrap();
    assert_eq!(
        report
            .units
            .iter()
            .filter(|u| u.status == "proven-bounded")
            .count(),
        2
    );
}

#[test]
fn receipt_cfg_selects_modules_items_and_inline_ancestors() {
    let root = fixture(&[
        (
            "src/lib.rs",
            r#"
#![cfg_attr(not(feature="std"), no_std)]
#[cfg(feature="selected")] mod a;
#[cfg(not(feature="selected"))] mod b;
verus! {
    #[cfg(any(feature="selected", test))] proof fn feature_item() {}
    #[cfg(all(not(test), custom))] proof fn custom_item() {}
    #[cfg(feature="selected")] mod inline { proof fn nested() {} }
    #[cfg(verus_keep_ghost)] proof fn ghost() {}
    #[cfg(unknown_predicate("x"))] proof fn uncertain() {}
    #[cfg_attr(unknown_predicate("x"), allow(dead_code))] proof fn harmless() {}
    #[cfg_attr(unknown_predicate("x"), changes_items)] proof fn macro_uncertain() {}
    proof fn sibling() {}
}"#,
        ),
        ("src/a.rs", "verus! { proof fn a() {} }"),
        ("src/b.rs", "verus! { proof fn b() {} }"),
    ]);
    let log = receipt(&root, "cfg", "src/lib.rs");
    for selected in [false, true] {
        change(&log, |r| {
            r.cfg = selected.then(|| vec!["feature=\"selected\"".into(), "custom".into()]);
        });
        let report = statuses(&root, std::slice::from_ref(&log));
        for (name, expected) in [
            ("a", selected),
            ("b", !selected),
            ("feature_item", selected),
            ("custom_item", selected),
            ("nested", selected),
            ("ghost", true),
            ("uncertain", false),
            ("macro_uncertain", false),
            ("harmless", true),
            ("sibling", true),
        ] {
            assert_eq!(
                report.units.iter().find(|u| u.name == name).unwrap().status == "verified",
                expected,
                "{name} selected={selected}"
            );
        }
        assert!(report
            .warnings
            .iter()
            .any(|w| w.contains("uncertain") && w.contains("unknown_predicate")));
    }
    change(&log, |r| {
        r.cfg = None;
        r.command.extend([
            "--cfg".into(),
            "feature=\"selected\"".into(),
            "--cfg=custom".into(),
        ]);
    });
    let inferred = statuses(&root, &[log]);
    assert_eq!(
        inferred
            .units
            .iter()
            .find(|u| u.name == "a")
            .unwrap()
            .status,
        "verified"
    );
}

#[test]
fn trusted_items_and_scoped_escapes_leave_callers_credited() {
    let root = fixture(&[
        (
            "src/lib.rs",
            r#"mod a;
verus! {
    #[verifier::external_body]
    fn trusted() ensures true, {}
    fn caller() ensures true, { trusted(); }
    fn dangerous() ensures true, { assume(false); }
    assume_specification [foreign](x:u8) ensures x == x;
}"#,
        ),
        ("src/a.rs", "verus! { proof fn lemma() {} }"),
        ("src/unrelated.rs", "unsafe impl Unrelated {}"),
    ]);
    let baseline = analyze(&root, None).unwrap();
    let log = receipt(&root, "trusted", "src/lib.rs");
    let report = statuses(&root, &[log]);
    assert_eq!(report.trusted_items.len(), 2);
    assert!(report.trusted_items.iter().all(|t| t.sloc > 0));
    assert_eq!(
        report
            .units
            .iter()
            .find(|u| u.name == "trusted")
            .unwrap()
            .status,
        "trusted"
    );
    let caller = report.units.iter().find(|u| u.name == "caller").unwrap();
    assert_eq!(caller.status, "proven");
    assert!(caller
        .proof_scope
        .as_ref()
        .unwrap()
        .contains("2 trusted items"));
    assert!(caller.proof_scope.as_ref().unwrap().contains("1 escapes"));
    assert_eq!(
        report
            .units
            .iter()
            .find(|u| u.name == "dangerous")
            .unwrap()
            .status,
        "hazard"
    );
    assert_eq!(report.escape_hatches.len(), 2);
    assert!(formal_coverage::gate(&report, 0., Some(&baseline), true)
        .unwrap()
        .iter()
        .any(|s| s.contains("new escape")));
    let md = formal_coverage::markdown(&report);
    assert!(md.contains("relying on 2 trusted items"));
    assert!(md.contains("## Trusted items"));
    assert!(md.contains("ESCAPES:"));
}

#[test]
fn distinct_file_scope_sites_keep_affected_units_separate() {
    let root = fixture(&[(
        "src/lib.rs",
        "unsafe impl Trait for Type {}\nverus! { proof fn a() {} proof fn b() {} }",
    )]);
    let log = receipt(&root, "distinct", "src/lib.rs");
    let report = statuses(&root, &[log]);
    assert_eq!(report.escape_hatches.len(), 1);
    assert_eq!(report.escape_hatches[0].affected_units.len(), 2);
    assert!(report.units.iter().all(|u| u.status == "hazard"));
    let mut old = report.clone();
    old.escape_hatches.push(old.escape_hatches[0].clone());
    assert!(!formal_coverage::gate(&report, 0., Some(&old), true)
        .unwrap()
        .iter()
        .any(|s| s.contains("new escape")));
}

#[test]
fn kani_harness_assumptions_and_cfg_modules_are_proof_code() {
    let root = fixture(&[(
        "src/lib.rs",
        r##"
#[kani::proof] fn harness() { kani::assume(true); runtime(); }
#[cfg(kani)] mod proofs { fn helper() { kani::assume(true); } }
fn runtime() { let _ = "#[kani::proof]"; }
fn bad() { kani::assume(false); }
"##,
    )]);
    let report = statuses(&root, &[]);
    assert_eq!(report.escape_hatches.len(), 1);
    assert_eq!(report.escape_hatches[0].kind, "kani::assume");
    assert_eq!(
        report.escape_hatches[0].affected_units,
        vec!["union_fixture::bad"]
    );
    for name in ["harness", "helper"] {
        assert_eq!(
            report
                .units
                .iter()
                .find(|u| u.name == name)
                .unwrap()
                .category,
            formal_coverage::Category::ProofCode
        );
    }
    assert_eq!(
        formal_coverage::summarize(&report).proof_code.total.units,
        2
    );
}

#[test]
fn out_of_scope_escape_cannot_remove_root_credit_but_fails_gate() {
    let root = fixture(&[
        ("src/lib.rs", "verus! { fn checked() ensures true, {} }"),
        ("src/other.rs", "fn escape() { assume(false); }"),
    ]);
    let baseline = analyze(&root, None).unwrap();
    let log = receipt(&root, "scope", "src/lib.rs");
    let report = statuses(&root, &[log]);
    let checked = report.units.iter().find(|u| u.name == "checked").unwrap();
    assert_eq!(checked.status, "proven");
    assert!(checked.proof_scope.as_ref().unwrap().contains("0 escapes"));
    assert_eq!(report.escape_hatches.len(), 1);
    assert!(formal_coverage::gate(&report, 0., Some(&baseline), true)
        .unwrap()
        .iter()
        .any(|s| s.contains("new escape")));
}

#[test]
fn cfg_owners_also_control_macro_modules_and_nested_functions() {
    let root = fixture(&[
        (
            "src/lib.rs",
            r#"
#[cfg(any())] verus! { mod hidden; }
mod inline { #![cfg(any())] mod leaf; }
verus! {
    #[cfg(any())] fn disabled() ensures true, { fn local() ensures true, {} }
    proof fn sibling() {}
}
"#,
        ),
        ("src/hidden.rs", "verus! { proof fn hidden() {} }"),
        ("src/inline/leaf.rs", "verus! { proof fn leaf() {} }"),
    ]);
    let log = receipt(&root, "owners", "src/lib.rs");
    let report = statuses(&root, &[log]);
    for name in ["hidden", "leaf", "disabled", "local"] {
        assert_eq!(
            report.units.iter().find(|u| u.name == name).unwrap().status,
            "unverified",
            "{name}"
        );
    }
    assert_eq!(
        report
            .units
            .iter()
            .find(|u| u.name == "sibling")
            .unwrap()
            .status,
        "verified"
    );
}

#[test]
fn receipt_scope_lists_only_cfg_active_trust_and_escapes() {
    let root = fixture(&[(
        "src/lib.rs",
        r#"verus! {
    #[cfg(feature="alloc")]
    #[verifier::external_body]
    fn trusted() ensures true, {}
    #[cfg(feature="alloc")]
    assume_specification [foreign](x:u8) ensures x == x;
    #[cfg(feature="alloc")]
    fn escape() { assume(false); }
    fn checked() ensures true, {}
}"#,
    )]);
    let log = receipt(&root, "active", "src/lib.rs");
    for enabled in [false, true] {
        change(&log, |r| {
            r.cfg = enabled.then(|| vec!["feature=\"alloc\"".into()])
        });
        let report = statuses(&root, std::slice::from_ref(&log));
        assert_eq!(report.trusted_items.len(), 2);
        assert_eq!(report.escape_hatches.len(), 1);
        let run = &report.evidence_runs[0];
        assert_eq!(run.trusted_sites.len(), if enabled { 2 } else { 0 });
        assert_eq!(run.escape_sites.len(), usize::from(enabled));
        assert_eq!(
            report
                .units
                .iter()
                .find(|u| u.name == "checked")
                .unwrap()
                .status,
            "proven"
        );
    }
}

#[test]
fn cfg_disabled_file_scope_escape_does_not_poison_active_siblings() {
    let root = fixture(&[(
        "src/lib.rs",
        r#"
verus! { fn checked() ensures true, {} }
#[cfg(any())] mod tests { unsafe impl Trait for Type {} }
"#,
    )]);
    let log = receipt(&root, "inactive_site", "src/lib.rs");
    let report = statuses(&root, &[log]);
    assert_eq!(report.escape_hatches.len(), 1);
    assert!(report.evidence_runs[0].escape_sites.is_empty());
    assert_eq!(
        report
            .units
            .iter()
            .find(|u| u.name == "checked")
            .unwrap()
            .status,
        "proven"
    );
    let baseline = analyze(&root, None).unwrap();
    assert!(formal_coverage::gate(&report, 0., Some(&baseline), true)
        .unwrap()
        .iter()
        .any(|s| s.contains("new escape")));
}

#[test]
fn trusted_verifier_exemptions_cannot_receive_kani_bounded_credit() {
    let root = fixture(&[(
        "src/lib.rs",
        "#[kani::proof] fn harness() { target(); }\n#[verifier::external_body] fn target() {}",
    )]);
    let log = receipt(&root, "trusted_kani", "src/lib.rs");
    let text = "Checking harness union_fixture::harness...\nVERIFICATION:- SUCCESSFUL\n";
    fs::write(&log, text).unwrap();
    change(&log, |r| {
        r.tool = "kani".into();
        r.command = [
            "cargo",
            "kani",
            "--harness",
            "union_fixture::harness",
            "--output-format=regular",
            "--unwind",
            "8",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        r.harnesses = vec!["union_fixture::harness".into()];
        r.covered_units.insert(
            "union_fixture::target".into(),
            "union_fixture::harness".into(),
        );
        r.log_sha256 = sha256(text.as_bytes());
    });
    let mut report = analyze(&root, None).unwrap();
    apply_evidence_many(&mut report, &root, &[], &[log]).unwrap();
    assert_eq!(
        report
            .units
            .iter()
            .find(|u| u.name == "target")
            .unwrap()
            .status,
        "trusted"
    );
    assert!(report.units.iter().all(|u| u.status != "proven-bounded"));
}
