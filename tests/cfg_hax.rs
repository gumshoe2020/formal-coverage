use formal_coverage::{analyze_with, Category, GhostLines, Options, SizeMode};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
fn fixture(source: &str, hax: bool) -> PathBuf {
    let root = PathBuf::from(format!(
        "target/v181/test-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname='cfg_fixture'\nversion='0.0.0'\nedition='2021'\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        root.join("src/lib.rs"),
        format!("{}\n{source}", if hax { "use hax_lib as hx;" } else { "" }),
    )
    .unwrap();
    root
}
fn report(root: &std::path::Path, mode: GhostLines) -> formal_coverage::Report {
    analyze_with(
        root,
        &Options {
            size_mode: SizeMode::WholeCrate,
            ghost_lines: mode,
            ..Options::default()
        },
    )
    .unwrap()
}
#[test]
fn exact_cfg_algebra_and_inherited_items_in_both_modes() {
    let root = fixture(
        r#"
#[cfg(hax)] fn plain() {}
#[cfg(all(hax, unix))] fn all() {}
#[cfg(any(hax, all(hax, windows)))] fn both() {}
#[cfg(any(hax, unix))] fn one() {}
#[cfg(not(hax))] fn negative() {}
#[cfg_attr(hax, inline)] fn attr() {}
#[cfg(feature="hax")] fn feature() {}
#[cfg(hax_pv)] fn pv() {}
#[cfg(hax)] mod nested { impl S { fn inherited() {} } }
#[cfg(hax)] mod external;
#[cfg(hax)] use something::Lemma;
"#,
        true,
    );
    fs::write(root.join("src/external.rs"), "fn external_child() {}\n").unwrap();
    for mode in [GhostLines::Separate, GhostLines::Runtime] {
        let r = report(&root, mode);
        assert_eq!(r.units.len(), 10);
        for u in &r.units {
            let proof =
                ["plain", "all", "both", "inherited", "external_child"].contains(&u.name.as_str());
            assert_eq!(u.category == Category::ProofCode, proof, "{}", u.id);
            assert_eq!(u.fstar_cfg_hax, proof);
            assert_eq!(u.status, "unverified");
            assert_eq!(u.fstar_annotation_sloc, 0);
        }
    }
}
#[test]
fn non_hax_crate_is_untouched() {
    let root = fixture("#[cfg(hax)] fn plain() {}", false);
    let r = report(&root, GhostLines::Separate);
    assert_eq!(r.units[0].category, Category::Runtime);
    assert!(!r.units[0].fstar_cfg_hax);
}
#[test]
fn uncredited_tested_unit_keeps_status_and_gate_accepts_vocabulary() {
    let root = fixture(
        "#[cfg(hax)] fn plain() {}\n#[test] fn check() { plain(); }",
        true,
    );
    let mut r = report(&root, GhostLines::Separate);
    formal_coverage::apply_evidence_many(&mut r, &root, &[], &[]).unwrap();
    assert_eq!(
        r.units.iter().find(|u| u.name == "plain").unwrap().status,
        "tested"
    );
    formal_coverage::gate(&r, 0., None, false).unwrap();
}

#[test]
fn test_classification_precedes_hax_items_and_inline_proof_in_both_modes() {
    for (source, sloc) in [
        ("#[cfg(all(test, hax))] fn helper() {}", 1),
        ("#[cfg(test)] mod tests {\n#[cfg(hax)] fn helper() {}\n}", 1),
        (
            "#[test] fn helper() {\n#[cfg(hax)]\n{ hax_lib::assert!(true); }\n}",
            4,
        ),
    ] {
        let root = fixture(source, true);
        for mode in [GhostLines::Separate, GhostLines::Runtime] {
            let options = Options {
                ghost_lines: mode,
                ..Options::default()
            };
            let mut r = analyze_with(&root, &options).unwrap();
            formal_coverage::apply_evidence_many(&mut r, &root, &[], &[]).unwrap();
            let u = &r.units[0];
            assert_eq!(u.category, Category::Test);
            assert_eq!(u.status, "unverified");
            assert!(!u.fstar_cfg_hax);
            assert_eq!(u.fstar_cfg_hax_inline_sloc, 0);
            assert_eq!(u.inline_proof_sloc, 0);
            assert_eq!(u.ghost_sloc, 0);
            // All source lines, including cfg attributes, remain test SLoC.
            assert_eq!(u.sloc, sloc);
        }
    }
}

#[test]
fn hax_marker_in_functionless_root_applies_to_sibling_module() {
    let root = fixture("mod child;", true);
    fs::write(root.join("src/child.rs"), "#[cfg(hax)] fn lemma() {}\n").unwrap();
    let r = report(&root, GhostLines::Separate);
    assert_eq!(r.units[0].category, Category::ProofCode);
}

#[test]
fn inline_statements_blocks_shared_lines_overlap_and_modes() {
    let root = fixture(
        r#"fn runtime() {
    let runtime = 1;
    #[cfg(hax)]
    {
        hax_lib::assert!(true);
        lemma();
    }
    #[cfg(all(hax, unix))]
    let proof = {
        helper()
    };
    #[cfg(hax)] lemma(); let real = 2;
    #[cfg(hax)] hax_lib::assert!(true);
    consume(runtime, real);
}
#[cfg(hax)]
#[hax_lib::ensures(true)]
fn lemma() {
    hax_lib::assert!(true);
}
"#,
        true,
    );
    let separate = report(&root, GhostLines::Separate);
    let runtime = report(&root, GhostLines::Runtime);
    let a = separate.units.iter().find(|u| u.name == "runtime").unwrap();
    let b = runtime.units.iter().find(|u| u.name == "runtime").unwrap();
    // cfg block 5 lines; cfg let 4 lines; one overlapped cfg/annotation line.
    assert_eq!(a.inline_proof_sloc, 10);
    assert_eq!(a.fstar_cfg_hax_inline_sloc, 10);
    assert_eq!(a.fstar_annotation_sloc, 2);
    assert_eq!(a.sloc + 10, b.sloc);
    assert_eq!(b.inline_proof_sloc, 0);
    let proof = separate.units.iter().find(|u| u.name == "lemma").unwrap();
    assert_eq!(proof.fstar_annotation_sloc, 0);
    assert_eq!(proof.inline_proof_sloc, 0);
    assert_eq!(proof.sloc, 5);
    let p = formal_coverage::proof_ratio(&separate);
    assert_eq!(p.all_first_party.fstar_cfg_hax_sloc, 5);
    assert_eq!(p.all_first_party.fstar_cfg_hax_inline_sloc, 10);
    assert_eq!(p.all_first_party.inline_proof_sloc, 10);
    assert_eq!(p.all_first_party.proof_sloc, 15);
    assert_eq!(p.crates[0].metrics.fstar_cfg_hax_sloc, 5);
    assert_eq!(p.modules[0].metrics.fstar_cfg_hax_inline_sloc, 10);
    let runtime_ratio = formal_coverage::proof_ratio(&runtime);
    assert_eq!(runtime_ratio.all_first_party.fstar_cfg_hax_sloc, 5);
    assert_eq!(runtime_ratio.all_first_party.fstar_cfg_hax_inline_sloc, 0);
    assert_eq!(p.runtime_proven_modes.runtime.all_first_party.proof_sloc, 5);
    assert_eq!(
        runtime_ratio
            .runtime_proven_modes
            .separate
            .all_first_party
            .proof_sloc,
        15
    );
}

