use formal_coverage::{gate, Report};
use std::{env, fs};
fn labels(failures: &[String]) -> Vec<String> {
    failures.iter().map(|f| if f.starts_with("first-party security proof ratio ") {
        if f.contains(" is below ") { "ratio below range".into() } else { "ratio above range".into() }
    } else { f.clone() }).collect()
}
fn main() {
    let paths: Vec<_> = env::args().skip(1).collect();
    let reports: Vec<Report> = paths.iter().map(|p| serde_json::from_slice(&fs::read(p).unwrap()).unwrap()).collect();
    let before = gate(&reports[0], 0., None, false).unwrap();
    for (path, r) in paths.iter().zip(&reports) {
        let failures = gate(r, 0., None, false).unwrap();
        println!("{path}: {}", serde_json::to_string(&failures).unwrap());
        assert_eq!(labels(&before), labels(&failures), "HONEST-FINDING: new Bertie gate failure");
        let comparison = gate(r, 0., Some(&reports[0]), true).unwrap();
        assert_eq!(labels(&failures), labels(&comparison), "HONEST-FINDING: baseline/escape gate");
    }
    println!("PASS: gate failure reasons unchanged; baseline and no-new-escape checks add no failures");
}
