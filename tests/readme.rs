//! Keep documented TOML syntax and the public ship schema executable.
use formal_coverage::Ship;

#[test]
fn readme_toml_examples_parse_including_ship_schema() {
    let readme = include_str!("../README.md");
    let mut examples = 0;
    let mut ship_examples = 0;
    for block in readme.split("```toml\n").skip(1) {
        let text = block.split("```").next().unwrap();
        let value: toml::Value = toml::from_str(text).expect("README TOML must parse");
        examples += 1;
        if let Some(ship) = value.get("ship") {
            let ship: Ship = ship
                .clone()
                .try_into()
                .expect("README ship schema must parse");
            assert!(!ship.packages.is_empty());
            assert!(!ship.reason.trim().is_empty());
            ship_examples += 1;
        }
    }
    assert!(examples > 0, "README TOML examples must be exercised");
    assert!(ship_examples > 0, "README must document [ship]");
}
