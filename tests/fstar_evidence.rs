//! Synthetic policy tests; no hax, F*, or fixture Rust execution.
use formal_coverage::{apply_fstar_evidence, inventory, sha256, FstarModule, FstarReceipt, Report};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture {
    root: PathBuf,
    log: PathBuf,
    receipt: FstarReceipt,
}
impl Fixture {
    fn new() -> Self {
        let root = PathBuf::from(format!(
            "logs/v1.7/fixtures/{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("fresh")).unwrap();
        fs::create_dir_all(root.join("checked")).unwrap();
        for file in ["Cargo.toml", "src/lib.rs", "hax-driver.py"] {
            fs::copy(PathBuf::from("fixtures/hax").join(file), root.join(file)).unwrap();
        }
        fs::write(root.join("Cargo.lock"), "version = 4\n").unwrap();
        let files = ["Cargo.toml", "Cargo.lock", "src/lib.rs", "hax-driver.py"]
            .into_iter()
            .map(|f| (f.to_string(), sha256(&fs::read(root.join(f)).unwrap())))
            .collect();
        let mut fresh = BTreeMap::new();
        let mut checked = BTreeMap::new();
        for name in ["Hax_fixture.fst", "Hax_fixture.fsti"] {
            let bytes = fs::read(PathBuf::from("fixtures/hax/extraction").join(name)).unwrap();
            for (dir, map) in [("fresh", &mut fresh), ("checked", &mut checked)] {
                let path = format!("{dir}/{name}");
                fs::write(root.join(&path), &bytes).unwrap();
                map.insert(path, sha256(&bytes));
            }
        }
        let log = root.join("run.log");
        fs::write(&log, "Verified module: Hax_fixture\n").unwrap();
        let receipt = FstarReceipt {
            tool: "hax-fstar".into(),
            hax_version: "synthetic policy fixture".into(),
            hax_commit: "1".repeat(40),
            fstar_version: "synthetic policy fixture".into(),
            extraction_command: ["python3", "hax-driver.py", "extract-fstar"]
                .map(String::from)
                .to_vec(),
            extraction_exit_code: 0,
            files,
            fresh,
            checked,
            patches: BTreeMap::new(),
            log_sha256: sha256(&fs::read(&log).unwrap()),
            modules: vec![FstarModule {
                file: "checked/Hax_fixture.fst".into(),
                command: vec!["fstar.exe".into(), "checked/Hax_fixture.fst".into()],
                exit_code: 0,
                outcome: "success".into(),
            }],
        };
        Self { root, log, receipt }
    }
    fn run(&self) -> Report {
        fs::write(
            format!("{}.evidence.json", self.log.display()),
            serde_json::to_vec(&self.receipt).unwrap(),
        )
        .unwrap();
        let mut r = inventory(&self.root).unwrap();
        apply_fstar_evidence(&mut r, &self.root, std::slice::from_ref(&self.log)).unwrap();
        r
    }
    fn replace(&mut self, file: &str, text: &str) {
        fs::write(self.root.join(file), text).unwrap();
        let map = if file.starts_with("fresh/") {
            &mut self.receipt.fresh
        } else {
            &mut self.receipt.checked
        };
        map.insert(file.into(), sha256(text.as_bytes()));
    }
    fn reject(&self, reason: &str) {
        let r = self.run();
        assert!(!r.evidence_runs[0].accepted, "{reason}");
        assert!(
            r.evidence_runs[0].reasons.join(" ").contains(reason),
            "{:?}",
            r.evidence_runs[0].reasons
        );
        assert!(!r.units.iter().any(|u| u.status == "proven-extracted"));
    }
}
#[test]
fn seven_required_cases() {
    let f = Fixture::new();
    let r = f.run();
    assert!(
        r.evidence_runs[0].accepted,
        "{:?}",
        r.evidence_runs[0].reasons
    );
    assert_eq!(
        r.units.iter().find(|u| u.name == "clean").unwrap().status,
        "proven-extracted"
    );
    assert!(r
        .fstar_definitions
        .iter()
        .any(|d| d.name.ends_with("round_trip")
            && d.contract_kind == "functional"
            && d.self_consistency_heuristic));
    assert!(formal_coverage::markdown(&r).contains("trusted hax extraction"));
    assert!(r.evidence_runs[0]
        .trusted_sites
        .iter()
        .any(|(_, _, k)| k.contains("hax extraction")));
    assert_eq!(
        formal_coverage::summarize(&r)
            .first_party
            .security_relevant
            .statuses["proven"],
        0
    );
    let mut f = Fixture::new();
    f.receipt.modules[0].command.push("--lax".into());
    f.reject("unsupported F* command");
    let mut f = Fixture::new();
    let body = "module Hax_fixture\nlet clean (x: u32) : Tot u32 = admit ()\n";
    f.replace("fresh/Hax_fixture.fst", body);
    f.replace("checked/Hax_fixture.fst", body);
    let r = f.run();
    assert!(!r.units.iter().any(|u| u.status == "proven-extracted"));
    assert!(r.escape_hatches.iter().any(|e| e.kind == "admit"));
    let mut f = Fixture::new();
    f.replace(
        "checked/Hax_fixture.fst",
        "module Hax_fixture\nlet clean (x: u32) : Tot u32 = 0\n",
    );
    let r = f.run();
    assert_eq!(
        r.units.iter().find(|u| u.name == "clean").unwrap().status,
        "trusted"
    );
    assert!(r
        .trusted_items
        .iter()
        .any(|t| t.kind.contains("patch-altered") && t.line == 2));
    let mut f = Fixture::new();
    f.receipt.checked.remove("checked/Hax_fixture.fst");
    fs::remove_file(f.root.join("checked/Hax_fixture.fst")).unwrap();
    f.receipt.modules[0].file = "checked/Hax_fixture.fsti".into();
    f.receipt.modules[0].command[1] = "checked/Hax_fixture.fsti".into();
    let r = f.run();
    assert!(!r.units.iter().any(|u| u.status == "proven-extracted"));
    assert!(r
        .trusted_items
        .iter()
        .any(|t| t.kind.contains("interface-only")));
    let f = Fixture::new();
    fs::write(f.root.join("src/lib.rs"), "pub fn changed() {}\n").unwrap();
    f.reject("stale hash");
}
#[test]
fn every_receipt_validation_boundary_rejects() {
    type ReceiptMutation = (&'static str, fn(&mut FstarReceipt));
    let changes: Vec<ReceiptMutation> = vec![
        ("tool mismatch", |r| r.tool = "verus".into()),
        ("missing tool version", |r| r.hax_version.clear()),
        ("missing tool version", |r| r.fstar_version.clear()),
        ("full hax commit", |r| r.hax_commit = "short".into()),
        ("log hash mismatch", |r| r.log_sha256 = "bad".into()),
        ("failed extraction", |r| r.extraction_exit_code = 1),
        ("unsupported or failed extraction", |r| {
            r.extraction_command.push("--admit".into())
        }),
        ("missing source hash", |r| {
            r.files.remove("src/lib.rs");
        }),
        ("missing source hash", |r| {
            r.files.remove("Cargo.lock");
        }),
        ("stale hash", |r| {
            r.fresh.insert("fresh/Hax_fixture.fst".into(), "bad".into());
        }),
        ("stale hash", |r| {
            r.checked
                .insert("checked/Hax_fixture.fst".into(), "bad".into());
        }),
        ("empty extraction", |r| r.fresh.clear()),
        ("empty extraction", |r| r.checked.clear()),
        ("empty extraction", |r| r.modules.clear()),
        ("duplicate module", |r| {
            r.modules.push(FstarModule {
                file: r.modules[0].file.clone(),
                command: r.modules[0].command.clone(),
                exit_code: 0,
                outcome: "success".into(),
            })
        }),
        ("unknown module outcome", |r| {
            r.modules[0].outcome = "maybe".into()
        }),
        ("inconsistent module outcome", |r| {
            r.modules[0].exit_code = 1
        }),
        ("unsupported F* command", |r| {
            r.modules[0].command.push("--admit_smt_queries".into())
        }),
        ("unsupported F* command", |r| {
            r.modules[0].command.push("--admit_except".into())
        }),
        ("unsupported F* command", |r| {
            r.modules[0].command.push("--unknown".into())
        }),
    ];
    for (reason, change) in changes {
        let mut f = Fixture::new();
        change(&mut f.receipt);
        f.reject(reason);
    }
    let f = Fixture::new();
    fs::write(f.root.join("checked/Unhashed.fst"), "module Unhashed").unwrap();
    f.reject("unhashed F* sibling");
    let mut f = Fixture::new();
    f.receipt.patches.insert("/absolute".into(), "bad".into());
    f.reject("absolute receipt path");
    let mut f = Fixture::new();
    fs::write(f.root.join("edit.patch"), "edit").unwrap();
    f.receipt.patches.insert("edit.patch".into(), "bad".into());
    f.reject("stale hash");
}
#[test]
fn failed_partial_unmapped_assumptions_and_file_options_never_promote() {
    for outcome in ["failed", "timeout", "skipped"] {
        let mut f = Fixture::new();
        f.receipt.modules[0].outcome = outcome.into();
        f.receipt.modules[0].exit_code = 1;
        let r = f.run();
        assert!(r.evidence_runs[0].accepted);
        assert!(r.evidence_runs[0].credited_units.is_empty());
    }
    for escape in [
        "assume (True)",
        "admit ()",
        "admitP ()",
        "#set-options \"--admit_smt_queries true\"",
        "#set-options \"--lax\"",
    ] {
        let mut f = Fixture::new();
        let text = format!("module Hax_fixture\n{escape}\nlet clean x : Tot u32 = x\n");
        f.replace("fresh/Hax_fixture.fst", &text);
        f.replace("checked/Hax_fixture.fst", &text);
        let r = f.run();
        assert!(r.evidence_runs[0].credited_units.is_empty(), "{escape}");
        assert!(!r.escape_hatches.is_empty());
    }
    let mut f = Fixture::new();
    let text = "module Hax_fixture\nlet missing x : Tot u32 = x\n";
    f.replace("fresh/Hax_fixture.fst", text);
    f.replace("checked/Hax_fixture.fst", text);
    let r = f.run();
    assert!(r.evidence_runs[0].credited_units.is_empty());
    assert!(r
        .fstar_definitions
        .iter()
        .any(|d| d.classification == "unmapped"));
}

#[test]
fn malformed_missing_and_path_inputs_fail_closed() {
    let f = Fixture::new();
    let mut r = inventory(&f.root).unwrap();
    apply_fstar_evidence(&mut r, &f.root, std::slice::from_ref(&f.log)).unwrap();
    assert!(!r.evidence_runs[0].accepted);
    let sidecar = format!("{}.evidence.json", f.log.display());
    for text in ["not JSON", "{\"unknown\": true}"] {
        fs::write(&sidecar, text).unwrap();
        let mut r = inventory(&f.root).unwrap();
        apply_fstar_evidence(&mut r, &f.root, std::slice::from_ref(&f.log)).unwrap();
        assert!(!r.evidence_runs[0].accepted);
    }
    let mut f = Fixture::new();
    fs::write(f.root.join("checked/input.txt"), "not Fstar").unwrap();
    f.receipt
        .checked
        .insert("checked/input.txt".into(), sha256(b"not Fstar"));
    f.reject("non-F* extraction input");
    let mut f = Fixture::new();
    fs::write(f.root.parent().unwrap().join("outside.patch"), "outside").unwrap();
    f.receipt
        .patches
        .insert("../outside.patch".into(), sha256(b"outside"));
    f.reject("outside root");
    let mut f = Fixture::new();
    f.receipt
        .patches
        .insert("missing.patch".into(), sha256(b"missing"));
    f.reject("No such file");
    let f = Fixture::new();
    fs::write(&f.log, "changed").unwrap();
    f.reject("log hash mismatch");
}

#[test]
fn file_options_and_cached_or_unlogged_results_cannot_bypass_checks() {
    for options in [
        "--already_cached *",
        "--verify_module false",
        "--unknown true",
        "--admit_smt_queries=true",
        "--lax",
    ] {
        let mut f = Fixture::new();
        let text =
            format!("module Hax_fixture\n#set-options \"{options}\"\nlet clean x : Tot u32 = x\n");
        f.replace("fresh/Hax_fixture.fst", &text);
        f.replace("checked/Hax_fixture.fst", &text);
        let r = f.run();
        assert!(r.evidence_runs[0].credited_units.is_empty(), "{options}");
        assert!(!r.escape_hatches.is_empty());
    }
    let f = Fixture::new();
    fs::write(f.root.join("checked/Hax_fixture.fst.checked"), "cached").unwrap();
    f.reject("cached F* artifact");
    let mut f = Fixture::new();
    fs::write(&f.log, "no proof output").unwrap();
    f.receipt.log_sha256 = sha256(b"no proof output");
    f.reject("matching F* success output");
    let mut f = Fixture::new();
    fs::create_dir_all(f.root.join("second")).unwrap();
    let bytes = fs::read(f.root.join("fresh/Hax_fixture.fst")).unwrap();
    fs::write(f.root.join("second/Hax_fixture.fst"), &bytes).unwrap();
    f.receipt
        .fresh
        .insert("second/Hax_fixture.fst".into(), sha256(&bytes));
    f.reject("ambiguous fresh extraction");
}

#[test]
fn failed_implementation_leaves_its_interface_trusted() {
    let mut f = Fixture::new();
    f.receipt.modules[0].outcome = "failed".into();
    f.receipt.modules[0].exit_code = 1;
    f.receipt.modules.push(FstarModule {
        file: "checked/Hax_fixture.fsti".into(),
        command: vec!["fstar.exe".into(), "checked/Hax_fixture.fsti".into()],
        exit_code: 0,
        outcome: "success".into(),
    });
    let r = f.run();
    assert!(r.evidence_runs[0].credited_units.is_empty());
    assert!(r
        .fstar_definitions
        .iter()
        .filter(|d| d.file.ends_with(".fsti"))
        .all(|d| d.classification == "trusted"));
}

#[test]
fn opaque_replacements_and_assumed_interfaces_stay_visible() {
    for attribute in ["opaque", "replace_body", "replace"] {
        let mut f = Fixture::new();
        let src = format!(
            "#[cfg_attr(any(), hax_lib::{attribute})]\npub fn clean(x: u32) -> u32 {{x}}\n"
        );
        fs::write(f.root.join("src/lib.rs"), &src).unwrap();
        f.receipt
            .files
            .insert("src/lib.rs".into(), sha256(src.as_bytes()));
        let r = f.run();
        assert_eq!(r.units[0].status, "trusted");
        assert!(!r
            .fstar_definitions
            .iter()
            .any(|d| d.rust_unit.is_some() && d.classification == "proven-extracted"));
        assert!(r.trusted_items.iter().any(|t| t.kind.contains(attribute)));
        assert!(r.evidence_runs[0].credited_units.is_empty());
    }
    let mut f = Fixture::new();
    let text = "module Hax_fixture\nassume val clean : Tot u32\n";
    f.replace("fresh/Hax_fixture.fsti", text);
    f.replace("checked/Hax_fixture.fsti", text);
    let r = f.run();
    assert!(r.evidence_runs[0].credited_units.is_empty());
    assert!(r
        .trusted_items
        .iter()
        .any(|t| t.kind.contains("assume val")));
    assert!(r.escape_hatches.iter().any(|e| e.kind == "assume"));
}