#[test]
fn inline_negative_cfgs_and_non_hax_crates_stay_runtime() {
    for (cfg, hax) in [
        ("hax", false),
        ("any(hax, unix)", true),
        ("not(hax)", true),
        ("feature=\"hax\"", true),
    ] {
        let root = fixture(
            &format!("fn run() {{\n#[cfg({cfg})]\n{{\nhelper();\n}}\nreal();\n}}"),
            hax,
        );
        let r = report(&root, GhostLines::Separate);
        assert_eq!(r.units[0].inline_proof_sloc, 0, "{cfg}");
    }
}

#[test]
fn rust_statement_grammar_keeps_following_runtime_owned() {
    for stmt in [
        "if check() { one(); } else { two(); }",
        "let p = if check() { one() } else { two() };",
        "let Some(x) = helper() else { return; };",
        "let p = || { one(); };",
        "helper! { one(); }",
        "while { check() } { one(); }",
        "Thing { value: 1 }.helper();",
    ] {
        let root = fixture(
            &format!("fn run() {{\n#[cfg(hax)]\n{stmt}\nreal();\n}}"),
            true,
        );
        let r = report(&root, GhostLines::Separate);
        assert_eq!(r.units[0].inline_proof_sloc, 2, "{stmt}");
        assert_eq!(r.units[0].sloc, 3, "{stmt}");
    }
}

