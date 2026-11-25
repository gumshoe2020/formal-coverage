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

#[test]
fn split_queries_region_is_credited_but_escapes_inside_it_still_block_and_name_the_option() {
    let body = "let clean (x: u32) : Tot u32 = x\n";
    let credited = |text: &str| {
        let mut f = Fixture::new();
        f.replace("fresh/Hax_fixture.fst", text);
        f.replace("checked/Hax_fixture.fst", text);
        f.run()
    };
    let region = |inside: &str| {
        format!("module Hax_fixture\n#push-options \"--split_queries always --z3rlimit 50\"\n{inside}#pop-options\n")
    };
    let r = credited(&region(body));
    assert!(r.escape_hatches.is_empty(), "{:?}", r.escape_hatches);
    assert!(r.evidence_runs[0]
        .credited_units
        .iter()
        .any(|u| u.ends_with("clean")));

    let r = credited(&region(&format!("assume (True)\n{body}")));
    assert!(r.evidence_runs[0].credited_units.is_empty());
    assert!(r.escape_hatches.iter().any(|e| e.kind == "assume"));

    for (pragma, shown) in [
        (
            "--split_queries always --admit_smt_queries true",
            "--admit_smt_queries true",
        ),
        ("--split_queries always --lax", "--lax"),
        ("--split_queries maybe", "--split_queries maybe"),
    ] {
        let text = format!("module Hax_fixture\n#push-options \"{pragma}\"\n{body}#pop-options\n");
        let r = credited(&text);
        assert!(r.evidence_runs[0].credited_units.is_empty(), "{pragma}");
        let kind = format!("unsupported F* option: {shown}");
        assert!(
            r.escape_hatches.iter().any(|e| e.kind == kind),
            "{pragma}: {:?}",
            r.escape_hatches
        );
    }
}

fn configured_proof(f: &Fixture, config: &str, evidence: bool) -> Report {
    fs::write(f.root.join("formal-coverage.toml"), config).unwrap();
    fs::write(
        format!("{}.evidence.json", f.log.display()),
        serde_json::to_vec(&f.receipt).unwrap(),
    )
    .unwrap();
    let mut r = formal_coverage::analyze_with(
        &f.root,
        &formal_coverage::Options {
            size_mode: formal_coverage::SizeMode::WholeCrate,
            ..Default::default()
        },
    )
    .unwrap();
    if evidence {
        apply_fstar_evidence(&mut r, &f.root, std::slice::from_ref(&f.log)).unwrap();
    }
    r
}

#[test]
fn configured_patch_hunks_require_strict_accepted_module_and_list_exclusions() {
    let mut f = Fixture::new();
    fs::write(f.root.join("proof.patch"), "--- old/Hax_fixture.fst\n+++ new/Hax_fixture.fst\n@@ -1,1 +1,5 @@\n module Hax_fixture\n+(* comment *)\n+\n+let proof = 1\n+let more = 2 // code\n--- old/Absent.fst\n+++ new/Absent.fst\n@@ -1 +1 @@\n+let absent = 0\n").unwrap();
    let config = "[fstar.proof]\npatches=['proof.patch']\n";
    let p = formal_coverage::proof_ratio(&configured_proof(&f, config, true));
    assert_eq!(p.security_relevant.fstar_patch_sloc, 2);
    assert_eq!(p.all_first_party.fstar_patch_sloc, 2);
    assert_eq!(p.crates[0].metrics.fstar_patch_sloc, 2);
    assert_eq!(p.modules[0].metrics.fstar_patch_sloc, 2);
    assert_eq!(p.security_relevant.excluded_patch_hunks.len(), 1);
    assert!(p.security_relevant.excluded_patch_hunks[0]
        .reason
        .contains("absent"));
    let p = formal_coverage::proof_ratio(&configured_proof(&f, config, false));
    assert_eq!(p.security_relevant.fstar_patch_sloc, 0);
    assert_eq!(p.security_relevant.excluded_patch_hunks.len(), 2);
    assert!(p.security_relevant.excluded_patch_hunks[0]
        .reason
        .contains("no accepted"));
    for hazard in ["assume True", "admit ()", "#set-options \"--lax\""] {
        f.replace(
            "checked/Hax_fixture.fst",
            &format!("module Hax_fixture\nlet clean (x: u32) : Tot u32 = x\n{hazard}\n"),
        );
        let r = configured_proof(&f, config, true);
        assert!(r.evidence_runs[0].accepted);
        let p = formal_coverage::proof_ratio(&r);
        assert_eq!(p.security_relevant.fstar_patch_sloc, 0);
        assert!(p.security_relevant.excluded_patch_hunks[0]
            .reason
            .contains("hazard"));
    }
    // Existing grammar rejects lax command lines: rejected receipts credit zero.
    f.receipt.modules[0].command.push("--lax".into());
    let r = configured_proof(&f, config, true);
    assert!(!r.evidence_runs[0].accepted);
    assert_eq!(
        formal_coverage::proof_ratio(&r)
            .security_relevant
            .fstar_patch_sloc,
        0
    );
}

