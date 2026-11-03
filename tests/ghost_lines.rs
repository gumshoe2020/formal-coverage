use formal_coverage::{analyze_with, GhostLines, Options, SizeMode};
use std::path::Path;

#[test]
fn hand_counted_inline_constructs_and_mixed_lines() {
    for mode in [GhostLines::Separate, GhostLines::Runtime] {
        let r = analyze_with(
            Path::new("fixtures/ghost"),
            &Options {
                size_mode: SizeMode::WholeCrate,
                ghost_lines: mode,
                ..Options::default()
            },
        )
        .unwrap();
        // Written from the fixture: braces/header/runtime statements stay runtime.
        for (name, runtime, ghost) in [
            ("signature", 4, 5),
            ("loops", 14, 7),
            ("statements", 5, 14),
            ("mixed", 1, 0),
            ("opaque", 6, 0),
            ("comments", 4, 0),
            ("literals", 3, 3),
            ("ordinary", 4, 0),
        ] {
            let u = r.units.iter().find(|u| u.name == name).unwrap();
            assert_eq!(u.ghost_sloc, ghost, "{name}");
            let expected = if mode == GhostLines::Separate {
                (runtime, ghost)
            } else {
                (runtime + ghost, 0)
            };
            assert_eq!(
                (u.runtime_sloc, u.inline_proof_sloc),
                expected,
                "{name} {mode:?}"
            );
            assert_eq!(u.sloc, u.runtime_sloc);
        }
    }
}

#[test]
fn unparseable_first_party_retains_every_nonblank_line_and_fails_closed() {
    let root = std::path::PathBuf::from(format!("logs/v1.6/malformed-{}", std::process::id()));
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname='malformed'\nversion='0.1.0'\n[workspace]\n",
    )
    .unwrap();
    std::fs::write(
        root.join("Cargo.lock"),
        "version = 4\n[[package]]\nname='malformed'\nversion='0.1.0'\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/lib.rs"),
        "fn broken() {\n // cannot prove this is harmless\n\n let x = 1;\n",
    )
    .unwrap();
    let mut r = analyze_with(
        &root,
        &Options {
            size_mode: SizeMode::WholeCrate,
            ..Options::default()
        },
    )
    .unwrap();
    formal_coverage::apply_evidence_many(&mut r, &root, &[], &[]).unwrap();
    assert_eq!(r.unparseable_files, ["src/lib.rs"]);
    assert_eq!((r.units[0].sloc, r.units[0].inline_proof_sloc), (3, 0));
    assert_eq!(r.units[0].status, "hazard");
    assert!(formal_coverage::gate(&r, 0., None, false)
        .unwrap()
        .iter()
        .any(|s| s.contains("unattributed file-scope hazard")));
    std::fs::remove_dir_all(root).unwrap();
}
