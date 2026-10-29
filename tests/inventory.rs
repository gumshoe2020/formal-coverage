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
