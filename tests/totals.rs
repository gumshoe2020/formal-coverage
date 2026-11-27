//! Line-total receipts (`report --totals-json`) on a bertie-style hax/F* fixture.
mod common;
use common::{git, run_git};
use formal_coverage::{
    sha256,
    totals::{parse_source, resolve_features, PackageSource, TotalsReceipt},
    FstarModule, FstarReceipt,
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
const URL: &str = "https://example.invalid/hax_fixture";
const MANIFEST: &str = "[package]\nname = \"hax_fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[features]\ndefault = [\"std\"]\nstd = []\nextra = [\"std\"]\n\n[workspace]\n";
const CONFIG: &str = "[ship]\npackages = [\"hax_fixture\"]\nfeatures = [\"default\"]\nreason = \"the library ships\"\n";

/// A committed hax crate with a [ship] section and a strict F* receipt beside it.
struct Fixture {
    root: PathBuf,
    log: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let base = std::env::temp_dir().join(format!(
            "fc-totals-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&base);
        let root = base.join("hax_fixture");
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("Cargo.toml"), MANIFEST).unwrap();
        fs::write(root.join("formal-coverage.toml"), CONFIG).unwrap();
        fs::write(root.join(".gitignore"), "/target\n").unwrap();
        for file in ["src/lib.rs", "hax-driver.py"] {
            fs::copy(Path::new("fixtures/hax").join(file), root.join(file)).unwrap();
        }
        let lock = Command::new("cargo")
            .args(["generate-lockfile", "--offline"])
            .current_dir(&root)
            .output()
            .unwrap();
        assert!(
            lock.status.success(),
            "{}",
            String::from_utf8_lossy(&lock.stderr)
        );
        let log = write_fstar_evidence(&root, &base.join("evidence"));
        run_git(&root, &["init", "-q"]);
        commit(&root);
        Self { root, log }
    }
    fn head(&self) -> String {
        let out = git(&self.root)
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        String::from_utf8(out.stdout).unwrap().trim().into()
    }
    fn source(&self) -> String {
        format!("git+{URL}#{}", self.head())
    }
    fn receipt_path(&self, name: &str) -> PathBuf {
        self.root.parent().unwrap().join(name)
    }
    fn report(&self, name: &str, source: &str, extra: &[&str]) -> Output {
        let parent = self.root.parent().unwrap();
        cli(&self.root)
            .arg("--fstar-log")
            .arg(&self.log)
            .arg("--check-target-dir")
            .arg(parent.join("target"))
            .arg("--json")
            .arg(parent.join(format!("{name}.report.json")))
            .arg("--totals-json")
            .arg(self.receipt_path(name))
            .arg("--totals-source")
            .arg(source)
            .args(extra)
            .output()
            .unwrap()
    }
    fn receipt(&self, name: &str) -> (Vec<u8>, TotalsReceipt) {
        let output = self.report(name, &self.source(), &[]);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let bytes = fs::read(self.receipt_path(name)).unwrap();
        let receipt = serde_json::from_slice(&bytes).unwrap();
        (bytes, receipt)
    }
}

fn cli(root: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_formal-coverage"));
    for var in common::local_env_vars() {
        command.env_remove(var);
    }
    command.arg("report").arg(root);
    command
}

fn commit(root: &Path) {
    run_git(root, &["add", "-A"]);
    run_git(root, &["commit", "-q", "-m", "fixture"]);
}

fn write_fstar_evidence(root: &Path, dir: &Path) -> PathBuf {
    let files = ["Cargo.toml", "Cargo.lock", "src/lib.rs", "hax-driver.py"]
        .into_iter()
        .map(|f| (f.to_string(), sha256(&fs::read(root.join(f)).unwrap())))
        .collect();
    let mut fresh = BTreeMap::new();
    let mut checked = BTreeMap::new();
    for name in ["Hax_fixture.fst", "Hax_fixture.fsti"] {
        let bytes = fs::read(Path::new("fixtures/hax/extraction").join(name)).unwrap();
        for (sub, map) in [("fresh", &mut fresh), ("checked", &mut checked)] {
            fs::create_dir_all(dir.join(sub)).unwrap();
            fs::write(dir.join(sub).join(name), &bytes).unwrap();
            map.insert(format!("{sub}/{name}"), sha256(&bytes));
        }
    }
    let log = dir.join("fstar.log");
    fs::write(&log, "Verified module: Hax_fixture\n").unwrap();
    let receipt = FstarReceipt {
        tool: "hax-fstar".into(),
        hax_version: "synthetic policy fixture".into(),
        hax_commit: "1".repeat(40),
        fstar_version: "synthetic policy fixture".into(),
        extraction_command: ["python3", "hax-driver.py", "extract-fstar"]
            .map(String::from)
            .to_vec(),
        extraction_exit_code: 0,
        files,
        fresh,
        checked,
        patches: BTreeMap::new(),
        log_sha256: sha256(&fs::read(&log).unwrap()),
        modules: vec![FstarModule {
            file: "checked/Hax_fixture.fst".into(),
            command: vec!["fstar.exe".into(), "checked/Hax_fixture.fst".into()],
            exit_code: 0,
            outcome: "success".into(),
        }],
    };
    fs::write(
        format!("{}.evidence.json", log.display()),
        serde_json::to_vec(&receipt).unwrap(),
    )
    .unwrap();
    log
}

fn refused(output: &Output, reason: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "expected refusal: {reason}");
    assert!(stderr.contains(reason), "{reason}: {stderr}");
}

