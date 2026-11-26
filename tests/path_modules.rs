//! `#[path = "..."]` module files are found as rustc finds them, and the
//! attributes on the `mod` item apply to every unit in the file.
use formal_coverage::{analyze_with, Category, Options, Report};
use std::{fs, path::PathBuf};

/// (path under the package, contents)
const FILES: &[(&str, &str)] = &[
    (
        "src/lib.rs",
        r#"pub mod tls;
pub mod nm;
pub mod m;
mod plain;

#[cfg(test)]
#[path = "sub/deep.rs"]
mod deep;

#[cfg(test)]
mod inline_root {
    #[path = "z.rs"]
    mod z;
}

#[cfg(test)]
#[path = "pd"]
mod path_dir {
    mod w;
}

#[path = "runtime_alias.rs"]
mod runtime_alias;
"#,
    ),
    // scratch-server shape: a cfg(test) #[path] chain from a non-mod-rs file.
    (
        "src/tls.rs",
        "pub fn handshake() {}\n#[cfg(test)]\n#[path = \"tls_test_provider.rs\"]\nmod test_provider;\n",
    ),
    (
        "src/tls_test_provider.rs",
        "fn provider() {}\n#[cfg(test)]\n#[path = \"record_tests.rs\"]\nmod record_tests;\n",
    ),
    (
        "src/record_tests.rs",
        "use super::*;\nfn compare_unpad() {}\n#[test]\nfn stage15_case() {}\n",
    ),
    // A #[path] file acts as mod.rs: its `mod child;` is a sibling.
    ("src/sub/deep.rs", "fn deep_helper() {}\nmod child;\n"),
    ("src/sub/child.rs", "fn child_helper() {}\n"),
    // Non-mod-rs file: top-level #[path] is relative to its directory, but
    // inside an inline module it starts under `nm/`.
    (
        "src/nm.rs",
        "#[cfg(test)]\n#[path = \"x.rs\"]\nmod x;\n#[cfg(test)]\nmod inl {\n    #[path = \"y.rs\"]\n    mod y;\n}\n",
    ),
    ("src/x.rs", "fn sibling_of_nm() {}\n"),
    ("src/nm/inl/y.rs", "fn under_nm_inline() {}\n"),
    // mod.rs file: relative to its own directory.
    (
        "src/m/mod.rs",
        "#[cfg(test)]\n#[path = \"t.rs\"]\nmod t;\n",
    ),
    ("src/m/t.rs", "fn mod_rs_sibling() {}\n"),
    ("src/inline_root/z.rs", "fn inline_in_mod_rs() {}\n"),
    ("src/pd/w.rs", "fn inside_path_dir() {}\n"),
    ("src/runtime_alias.rs", "pub fn not_test_code() {}\n"),
    ("src/plain.rs", "pub fn plain_runtime() {}\n"),
];

fn fixture(name: &str, extra_lib: &str) -> PathBuf {
    let root = PathBuf::from(format!("logs/path-modules-{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    let package = root.join("pkg");
    fs::create_dir_all(&package).unwrap();
    fs::write(
        package.join("Cargo.toml"),
        "[package]\nname = \"pathmods\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n",
    )
    .unwrap();
    for (path, source) in FILES {
        let path = package.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let source = if path.ends_with("src/lib.rs") {
            format!("{source}{extra_lib}")
        } else {
            source.to_string()
        };
        fs::write(path, source).unwrap();
    }
    root
}

fn unit<'a>(r: &'a Report, name: &str) -> &'a formal_coverage::Unit {
    let units: Vec<_> = r.units.iter().filter(|u| u.name == name).collect();
    assert_eq!(units.len(), 1, "{name}");
    units[0]
}

fn evidence(r: &Report, name: &str) -> String {
    let value = serde_json::to_value(unit(r, name)).unwrap()["test_evidence"].clone();
    value.as_str().unwrap().to_string()
}

#[test]
fn path_modules_take_the_attributes_of_their_declaration() {
    let root = fixture("resolve", "");
    let r = analyze_with(&root.join("pkg"), &Options::default()).unwrap();
    for name in [
        "provider",
        "compare_unpad",
        "deep_helper",
        "child_helper",
        "sibling_of_nm",
        "under_nm_inline",
        "mod_rs_sibling",
        "inline_in_mod_rs",
        "inside_path_dir",
    ] {
        assert_eq!(unit(&r, name).category, Category::Test, "{name}");
        assert_eq!(evidence(&r, name), "cfg-test", "{name}");
    }
    // `use super::*` may bind `test`, so the cfg(test) chain is the evidence.
    assert_eq!(unit(&r, "stage15_case").category, Category::Test);
    assert_eq!(evidence(&r, "stage15_case"), "cfg-test");
    for name in ["handshake", "not_test_code", "plain_runtime"] {
        assert_eq!(unit(&r, name).category, Category::Runtime, "{name}");
        assert_eq!(evidence(&r, name), "none", "{name}");
    }
    // Unit ids keep their file-path module names.
    assert_eq!(
        unit(&r, "provider").id,
        "pathmods::tls_test_provider::provider"
    );
    assert!(
        r.escape_hatches.is_empty(),
        "unexpected hazards: {:?}",
        r.escape_hatches
    );
    fs::remove_dir_all(root).unwrap();
}

