//! Synthetic receipts below test reporting/gating policy, not mathematical validity.
use formal_coverage::{
    analyze_with, apply_evidence_many, gate, proof_ratio, sha256, Options, ProofRatioRange,
    Receipt, SizeMode,
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
const SOURCE: &str = r#"verus! {
fn run() -> (r: u64)
    ensures r == 1,
{
    1
}
proof fn lemma() {
    assert(true);
    assert(true);
    assert(true);
    assert(true);
    assert(true);
    assert(true);
}
mod tiny {
    fn second() -> (r: u64)
        ensures r == 2,
    {
        2
    }
}
}
"#;
fn fixture() -> PathBuf {
    let root = PathBuf::from(format!(
        "logs/v1.6/ratio-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname='ratio_fixture'\nversion='0.1.0'\nedition='2021'\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        root.join("Cargo.lock"),
        "version=4\n[[package]]\nname='ratio_fixture'\nversion='0.1.0'\n",
    )
    .unwrap();
    fs::write(root.join("src/lib.rs"), SOURCE).unwrap();
    let log = "verification results:: 3 verified, 0 errors\n";
    fs::write(root.join("verus.log"), log).unwrap();
    let receipt = Receipt {
        tool: "verus".into(),
        tool_version: "synthetic policy fixture".into(),
        command: vec![
            "verus".into(),
            "--crate-type".into(),
            "lib".into(),
            "src/lib.rs".into(),
        ],
        exit_code: 0,
        cfg: None,
        files: BTreeMap::from([("src/lib.rs".into(), sha256(SOURCE.as_bytes()))]),
        log_sha256: sha256(log.as_bytes()),
        harnesses: vec![],
        covered_units: BTreeMap::new(),
    };
    fs::write(
        root.join("verus.log.evidence.json"),
        serde_json::to_vec(&receipt).unwrap(),
    )
    .unwrap();
    root
}
fn report(
    root: &Path,
    config: &str,
    cli: Option<&str>,
    disabled: bool,
) -> anyhow::Result<formal_coverage::Report> {
    fs::write(root.join("formal-coverage.toml"), config)?;
    let mut r = analyze_with(
        root,
        &Options {
            size_mode: SizeMode::WholeCrate,
            proof_ratio_range: cli.map(str::parse).transpose()?,
            no_proof_ratio_gate: disabled,
            ..Options::default()
        },
    )?;
    apply_evidence_many(&mut r, root, &[root.join("verus.log")], &[])?;
    Ok(r)
}
#[test]
fn arithmetic_attribution_both_modes_and_bounded_column() {
    let root = fixture();
    let mut r = report(&root, "", None, false).unwrap();
    assert!(r.evidence_runs[0].accepted);
    let p = proof_ratio(&r);
    assert_eq!(
        (
            p.security_relevant.standalone_proof_sloc,
            p.security_relevant.inline_proof_sloc,
            p.security_relevant.proof_sloc,
            p.security_relevant.proven_runtime_sloc
        ),
        (8, 2, 10, 8)
    );
    assert_eq!(p.security_relevant.ratio, Some(1.25));
    assert_eq!(p.security_relevant.status, "within");
    assert_eq!(p.crates.len(), 1);
    assert_eq!(p.modules.len(), 2);
    assert_eq!(p.modules[0].module.as_deref(), Some("<unattributed>"));
    assert_eq!(p.modules[0].metrics.standalone_proof_sloc, 8);
    assert_eq!(p.modules[1].metrics.status, "below");
    assert!(gate(&r, 0., None, false).unwrap().is_empty());
    assert!(formal_coverage::markdown(&r).contains("Warning: module ratio_fixture::tiny"));
    assert_eq!(
        p.runtime_proven_modes
            .runtime
            .security_relevant
            .proven_runtime_sloc,
        10
    );
    assert_eq!(
        p.runtime_proven_modes
            .separate
            .security_relevant
            .proven_runtime_sloc,
        8
    );
    r.units
        .iter_mut()
        .find(|u| u.name == "second")
        .unwrap()
        .status = "proven-bounded".into();
    let p = proof_ratio(&r);
    assert_eq!(
        (
            p.security_relevant.proven_runtime_sloc,
            p.security_relevant.proven_bounded_runtime_sloc,
            p.security_relevant.ratio
        ),
        (4, 4, Some(2.5))
    );
    for u in &mut r.units {
        u.status = "unverified".into();
    }
    let p = proof_ratio(&r);
    assert_eq!(p.security_relevant.ratio, None);
    assert_eq!(p.security_relevant.status, "n/a");
    assert!(gate(&r, 0., None, false).unwrap().is_empty());
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn range_precedence_open_bounds_validation_and_gate() {
    let root = fixture();
    let config = "[proof_ratio]\nmin=2.0\nmax=3.0\nreason='reviewed cost budget'\n";
    let r = report(&root, config, None, false).unwrap();
    assert_eq!(r.proof_ratio_range.source, "config");
    assert!(formal_coverage::markdown(&r).contains("| proof_ratio | `range` | acceptance range | reviewed cost budget | first-party security-relevant | 0 | 0 | n/a |"));
    assert_eq!(
        r.proof_ratio_range.reason.as_deref(),
        Some("reviewed cost budget")
    );
    assert_eq!(proof_ratio(&r).security_relevant.status, "below");
    assert_eq!(gate(&r, 0., None, false).unwrap().len(), 1);
    for (range, status, pass) in [
        ("1:2", "within", true),
        ("2:", "below", false),
        (":1", "above", false),
        (":1.25", "within", true),
        ("1.25:", "within", true),
    ] {
        let r = report(&root, config, Some(range), false).unwrap();
        assert_eq!(r.proof_ratio_range.source, "cli");
        assert_eq!(proof_ratio(&r).security_relevant.status, status);
        assert_eq!(gate(&r, 0., None, false).unwrap().is_empty(), pass);
    }
    let r = report(&root, config, Some(":1"), true).unwrap();
    assert!(gate(&r, 0., None, false).unwrap().is_empty());
    assert!(!proof_ratio(&r).gate_enabled);
    let r = report(
        &root,
        &config.replace("reviewed cost budget", "TODO"),
        Some("1:2"),
        true,
    )
    .unwrap();
    assert!(gate(&r, 0., None, false)
        .unwrap()
        .iter()
        .any(|s| s.contains("TODO")));
    for config in [
        "[proof_ratio]\nmin=1.0\n",
        "[proof_ratio]\nmin=1.0\nreason=' '\n",
        "[proof_ratio]\nreason='valid'\n",
        "[proof_ratio]\nmin=3.0\nmax=2.0\nreason='valid'\n",
        "[proof_ratio]\nmin=nan\nreason='valid'\n",
        "[proof_ratio]\nmax=-1.0\nreason='valid'\n",
    ] {
        assert!(
            report(&root, config, Some("1:2"), false).is_err(),
            "{config}"
        );
    }
    for text in [":", "", "1", "1:2:3", "nan:2", "1:inf", "-1:2", "3:2"] {
        assert!(text.parse::<ProofRatioRange>().is_err(), "{text}");
    }
    for config in [
        "[proof_ratio]\nmin=1.0\nreason='valid'\n",
        "[proof_ratio]\nmax=2.0\nreason='valid'\n",
    ] {
        assert!(gate(
            &report(&root, config, None, false).unwrap(),
            0.,
            None,
            false
        )
        .unwrap()
        .is_empty());
    }
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn cli_records_options_and_gate_exit_status() {
    let root = fixture();
    for (args, success, source, status) in [
        (vec![], true, "default", "within"),
        (vec!["--proof-ratio-range", "2:"], false, "cli", "below"),
        (vec!["--proof-ratio-range", ":1"], false, "cli", "above"),
        (
            vec!["--proof-ratio-range", ":1", "--no-proof-ratio-gate"],
            true,
            "cli",
            "above",
        ),
    ] {
        let out = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
            .arg("gate")
            .arg(&root)
            .args([
                "--size-mode",
                "whole-crate",
                "--min-security-proven",
                "0",
                "--verus-log",
            ])
            .arg(root.join("verus.log"))
            .arg("--json")
            .arg(root.join("report.json"))
            .args(&args)
            .output()
            .unwrap();
        fs::write(
            root.join("cli.log"),
            [out.stdout.as_slice(), out.stderr.as_slice()].concat(),
        )
        .unwrap();
        assert_eq!(
            out.status.success(),
            success,
            "{args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let json: serde_json::Value =
            serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
        assert_eq!(json["proof_ratio"]["range"]["source"], source);
        assert_eq!(json["proof_ratio"]["status"], status);
        assert_eq!(json["proof_ratio"]["proven_runtime_sloc"], 8);
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ghost_separation_preserves_full_trusted_footprint_and_receipt_scope() {
    let root = fixture();
    let source = SOURCE.replacen("verus! {\n", "verus! {\n#[verifier::external_body]\nfn trusted(x: u64) -> (r: u64)\n    ensures r == x,\n{\n    x\n}\n", 1);
    fs::write(root.join("src/lib.rs"), &source).unwrap();
    let path = root.join("verus.log.evidence.json");
    let mut receipt: Receipt = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    receipt
        .files
        .insert("src/lib.rs".into(), sha256(source.as_bytes()));
    fs::write(path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    let mut previous = None;
    for ghost_lines in [
        formal_coverage::GhostLines::Runtime,
        formal_coverage::GhostLines::Separate,
    ] {
        let mut r = analyze_with(
            &root,
            &Options {
                size_mode: SizeMode::WholeCrate,
                ghost_lines,
                ..Options::default()
            },
        )
        .unwrap();
        apply_evidence_many(&mut r, &root, &[root.join("verus.log")], &[]).unwrap();
        assert!(r.evidence_runs[0].accepted);
        assert_eq!(r.trusted_items.len(), 1);
        assert_eq!(r.trusted_items[0].sloc, 6);
        assert_eq!(r.evidence_runs[0].trusted_sloc, 6);
        let t = r.units.iter().find(|u| u.name == "trusted").unwrap();
        assert_eq!(t.sloc + t.inline_proof_sloc, 6);
        assert_eq!(t.status, "trusted");
        let scope = serde_json::to_value(&r.evidence_runs).unwrap();
        if let Some(old) = previous {
            assert_eq!(scope, old);
        }
        previous = Some(scope);
    }
    fs::remove_dir_all(root).unwrap();
}