#[test]
fn hax_fixture_receipt_end_to_end() {
    let f = Fixture::new();
    let (bytes, receipt) = f.receipt("receipt.json");
    assert_eq!(receipt.schema, 1);
    assert_eq!(receipt.kit.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(receipt.kit.binary_sha256.len(), 64);
    assert_eq!(receipt.package.name, "hax_fixture");
    assert_eq!(receipt.package.version, "0.1.0");
    assert_eq!(
        receipt.package.source,
        PackageSource::Git {
            url: URL.into(),
            rev: f.head()
        }
    );
    assert_eq!(receipt.features, ["default", "std"]);
    assert_eq!(receipt.scope, "first-party security runtime");
    assert_eq!(receipt.bucket_map["unverified"], ["unverified", "hazard"]);
    assert_eq!(receipt.bucket_map["proven"].len(), 4);
    assert!(receipt.totals.proven > 0, "{receipt:?}");
    assert_eq!(receipt.totals.sum(), receipt.total);
    assert!(receipt.evidence[0].starts_with("hax-fstar receipt accepted"));
    let report = fs::read(f.receipt_path("receipt.json.report.json")).unwrap();
    assert_eq!(receipt.report_sha256, sha256(&report));
    let summary: serde_json::Value = serde_json::from_slice(&report).unwrap();
    let side = &summary["summary"]["first_party"]["security_relevant"];
    assert_eq!(side["sloc"], receipt.total);
    assert_eq!(side["statuses"]["proven-extracted"], receipt.totals.proven);
    let text = String::from_utf8(bytes).unwrap();
    assert!(!text.contains(&*std::env::temp_dir().to_string_lossy()));
    assert!(!text.contains("\"/"), "absolute path in receipt");
    assert!(text.ends_with("}\n"));
}

#[test]
fn receipt_bytes_are_identical_across_runs() {
    let f = Fixture::new();
    let (first, _) = f.receipt("first.json");
    let (second, _) = f.receipt("second.json");
    assert_eq!(first, second);
}

#[test]
fn ship_features_override_is_recorded_resolved() {
    let f = Fixture::new();
    let output = f.report("extra.json", &f.source(), &["--ship-features", "extra"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: TotalsReceipt =
        serde_json::from_slice(&fs::read(f.receipt_path("extra.json")).unwrap()).unwrap();
    assert_eq!(receipt.features, ["extra", "std"]);
}

#[test]
fn dirty_checkout_is_refused() {
    let f = Fixture::new();
    fs::write(
        f.root.join("src/lib.rs"),
        "pub fn clean(x: u32) -> u32 { x + 0 }\n",
    )
    .unwrap();
    refused(&f.report("r.json", &f.source(), &[]), "uncommitted changes");
    assert!(!f.receipt_path("r.json").exists());
    let f = Fixture::new();
    fs::write(f.root.join("notes.txt"), "untracked\n").unwrap();
    refused(&f.report("r.json", &f.source(), &[]), "notes.txt");
}

#[test]
fn rev_other_than_head_is_refused() {
    let f = Fixture::new();
    let other = format!("git+{URL}#{}", "0".repeat(40));
    refused(&f.report("r.json", &other, &[]), "is not the receipt's rev");
}

#[test]
fn unmatched_or_dirty_config_is_refused() {
    let f = Fixture::new();
    fs::write(f.root.join("formal-coverage.toml"), "").unwrap();
    commit(&f.root);
    refused(&f.report("r.json", &f.source(), &[]), "needs [ship]");

    let f = Fixture::new();
    let todo = format!(
        "{CONFIG}\n[[security.include]]\npattern = \"hax_fixture::clean\"\nreason = \"TODO\"\n"
    );
    fs::write(f.root.join("formal-coverage.toml"), todo).unwrap();
    commit(&f.root);
    refused(&f.report("r.json", &f.source(), &[]), "dirty config");

    let f = Fixture::new();
    refused(
        &f.report("r.json", &f.source(), &["--ship-features", "missing"]),
        "unmatched config",
    );
}

#[test]
fn malformed_sources_are_refused() {
    let f = Fixture::new();
    let short = format!("git+{URL}#{}", &f.head()[..12]);
    refused(&f.report("r.json", &short, &[]), "full 40-hex commit");
    refused(
        &f.report("r.json", "path+file:///x", &[]),
        "package source must be",
    );
}

#[test]
fn cargo_source_strings_parse_to_package_sources() {
    let rev = "9b55643a3724bf347dd45e998844bf18776dfccd";
    let lock = format!("git+https://github.com/gumshoe2020/bertie-tls?rev={rev}#{rev}");
    assert_eq!(
        parse_source(&lock).unwrap(),
        PackageSource::Git {
            url: "https://github.com/gumshoe2020/bertie-tls".into(),
            rev: rev.into()
        }
    );
    let dotgit = format!("git+https://github.com/gumshoe2020/bertie-tls.git#{rev}");
    assert_eq!(parse_source(&dotgit).unwrap(), parse_source(&lock).unwrap());
    let sum = "a".repeat(64);
    assert_eq!(
        parse_source(&format!("checksum:{sum}")).unwrap(),
        PackageSource::Checksum(sum)
    );
    assert!(parse_source("checksum:ABC").is_err());
}

#[test]
fn features_expand_like_cargo() {
    let table: BTreeMap<String, Vec<String>> = [
        ("default", vec!["api", "std"]),
        ("api", vec![]),
        ("std", vec![]),
        ("aead", vec!["std", "dep:aead-crate"]),
        ("weak", vec!["aead?/x"]),
        ("strong", vec!["aead/x"]),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.into_iter().map(String::from).collect()))
    .collect();
    let resolve = |r: &[&str]| {
        resolve_features(&table, &r.iter().map(|s| s.to_string()).collect::<Vec<_>>()).unwrap()
    };
    assert_eq!(resolve(&["default"]), ["api", "default", "std"]);
    assert_eq!(resolve(&["aead"]), ["aead", "std"]);
    assert_eq!(resolve(&["weak"]), ["weak"]);
    assert_eq!(resolve(&["strong"]), ["aead", "std", "strong"]);
    assert!(resolve(&[]).is_empty());
    assert!(resolve_features(&table, &["nope".into()]).is_err());
}

/// A consumer crate depending on the committed fixture through a local git URL.
struct Link {
    producer: Fixture,
    consumer: PathBuf,
    receipt: serde_json::Value,
}

impl Link {
    fn new() -> Self {
        let producer = Fixture::new();
        let url = format!("file://{}", producer.root.display());
        let source = format!("git+{url}#{}", producer.head());
        let output = producer.report("link.json", &source, &["--ship-features", "extra"]);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let receipt =
            serde_json::from_slice(&fs::read(producer.receipt_path("link.json")).unwrap()).unwrap();
        let consumer = producer.root.parent().unwrap().join("consumer");
        fs::create_dir_all(consumer.join("src")).unwrap();
        let manifest = format!(
            "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nhax_fixture = {{ git = \"{url}\", rev = \"{}\", default-features = false, features = [\"extra\"] }}\n\n[workspace]\n",
            producer.head()
        );
        fs::write(consumer.join("Cargo.toml"), manifest).unwrap();
        fs::write(
            consumer.join("src/main.rs"),
            "fn main() {\n    println!(\"{}\", hax_fixture::clean(1));\n}\n",
        )
        .unwrap();
        let link = Self {
            producer,
            consumer,
            receipt,
        };
        for args in [&["generate-lockfile"][..], &["fetch"][..]] {
            let out = link
                .cargo_env(&mut Command::new("cargo"))
                .args(args)
                .current_dir(&link.consumer)
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
        link
    }
    fn cargo_env<'a>(&self, command: &'a mut Command) -> &'a mut Command {
        command.env(
            "CARGO_HOME",
            self.producer.root.parent().unwrap().join("cargo-home"),
        )
    }
    fn write_receipt(&self, name: &str, edit: impl FnOnce(&mut serde_json::Value)) -> PathBuf {
        let mut receipt = self.receipt.clone();
        edit(&mut receipt);
        let path = self.producer.root.parent().unwrap().join(name);
        fs::write(&path, serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
        path
    }
    fn run(&self, name: &str, receipts: &[&Path], extra: &[&str]) -> Output {
        let parent = self.producer.root.parent().unwrap();
        let mut command = cli(&self.consumer);
        self.cargo_env(&mut command)
            .arg("--check-target-dir")
            .arg(parent.join("consumer-target"))
            .arg("--json")
            .arg(parent.join(format!("{name}.json")))
            .arg("--markdown")
            .arg(parent.join(format!("{name}.md")))
            .args(extra);
        if !receipts.is_empty() {
            command.arg("--dependency-totals").args(receipts);
        }
        command.output().unwrap()
    }
    fn output(&self, name: &str, ext: &str) -> Vec<u8> {
        fs::read(
            self.producer
                .root
                .parent()
                .unwrap()
                .join(format!("{name}.{ext}")),
        )
        .unwrap()
    }
    fn refused(&self, reason: &str, edit: impl FnOnce(&mut serde_json::Value)) {
        let path = self.write_receipt("edited.json", edit);
        refused(&self.run("edited", &[&path], &[]), reason);
    }
}

fn json(bytes: &[u8]) -> serde_json::Value {
    serde_json::from_slice(bytes).unwrap()
}

#[test]
fn consumer_counts_receipt_buckets_and_keeps_first_party_bytes() {
    let link = Link::new();
    let receipt = link.write_receipt("ok.json", |_| {});
    let without = link.run("without", &[], &[]);
    assert!(
        without.status.success(),
        "{}",
        String::from_utf8_lossy(&without.stderr)
    );
    let with = link.run("with", &[&receipt], &[]);
    assert!(
        with.status.success(),
        "{}",
        String::from_utf8_lossy(&with.stderr)
    );

    let (a, b) = (
        json(&link.output("without", "json")),
        json(&link.output("with", "json")),
    );
    for key in [
        "units",
        "proof_ratio",
        "summary",
        "summary_default_rules",
        "inputs",
    ] {
        assert_eq!(
            serde_json::to_vec(&a[key]).unwrap(),
            serde_json::to_vec(&b[key]).unwrap(),
            "{key} changed with a receipt"
        );
    }
    let mut stripped = b.clone();
    stripped.as_object_mut().unwrap().remove("shipped_tree");
    for d in stripped["dependencies"].as_array_mut().unwrap() {
        d.as_object_mut().unwrap().remove("receipt");
    }
    assert_eq!(
        serde_json::to_vec(&a).unwrap(),
        serde_json::to_vec(&stripped).unwrap()
    );
    assert!(a.get("shipped_tree").is_none());

    let dependency = b["dependencies"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["name"] == "hax_fixture")
        .unwrap();
    let r = &link.receipt;
    assert_eq!(dependency["receipt"]["buckets"], r["totals"]);
    let sloc = dependency["sloc"].as_u64().unwrap();
    assert_eq!(sloc, r["shipped_files_sloc"].as_u64().unwrap());
    let outside = sloc - r["total"].as_u64().unwrap();
    assert_eq!(dependency["receipt"]["outside_receipt"], outside);
    assert_eq!(
        dependency["receipt"]["receipt_sha256"],
        sha256(&fs::read(&receipt).unwrap())
    );
    let tree = &b["shipped_tree"];
    let first = &b["summary"]["first_party"]["security_relevant"];
    assert_eq!(tree["first_party"]["tested"], first["statuses"]["tested"]);
    assert_eq!(tree["third_party"]["proven"], r["totals"]["proven"]);
    assert_eq!(
        tree["third_party"]["unverified"].as_u64().unwrap(),
        r["totals"]["unverified"].as_u64().unwrap() + outside
    );
    let total: u64 = ["proven", "tested", "trusted", "unverified"]
        .iter()
        .map(|k| tree["total"][k].as_u64().unwrap())
        .sum();
    assert_eq!(total, first["sloc"].as_u64().unwrap() + sloc);
    assert_eq!(tree["receipts"], 1);

    let (md_a, md_b) = (
        String::from_utf8(link.output("without", "md")).unwrap(),
        String::from_utf8(link.output("with", "md")).unwrap(),
    );
    let start = md_b.find("\n## Shipped tree").unwrap();
    let end = md_b.find("\n## Evidence and limitations").unwrap();
    assert_eq!(format!("{}{}", &md_b[..start], &md_b[end..]), md_a);
    assert!(md_b[start..end].contains("| hax_fixture 0.1.0 |"));
}

#[test]
fn every_rule_has_a_failing_receipt() {
    let link = Link::new();
    link.refused("is not this build's locked source", |r| {
        r["package"]["source"]["git"]["rev"] = "0".repeat(40).into()
    });
    link.refused("is not this build's locked source", |r| {
        r["package"]["source"]["git"]["url"] = "https://example.invalid/other".into()
    });
    link.refused("resolved features", |r| {
        r["features"] = serde_json::json!(["default", "std"])
    });
    link.refused("is not this build's target", |r| {
        r["target"] = "x86_64-unknown-none".into()
    });
    link.refused("never mix kit versions", |r| {
        r["kit"]["version"] = "1.12.3".into()
    });
    link.refused("matches no runtime dependency", |r| {
        r["package"]["name"] = "other".into()
    });
    link.refused("do not add up", |r| {
        r["totals"]["proven"] = (r["totals"]["proven"].as_u64().unwrap() + 1).into()
    });
    link.refused("oversized receipt", |r| {
        let extra = r["shipped_files_sloc"].as_u64().unwrap() + 1;
        r["totals"]["unverified"] = (r["totals"]["unverified"].as_u64().unwrap() + extra).into();
        r["total"] = (r["total"].as_u64().unwrap() + extra).into();
        r["shipped_files_sloc"] = (r["shipped_files_sloc"].as_u64().unwrap() + extra).into();
    });
    link.refused("not the same files", |r| {
        r["shipped_files_sloc"] = (r["shipped_files_sloc"].as_u64().unwrap() + 1).into()
    });
    let ok = link.write_receipt("ok.json", |_| {});
    refused(
        &link.run("duplicate", &[&ok, &ok], &[]),
        "count each package once",
    );
    let other_features = link.write_receipt("features.json", |r| {
        r["features"] = serde_json::json!(["std"])
    });
    refused(
        &link.run("twice", &[&ok, &other_features], &[]),
        "count each package once",
    );
    refused(
        &link.run("whole", &[&ok], &["--size-mode", "whole-crate"]),
        "compiled-files measurement",
    );
}
