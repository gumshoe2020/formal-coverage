use std::path::Path;
#[test]
fn exact_split_and_overrides() {
    let r = formal_coverage::analyze(Path::new("fixtures/classification"), None).unwrap();
    let actual = serde_json::json!({
        "security": r.units.iter().filter(|u|u.security).map(|u|&u.id).collect::<Vec<_>>(),
        "everything_else": r.units.iter().filter(|u|!u.security).map(|u|&u.id).collect::<Vec<_>>(),
        "dependencies": r.dependencies.iter().filter(|d|d.security).map(|d|&d.name).collect::<Vec<_>>(),
        "overrides": r.overrides.iter().map(|o|serde_json::json!([o.pattern,o.security])).collect::<Vec<_>>()
    });
    let expected: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string("fixtures/classification/expected.json").unwrap(),
    )
    .unwrap();
    assert_eq!(actual, expected);
    assert!(r.dependencies.iter().all(|d| d.size_known && d.sloc == 1));
}

#[test]
fn known_edges_and_unresolved_empty_or_generic_calls() {
    let root = std::path::PathBuf::from(format!("logs/call-test-{}", std::process::id()));
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"calls\"\nversion = \"0.1.0\"\n[workspace]\n",
    )
    .unwrap();
    std::fs::write(
        root.join("formal-coverage.toml"),
        "entry_points = [\"calls::entry\"]\n",
    )
    .unwrap();
    for (body, all) in [
        ("parser();", false),
        ("external::empty();", true),
        ("external::generic::<u8>();", true),
        ("external!();", true),
    ] {
        std::fs::write(
            root.join("src/lib.rs"),
            format!("fn entry() {{ {body} }}\nfn parser() {{}}\nfn unrelated() {{}}\n"),
        )
        .unwrap();
        let r = formal_coverage::analyze(&root, None).unwrap();
        assert_eq!(
            r.units
                .iter()
                .find(|u| u.name == "unrelated")
                .unwrap()
                .security,
            all,
            "{body}"
        );
        assert!(r.units.iter().find(|u| u.name == "entry").unwrap().security);
    }
}