#[test]
fn interface_only_patches_and_generated_files_are_not_proof() {
    let mut f = Fixture::new();
    fs::write(
        f.root.join("proof.patch"),
        "--- old/Hax_fixture.fsti\n+++ new/Hax_fixture.fsti\n@@ -1 +1 @@\n+val clean : Tot u32\n",
    )
    .unwrap();
    let config = "[fstar.proof]\npatches=['proof.patch']\nfiles=['checked/Hax_fixture.fsti']\n";
    let r = configured_proof(&f, config, true);
    assert_eq!(
        formal_coverage::proof_ratio(&r)
            .security_relevant
            .fstar_file_sloc,
        0
    );
    assert!(r
        .fstar_proof
        .inputs
        .iter()
        .any(|i| i.reason.as_deref() == Some("generated extraction")));
    fs::remove_file(f.root.join("checked/Hax_fixture.fst")).unwrap();
    f.receipt.checked.remove("checked/Hax_fixture.fst");
    f.receipt.modules = vec![FstarModule {
        file: "checked/Hax_fixture.fsti".into(),
        command: vec!["fstar.exe".into(), "checked/Hax_fixture.fsti".into()],
        exit_code: 0,
        outcome: "success".into(),
    }];
    let p = formal_coverage::proof_ratio(&configured_proof(&f, config, true));
    assert_eq!(p.security_relevant.fstar_patch_sloc, 0);
    assert!(p.security_relevant.excluded_patch_hunks[0]
        .reason
        .contains("interface-only"));
}

#[test]
fn handwritten_files_need_matching_verified_bytes_and_models_stay_separate() {
    let mut f = Fixture::new();
    let text = "module Lemmas\n(* multi\n comment *)\nlet lemma (_:unit) : Tot unit = ()\n";
    f.replace("checked/Lemmas.fst", text);
    fs::write(f.root.join("Lemmas.fst"), text).unwrap();
    fs::write(
        f.root.join("Models.fsti"),
        "module Models\n// comment\nassume val axiom : bool\n",
    )
    .unwrap();
    f.receipt.modules.push(FstarModule {
        file: "checked/Lemmas.fst".into(),
        command: vec!["fstar.exe".into(), "checked/Lemmas.fst".into()],
        exit_code: 0,
        outcome: "success".into(),
    });
    fs::write(
        &f.log,
        "Verified module: Hax_fixture\nVerified module: Lemmas\n",
    )
    .unwrap();
    f.receipt.log_sha256 = sha256(&fs::read(&f.log).unwrap());
    let config = "[fstar.proof]\nfiles=['Lemmas.fst']\nmodels=['Models.fsti']\n";
    let no_receipt = formal_coverage::proof_ratio(&configured_proof(&f, config, false));
    assert_eq!(no_receipt.security_relevant.fstar_file_sloc, 0);
    assert_eq!(no_receipt.security_relevant.fstar_model_sloc, 2);
    let r = configured_proof(&f, config, true);
    assert!(r.evidence_runs[0].accepted);
    let p = formal_coverage::proof_ratio(&r);
    assert_eq!(p.security_relevant.fstar_file_sloc, 2);
    assert_eq!(p.security_relevant.fstar_model_sloc, 2);
    assert_eq!(
        p.security_relevant.proof_sloc,
        p.security_relevant.standalone_proof_sloc + p.security_relevant.inline_proof_sloc + 2
    );
    assert_eq!(
        p.crates
            .iter()
            .map(|c| c.metrics.fstar_file_sloc)
            .sum::<usize>(),
        2
    );
    assert_eq!(
        p.modules
            .iter()
            .map(|c| c.metrics.fstar_model_sloc)
            .sum::<usize>(),
        2
    );
    fs::write(
        f.root.join("Lemmas.fst"),
        "module Lemmas\nlet changed = 1\n",
    )
    .unwrap();
    let r = configured_proof(&f, config, true);
    assert_eq!(
        formal_coverage::proof_ratio(&r)
            .security_relevant
            .fstar_file_sloc,
        0
    );
    assert!(r
        .fstar_proof
        .inputs
        .iter()
        .any(|i| i.reason.as_deref() == Some("file differs from accepted receipt")));
}