#[test]
fn annotations_in_nested_proof_units_are_not_runtime_annotation_cost() {
    let root = fixture("fn outer() {\n#[cfg(hax)]\n#[hax_lib::ensures(true)]\nfn lemma() {\nhax_lib::assert!(true);\n}\nreal();\n}", true);
    let r = report(&root, GhostLines::Separate);
    assert!(r.units.iter().all(|u| u.fstar_annotation_sloc == 0));
    let proof = r.units.iter().find(|u| u.name == "lemma").unwrap();
    assert_eq!(proof.category, Category::ProofCode);
    assert_eq!(proof.sloc, 5);
    assert_eq!(
        formal_coverage::proof_ratio(&r)
            .all_first_party
            .fstar_annotation_sloc,
        0
    );
}

#[test]
fn already_proof_code_does_not_gain_a_tested_status() {
    let root = fixture(
        "verus! {\n#[cfg(hax)]\nproof fn lemma() {}\n}\n#[test] fn check() { lemma(); }",
        true,
    );
    let mut r = report(&root, GhostLines::Separate);
    formal_coverage::apply_evidence_many(&mut r, &root, &[], &[]).unwrap();
    let proof = r.units.iter().find(|u| u.name == "lemma").unwrap();
    assert_eq!(proof.category, Category::ProofCode);
    assert_eq!(proof.status, "unverified");
}

#[test]
fn cfg_reclassification_preserves_verus_eligibility_and_status() {
    let source = "verus! {\n#[cfg(hax)]\nfn credited() -> (r: u64) ensures r == 1, { 1 }\n#[cfg(hax)]\nfn no_contract() {}\n#[cfg(hax)]\nproof fn existing() {}\n}";
    let root = fixture(source, true);
    let log = "verification results:: 3 verified, 0 errors\n";
    fs::write(root.join("verus.log"), log).unwrap();
    let receipt = formal_coverage::Receipt {
        tool: "verus".into(),
        tool_version: "synthetic policy test".into(),
        command: ["verus", "--crate-type", "lib", "src/lib.rs"]
            .map(String::from)
            .to_vec(),
        exit_code: 0,
        cfg: Some(vec!["hax".into()]),
        files: std::collections::BTreeMap::from([(
            "src/lib.rs".into(),
            formal_coverage::sha256(&fs::read(root.join("src/lib.rs")).unwrap()),
        )]),
        log_sha256: formal_coverage::sha256(log.as_bytes()),
        harnesses: vec![],
        covered_units: Default::default(),
    };
    fs::write(
        root.join("verus.log.evidence.json"),
        serde_json::to_vec(&receipt).unwrap(),
    )
    .unwrap();
    let mut r = report(&root, GhostLines::Separate);
    formal_coverage::apply_evidence_many(&mut r, &root, &[root.join("verus.log")], &[]).unwrap();
    assert!(r.evidence_runs[0].accepted);
    for (name, status) in [
        ("credited", "proven"),
        ("no_contract", "unverified"),
        ("existing", "verified"),
    ] {
        let u = r.units.iter().find(|u| u.name == name).unwrap();
        assert_eq!(u.category, Category::ProofCode);
        assert_eq!(u.status, status, "{name}");
    }
    formal_coverage::gate(&r, 0., None, false).unwrap();
}
