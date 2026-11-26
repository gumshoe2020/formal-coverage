//! Historical strict checks are reusable only against a fresh dependency context.
use super::{command, inside, FstarModule, FstarReceipt};
use crate::sha256;
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Toolchain {
    pub fstar: String,
    pub z3: String,
    pub hax: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleContext {
    pub file: String,
    pub command: Vec<String>,
    pub dependencies: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckContext {
    pub schema: u32,
    pub toolchain: Toolchain,
    pub modules: Vec<ModuleContext>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleReceipt {
    pub schema: u32,
    pub module: String,
    pub main_commit: String,
    pub file: String,
    pub toolchain: Toolchain,
    pub command: Vec<String>,
    pub dependencies: BTreeMap<String, String>,
    pub log: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReuseManifest {
    pub schema: u32,
    pub data_branch: String,
    pub main_commit: String,
    pub receipts: BTreeMap<String, ModuleReceipt>,
}
fn commit(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|b| b.is_ascii_hexdigit())
}
fn module(file: &str) -> Result<&str> {
    Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .context("module file")
}
fn strict_command(
    base: &Path,
    args: &[String],
    file: &str,
    deps: &BTreeMap<String, String>,
) -> bool {
    let mut scope = deps.clone();
    for pair in args.windows(2).filter(|p| p[0] == "--include") {
        if !inside(base, &pair[1]).is_ok_and(|p| p.is_dir()) {
            return false;
        }
        scope.insert(format!("{}/__include_scope.fst", pair[1]), String::new());
    }
    command(args, file, &scope)
}
pub fn load_context(path: &Path) -> Result<CheckContext> {
    let ctx: CheckContext = serde_json::from_slice(&fs::read(path)?)?;
    ensure!(ctx.schema == 1, "context schema");
    ensure!(
        !ctx.toolchain.fstar.trim().is_empty()
            && !ctx.toolchain.z3.trim().is_empty()
            && !ctx.toolchain.hax.trim().is_empty(),
        "context toolchain identity"
    );
    ensure!(!ctx.modules.is_empty(), "context modules empty");
    let mut seen = BTreeSet::new();
    for m in &ctx.modules {
        let mut check = || -> Result<()> {
            ensure!(seen.insert(&m.file), "duplicate context module");
            ensure!(m.file.ends_with(".fst"), "module file extension");
            let deps: BTreeMap<_, _> = m
                .dependencies
                .iter()
                .map(|p| (p.clone(), String::new()))
                .collect();
            ensure!(
                deps.len() == m.dependencies.len(),
                "duplicate dependency entry"
            );
            ensure!(
                deps.contains_key(&m.file),
                "missing module dependency entry"
            );
            ensure!(
                strict_command(
                    path.parent().unwrap_or(Path::new(".")),
                    &m.command,
                    &m.file,
                    &deps
                ),
                "options: unsupported strict command"
            );
            for p in &m.dependencies {
                ensure!(
                    matches!(
                        Path::new(p).extension().and_then(|s| s.to_str()),
                        Some("fst" | "fsti")
                    ),
                    "dependency extension: {p}"
                );
                inside(path.parent().unwrap_or(Path::new(".")), p)
                    .with_context(|| format!("dependency path: {p}"))?;
            }
            Ok(())
        };
        check().with_context(|| format!("{} context", m.file))?;
    }
    Ok(ctx)
}
/// Compare the set, not just recorded entries: omissions must never pass.
pub fn verify(
    base: &Path,
    ctx: &CheckContext,
    current: &ModuleContext,
    r: &ModuleReceipt,
) -> Result<()> {
    let check = || -> Result<()> {
        ensure!(r.schema == 1, "schema");
        ensure!(commit(&r.main_commit), "main_commit");
        ensure!(
            r.file == current.file && r.module == module(&current.file)?,
            "module identity"
        );
        ensure!(r.toolchain.fstar == ctx.toolchain.fstar, "toolchain.fstar");
        ensure!(r.toolchain.z3 == ctx.toolchain.z3, "toolchain.z3");
        ensure!(r.toolchain.hax == ctx.toolchain.hax, "toolchain.hax");
        ensure!(r.command == current.command, "options");
        ensure!(
            r.dependencies.keys().collect::<BTreeSet<_>>() == current.dependencies.iter().collect(),
            "dependencies: missing or extra entry"
        );
        ensure!(
            strict_command(base, &r.command, &r.file, &r.dependencies),
            "options: unsupported strict command"
        );
        for (p, hash) in &r.dependencies {
            ensure!(
                sha256(&fs::read(
                    inside(base, p).with_context(|| format!("dependency path: {p}"))?
                )?) == *hash,
                "dependencies hash: {p}"
            );
        }
        ensure!(
            r.log
                .lines()
                .any(|l| l.trim() == format!("Verified module: {}", r.module)),
            "log: missing verified module"
        );
        Ok(())
    };
    check().with_context(|| format!("module {} receipt", current.file))
}
/// Selection and report use exactly the same verifier.
pub fn check_receipt(context: &Path, receipt: &Path) -> Result<()> {
    let ctx = load_context(context)?;
    let r: ModuleReceipt = serde_json::from_slice(&fs::read(receipt)?)?;
    let m = ctx
        .modules
        .iter()
        .find(|m| m.file == r.file)
        .context("receipt for another module")?;
    verify(context.parent().unwrap_or(Path::new(".")), &ctx, m, &r)
}

fn receipt_fields(value: serde_json::Value) -> Result<ModuleReceipt> {
    fn field<T: serde::de::DeserializeOwned>(value: &serde_json::Value, key: &str) -> Result<T> {
        serde_json::from_value(
            value
                .get(key)
                .with_context(|| format!("missing field {key}"))?
                .clone(),
        )
        .with_context(|| format!("field {key}"))
    }
    let object = value
        .as_object()
        .context("receipt fields must be an object")?;
    for key in object.keys() {
        ensure!(
            [
                "schema",
                "module",
                "main_commit",
                "file",
                "toolchain",
                "command",
                "dependencies",
                "log"
            ]
            .contains(&key.as_str()),
            "unknown field {key}"
        );
    }
    let schema = field(&value, "schema")?;
    let tools = value.get("toolchain").context("missing field toolchain")?;
    let parse_tools = || -> Result<Toolchain> {
        for key in tools
            .as_object()
            .context("expected toolchain object")?
            .keys()
        {
            ensure!(
                ["fstar", "z3", "hax"].contains(&key.as_str()),
                "unknown field {key}"
            );
        }
        Ok(Toolchain {
            fstar: field(tools, "fstar")?,
            z3: field(tools, "z3")?,
            hax: field(tools, "hax")?,
        })
    };
    Ok(ModuleReceipt {
        schema,
        module: field(&value, "module")?,
        main_commit: field(&value, "main_commit")?,
        file: field(&value, "file")?,
        toolchain: parse_tools().context("field toolchain")?,
        command: field(&value, "command")?,
        dependencies: field(&value, "dependencies")?,
        log: field(&value, "log")?,
    })
}

pub(super) fn load_manifest(path: &Path) -> Result<ReuseManifest> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct RawManifest {
        schema: u32,
        data_branch: String,
        main_commit: String,
        receipts: BTreeMap<String, serde_json::Value>,
    }
    let raw: RawManifest =
        serde_json::from_slice(&fs::read(path)?).context("reuse manifest schema/JSON")?;
    let mut receipts = BTreeMap::new();
    for (file, value) in raw.receipts {
        let r = receipt_fields(value).with_context(|| format!("module {file} receipt fields"))?;
        receipts.insert(file, r);
    }
    Ok(ReuseManifest {
        schema: raw.schema,
        data_branch: raw.data_branch,
        main_commit: raw.main_commit,
        receipts,
    })
}
/// Bind supplied historical records back to the fetched main-only data branch.
/// Hash comparison alone cannot distinguish a PR-forged record from main's record.
pub(super) fn verify_origin(root: &Path, reuse: &ReuseManifest) -> Result<()> {
    let branch = &reuse.data_branch;
    ensure!(
        !["main", "master", "badges", "HEAD"].contains(&branch.as_str())
            && !branch.contains("..")
            && !branch.ends_with(".lock")
            && branch.split('/').all(|part| !part.is_empty()
                && part
                    .bytes()
                    .next()
                    .is_some_and(|b| b.is_ascii_alphanumeric() || b == b'_')
                && part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))),
        "reuse data_branch"
    );
    ensure!(commit(&reuse.main_commit), "reuse main_commit");
    for (file, supplied) in &reuse.receipts {
        let name = module(file)?;
        ensure!(
            name.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_.".contains(&b)),
            "module {file} identity"
        );
        let mut recorded = None;
        for prefix in ["refs/heads/", "refs/remotes/origin/"] {
            let spec = format!(
                "{prefix}{branch}:receipts/{}/{name}.json",
                reuse.main_commit
            );
            let output = std::process::Command::new("git")
                .arg("-C")
                .arg(root)
                .args(["show", &spec])
                .output()?;
            if output.status.success() {
                recorded = Some(output.stdout);
                break;
            }
        }
        let bytes = recorded
            .with_context(|| format!("module {file} origin: missing main data-branch receipt"))?;
        let historical: serde_json::Value = serde_json::from_slice(&bytes)
            .with_context(|| format!("module {file} origin receipt JSON"))?;
        ensure!(
            historical == serde_json::to_value(supplied)?,
            "module {file} origin: supplied receipt differs from main data-branch fields"
        );
    }
    Ok(())
}
pub(super) fn merge(
    log_path: &Path,
    receipt: &mut FstarReceipt,
    log: &mut Vec<u8>,
    reuse: &ReuseManifest,
) -> Result<Vec<String>> {
    let ctx = load_context(Path::new(&format!("{}.context.json", log_path.display())))?;
    ensure!(
        reuse.schema == 1 && commit(&reuse.main_commit),
        "reuse schema/main_commit"
    );
    let base = log_path.parent().unwrap_or(Path::new("."));
    ensure!(
        ctx.toolchain
            .hax
            .starts_with(&format!("{}@{}@", receipt.hax_version, receipt.hax_commit)),
        "module {} context toolchain.hax does not identify current extraction",
        ctx.modules[0].file
    );
    ensure!(
        ctx.toolchain
            .fstar
            .starts_with(&format!("{}@", receipt.fstar_version)),
        "module {} context toolchain.fstar does not identify current run",
        ctx.modules[0].file
    );
    let mut labels = vec![];
    let mut consumed = BTreeSet::new();
    for m in &ctx.modules {
        // Current extraction and dependency analysis must agree on every project/library file.
        for p in &m.dependencies {
            if let Some(hash) = receipt.checked.get(p) {
                ensure!(
                    sha256(&fs::read(inside(base, p)?)?) == *hash,
                    "module {} current dependency hash: {p}",
                    m.file
                );
            }
        }
        let fresh: Vec<_> = receipt
            .modules
            .iter()
            .filter(|f| f.file == m.file)
            .collect();
        if let Some(r) = reuse.receipts.get(&m.file) {
            ensure!(fresh.is_empty(), "module {}: both fresh and reused", m.file);
            verify(base, &ctx, m, r)?;
            ensure!(
                r.main_commit == reuse.main_commit,
                "module {} main_commit",
                m.file
            );
            ensure!(
                receipt.fresh.iter().any(|(p, h)| Path::new(p).file_name()
                    == Path::new(&m.file).file_name()
                    && Some(h) == r.dependencies.get(&m.file)),
                "module {} extracted module hash",
                m.file
            );
            let interface = Path::new(&m.file)
                .with_extension("fsti")
                .to_string_lossy()
                .to_string();
            ensure!(
                !receipt.checked.contains_key(&interface)
                    || r.dependencies.get(&interface) == receipt.checked.get(&interface),
                "module {} missing interface dependency entry",
                m.file
            );
            receipt.modules.push(FstarModule {
                file: m.file.clone(),
                command: m.command.clone(),
                exit_code: 0,
                outcome: "success".into(),
            });
            log.extend_from_slice(b"\n");
            log.extend_from_slice(r.log.as_bytes());
            labels.push(format!("| {} | reused | {} |", r.module, r.main_commit));
            consumed.insert(&m.file);
        } else {
            ensure!(
                fresh.len() == 1 && fresh[0].command == m.command,
                "module {} fresh outcome/options missing or duplicated",
                m.file
            );
            labels.push(format!("| {} | fresh | current run |", module(&m.file)?));
        }
    }
    ensure!(
        consumed.len() == reuse.receipts.len(),
        "reuse receipt for another module"
    );
    ensure!(
        receipt.modules.len() == ctx.modules.len(),
        "context module scope mismatch"
    );
    // Preserve the old module ordering for byte-identical coverage reports.
    receipt.modules.sort_by(|a, b| a.file.cmp(&b.file));
    Ok(labels)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TempDir(std::path::PathBuf);
    impl TempDir {
        fn new() -> std::io::Result<Self> {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let p = std::env::temp_dir().join(format!(
                "fc-reuse-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ));
            fs::create_dir(&p)?;
            Ok(Self(p))
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn fixture() -> (TempDir, CheckContext, ModuleReceipt) {
        let dir = TempDir::new().unwrap();
        let files = ["Root.fst", "Root.fsti", "Dep.fst", "Library.fsti"];
        for p in files {
            fs::write(dir.path().join(p), p).unwrap();
        }
        let toolchain = Toolchain {
            fstar: "F* identity".into(),
            z3: "Z3 identity".into(),
            hax: "hax identity".into(),
        };
        let command = vec!["fstar.exe".into(), "Root.fst".into()];
        let ctx = CheckContext {
            schema: 1,
            toolchain: toolchain.clone(),
            modules: vec![ModuleContext {
                file: "Root.fst".into(),
                command: command.clone(),
                dependencies: files.iter().map(|s| s.to_string()).collect(),
            }],
        };
        let r = ModuleReceipt {
            schema: 1,
            module: "Root".into(),
            main_commit: "a".repeat(40),
            file: "Root.fst".into(),
            toolchain,
            command,
            dependencies: files
                .iter()
                .map(|p| (p.to_string(), sha256(p.as_bytes())))
                .collect(),
            log: "Verified module: Root\n".into(),
        };
        (dir, ctx, r)
    }
    #[test]
    fn identical_inputs_only() {
        let (dir, ctx, r) = fixture();
        verify(dir.path(), &ctx, &ctx.modules[0], &r).unwrap();
        for p in ["Root.fst", "Root.fsti", "Dep.fst", "Library.fsti"] {
            fs::write(dir.path().join(p), "changed").unwrap();
            let error = verify(dir.path(), &ctx, &ctx.modules[0], &r).unwrap_err();
            assert!(format!("{error:#}").contains(p));
            fs::write(dir.path().join(p), p).unwrap();
        }
    }
    #[test]
    fn every_receipt_tamper_fails_with_module_and_field() {
        let (dir, ctx, r) = fixture();
        let cases: Vec<(&str, ModuleReceipt)> = vec![
            (
                "toolchain.fstar",
                ModuleReceipt {
                    toolchain: Toolchain {
                        fstar: "other".into(),
                        ..r.toolchain.clone()
                    },
                    ..r.clone()
                },
            ),
            (
                "toolchain.z3",
                ModuleReceipt {
                    toolchain: Toolchain {
                        z3: "other".into(),
                        ..r.toolchain.clone()
                    },
                    ..r.clone()
                },
            ),
            (
                "toolchain.hax",
                ModuleReceipt {
                    toolchain: Toolchain {
                        hax: "other".into(),
                        ..r.toolchain.clone()
                    },
                    ..r.clone()
                },
            ),
            (
                "options",
                ModuleReceipt {
                    command: vec!["fstar.exe".into(), "--lax".into(), "Root.fst".into()],
                    ..r.clone()
                },
            ),
            (
                "dependencies",
                ModuleReceipt {
                    dependencies: r
                        .dependencies
                        .iter()
                        .filter(|(p, _)| *p != "Dep.fst")
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                    ..r.clone()
                },
            ),
            (
                "log",
                ModuleReceipt {
                    log: "Verified module: Other".into(),
                    ..r.clone()
                },
            ),
            (
                "module identity",
                ModuleReceipt {
                    module: "Other".into(),
                    ..r.clone()
                },
            ),
            (
                "module identity",
                ModuleReceipt {
                    file: "Other.fst".into(),
                    ..r.clone()
                },
            ),
            (
                "schema",
                ModuleReceipt {
                    schema: 2,
                    ..r.clone()
                },
            ),
            (
                "main_commit",
                ModuleReceipt {
                    main_commit: "short".into(),
                    ..r.clone()
                },
            ),
        ];
        for (field, changed) in cases {
            let err = format!(
                "{:#}",
                verify(dir.path(), &ctx, &ctx.modules[0], &changed).unwrap_err()
            );
            assert!(err.contains("Root.fst") && err.contains(field), "{err}");
        }
    }
    #[test]
    fn malformed_and_unknown_fields_rejected() {
        let (_, _, r) = fixture();
        let mut value = serde_json::to_value(r).unwrap();
        value["unexpected"] = true.into();
        assert!(serde_json::from_value::<ModuleReceipt>(value).is_err());
        assert!(serde_json::from_str::<ModuleReceipt>("{}").is_err());
    }
    #[test]
    fn current_context_cannot_enable_cached_or_lax_commands() {
        let (dir, mut ctx, _) = fixture();
        for flag in [
            "--lax",
            "--cache_checked_modules",
            "--use_hints",
            "--admit_smt_queries",
        ] {
            ctx.modules[0].command = vec!["fstar.exe".into(), flag.into(), "Root.fst".into()];
            let path = dir.path().join("context.json");
            fs::write(&path, serde_json::to_vec(&ctx).unwrap()).unwrap();
            assert!(load_context(&path).is_err(), "{flag}");
        }
    }
    #[test]
    fn malformed_manifest_receipt_names_module_and_field() {
        let (dir, _, _) = fixture();
        let path = dir.path().join("reuse.json");
        fs::write(
            &path,
            serde_json::to_vec(&serde_json::json!({
                "schema": 1, "data_branch": "coverage-data", "main_commit": "a".repeat(40), "receipts": {"Root.fst": {}}
            }))
            .unwrap(),
        )
        .unwrap();
        let error = format!("{:#}", load_manifest(&path).unwrap_err());
        assert!(
            error.contains("Root.fst") && error.contains("schema"),
            "{error}"
        );
    }
    #[test]
    fn malformed_field_types_name_the_field() {
        let (dir, _, r) = fixture();
        let path = dir.path().join("reuse.json");
        for (key, nested) in [("command", false), ("z3", true), ("dependencies", false)] {
            let mut value = serde_json::to_value(&r).unwrap();
            if nested {
                value["toolchain"][key] = true.into();
            } else {
                value[key] = true.into();
            }
            fs::write(&path, serde_json::to_vec(&serde_json::json!({"schema":1, "data_branch":"coverage-data", "main_commit":"a".repeat(40), "receipts":{"Root.fst":value}})).unwrap()).unwrap();
            let error = format!("{:#}", load_manifest(&path).unwrap_err());
            assert!(error.contains("Root.fst") && error.contains(key), "{error}");
        }
    }
}