#[test]
fn missing_globs_and_unknown_fstar_config_fields_are_errors() {
    let f = Fixture::new();
    for config in [
        "[fstar.proof]\npatches=['absent.patch']\n",
        "[fstar.proof]\nfiles=['absent*.fst']\n",
        "[fstar.proof]\nmodels=['absent/*.fsti']\n",
        "[fstar.proof]\nunknown=[]\n",
        "[fstar]\nunknown=[]\n",
    ] {
        fs::write(f.root.join("formal-coverage.toml"), config).unwrap();
        assert!(
            formal_coverage::analyze_with(
                &f.root,
                &formal_coverage::Options {
                    size_mode: formal_coverage::SizeMode::WholeCrate,
                    ..Default::default()
                }
            )
            .is_err(),
            "{config}"
        );
    }
}

#[test]
fn schema_two_proof_inputs_survive_config_migration() {
    let f = Fixture::new();
    fs::write(
        f.root.join("model.fsti"),
        "module Model\nassume val axiom : bool\n",
    )
    .unwrap();
    let config = "security=['src/lib.rs']\n[fstar.proof]\nmodels=['model.fsti']\n";
    assert_eq!(
        formal_coverage::proof_ratio(&configured_proof(&f, config, false))
            .security_relevant
            .fstar_model_sloc,
        2
    );
    let migrated = formal_coverage::migrate_config(config).unwrap();
    assert_eq!(
        formal_coverage::proof_ratio(&configured_proof(&f, &migrated, false))
            .security_relevant
            .fstar_model_sloc,
        2
    );
}

#[test]
fn credited_cfg_hax_proof_unit_is_verified_and_retains_receipt_credit() {
    let mut f = Fixture::new();
    let original = fs::read_to_string(f.root.join("src/lib.rs")).unwrap();
    let source = format!("#[cfg(hax)]\n{original}");
    fs::write(f.root.join("src/lib.rs"), &source).unwrap();
    f.receipt
        .files
        .insert("src/lib.rs".into(), sha256(source.as_bytes()));
    fs::write(
        format!("{}.evidence.json", f.log.display()),
        serde_json::to_vec(&f.receipt).unwrap(),
    )
    .unwrap();
    let mut r = formal_coverage::analyze_with(
        &f.root,
        &formal_coverage::Options {
            size_mode: formal_coverage::SizeMode::WholeCrate,
            ..Default::default()
        },
    )
    .unwrap();
    apply_fstar_evidence(&mut r, &f.root, std::slice::from_ref(&f.log)).unwrap();
    let u = r.units.iter().find(|u| u.name == "clean").unwrap();
    assert_eq!(u.category, formal_coverage::Category::ProofCode);
    assert_eq!(u.status, "verified");
    assert!(r.evidence_runs[0].credited_units.contains(&u.id));
    formal_coverage::gate(&r, 0., None, false).unwrap();
}

