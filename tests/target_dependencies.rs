use std::{fs, path::PathBuf, process::Command};

fn fixture() -> PathBuf {
    let root = PathBuf::from(format!("logs/target-deps-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/lib.rs"), "pub fn entry() {}\n").unwrap();
    fs::write(root.join("Cargo.toml"), r#"[package]
name = "target-fixture"
version = "0.1.0"
edition = "2021"
[workspace]
exclude = ["vendor/normal", "vendor/transitive", "vendor/build", "vendor/dev", "vendor/optional", "vendor/windows", "vendor/macros", "vendor/build-transitive"]
[dependencies]
normal = { path = "vendor/normal" }
optional = { path = "vendor/optional", optional = true }
macros = { path = "vendor/macros" }
[build-dependencies]
build = { path = "vendor/build" }
[dev-dependencies]
dev = { path = "vendor/dev" }
[target.'cfg(windows)'.dependencies]
windows = { path = "vendor/windows" }
"#).unwrap();
    for name in [
        "normal",
        "transitive",
        "build",
        "dev",
        "optional",
        "windows",
        "macros",
        "build-transitive",
    ] {
        let dir = root.join("vendor").join(name);
        fs::create_dir_all(dir.join("src")).unwrap();
        let extra = match name {
            "normal" => {
                r#"[dependencies]
transitive = { path = "../transitive" }
renamed-optional = { package = "optional", path = "../optional", optional = true }
[features]
default = ["alloc"]
alloc = ["renamed-optional?/alloc"]
"#
            }
            "optional" => "[features]\nalloc = []\n",
            "build" => "[dependencies]\nbuild-transitive = { path = \"../build-transitive\" }\n",
            "macros" => "[lib]\nproc-macro = true\n",
            _ => "",
        };
        fs::write(dir.join("Cargo.toml"), format!("[package]\nname = {name:?}\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n{extra}")).unwrap();
        fs::write(
            dir.join("src/lib.rs"),
            "pub fn dependency() { unsafe {} }\n",
        )
        .unwrap();
    }
    assert!(Command::new("cargo")
        .args(["generate-lockfile", "--offline"])
        .current_dir(&root)
        .output()
        .unwrap()
        .status
        .success());
    root
}
#[test]
fn target_runtime_build_dev_optional_and_fallback() {
    let root = fixture();
    let mut r =
        formal_coverage::analyze_target(&root, None, Some("x86_64-unknown-linux-gnu")).unwrap();
    assert_eq!(
        r.closure, "cargo metadata (runtime)",
        "{:?}",
        r.closure_reason
    );
    assert!(r.dependency_inventory_complete);
    let names =
        |ds: &[formal_coverage::Dependency]| ds.iter().map(|d| d.name.clone()).collect::<Vec<_>>();
    assert_eq!(names(&r.dependencies), ["normal", "transitive"]);
    assert_eq!(
        names(&r.build_dependencies),
        ["build", "build-transitive", "macros"]
    );
    assert_eq!(r.dev_dependencies_excluded, 1);
    formal_coverage::apply_evidence_many(&mut r, &root, &[], &[]).unwrap();
    assert!(r
        .build_dependencies
        .iter()
        .all(|d| d.sloc == 1 && d.unsafe_sites == Some(1)));
    assert_eq!(
        formal_coverage::summarize(&r)
            .third_party
            .security_relevant
            .sloc,
        2
    );
    let windows =
        formal_coverage::analyze_target(&root, None, Some("x86_64-pc-windows-msvc")).unwrap();
    assert_eq!(
        names(&windows.dependencies),
        ["normal", "transitive", "windows"]
    );
    let normal_manifest = root.join("vendor/normal/Cargo.toml");
    let weak = fs::read_to_string(&normal_manifest).unwrap();
    fs::write(
        &normal_manifest,
        weak.replace("renamed-optional?/alloc", "dep:renamed-optional"),
    )
    .unwrap();
    let optional_enabled =
        formal_coverage::analyze_target(&root, None, Some("x86_64-unknown-linux-gnu")).unwrap();
    assert_eq!(
        names(&optional_enabled.dependencies),
        ["normal", "optional", "transitive"]
    );
    fs::write(&normal_manifest, weak).unwrap();
    // An invalid target deterministically makes full metadata fail; --no-deps inventory still succeeds.
    let fallback =
        formal_coverage::analyze_target(&root, None, Some("nonexistent-target-for-fallback"))
            .unwrap();
    assert_eq!(fallback.closure, "lockfile (fallback)");
    assert!(formal_coverage::markdown(&fallback).contains("(exclusions unconfirmed on fallback)"));
    assert!(!formal_coverage::markdown(&windows).contains("(exclusions unconfirmed on fallback)"));
    assert!(!fallback.closure_reason.as_deref().unwrap().is_empty());
    let lock: toml::Value =
        toml::from_str(&fs::read_to_string(root.join("Cargo.lock")).unwrap()).unwrap();
    let expected: std::collections::BTreeSet<_> = lock["package"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|p| p["name"].as_str() != Some("target-fixture"))
        .map(|p| {
            (
                p["name"].as_str().unwrap().to_string(),
                p["version"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(
        fallback
            .dependencies
            .iter()
            .map(|d| (d.name.clone(), d.version.clone()))
            .collect::<std::collections::BTreeSet<_>>(),
        expected
    );
    assert!(fallback.build_dependencies.is_empty());
    assert!(formal_coverage::markdown(&fallback).contains("closure: lockfile (fallback)"));
    assert!(formal_coverage::gate(&fallback, 0., None, false)
        .unwrap()
        .iter()
        .any(|f| f.contains("unknown dependency")));
}
