use formal_coverage::{analyze, gate, markdown, summarize, Escape};
use std::{path::Path, process::Command};
#[test]
fn weighted_reports_keep_both_groups() {
    let mut r = analyze(Path::new("fixtures/inventory"), None).unwrap();
    r.units[0].status = "proven".into();
    r.units[1].security = false;
    let s = summarize(&r);
    assert_eq!(s.all.security_relevant.sloc, 12);
    assert_eq!(s.all.everything_else.sloc, 1);
    assert_eq!(s.all.security_relevant.percent("proven"), 25.);
    let md = markdown(&r);
    assert!(md.contains("SECURITY-RELEVANT") && md.contains("EVERYTHING ELSE"));
    assert!(md.contains("First-party") && md.contains("Third-party"));
    let json = serde_json::to_value(&s).unwrap();
    assert!(
        json["all"].get("security_relevant").is_some()
            && json["all"].get("everything_else").is_some()
    );
}
#[test]
fn all_gate_conditions() {
    let mut r = analyze(Path::new("fixtures/inventory"), None).unwrap();
    assert!(gate(&r, 1., None, false)
        .unwrap()
        .iter()
        .any(|s| s.contains("below")));
    assert!(gate(&r, 0., None, false).unwrap().is_empty());
    let mut baseline = r.clone();
    baseline.units[0].status = "proven".into();
    assert!(gate(&r, 0., Some(&baseline), false)
        .unwrap()
        .iter()
        .any(|s| s.contains("dropped")));
    r.escape_hatches.push(Escape {
        affected_units: vec![],
        unit: "fixture".into(),
        file: "src/lib.rs".into(),
        line: 2,
        kind: "assume".into(),
        security: true,
        first_party: true,
    });
    assert!(gate(&r, 0., Some(&baseline), true)
        .unwrap()
        .iter()
        .any(|s| s.contains("new escape")));
    assert!(gate(&r, f64::NAN, None, false).is_err());
    assert!(gate(&r, 101., None, false).is_err());
    assert!(gate(&r, 0., None, true).is_err());
}
#[test]
fn cli_gate_requires_at_least_one_explicit_threshold() {
    let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args(["gate", "fixtures/inventory"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("required arguments"), "{error}");
    assert!(error.contains("--min-security-proven"), "{error}");
    assert!(
        error.contains("--min-first-party-security-proven"),
        "{error}"
    );
    assert!(output.stdout.is_empty());

    // Explicit zero is valid; positive thresholds must still be enforced.
    for (thresholds, expected_code) in [
        (vec!["--min-security-proven", "0"], 0),
        (vec!["--min-first-party-security-proven", "0"], 0),
        (
            vec![
                "--min-security-proven",
                "0",
                "--min-first-party-security-proven",
                "0",
            ],
            0,
        ),
        (vec!["--min-security-proven", "1"], 1),
        (vec!["--min-first-party-security-proven", "1"], 1),
        (
            vec![
                "--min-security-proven",
                "1",
                "--min-first-party-security-proven",
                "0",
            ],
            1,
        ),
        (
            vec![
                "--min-security-proven",
                "0",
                "--min-first-party-security-proven",
                "1",
            ],
            1,
        ),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
            .args(["gate", "fixtures/inventory"])
            .args(&thresholds)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(expected_code),
            "{thresholds:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn cli_exit_and_artifacts() {
    let dir = format!("logs/report-test-{}", std::process::id());
    std::fs::create_dir_all(&dir).unwrap();
    let json = format!("{dir}/report.json");
    let md = format!("{dir}/report.md");
    let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args([
            "report",
            "fixtures/inventory",
            "--json",
            &json,
            "--markdown",
            &md,
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let r: formal_coverage::Report =
        serde_json::from_slice(&std::fs::read(&json).unwrap()).unwrap();
    assert_eq!(r.units.len(), 4);
    let text = std::fs::read_to_string(md).unwrap();
    assert!(text.contains("SECURITY-RELEVANT") && text.contains("EVERYTHING ELSE"));
    let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args([
            "gate",
            "fixtures/inventory",
            "--min-security-proven",
            "1",
            "--baseline",
            &json,
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args([
            "gate",
            "fixtures/inventory",
            "--min-security-proven",
            "0",
            "--baseline",
            &json,
            "--no-new-escape-hatches",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
}
#[test]
fn unknown_dependency_size_fails_closed() {
    let mut r = analyze(Path::new("fixtures/classification"), None).unwrap();
    r.dependencies[0].size_known = false;
    assert!(gate(&r, 0., None, false)
        .unwrap()
        .iter()
        .any(|s| s.contains("incomplete")));
}

#[test]
fn missing_lockfile_never_hides_dependency_weight() {
    let root = std::path::PathBuf::from(format!("logs/missing-lock-{}", std::process::id()));
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("Cargo.toml"),"[package]\nname=\"no-lock\"\nversion=\"0.1.0\"\n[dependencies]\nserde=\"1\"\n[workspace]\n").unwrap();
    std::fs::write(root.join("src/lib.rs"), "fn entry() {}\n").unwrap();
    let r = formal_coverage::analyze(&root, None).unwrap();
    assert!(!r.dependency_inventory_complete);
    assert!(formal_coverage::markdown(&r).contains("INCOMPLETE"));
    assert!(formal_coverage::gate(&r, 0., None, false)
        .unwrap()
        .iter()
        .any(|s| s.contains("inventory")));
}

#[test]
fn baseline_schema_change_requires_explicit_acknowledgment() {
    let r = analyze(Path::new("fixtures/inventory"), None).unwrap();
    assert_eq!(r.schema, 2);
    let mut old = serde_json::to_value(&r).unwrap();
    old["schema"] = 1.into();
    for u in old["units"].as_array_mut().unwrap() {
        u.as_object_mut().unwrap().remove("category");
        u["status"] = "proven".into();
    }
    let dir = format!("logs/schema-test-{}", std::process::id());
    std::fs::create_dir_all(&dir).unwrap();
    let baseline = format!("{dir}/v1.json");
    std::fs::write(&baseline, serde_json::to_vec(&old).unwrap()).unwrap();
    for allow in [false, true] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_formal-coverage"));
        command.args([
            "gate",
            "fixtures/inventory",
            "--min-security-proven",
            "0",
            "--baseline",
            &baseline,
        ]);
        if allow {
            command.arg("--allow-schema-change");
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.success(), allow);
        if !allow {
            let error = String::from_utf8_lossy(&output.stderr);
            assert!(error.contains("baseline schema 1") && error.contains("--allow-schema-change"));
        }
    }
    let old_report = serde_json::from_value(old).unwrap();
    let mut current = r;
    current.escape_hatches.push(Escape {
        affected_units: vec![],
        unit: "fixture".into(),
        file: "src/lib.rs".into(),
        line: 1,
        kind: "assume".into(),
        security: true,
        first_party: true,
    });
    let failures =
        formal_coverage::gate_with_schema_change(&current, 1., Some(&old_report), true, true)
            .unwrap();
    assert!(failures.iter().any(|f| f.contains("new escape")));
    assert!(failures.iter().any(|f| f.contains("below")));
}

#[test]
fn first_party_headline_and_threshold_are_independent_of_combined() {
    let mut r = analyze(Path::new("fixtures/classification"), None).unwrap();
    r.units.iter_mut().for_each(|u| u.status = "proven".into());
    let s = summarize(&r);
    let first = &s.first_party.security_relevant;
    assert_eq!(first.percent("proven"), 100.);
    assert!(s.all.security_relevant.percent("proven") < 100.);
    let md = markdown(&r);
    assert!(md
        .lines()
        .next()
        .unwrap()
        .starts_with("SECURITY-RELEVANT First-party"));
    assert!(md.lines().next().unwrap().contains("proven 100.00%"));
    for side in ["SECURITY-RELEVANT", "EVERYTHING ELSE"] {
        for origin in ["First-party", "Third-party (runtime)", "Combined"] {
            assert!(md.contains(&format!("{side} {origin}")));
        }
    }
    assert!(formal_coverage::gate_first_party_security(&r, 100.)
        .unwrap()
        .is_empty());
    assert!(gate(&r, 100., None, false)
        .unwrap()
        .iter()
        .any(|f| f.contains("below")));
    assert!(formal_coverage::gate_first_party_security(&r, f64::NAN).is_err());
    assert!(formal_coverage::gate_first_party_security(&r, 101.).is_err());
    r.units
        .iter_mut()
        .for_each(|u| u.status = "unverified".into());
    assert!(formal_coverage::gate_first_party_security(&r, 1.)
        .unwrap()
        .iter()
        .any(|f| f.contains("first-party-security-proven")));
    let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args([
            "gate",
            "fixtures/inventory",
            "--target",
            "x86_64-unknown-linux-gnu",
            "--min-first-party-security-proven",
            "1",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("first-party-security-proven"));
}
