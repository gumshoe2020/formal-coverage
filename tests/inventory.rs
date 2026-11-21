use std::path::Path;
#[test]
fn exact_inventory() {
    let report = formal_coverage::inventory(Path::new("fixtures/inventory")).unwrap();
    let actual: Vec<_> = report
        .units
        .iter()
        .map(|u| (u.id.as_str(), u.line, u.sloc))
        .collect();
    assert_eq!(
        actual,
        vec![
            ("inventory_fixture::entry", 2, 3),
            ("inventory_fixture::helper", 5, 1),
            ("inventory_fixture::nested::other", 7, 3),
            ("inventory_fixture::checked", 13, 6)
        ]
    );
}
#[test]
fn comments_and_strings() {
    assert_eq!(
        formal_coverage::sloc("/* a\n /* b */\n */\nfn a() { // x\n let s = \"// hi\";\n}"),
        3
    );
}

#[test]
fn raw_literals_count_as_code() {
    let text = "fn f() {\n let s = r#\"a\n // literal content\n /* literal content */\n\"#;\n let c = '\"';\n}\n// actual comment";
    assert_eq!(formal_coverage::sloc(text), 7);
}

#[test]
fn ten_line_doc_example_does_not_inflate_a_unit() {
    let report = formal_coverage::inventory(Path::new("fixtures/doc_comments")).unwrap();
    let unit = report
        .units
        .iter()
        .find(|u| u.name == "documented")
        .unwrap();
    assert_eq!(unit.sloc, 3);
    assert_eq!(
        formal_coverage::sloc(include_str!("../fixtures/doc_comments/src/lib.rs")),
        3
    );
}

#[test]
fn all_doc_forms_are_ignored_at_every_token_depth() {
    let source = r#"//! Module docs
/*! Inner block
 * docs */
#![doc = include_str!("guide.md")]
/** Outer block
 * docs */
#[doc(hidden)]
#[doc = "x"]
fn f() {
    //! Nested docs
    /** Nested block */
    #[doc = "local"] let x = 1; /// not really
}
"#;
    assert_eq!(formal_coverage::sloc(source), 3);
    assert_eq!(formal_coverage::sloc("#[doc = \"x\"] fn f() {}"), 1);
}

#[test]
fn other_attributes_and_doc_text_in_literals_still_count() {
    let source = r##"#[inline]
#[cfg(test)]
#[hax_lib::requires(true)]
#[verifier::external_body]
#[documented]
fn f() {
    let text = "/// doc-ish";
    let attr = r#"#[doc = "x"]"#;
    let multiline = "///
#[doc = text]
";
}
"##;
    assert_eq!(formal_coverage::sloc(source), 12);
}

#[test]
fn malformed_fallback_still_includes_comments() {
    assert_eq!(
        formal_coverage::sloc("fn broken() {\n/// doc\n// ordinary\n\n"),
        3
    );
}