fn hazards(extra_lib: &str) -> Vec<formal_coverage::Escape> {
    let root = fixture(&format!("hazard-{}", extra_lib.len()), extra_lib);
    fs::write(root.join("outside.rs"), "pub fn outside() {}\n").unwrap();
    fs::create_dir_all(root.join("pkg/target")).unwrap();
    fs::write(
        root.join("pkg/target/generated.rs"),
        "pub fn generated() {}\n",
    )
    .unwrap();
    let r = analyze_with(&root.join("pkg"), &Options::default()).unwrap();
    let gate = formal_coverage::gate(&r, 0.0, None, false).unwrap();
    assert!(
        gate.iter().any(|f| f.contains("file-scope hazard")),
        "{gate:?}"
    );
    fs::remove_dir_all(root).unwrap();
    r.escape_hatches
}

#[test]
fn unfollowable_path_modules_are_hazards() {
    for (declaration, reason) in [
        (
            "#[cfg(test)]\n#[path = \"../../outside.rs\"]\nmod escapes;\n",
            "outside the measured workspace sources",
        ),
        (
            "#[cfg(test)]\n#[path = \"../target/generated.rs\"]\nmod generated;\n",
            "outside the measured workspace sources",
        ),
        (
            "#[cfg(test)]\n#[path = \"missing.rs\"]\nmod missing;\n",
            "missing or unreadable",
        ),
        (
            "#[cfg(test)]\n#[path = \"plain.rs\"]\nmod plain_again;\n",
            "already loaded by another module declaration",
        ),
        (
            "#[path = \"lib.rs\"]\nmod itself;\n",
            "circular module declaration",
        ),
        (
            "#[cfg_attr(test, path = \"x.rs\")]\nmod conditional;\n",
            "cfg_attr",
        ),
    ] {
        let found = hazards(declaration);
        assert_eq!(found.len(), 1, "{declaration}: {found:?}");
        let hazard = &found[0];
        assert_eq!(hazard.file, "src/lib.rs", "{declaration}");
        assert_eq!(hazard.unit, "src/lib.rs::<file-scope>");
        assert!(
            hazard.kind.starts_with("#[path] module: "),
            "{}",
            hazard.kind
        );
        assert!(
            hazard.kind.contains(reason),
            "{declaration}: {}",
            hazard.kind
        );
        assert!(hazard.line > 20, "{declaration}: line {}", hazard.line);
    }
}

#[test]
fn a_file_loaded_twice_is_test_only_if_every_declaration_is() {
    let root = fixture(
        "twice",
        "#[cfg(test)]\n#[path = \"plain.rs\"]\nmod plain_again;\n",
    );
    let r = analyze_with(&root.join("pkg"), &Options::default()).unwrap();
    assert_eq!(unit(&r, "plain_runtime").category, Category::Runtime);
    let hazard = &r.escape_hatches[0];
    assert_eq!(
        hazard.affected_units,
        vec!["pathmods::plain::plain_runtime"]
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn a_path_into_another_workspace_member_is_followed() {
    let root = PathBuf::from(format!("logs/path-modules-members-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    let files = [
        (
            "Cargo.toml",
            "[workspace]\nmembers = [\"a\", \"b\"]\nresolver = \"2\"\n",
        ),
        (
            "a/Cargo.toml",
            "[package]\nname = \"a\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        ),
        (
            "b/Cargo.toml",
            "[package]\nname = \"b\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        ),
        (
            "a/src/lib.rs",
            "#[path = \"../../b/src/shared.rs\"]\npub mod shared;\n#[cfg(test)]\n#[path = \"../../b/src/only_a_tests.rs\"]\nmod only_a_tests;\n",
        ),
        ("b/src/lib.rs", "pub mod shared;\n"),
        ("b/src/shared.rs", "pub fn shared_parser() {}\n"),
        ("b/src/only_a_tests.rs", "fn loaded_only_by_a_tests() {}\n"),
    ];
    for (path, source) in files {
        let path = root.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, source).unwrap();
    }
    let r = analyze_with(&root, &Options::default()).unwrap();
    assert!(r.escape_hatches.is_empty(), "{:?}", r.escape_hatches);
    // b compiles shared.rs as runtime, so a's declaration cannot make it test.
    assert_eq!(unit(&r, "shared_parser").category, Category::Runtime);
    assert_eq!(unit(&r, "loaded_only_by_a_tests").category, Category::Test);
    assert_eq!(evidence(&r, "loaded_only_by_a_tests"), "cfg-test");
    fs::remove_dir_all(root).unwrap();
}