fn trust_fixture(source: &str, config: &str) -> Report {
    let mut f = Fixture::new();
    fs::write(f.root.join("src/lib.rs"), source).unwrap();
    f.receipt
        .files
        .insert("src/lib.rs".into(), sha256(source.as_bytes()));
    configured_proof(&f, config, true)
}

#[test]
fn backend_trust_is_visible_without_demoting_fstar_evidence() {
    for attribute in [
        "hax_lib::proverif::replace(\"...\")",
        "cfg_attr(feature = \"hax-pv\", proverif::replace(\"...\"))",
        "hax_lib::coq::opaque",
        "ssprove::replace_body(\"(0)\")",
    ] {
        let r = trust_fixture(
            &format!("#[{attribute}]\npub fn clean(x: u32) -> u32 {{x}}\n"),
            "",
        );
        assert_eq!(r.units[0].status, "proven-extracted", "{attribute}");
        assert_eq!(r.non_fstar_trust.len(), 1, "{attribute}");
        let item = &r.non_fstar_trust[0];
        assert_eq!(item.file, "src/lib.rs");
        assert_eq!(item.line, 1);
        assert_eq!(item.affected_units, vec![r.units[0].id.clone()]);
        assert_eq!(item.sloc, r.units[0].sloc);
        assert!(item.kind.contains("non-F* trust:"));
        assert!(item.kind.contains(if attribute.contains("coq") {
            "coq"
        } else if attribute.contains("ssprove") {
            "ssprove"
        } else {
            "proverif"
        }));
        assert!(r
            .trusted_items
            .iter()
            .all(|t| !t.kind.contains("syntactic")));
        assert!(formal_coverage::markdown(&r).contains("1 sites ("));
    }
}

#[test]
fn only_declared_plain_inactive_features_remove_fstar_trust() {
    let config =
        "[fstar.extraction]\ninactive_features=['hax-pv']\nreason='F* driver excludes hax-pv'\n";
    for (attribute, cfg, trusted) in [
        (
            "cfg_attr(feature = \"hax-pv\", hax_lib::opaque)",
            config,
            false,
        ),
        ("cfg_attr(feature = \"hax-pv\", hax_lib::opaque)", "", true),
        (
            "cfg_attr(all(feature = \"hax-pv\", test), hax_lib::opaque)",
            config,
            true,
        ),
        (
            "cfg_attr(any(feature = \"hax-pv\"), hax_lib::opaque)",
            config,
            true,
        ),
        (
            "cfg_attr(not(feature = \"hax-pv\"), hax_lib::opaque)",
            config,
            true,
        ),
        (
            "cfg_attr(feature = \"other\", hax_lib::opaque)",
            config,
            true,
        ),
        ("hax_lib::opaque", config, true),
        ("hax_lib::fstar::replace(\"...\")", config, true),
        ("fstar::replace_body(\"...\")", config, true),
        ("unknown::replace(\"...\")", config, true),
        ("replace", config, true),
    ] {
        let r = trust_fixture(
            &format!("use hax_lib;\n#[{attribute}]\npub fn clean(x: u32) -> u32 {{x}}\n"),
            cfg,
        );
        assert_eq!(
            r.units[0].status,
            if trusted {
                "trusted"
            } else {
                "proven-extracted"
            },
            "{attribute}"
        );
        assert_eq!(
            r.non_fstar_trust.len(),
            usize::from(!trusted),
            "{attribute}"
        );
        assert_eq!(
            r.trusted_items.iter().any(|t| t.kind.contains("syntactic")),
            trusted,
            "{attribute}"
        );
        if !trusted {
            assert!(r.non_fstar_trust[0]
                .kind
                .contains("inactive feature hax-pv"));
            assert!(formal_coverage::markdown(&r).contains("F* driver excludes hax-pv"));
            let json = serde_json::to_value(&r).unwrap();
            assert_eq!(json["fstar_extraction"]["inactive_features"][0], "hax-pv");
        }
    }
}

