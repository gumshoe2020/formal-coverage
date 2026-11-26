//! A bare `#[test]` or `#[bench]` counts as the built-in only when no import
//! in its module (or an enclosing fn or impl body) can bind that name.
use formal_coverage::{analyze_with, Category, Options, Report};
use std::{fs, path::PathBuf};

const LIB: &str = r#"
mod std_glob {
    use std::fmt::*;
    #[test]
    fn std_glob_builtin() {}
}
mod super_glob {
    use super::*;
    #[test]
    fn super_glob_shadowed() {}
}
mod foreign_glob {
    use attrs::*;
    #[test]
    fn foreign_glob_shadowed() {}
}
mod renamed_to_test {
    use attrs::check as test;
    #[test]
    fn renamed_shadowed() {}
}
mod renamed_away {
    use attrs::test as check;
    #[test]
    fn renamed_away_builtin() {}
}
mod self_in_group {
    use attrs::test::{self};
    #[test]
    fn self_group_shadowed() {}
}
mod grouped {
    use attrs::{other, test};
    #[test]
    fn group_shadowed() {}
}
mod item_macro {
    some_macro! {
        use attrs::test;
    }
    #[test]
    fn macro_body_shadowed() {}
}
mod bench_import {
    use attrs::bench;
    #[bench]
    fn bench_shadowed() {}
}
mod prelude_path {
    use attrs::test;
    #[std::prelude::rust_2021::test]
    fn prelude_still_builtin() {}
}
mod child_of_shadowed {
    use attrs::test;
    mod fresh {
        #[test]
        fn fresh_scope_builtin() {}
    }
}
"#;

fn fixture(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = PathBuf::from(format!("logs/test-shadowing-{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"shadow\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n",
    )
    .unwrap();
    for (path, source) in files {
        fs::write(root.join(path), source).unwrap();
    }
    root
}

fn evidence(r: &Report, name: &str) -> (Category, String) {
    let units: Vec<_> = r.units.iter().filter(|u| u.name == name).collect();
    assert_eq!(units.len(), 1, "{name}");
    let value = serde_json::to_value(units[0]).unwrap()["test_evidence"].clone();
    (units[0].category, value.as_str().unwrap().to_string())
}

#[test]
fn imports_that_may_bind_test_make_the_bare_attribute_unrecognized() {
    let root = fixture("scopes", &[("src/lib.rs", LIB)]);
    let r = analyze_with(&root, &Options::default()).unwrap();
    for name in [
        "std_glob_builtin",
        "renamed_away_builtin",
        "prelude_still_builtin",
        "fresh_scope_builtin",
    ] {
        assert_eq!(
            evidence(&r, name),
            (Category::Test, "builtin-attribute".into()),
            "{name}"
        );
    }
    for name in [
        "super_glob_shadowed",
        "foreign_glob_shadowed",
        "renamed_shadowed",
        "self_group_shadowed",
        "group_shadowed",
        "macro_body_shadowed",
        "bench_shadowed",
    ] {
        assert_eq!(
            evidence(&r, name),
            (Category::Runtime, "none".into()),
            "{name}"
        );
        assert!(
            r.warnings
                .iter()
                .any(|w| w.contains(name) && w.contains("unrecognized test attribute")),
            "{name}"
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn macro_use_extern_crate_shadows_test_in_every_module() {
    let root = fixture(
        "macro-use",
        &[
            (
                "src/lib.rs",
                "#[macro_use]\nextern crate attrs;\nmod child;\n#[test]\nfn root_case() {}\n",
            ),
            (
                "src/child.rs",
                "mod inner {\n    #[test]\n    fn child_case() {}\n}\n",
            ),
        ],
    );
    let r = analyze_with(&root, &Options::default()).unwrap();
    for name in ["root_case", "child_case"] {
        assert_eq!(
            evidence(&r, name),
            (Category::Runtime, "none".into()),
            "{name}"
        );
    }
    fs::remove_dir_all(&root).unwrap();

    let root = fixture(
        "macro-use-std",
        &[(
            "src/lib.rs",
            "#[macro_use]\nextern crate std;\n#[test]\nfn std_case() {}\n",
        )],
    );
    let r = analyze_with(&root, &Options::default()).unwrap();
    assert_eq!(
        evidence(&r, "std_case"),
        (Category::Test, "builtin-attribute".into())
    );
    fs::remove_dir_all(root).unwrap();
}
