//! v1.5 acceptance E: explicit inputs with reasons, default-rules figures and migration.
use formal_coverage::{analyze_with, Options, Report, SizeMode};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);

const LIB: &str = "pub fn entry() {\n    parse();\n    helper();\n}\npub fn parse() {}\nfn helper() {\n    let _ = 1;\n}\npub fn startup() {\n    let _ = 2;\n    let _ = 3;\n}\n";

fn write(path: PathBuf, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

/// `svc` with `entry` reaching `parse` and `helper`; `startup` is unreachable.
/// Runtime dependencies: `dup` at two versions and `other`.
fn fixture() -> PathBuf {
    let root = PathBuf::from(format!(
        "logs/inputs-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    write(root.join("Cargo.toml"), "[package]\nname = \"svc\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\nexclude = [\"vendor\"]\n[dependencies]\ndup1 = { package = \"dup\", path = \"vendor/dup1\" }\ndup2 = { package = \"dup\", path = \"vendor/dup2\" }\nother = { path = \"vendor/other\" }\n");
    write(root.join("src/lib.rs"), LIB);
    for (dir, name, version, body) in [
        ("dup1", "dup", "0.1.0", "pub fn one() {}\n"),
        (
            "dup2",
            "dup",
            "0.2.0",
            "pub fn two() {}\npub fn three() {}\n",
        ),
        (
            "other",
            "other",
            "0.1.0",
            "pub fn other() {\n    let _ = 4;\n}\n",
        ),
    ] {
        write(root.join(format!("vendor/{dir}/Cargo.toml")), &format!("[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n[workspace]\n"));
        write(root.join(format!("vendor/{dir}/src/lib.rs")), body);
    }
    let status = Command::new("cargo")
        .args(["generate-lockfile", "--offline"])
        .current_dir(&root)
        .status()
        .unwrap();
    assert!(status.success());
    root
}

fn run(root: &Path, config: &str) -> anyhow::Result<Report> {
    let path = root.join("formal-coverage.toml");
    fs::write(&path, config).unwrap();
    analyze_with(
        root,
        &Options {
            config: Some(path),
            size_mode: SizeMode::WholeCrate,
            ..Options::default()
        },
    )
}

fn security(r: &Report, name: &str) -> bool {
    r.units.iter().find(|u| u.name == name).unwrap().security
}

const SCHEMA3: &str = r#"entry_points = ["svc::entry"]

[[security.include]]
pattern = "svc::startup"
reason = "reads the listen address from the environment"

[[security.exclude]]
pattern = "svc::helper"
reason = "pure constant; no input"

[dependencies."dup@0.2.0"]
security = false
reason = "never called at runtime"

[dependencies.other]
sloc = 10
reason = "measured by hand"
"#;

#[test]
fn schema3_inputs_apply_with_reasons() {
    let root = fixture();
    let r = run(&root, SCHEMA3).unwrap();
    assert_eq!(r.config_schema, 3);
    assert!(security(&r, "entry") && security(&r, "parse") && security(&r, "startup"));
    assert!(!security(&r, "helper"));
    assert!(!r
        .warnings
        .iter()
        .any(|w| w.starts_with("override has no reason")));
    let dup2 = r
        .dependencies
        .iter()
        .find(|d| d.version == "0.2.0")
        .unwrap();
    assert!(!dup2.security && dup2.default_security == Some(true));
    let other = r.dependencies.iter().find(|d| d.name == "other").unwrap();
    assert_eq!(
        (other.sloc, other.default_sloc, other.size_mode.as_str()),
        (10, Some(3), "configured")
    );
    let kinds: Vec<_> = r
        .inputs
        .iter()
        .map(|i| {
            (
                i.kind.as_str(),
                i.key.as_str(),
                i.reason.as_deref().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (
                "security.include",
                "svc::startup",
                "reads the listen address from the environment"
            ),
            ("security.exclude", "svc::helper", "pure constant; no input"),
            ("dependencies", "dup@0.2.0", "never called at runtime"),
            ("dependencies", "other", "measured by hand"),
        ]
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn schema3_rejects_missing_reasons_unmatched_keys_and_ambiguous_names() {
    let root = fixture();
    let cases = [
        (
            "entry_points = [\"svc::entry\"]\n[[security.include]]\npattern = \"svc::startup\"\n",
            "reason",
        ),
        (
            "[[security.include]]\npattern = \"svc::startup\"\nreason = \"  \"\n",
            "empty reason",
        ),
        (
            "[[security.exclude]]\npattern = \"svc::nothing\"\nreason = \"r\"\n",
            "matches no function",
        ),
        (
            "[dependencies.nosuch]\nsecurity = false\nreason = \"r\"\n",
            "matches no runtime dependency",
        ),
        (
            "[dependencies.\"other@9.9.9\"]\nsecurity = false\nreason = \"r\"\n",
            "matches no runtime dependency",
        ),
        (
            "[dependencies.dup]\nsecurity = false\nreason = \"r\"\n",
            "ambiguous: dup@0.1.0, dup@0.2.0",
        ),
        (
            "[dependencies.other]\nreason = \"r\"\n",
            "neither security nor sloc",
        ),
        (
            "[[security.include]]\npattern = \"svc::startup\"\nreason = \"r\"\nwhy = \"x\"\n",
            "unknown field",
        ),
    ];
    for (config, expected) in cases {
        let error = format!("{:#}", run(&root, config).unwrap_err());
        assert!(error.contains(expected), "{config}\n=> {error}");
    }
    assert!(run(
        &root,
        "[dependencies.\"dup@0.1.0\"]\nsecurity = false\nreason = \"r\"\n"
    )
    .is_ok());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn todo_reason_fails_gate() {
    let root = fixture();
    let r = run(&root, &SCHEMA3.replace("measured by hand", "TODO")).unwrap();
    let failures = formal_coverage::gate(&r, 0., None, false).unwrap();
    assert_eq!(
        failures,
        ["input dependencies \"other\" has a TODO reason; replace it with the justification"]
    );
    assert!(
        formal_coverage::gate(&run(&root, SCHEMA3).unwrap(), 0., None, false)
            .unwrap()
            .is_empty()
    );
    assert!(
        formal_coverage::todo_reason("todo: check") && !formal_coverage::todo_reason("mastodon")
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn migrate_config_round_trips_scratch_server_overrides() {
    let original = fs::read_to_string("fixtures/v1.5/v1.4.2-scratch-server.toml").unwrap();
    let migrated = formal_coverage::migrate_config(&original).unwrap();
    let parsed: toml::Value = toml::from_str(&migrated).unwrap();
    let old: toml::Value = toml::from_str(&original).unwrap();
    assert_eq!(parsed["entry_points"], old["entry_points"]);
    for (table, flat) in [("include", "security"), ("exclude", "not_security")] {
        let rules = parsed["security"][table].as_array().unwrap();
        let patterns: Vec<_> = rules.iter().map(|r| r["pattern"].clone()).collect();
        assert_eq!(&patterns, old[flat].as_array().unwrap());
        assert!(rules.iter().all(|r| r["reason"].as_str() == Some("TODO")));
    }
    assert!(formal_coverage::migrate_config(&migrated).is_err());

    // Same classification from both schemas; schema 2 warns once per bare override.
    let root = fixture();
    let legacy = "entry_points = [\"svc::entry\"]\nsecurity = [\"svc::startup\"]\nnot_security = [\"svc::helper\", \"svc::pa*\"]\n";
    let before = run(&root, legacy).unwrap();
    assert_eq!(before.config_schema, 2);
    assert_eq!(
        before
            .warnings
            .iter()
            .filter(|w| w.starts_with("override has no reason: "))
            .count(),
        3
    );
    let output = root.join("migrated.toml");
    let status = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .args([
            "migrate-config",
            root.join("formal-coverage.toml").to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(status.success());
    let after = run(&root, &fs::read_to_string(&output).unwrap()).unwrap();
    assert_eq!(after.config_schema, 3);
    let classes = |r: &Report| {
        r.units
            .iter()
            .map(|u| (u.id.clone(), u.security))
            .collect::<Vec<_>>()
    };
    assert_eq!(classes(&before), classes(&after));
    let overrides = |r: &Report| {
        r.overrides
            .iter()
            .map(|o| (o.pattern.clone(), o.security, o.matched.clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(overrides(&before), overrides(&after));
    assert!(!formal_coverage::gate(&after, 0., None, false)
        .unwrap()
        .is_empty());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn default_rules_figure_equals_a_run_with_the_inputs_deleted() {
    let root = fixture();
    let with = run(&root, SCHEMA3).unwrap();
    let without = run(&root, "entry_points = [\"svc::entry\"]\n").unwrap();
    assert!(
        without.inputs.is_empty() && without.units.iter().all(|u| u.default_security.is_none())
    );
    let value = |s: formal_coverage::Summary| {
        let v = serde_json::to_value(s).unwrap();
        ["first_party", "third_party", "all", "proof_code"].map(|k| v[k].clone())
    };
    assert_eq!(
        value(formal_coverage::summarize_default_rules(&with)),
        value(formal_coverage::summarize(&without))
    );
    assert_ne!(
        value(formal_coverage::summarize(&with)),
        value(formal_coverage::summarize(&without))
    );
    // Without inputs both figures coincide.
    assert_eq!(
        value(formal_coverage::summarize_default_rules(&without)),
        value(formal_coverage::summarize(&without))
    );

    // Inputs section: SLoC moved per input (startup 4 in, helper 3 out, dup@0.2.0 2 out, other +7).
    let md = formal_coverage::markdown(&with);
    for row in [
        "| security.include | `svc::startup` | into security | reads the listen address from the environment | 1 units | 4 | 0 | n/a |",
        "| security.exclude | `svc::helper` | out of security | pure constant; no input | 1 units | 0 | 3 | n/a |",
        "| dependencies | `dup@0.2.0` | not security | never called at runtime | dup@0.2.0 | 0 | 2 | n/a |",
        "| dependencies | `other` | size 10 | measured by hand | other@0.1.0 | 0 | 0 | +7 |",
    ] {
        assert!(md.contains(row), "missing {row}\n{md}");
    }
    let s = formal_coverage::summarize(&with);
    let d = formal_coverage::summarize_default_rules(&with);
    for (origin, a, b) in [
        ("First-party", &s.first_party, &d.first_party),
        ("Third-party (runtime)", &s.third_party, &d.third_party),
        ("Combined", &s.all, &d.all),
    ] {
        for (label, totals) in [("with inputs", a), ("default rules only", b)] {
            let line = format!(
                "SECURITY-RELEVANT {origin} ({label})  {} measured SLoC",
                totals.security_relevant.sloc
            );
            assert!(md.contains(&line), "missing {line}");
        }
    }
    // entry 4 + parse 1 + startup 4 with inputs; entry 4 + parse 1 + helper 3 by default.
    assert_eq!(s.first_party.security_relevant.sloc, 4 + 1 + 4);
    assert_eq!(d.first_party.security_relevant.sloc, 4 + 1 + 3);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn max_override_shift_compares_first_party_security_with_and_without_inputs() {
    let root = fixture();
    let mut r = run(&root, SCHEMA3).unwrap();
    for u in &mut r.units {
        if u.name == "parse" || u.name == "entry" {
            u.status = "proven".into();
        }
    }
    let with = formal_coverage::summarize(&r)
        .first_party
        .security_relevant
        .percent("proven");
    let without = formal_coverage::summarize_default_rules(&r)
        .first_party
        .security_relevant
        .percent("proven");
    // with inputs: 3 proven of entry 4 + parse 1 + startup 4 = 9; default: 3 of entry 4 + parse 1 + helper 3 = 8.
    assert_eq!(
        (format!("{with:.4}"), format!("{without:.4}")),
        ("55.5556".into(), "62.5000".into())
    );
    let failures = formal_coverage::gate_override_shift(&r, 5.).unwrap();
    assert_eq!(failures.len(), 1);
    assert!(failures[0]
        .starts_with("inputs shift first-party security proven by 6.9444 percentage points"));
    assert!(formal_coverage::gate_override_shift(&r, 7.)
        .unwrap()
        .is_empty());
    assert!(formal_coverage::gate_override_shift(&r, f64::NAN).is_err());
    fs::remove_dir_all(root).unwrap();
}