#[test]
fn method_and_inherited_scope_use_the_same_backend_rules() {
    for (scope, attribute, trusted) in [
        (false, "hax_lib::proverif::replace_body(\"(0)\")", false),
        (true, "hax_lib::proverif::replace(\"...\")", false),
        (true, "hax_lib::opaque", true),
        (
            true,
            "cfg_attr(feature = \"hax-pv\", hax_lib::opaque)",
            false,
        ),
    ] {
        let source = if scope {
            format!(
                "struct Foo;\n#[{attribute}]\nimpl Foo {{\npub fn clean(x: u32) -> u32 {{x}}\n}}\n"
            )
        } else {
            format!(
                "struct Foo;\nimpl Foo {{\n#[{attribute}]\npub fn clean(x: u32) -> u32 {{x}}\n}}\n"
            )
        };
        let r = trust_fixture(
            &source,
            "[fstar.extraction]\ninactive_features=['hax-pv']\nreason='driver excludes hax-pv'\n",
        );
        assert_eq!(
            r.units[0].status,
            if trusted { "trusted" } else { "unverified" }
        );
        let items = if trusted {
            &r.trusted_items
        } else {
            &r.non_fstar_trust
        };
        assert!(items
            .iter()
            .any(|t| t.affected_units.contains(&r.units[0].id)));
        assert_eq!(r.non_fstar_trust.len(), usize::from(!trusted));
    }
    // Module-level scope, with an unrelated sibling opaque site, must not spill trust.
    let r = trust_fixture("#[hax_lib::proverif::replace(\"...\")]\nmod m { pub fn clean() {} }\n#[hax_lib::opaque]\nmod n { pub fn other() {} }\n", "");
    assert_eq!(
        r.units.iter().find(|u| u.name == "clean").unwrap().status,
        "unverified"
    );
    assert_eq!(
        r.units.iter().find(|u| u.name == "other").unwrap().status,
        "trusted"
    );
    assert_eq!(r.non_fstar_trust[0].affected_units.len(), 1);
}

#[test]
fn inactive_feature_declaration_requires_a_reason_and_survives_migration() {
    let f = Fixture::new();
    for reason in ["", "reason=''\n", "reason='  '\n"] {
        fs::write(
            f.root.join("formal-coverage.toml"),
            format!("[fstar.extraction]\ninactive_features=['hax-pv']\n{reason}"),
        )
        .unwrap();
        let error = formal_coverage::analyze_with(
            &f.root,
            &formal_coverage::Options {
                size_mode: formal_coverage::SizeMode::WholeCrate,
                ..Default::default()
            },
        )
        .unwrap_err();
        assert!(error.to_string().contains("requires a non-empty reason"));
    }
    let config = "security=['src/lib.rs']\n[fstar.extraction]\ninactive_features=['hax-pv']\nreason='F* driver excludes hax-pv'\n";
    let migrated = formal_coverage::migrate_config(config).unwrap();
    assert!(migrated.contains("[fstar.extraction]"));
    assert_eq!(
        configured_proof(&f, &migrated, false)
            .fstar_extraction
            .inactive_features,
        vec!["hax-pv"]
    );
}

#[test]
fn non_fstar_annotations_do_not_replace_evidence_requirements() {
    let mut f = Fixture::new();
    let source = "#[hax_lib::proverif::replace(\"...\")]\npub fn clean(x:u32)->u32{x}\n";
    fs::write(f.root.join("src/lib.rs"), source).unwrap();
    f.receipt
        .files
        .insert("src/lib.rs".into(), sha256(source.as_bytes()));
    f.receipt.modules[0].outcome = "failed".into();
    f.receipt.modules[0].exit_code = 1;
    let r = configured_proof(&f, "", true);
    assert_ne!(r.units[0].status, "proven-extracted");
    assert_eq!(r.non_fstar_trust.len(), 1);
    assert!(r.evidence_runs[0].credited_units.is_empty());
}

