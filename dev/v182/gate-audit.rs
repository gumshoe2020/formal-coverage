use formal_coverage::{gate, Report};
use std::{env, fs};

fn main() {
    for path in env::args().skip(1) {
        let report: Report = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let failures = gate(&report, 0., None, false).unwrap();
        println!("{path}: {}", serde_json::to_string(&failures).unwrap());
        assert!(failures.is_empty(), "HONEST-FINDING: Bertie gate tripped");
    }
}
