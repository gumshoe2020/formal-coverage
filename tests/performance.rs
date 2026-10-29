use std::{fs, path::PathBuf, process::Command, time::Instant};

#[test]
#[ignore = "release performance acceptance: cargo test --release --frozen --test performance -- --ignored --nocapture"]
fn large_workspace_reports_under_two_seconds() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("logs/perf-workspace-{}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"perf-fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        root.join("Cargo.lock"),
        "version = 3\n[[package]]\nname = \"perf-fixture\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    let mut source = String::new();
    for i in 0..5000 {
        source.push_str(&format!("pub fn work_{i}() {{}}\n"));
    }
    for i in 0..2000 {
        source.push_str(&format!(
            "#[test] fn test_{i}() {{ work_{i}(); work_{}(); }}\n",
            i + 2000
        ));
    }
    fs::write(root.join("src/lib.rs"), source).unwrap();
    let start = Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_formal-coverage"))
        .arg("report")
        .arg(&root)
        .arg("--json")
        .arg(root.join("report.json"))
        .output()
        .unwrap();
    let elapsed = start.elapsed();
    assert!(output.status.success(), "{:?}", output.stderr);
    let report: formal_coverage::Report =
        serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
    assert_eq!(report.units.len(), 7000);
    for unit in report.units.iter().filter(|u| u.name.starts_with("work_")) {
        let index: usize = unit.name.strip_prefix("work_").unwrap().parse().unwrap();
        assert_eq!(
            unit.status,
            if index < 4000 { "tested" } else { "unverified" }
        );
    }
    println!("5000 runtime units + 2000 test functions: {elapsed:?}");
    assert!(elapsed.as_secs_f64() < 2.0, "report took {elapsed:?}");
}