#[test]
fn inherent_method_mapping_uses_normal_evidence_and_trust_classification() {
    for (attribute, status) in [
        ("", "proven-extracted"),
        (
            "#[hax_lib::proverif::replace_body(\"(0)\")]",
            "proven-extracted",
        ),
        ("#[hax_lib::opaque]", "trusted"),
    ] {
        let mut f = Fixture::new();
        let source =
            format!("struct Foo;\nimpl Foo {{\n{attribute}\npub fn bar(x: u32) -> u32 {{x}}\n}}\n");
        fs::write(f.root.join("src/lib.rs"), &source).unwrap();
        f.receipt
            .files
            .insert("src/lib.rs".into(), sha256(source.as_bytes()));
        for dir in ["fresh", "checked"] {
            f.replace(
                &format!("{dir}/Hax_fixture.fst"),
                "module Hax_fixture\nlet impl_Foo__bar (x: u32) : Tot u32 = x\n",
            );
            f.replace(
                &format!("{dir}/Hax_fixture.fsti"),
                "module Hax_fixture\nval impl_Foo__bar : u32 -> Tot u32\n",
            );
        }
        let r = configured_proof(&f, "", true);
        assert_eq!(r.units[0].id, "hax_fixture::Foo::bar");
        assert_eq!(r.units[0].status, status);
        assert!(r
            .fstar_definitions
            .iter()
            .filter(|d| d.name.ends_with("impl_Foo__bar"))
            .all(|d| d.rust_unit.as_deref() == Some("hax_fixture::Foo::bar")
                && d.classification == status));
        assert_eq!(
            r.non_fstar_trust.len(),
            usize::from(attribute.contains("proverif"))
        );
    }
}

#[test]
fn uppercase_value_mapping_uses_normal_evidence_classification() {
    for case in [
        "success",
        "failed",
        "lax",
        "stale",
        "no-body",
        "unclassified",
        "opaque",
    ] {
        let mut f = Fixture::new();
        let attribute = if case == "opaque" {
            "#[hax_lib::opaque]\n"
        } else {
            ""
        };
        let source = format!("{attribute}pub fn XTR(x: u32) -> u32 {{x}}\n");
        fs::write(f.root.join("src/lib.rs"), &source).unwrap();
        f.receipt
            .files
            .insert("src/lib.rs".into(), sha256(source.as_bytes()));
        for dir in ["fresh", "checked"] {
            let option = if case == "lax" {
                "#set-options \"--lax\"\n"
            } else {
                ""
            };
            let declaration = if case == "no-body" {
                "val v_XTR : u32 -> Tot u32"
            } else if case == "unclassified" {
                "let v_XTR x = x"
            } else {
                "let v_XTR (x: u32) : Tot u32 = x"
            };
            f.replace(
                &format!("{dir}/Hax_fixture.fst"),
                &format!("module Hax_fixture\n{option}{declaration}\n"),
            );
            let contract = if case == "unclassified" {
                "u32 -> u32"
            } else {
                "u32 -> Tot u32"
            };
            f.replace(
                &format!("{dir}/Hax_fixture.fsti"),
                &format!("module Hax_fixture\nval v_XTR : {contract}\n"),
            );
        }
        if case == "failed" {
            f.receipt.modules[0].outcome = "failed".into();
            f.receipt.modules[0].exit_code = 1;
        }
        if case == "stale" {
            f.replace(
                "checked/Hax_fixture.fst",
                "module Hax_fixture\nlet v_XTR (x: u32) : Tot u32 = 0\n",
            );
        }
        let r = configured_proof(&f, "", true);
        assert!(r.evidence_runs[0].accepted, "{case}");
        assert!(
            r.fstar_definitions
                .iter()
                .all(|d| d.rust_unit.as_deref() == Some("hax_fixture::XTR")),
            "{case}"
        );
        let implementation = r
            .fstar_definitions
            .iter()
            .find(|d| d.file.ends_with(".fst"))
            .unwrap();
        let expected = match case {
            "success" => "proven-extracted",
            "failed" | "unclassified" => "unverified",
            "lax" => "hazard",
            _ => "trusted",
        };
        assert_eq!(implementation.classification, expected, "{case}");
        assert_eq!(
            r.units[0].status == "proven-extracted",
            case == "success",
            "{case}"
        );
        assert_eq!(
            !r.evidence_runs[0].credited_units.is_empty(),
            case == "success",
            "{case}"
        );
    }
}

