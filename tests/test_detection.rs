//! v1.5 acceptance D (fixture): structural test detection, whatever the module is named.
use formal_coverage::{analyze_with, Category, Options, SizeMode};
use std::{fs, path::PathBuf};

const LIB: &str = r#"pub fn entry() -> u32 {
    helper_runtime()
}
fn helper_runtime() -> u32 {
    1
}

#[cfg(test)]
mod policy_tests {
    fn diagnostic_classes_and_privacy() {
        assert_eq!(super::entry(), 1);
    }
}

#[cfg(test)]
mod a {
    mod b {
        fn helper() {}
    }
}

#[test]
fn free_test() {
    assert_eq!(entry(), 1);
}

#[tokio::test]
async fn tokio_case() {}

#[attrs::test]
fn authorize() {}

mod shadowed {
    #[test]
    fn release_callable() {}
    use attrs::test;
}

mod shadowed_scopes {
    use attrs::test;
    pub struct S;
    impl S {
        #[test]
        fn method_callable() {}
    }
    pub fn outer() {
        #[test]
        fn inner_callable() {}
    }
    mod fresh {
        use std::fmt;
        #[test]
        fn still_builtin() {}
    }
}

#[std::prelude::rust_2021::test]
fn qualified() {}

#[core::prelude::v1::test]
fn qualified_core() {}

#[bench]
fn benchmark() {}

#[cfg(all(test, unix))]
fn all_test() {}

#[cfg(any(test, feature = "extra"))]
fn any_test() {}

#[cfg(not(test))]
fn not_test() {}

#[cfg(test)]
mod declared;
mod plain;
"#;

fn fixture() -> PathBuf {
    let root = PathBuf::from(format!("logs/test-detection-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("tests")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"detect\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[features]\nextra = []\n[workspace]\n",
    )
    .unwrap();
    fs::write(root.join("src/lib.rs"), LIB).unwrap();
    fs::write(
        root.join("src/declared.rs"),
        "fn out_of_line_helper() {}\nmod deeper {\n    fn nested_helper() {}\n}\n",
    )
    .unwrap();
    fs::write(root.join("src/plain.rs"), "pub fn plain_runtime() {}\n").unwrap();
    fs::write(
        root.join("tests/integration.rs"),
        "fn shared_helper() {}\n#[test]\nfn integration() {\n    shared_helper();\n}\n",
    )
    .unwrap();
    root
}

fn category(r: &formal_coverage::Report, name: &str) -> Category {
    let units: Vec<_> = r.units.iter().filter(|u| u.name == name).collect();
    assert_eq!(units.len(), 1, "{name}");
    units[0].category
}

#[test]
fn test_code_is_detected_structurally_at_any_depth() {
    let root = fixture();
    let r = analyze_with(&root, &Options::default()).unwrap();
    for name in [
        "diagnostic_classes_and_privacy",
        "helper",
        "free_test",
        "qualified",
        "qualified_core",
        "benchmark",
        "all_test",
        "out_of_line_helper",
        "nested_helper",
        "shared_helper",
        "integration",
        "still_builtin",
    ] {
        assert_eq!(category(&r, name), Category::Test, "{name}");
    }
    for name in [
        "tokio_case",
        "authorize",
        "release_callable",
        "method_callable",
        "inner_callable",
        "entry",
        "helper_runtime",
        "any_test",
        "not_test",
        "plain_runtime",
    ] {
        assert_eq!(category(&r, name), Category::Runtime, "{name}");
    }
    for (name, evidence) in [
        ("qualified", "builtin-attribute"),
        ("nested_helper", "cfg-test"),
        ("shared_helper", "test-target"),
        ("authorize", "none"),
        ("method_callable", "none"),
        ("inner_callable", "none"),
        ("still_builtin", "builtin-attribute"),
    ] {
        let unit = r.units.iter().find(|u| u.name == name).unwrap();
        assert_eq!(
            serde_json::to_value(unit).unwrap()["test_evidence"],
            evidence
        );
    }
    for name in ["tokio_case", "authorize"] {
        assert!(r.warnings.contains(&format!(
            "detect::{name}: unrecognized test attribute; measured as runtime"
        )));
    }
    let s = formal_coverage::summarize(&r);
    let test_units = r.units.iter().filter(|u| u.category == Category::Test);
    assert_eq!(
        s.test_code.sloc,
        test_units.clone().map(|u| u.sloc).sum::<usize>()
    );
    assert_eq!(s.test_code.units, 12);
    let runtime: usize = r
        .units
        .iter()
        .filter(|u| u.category == Category::Runtime)
        .map(|u| u.sloc)
        .sum();
    assert_eq!(
        s.first_party.security_relevant.sloc + s.first_party.everything_else.sloc,
        runtime
    );
    let md = formal_coverage::markdown(&r);
    assert!(md.contains(&format!(
        "TEST CODE: {} SLoC in 12 first-party units",
        s.test_code.sloc
    )));
    assert!(md.contains("| detect::policy_tests::diagnostic_classes_and_privacy | src/lib.rs:"));
    assert!(md.contains("| test-code |"));

    // whole-crate keeps the v1.4.2 categories for byte-identical comparison.
    let legacy = analyze_with(
        &root,
        &Options {
            size_mode: SizeMode::WholeCrate,
            ..Options::default()
        },
    )
    .unwrap();
    assert!(legacy.units.iter().all(|u| u.category == Category::Runtime));
    assert!(formal_coverage::summarize(&legacy).test_code.is_empty());
    fs::remove_dir_all(root).unwrap();
}
