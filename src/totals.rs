//! Line-total receipts: one package's first-party security runtime lines in four buckets.
//! A consumer counts a receipt in place of the dependency's all-unverified lines.
use crate::{sha256, Report};
use anyhow::{bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    process::Command,
};

pub const TOTALS_SCHEMA: u32 = 1;
pub const TOTALS_SCOPE: &str = "first-party security runtime";
/// Kit statuses per bucket. Hazard lines count as unverified.
pub const BUCKETS: [(&str, &[&str]); 4] = [
    (
        "proven",
        &[
            "proven",
            "proven-bounded",
            "proven-extracted",
            "proven-upstream",
        ],
    ),
    ("tested", &["tested"]),
    ("trusted", &["trusted"]),
    ("unverified", &["unverified", "hazard"]),
];
const SHOWN_PATHS: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TotalsReceipt {
    pub schema: u32,
    pub kit: Kit,
    pub package: PackageId,
    /// Resolved feature set of the package, sorted.
    pub features: Vec<String>,
    pub target: String,
    pub scope: String,
    pub bucket_map: BTreeMap<String, Vec<String>>,
    pub totals: Buckets,
    pub total: usize,
    /// SLoC of every shipped source file: the consumer's compiled-files measure.
    pub shipped_files_sloc: usize,
    pub evidence: Vec<String>,
    /// sha256 of the full JSON report this receipt was taken from.
    pub report_sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Kit {
    pub version: String,
    pub binary_sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageId {
    pub name: String,
    pub version: String,
    pub source: PackageSource,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub enum PackageSource {
    Git { url: String, rev: String },
    Checksum(String),
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Buckets {
    pub proven: usize,
    pub tested: usize,
    pub trusted: usize,
    pub unverified: usize,
}
impl Buckets {
    pub fn sum(&self) -> usize {
        self.proven + self.tested + self.trusted + self.unverified
    }
    pub fn add(&mut self, other: &Buckets) {
        self.proven += other.proven;
        self.tested += other.tested;
        self.trusted += other.trusted;
        self.unverified += other.unverified;
    }
    pub fn from_statuses(statuses: &BTreeMap<String, usize>) -> Result<Self> {
        let known: BTreeSet<&str> = BUCKETS
            .iter()
            .flat_map(|(_, s)| s.iter().copied())
            .collect();
        if let Some(status) = statuses.keys().find(|s| !known.contains(s.as_str())) {
            bail!("status {status:?} has no bucket");
        }
        let count = |bucket: &str| -> usize {
            let (_, members) = BUCKETS.iter().find(|(name, _)| *name == bucket).unwrap();
            members.iter().map(|s| statuses.get(*s).unwrap_or(&0)).sum()
        };
        Ok(Self {
            proven: count("proven"),
            tested: count("tested"),
            trusted: count("trusted"),
            unverified: count("unverified"),
        })
    }
}

pub fn bucket_map() -> BTreeMap<String, Vec<String>> {
    BUCKETS
        .iter()
        .map(|(name, statuses)| {
            (
                name.to_string(),
                statuses.iter().map(|s| s.to_string()).collect(),
            )
        })
        .collect()
}

/// `git+URL[?query]#REV` (a Cargo source string with a full 40-hex rev) or `checksum:SHA256`.
pub fn parse_source(text: &str) -> Result<PackageSource> {
    if let Some(checksum) = text.strip_prefix("checksum:") {
        ensure!(
            is_hex(checksum, 64),
            "checksum must be 64 lowercase hex digits"
        );
        return Ok(PackageSource::Checksum(checksum.into()));
    }
    let Some(rest) = text.strip_prefix("git+") else {
        bail!("package source must be git+URL#REV or checksum:SHA256, got {text:?}");
    };
    let (url, rev) = rest
        .rsplit_once('#')
        .context("git source needs #REV with the full commit")?;
    ensure!(
        is_hex(rev, 40),
        "git source rev must be a full 40-hex commit"
    );
    let url = url.split('?').next().unwrap_or(url);
    let url = url.trim_end_matches('/');
    let url = url.strip_suffix(".git").unwrap_or(url);
    ensure!(!url.is_empty(), "git source needs a URL");
    Ok(PackageSource::Git {
        url: url.into(),
        rev: rev.into(),
    })
}
fn is_hex(text: &str, len: usize) -> bool {
    text.len() == len && text.bytes().all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f'))
}

/// Expand requested features through the package's own `[features]` table, as Cargo does.
pub fn resolve_features(
    table: &BTreeMap<String, Vec<String>>,
    requested: &[String],
) -> Result<Vec<String>> {
    let mut pending: Vec<&str> = requested.iter().map(String::as_str).collect();
    let mut enabled = BTreeSet::new();
    while let Some(feature) = pending.pop() {
        if feature.starts_with("dep:") {
            continue;
        }
        let feature = match feature.split_once('/') {
            Some((dep, _)) if dep.ends_with('?') => continue,
            Some((dep, _)) if table.contains_key(dep) => dep,
            Some(_) => continue,
            None => feature,
        };
        ensure!(table.contains_key(feature), "unknown feature {feature:?}");
        if enabled.insert(feature.to_string()) {
            pending.extend(table[feature].iter().map(String::as_str));
        }
    }
    Ok(enabled.into_iter().collect())
}

/// Build the receipt for `report`, refusing anything a consumer could not trust as the
/// shipped configuration of one committed package.
pub fn receipt(
    report: &Report,
    root: &Path,
    source: &PackageSource,
    report_json: &[u8],
) -> Result<TotalsReceipt> {
    let ship = shipped_package(report)?;
    refuse_dirty_config(report)?;
    refuse_dirty_checkout(root, source)?;
    let (version, table) = package_manifest(root, &ship.packages[0])?;
    let totals_side = &crate::summarize(report).first_party.security_relevant;
    let totals = Buckets::from_statuses(&totals_side.statuses)?;
    ensure!(
        totals.sum() == totals_side.sloc,
        "buckets do not add up to the security runtime total"
    );
    Ok(TotalsReceipt {
        schema: TOTALS_SCHEMA,
        kit: Kit {
            version: env!("CARGO_PKG_VERSION").into(),
            binary_sha256: binary_sha256()?,
        },
        package: PackageId {
            name: ship.packages[0].clone(),
            version,
            source: source.clone(),
        },
        features: resolve_features(&table, &ship.features)?,
        target: report.target.clone(),
        scope: TOTALS_SCOPE.into(),
        bucket_map: bucket_map(),
        totals,
        total: totals_side.sloc,
        shipped_files_sloc: shipped_files_sloc(report, root)?,
        evidence: evidence(report),
        report_sha256: sha256(report_json),
    })
}

fn shipped_files_sloc(report: &Report, root: &Path) -> Result<usize> {
    let mut total = 0;
    for file in &report.source_files {
        total += crate::sloc(&std::fs::read_to_string(root.join(file))?);
    }
    Ok(total)
}

pub fn binary_sha256() -> Result<String> {
    let exe = std::env::current_exe().context("locating the formal-coverage binary")?;
    Ok(sha256(&std::fs::read(exe)?))
}

/// Deterministic bytes: fixed field order, sorted maps, trailing newline.
pub fn to_bytes(receipt: &TotalsReceipt) -> Result<Vec<u8>> {
    let mut text = serde_json::to_string_pretty(receipt)?;
    text.push('\n');
    Ok(text.into_bytes())
}

fn shipped_package(report: &Report) -> Result<&crate::Ship> {
    let ship = report
        .ship
        .as_ref()
        .context("unmatched config: --totals-json needs [ship] naming the measured package")?;
    ensure!(
        ship.packages.len() == 1,
        "unmatched config: [ship] must name exactly one package for --totals-json, got {:?}",
        ship.packages
    );
    if let Some(warning) = report.warnings.iter().find(|w| w.starts_with("ship: ")) {
        bail!("unmatched config: [ship] scope not confirmed by the build: {warning}");
    }
    Ok(ship)
}

fn refuse_dirty_config(report: &Report) -> Result<()> {
    for input in &report.inputs {
        ensure!(
            !input.reason.as_deref().is_some_and(crate::todo_reason),
            "dirty config: input {} {:?} has a TODO reason",
            input.kind,
            input.key
        );
        ensure!(
            !["security", "not_security"].contains(&input.kind.as_str()),
            "dirty config: schema-2 input {} {:?} has no reason",
            input.kind,
            input.key
        );
    }
    ensure!(
        report.unparseable_files.is_empty(),
        "unparseable files: security classification is incomplete: {:?}",
        report.unparseable_files
    );
    Ok(())
}

/// Inside a git checkout the measured files must be committed and HEAD must be the
/// claimed rev. Outside one (a Nix source), the caller's pinned fetch vouches for it.
fn refuse_dirty_checkout(root: &Path, source: &PackageSource) -> Result<()> {
    let root = root.canonicalize()?;
    if !root.ancestors().any(|dir| dir.join(".git").exists()) {
        return Ok(());
    }
    let status = git(
        &root,
        &[
            "status",
            "--porcelain",
            "-z",
            "--untracked-files=all",
            "--",
            ".",
        ],
    )?;
    let paths: Vec<_> = status
        .split('\0')
        .filter(|e| e.len() > 3)
        .map(|e| format!("  {}", &e[3..]))
        .collect();
    if !paths.is_empty() {
        let mut shown = paths[..paths.len().min(SHOWN_PATHS)].to_vec();
        if paths.len() > SHOWN_PATHS {
            shown.push(format!("  ... and {} more", paths.len() - SHOWN_PATHS));
        }
        bail!(
            "uncommitted changes in {}; commit them first: a receipt describes a commit\n{}",
            root.display(),
            shown.join("\n")
        );
    }
    if let PackageSource::Git { rev, .. } = source {
        let head = git(&root, &["rev-parse", "HEAD"])?;
        ensure!(
            head.trim() == rev,
            "checkout HEAD {} is not the receipt's rev {rev}",
            head.trim()
        );
    }
    Ok(())
}
/// `git -C root`, ignoring any repository a calling hook exported (GIT_DIR and the like).
fn git(root: &Path, args: &[&str]) -> Result<String> {
    let listed = Command::new("git")
        .args(["rev-parse", "--local-env-vars"])
        .output()
        .context("running git")?;
    let mut command = Command::new("git");
    for var in String::from_utf8_lossy(&listed.stdout).lines() {
        command.env_remove(var);
    }
    let output = command
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .context("running git")?;
    ensure!(
        output.status.success(),
        "git {}: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr).trim()
    );
    Ok(String::from_utf8(output.stdout)?)
}

#[derive(Deserialize)]
struct Manifests {
    packages: Vec<ManifestPackage>,
    workspace_members: Vec<String>,
}
#[derive(Deserialize)]
struct ManifestPackage {
    id: String,
    name: String,
    version: String,
    features: BTreeMap<String, Vec<String>>,
}
fn package_manifest(root: &Path, name: &str) -> Result<(String, BTreeMap<String, Vec<String>>)> {
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--offline",
        ])
        .current_dir(root)
        .output()
        .context("running cargo metadata")?;
    ensure!(
        output.status.success(),
        "cargo metadata: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
    let metadata: Manifests = serde_json::from_slice(&output.stdout)?;
    let package = metadata
        .packages
        .into_iter()
        .find(|p| p.name == name && metadata.workspace_members.contains(&p.id))
        .with_context(|| format!("[ship] unknown workspace package {name:?}"))?;
    Ok((package.version, package.features))
}

fn evidence(report: &Report) -> Vec<String> {
    let mut lines: Vec<String> = report
        .evidence_runs
        .iter()
        .map(|run| {
            if run.accepted {
                format!(
                    "{} receipt accepted, {} verified files, {} credited units",
                    run.tool,
                    run.verified_files.len(),
                    run.credited_units.len()
                )
            } else {
                format!(
                    "{} receipt rejected, {} reasons in the report",
                    run.tool,
                    run.reasons.len()
                )
            }
        })
        .collect();
    if lines.is_empty() {
        lines.push("no prover evidence".into());
    }
    lines
}