#[test]
fn numbered_methods_receive_body_credit_and_interface_only_trust() {
    let mut f = Fixture::new();
    let source = "struct Foo;\nimpl Foo { pub fn body(x: u32) -> u32 { x } }\nimpl Foo { pub fn external(x: u32) -> u32 { x } }\n";
    fs::write(f.root.join("src/lib.rs"), source).unwrap();
    f.receipt
        .files
        .insert("src/lib.rs".into(), sha256(source.as_bytes()));
    for dir in ["fresh", "checked"] {
        f.replace(
            &format!("{dir}/Hax_fixture.fst"),
            "module Hax_fixture\nlet impl_99__body (x: t_Foo) : Tot u32 = 0\n",
        );
        f.replace(&format!("{dir}/Hax_fixture.fsti"),
            "module Hax_fixture\nval impl_99__body : t_Foo -> Tot u32\nval impl_3__external : t_Foo -> Tot u32\nval impl_4__missing : t_Foo -> Tot u32\n");
    }
    let r = f.run();
    assert!(r.evidence_runs[0].accepted);
    assert_eq!(
        r.units.iter().find(|u| u.name == "body").unwrap().status,
        "proven-extracted"
    );
    assert_eq!(
        r.units
            .iter()
            .find(|u| u.name == "external")
            .unwrap()
            .status,
        "trusted"
    );
    let missing = r
        .fstar_definitions
        .iter()
        .find(|d| d.name.ends_with("impl_4__missing"))
        .unwrap();
    assert!(missing.rust_unit.is_none());
    assert!(missing
        .mapping_reason
        .as_ref()
        .unwrap()
        .contains("0 qualifying"));
    assert!(formal_coverage::markdown(&r).contains("0 qualifying"));
    assert!(serde_json::to_string(&r)
        .unwrap()
        .contains("mapping_reason"));
}

#[test]
fn v191_forms_with_a_hash_bound_synthetic_crate_and_receipt() {
    let mut f = Fixture::new();
    f.receipt.fstar_version = "2025.10.06".into();
    let source = r#"
pub fn good_2() {}
pub fn wrong2() {}
#[cfg(feature="on")] pub fn twin() {}
#[cfg(not(feature="on"))] pub fn twin() {}
pub struct Foo;
pub struct Other;
pub trait Convert<T> { fn convert(); }
impl Convert<u8> for Foo { fn convert() {} }
pub trait Decode<T> { fn decode(); }
impl Decode<u8> for Foo { fn decode() {} }
impl Foo { pub fn new() {} }
"#;
    fs::write(f.root.join("src/lib.rs"), source).unwrap();
    f.receipt
        .files
        .insert("src/lib.rs".into(), sha256(source.as_bytes()));
    let implementation = r#"module Hax_fixture
let good_2_ () : Tot unit = ()
let wrong2_ () : Tot unit = ()
let twin () : Tot unit = ()
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: t_Convert t_Foo u8 = { f_convert = fun x -> x }
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1: t_Convert t_Other u8 = { f_convert = fun x -> x }
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_2: t_Convert t_Foo u8 = { f_convert_pre = true; f_convert_post = true }
let impl__new (x: t_Foo) : Tot t_Foo = x
let impl__missing (x: t_Foo) : Tot t_Foo = x
"#;
    // The empty-segment group is indivisible: the missing method refuses both.
    let interface = r#"module Hax_fixture
val good_2_ : Tot unit
val impl__new : t_Foo -> Tot t_Foo
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3: t_Decode t_Foo u8 = { f_decode = fun x -> x }
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_4: t_Decode t_Other u8 = { f_decode = fun x -> x }
"#;
    for dir in ["fresh", "checked"] {
        f.replace(&format!("{dir}/Hax_fixture.fst"), implementation);
        f.replace(&format!("{dir}/Hax_fixture.fsti"), interface);
    }
    let r = f.run();
    assert!(r.evidence_runs[0].accepted);
    let definition = |suffix: &str| {
        r.fstar_definitions
            .iter()
            .find(|d| d.name == format!("Hax_fixture.{suffix}"))
            .unwrap()
    };
    assert_eq!(definition("good_2_").classification, "proven-extracted");
    assert!(definition("wrong2_").rust_unit.is_none());
    assert!(definition("twin")
        .mapping_reason
        .as_ref()
        .unwrap()
        .contains("not recorded"));
    assert_eq!(definition("impl").classification, "proven-extracted");
    assert!(definition("impl_1").rust_unit.is_none());
    assert!(definition("impl_2").rust_unit.is_none());
    assert_eq!(definition("impl_3").classification, "trusted");
    assert!(definition("impl_3").rust_unit.is_some());
    assert!(definition("impl_4").rust_unit.is_none());
    assert!(definition("impl__new").rust_unit.is_none());
    assert!(definition("impl__missing").rust_unit.is_none());
    assert!(formal_coverage::markdown(&r).contains("not recorded"));
    f.receipt.fstar_version = "uncanaried".into();
    let refused = f.run();
    assert!(refused
        .fstar_definitions
        .iter()
        .find(|d| d.name == "Hax_fixture.impl_3")
        .unwrap()
        .rust_unit
        .is_none());
    f.receipt.fstar_version = "2025.10.06".into();
    let implementation = implementation.replace("let impl__missing (x: t_Foo) : Tot t_Foo = x", "");
    for dir in ["fresh", "checked"] {
        f.replace(&format!("{dir}/Hax_fixture.fst"), &implementation);
    }
    let r = f.run();
    assert_eq!(
        r.fstar_definitions
            .iter()
            .find(|d| d.name == "Hax_fixture.impl__new")
            .unwrap()
            .classification,
        "proven-extracted"
    );
}

#[test]
fn v191_fallbacks_preserve_existing_owners_and_refuse_failed_or_hazard_modules() {
    let mut f = Fixture::new();
    let source = "pub struct Foo; impl Foo { pub fn new() {} }";
    fs::write(f.root.join("src/lib.rs"), source).unwrap();
    f.receipt
        .files
        .insert("src/lib.rs".into(), sha256(source.as_bytes()));
    let implementation =
        "module Hax_fixture\nlet impl_Foo__new (x:t_Foo) = x\nlet impl__new (x:t_Foo) = x\n";
    let interface="module Hax_fixture\nval impl_Foo__new : t_Foo -> Tot t_Foo\nval impl__new : t_Foo -> Tot t_Foo\n";
    for dir in ["fresh", "checked"] {
        f.replace(&format!("{dir}/Hax_fixture.fst"), implementation);
        f.replace(&format!("{dir}/Hax_fixture.fsti"), interface);
    }
    let r = f.run();
    let d = r
        .fstar_definitions
        .iter()
        .find(|d| d.name == "Hax_fixture.impl__new" && d.file.ends_with(".fst"))
        .unwrap();
    assert!(d.rust_unit.is_none());
    assert!(d.mapping_reason.as_ref().unwrap().contains("injectivity"));
    assert_eq!(
        r.units.iter().find(|u| u.name == "new").unwrap().status,
        "proven-extracted"
    );
    let implementation = "module Hax_fixture\nlet impl__new (x:t_Foo) = x\n";
    for dir in ["fresh", "checked"] {
        f.replace(&format!("{dir}/Hax_fixture.fst"), implementation);
    }
    f.receipt.modules[0].outcome = "failed".into();
    f.receipt.modules[0].exit_code = 1;
    let r = f.run();
    assert!(r
        .fstar_definitions
        .iter()
        .filter(|d| d.name == "Hax_fixture.impl__new")
        .all(|d| d.rust_unit.is_none()));
    f.receipt.modules[0].outcome = "success".into();
    f.receipt.modules[0].exit_code = 0;
    for dir in ["fresh", "checked"] {
        f.replace(
            &format!("{dir}/Hax_fixture.fst"),
            &format!("{implementation}assume val danger: unit\n"),
        );
    }
    let r = f.run();
    assert!(r
        .fstar_definitions
        .iter()
        .filter(|d| d.name == "Hax_fixture.impl__new")
        .all(|d| d.rust_unit.is_none()));
}
