//! Consumer side of line-total receipts: count a dependency's own measured buckets in
//! place of its all-unverified lines. Informational; gates and the ratchet ignore it.
use crate::totals::{self, Buckets, PackageId, PackageSource, TotalsReceipt};
use crate::{sha256, Report};
use anyhow::{bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// What a matched receipt contributed to one `dependencies[]` entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyReceipt {
    pub buckets: Buckets,
    /// Compiled lines of the dependency outside the receipt's scope, counted unverified.
    pub outside_receipt: usize,
    pub receipt_sha256: String,
    pub kit_binary_sha256: String,
}

/// Security-relevant shipped code: first-party plus runtime dependencies, by bucket.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShippedTree {
    pub scope: String,
    pub first_party: Buckets,
    pub third_party: Buckets,
    pub total: Buckets,
    pub receipts: usize,
    pub outside_receipts: usize,
    pub unknown_dependency_sizes: usize,
}

/// Validate each receipt against this build (rules 1–5) and record the result.
pub fn apply(report: &mut Report, root: &Path, paths: &[PathBuf]) -> Result<()> {
    let build = Build::read(root, &report.target)?;
    let mut seen = BTreeSet::new();
    for path in paths {
        let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
        let receipt = parse(&bytes).with_context(|| format!("receipt {}", path.display()))?;
        let key = (
            receipt.package.clone(),
            receipt.features.clone(),
            receipt.target.clone(),
        );
        ensure!(
            seen.insert(key),
            "two receipts for package {} {}: count each package once",
            receipt.package.name,
            receipt.package.version
        );
        let index = matching_dependency(report, &receipt.package)?;
        let dependency = &report.dependencies[index];
        build.check(dependency, &receipt, &report.target)?;
        let outside = lines_outside(dependency, &receipt)?;
        report.dependencies[index].receipt = Some(DependencyReceipt {
            buckets: receipt.totals,
            outside_receipt: outside,
            receipt_sha256: sha256(&bytes),
            kit_binary_sha256: receipt.kit.binary_sha256,
        });
    }
    report.shipped_tree = Some(summarize(report));
    Ok(())
}

fn parse(bytes: &[u8]) -> Result<TotalsReceipt> {
    let receipt: TotalsReceipt = serde_json::from_slice(bytes)?;
    ensure!(
        receipt.schema == totals::TOTALS_SCHEMA,
        "unsupported receipt schema {}",
        receipt.schema
    );
    ensure!(
        receipt.kit.version == env!("CARGO_PKG_VERSION"),
        "receipt from formal-coverage {}, this is {}: never mix kit versions",
        receipt.kit.version,
        env!("CARGO_PKG_VERSION")
    );
    ensure!(
        receipt.scope == totals::TOTALS_SCOPE && receipt.bucket_map == totals::bucket_map(),
        "receipt scope or bucket map differs from this kit's"
    );
    ensure!(
        receipt.totals.sum() == receipt.total && receipt.total <= receipt.shipped_files_sloc,
        "receipt buckets do not add up"
    );
    Ok(receipt)
}

fn matching_dependency(report: &Report, package: &PackageId) -> Result<usize> {
    let found: Vec<_> = report
        .dependencies
        .iter()
        .enumerate()
        .filter(|(_, d)| d.name == package.name && d.version == package.version)
        .map(|(i, _)| i)
        .collect();
    match found[..] {
        [index] if report.dependencies[index].receipt.is_none() => Ok(index),
        [_] => bail!(
            "two receipts for package {} {}: count each package once",
            package.name,
            package.version
        ),
        [] => bail!(
            "receipt for {} {} matches no runtime dependency of this build",
            package.name,
            package.version
        ),
        _ => bail!(
            "receipt for {} {} matches several dependencies",
            package.name,
            package.version
        ),
    }
}

/// Rule 5. Only a compiled-files measurement of the same files can absorb a receipt.
fn lines_outside(dependency: &crate::Dependency, receipt: &TotalsReceipt) -> Result<usize> {
    let name = &dependency.name;
    ensure!(
        dependency.size_known
            && dependency.size_mode == crate::SizeMode::CompiledFiles.as_str()
            && dependency.default_sloc.is_none(),
        "{name}: a receipt needs a compiled-files measurement of the dependency, got {} ({}){}",
        dependency.size_mode,
        if dependency.size_known {
            "known"
        } else {
            "unknown size"
        },
        if dependency.default_sloc.is_some() {
            ", replaced by a configured sloc"
        } else {
            ""
        }
    );
    ensure!(
        receipt.total <= dependency.sloc,
        "{name}: oversized receipt: it claims {} lines, this build compiles {}",
        receipt.total,
        dependency.sloc
    );
    ensure!(
        receipt.shipped_files_sloc == dependency.sloc,
        "{name}: receipt measured {} lines of shipped files, this build compiles {}: not the same files",
        receipt.shipped_files_sloc,
        dependency.sloc
    );
    Ok(dependency.sloc - receipt.total)
}

/// Sources, checksums and resolved features of this build's packages.
struct Build {
    packages: BTreeMap<String, (Option<String>, Vec<String>)>,
    checksums: BTreeMap<(String, String, String), String>,
}
#[derive(Deserialize)]
struct Metadata {
    packages: Vec<MetadataPackage>,
    resolve: Resolve,
}
#[derive(Deserialize)]
struct MetadataPackage {
    id: String,
    source: Option<String>,
}
#[derive(Deserialize)]
struct Resolve {
    nodes: Vec<Node>,
}
#[derive(Deserialize)]
struct Node {
    id: String,
    features: Vec<String>,
}

impl Build {
    fn read(root: &Path, target: &str) -> Result<Self> {
        let output = Command::new("cargo")
            .args(["metadata", "--format-version", "1", "--frozen"])
            .args(["--filter-platform", target])
            .current_dir(root)
            .output()
            .context("running cargo metadata")?;
        ensure!(
            output.status.success(),
            "cargo metadata: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
        let metadata: Metadata = serde_json::from_slice(&output.stdout)?;
        let features: BTreeMap<_, _> = metadata
            .resolve
            .nodes
            .into_iter()
            .map(|n| (n.id, n.features))
            .collect();
        let packages = metadata
            .packages
            .into_iter()
            .map(|p| {
                let mut f = features.get(&p.id).cloned().unwrap_or_default();
                f.sort();
                (p.id, (p.source, f))
            })
            .collect();
        Ok(Self {
            packages,
            checksums: lock_checksums(&root.join("Cargo.lock"))?,
        })
    }

    /// Rule 2: the receipt's source, features and target are this build's.
    fn check(
        &self,
        dependency: &crate::Dependency,
        receipt: &TotalsReceipt,
        target: &str,
    ) -> Result<()> {
        let name = &dependency.name;
        let id = dependency.package_id.as_deref().with_context(|| {
            format!("{name}: no cargo metadata package id (lockfile fallback inventory)")
        })?;
        let (source, features) = self
            .packages
            .get(id)
            .with_context(|| format!("{name}: not in cargo metadata"))?;
        let ours = self.source(name, &dependency.version, source.as_deref())?;
        ensure!(
            ours == receipt.package.source,
            "{name}: receipt source {:?} is not this build's locked source {:?}",
            receipt.package.source,
            ours
        );
        ensure!(
            *features == receipt.features,
            "{name}: receipt features {:?} are not this build's resolved features {:?}",
            receipt.features,
            features
        );
        ensure!(
            receipt.target == target,
            "{name}: receipt target {} is not this build's target {target}",
            receipt.target
        );
        Ok(())
    }

    fn source(&self, name: &str, version: &str, source: Option<&str>) -> Result<PackageSource> {
        let source =
            source.with_context(|| format!("{name}: a path dependency has no exact source"))?;
        if source.starts_with("git+") {
            return totals::parse_source(source);
        }
        let key = (name.to_string(), version.to_string(), source.to_string());
        let checksum = self
            .checksums
            .get(&key)
            .with_context(|| format!("{name}: no Cargo.lock checksum for {source}"))?;
        totals::parse_source(&format!("checksum:{checksum}"))
    }
}

fn lock_checksums(path: &Path) -> Result<BTreeMap<(String, String, String), String>> {
    let lock: toml::Value = toml::from_str(&fs::read_to_string(path)?)?;
    let mut checksums = BTreeMap::new();
    for p in lock
        .get("package")
        .and_then(|p| p.as_array())
        .into_iter()
        .flatten()
    {
        let field = |k: &str| p.get(k).and_then(|v| v.as_str()).map(String::from);
        if let (Some(name), Some(version), Some(source), Some(sum)) = (
            field("name"),
            field("version"),
            field("source"),
            field("checksum"),
        ) {
            checksums.insert((name, version, source), sum);
        }
    }
    Ok(checksums)
}

fn summarize(report: &Report) -> ShippedTree {
    let first = &crate::summarize(report).first_party.security_relevant;
    let mut tree = ShippedTree {
        scope: "security-relevant shipped code".into(),
        first_party: Buckets::from_statuses(&first.statuses).unwrap_or_default(),
        ..ShippedTree::default()
    };
    for d in report.dependencies.iter().filter(|d| d.security) {
        tree.unknown_dependency_sizes += usize::from(!d.size_known);
        let buckets = match &d.receipt {
            Some(receipt) => {
                tree.receipts += 1;
                tree.outside_receipts += receipt.outside_receipt;
                let mut b = receipt.buckets;
                b.unverified += receipt.outside_receipt;
                b
            }
            None => Buckets::from_statuses(&BTreeMap::from([(d.status.clone(), d.sloc)]))
                .unwrap_or(Buckets {
                    unverified: d.sloc,
                    ..Buckets::default()
                }),
        };
        tree.third_party.add(&buckets);
    }
    tree.total = tree.first_party;
    tree.total.add(&tree.third_party);
    tree
}

pub fn markdown(report: &Report) -> String {
    let Some(tree) = &report.shipped_tree else {
        return String::new();
    };
    let row = |label: &str, b: &Buckets| {
        format!(
            "| {label} | {} | {} | {} | {} | {} |\n",
            b.proven,
            b.tested,
            b.trusted,
            b.unverified,
            b.sum()
        )
    };
    let share = if tree.total.sum() == 0 {
        0.
    } else {
        tree.total.proven as f64 * 100. / tree.total.sum() as f64
    };
    let mut out = String::from("\n## Shipped tree\n\nSecurity-relevant shipped code: first-party plus runtime dependencies. A dependency with a line-total receipt counts its own measured buckets; its compiled lines outside the receipt and every dependency without one count as now. Informational: gates and the ratchet use first-party totals.\n\n| Part | Proven | Tested | Trusted | Unverified | Total |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    out += &row("first-party", &tree.first_party);
    out += &row("third-party", &tree.third_party);
    out += &row("shipped tree", &tree.total);
    out += &format!(
        "\nProven somewhere in the tree: {share:.2}% ({} receipts; {} dependency lines outside receipts counted unverified; {} unknown dependency sizes).\n",
        tree.receipts, tree.outside_receipts, tree.unknown_dependency_sizes
    );
    out += "\n| Dependency | Proven | Tested | Trusted | Unverified | Outside receipt | Receipt sha256 |\n| --- | ---: | ---: | ---: | ---: | ---: | --- |\n";
    for d in &report.dependencies {
        if let Some(r) = &d.receipt {
            out += &format!(
                "| {} {} | {} | {} | {} | {} | {} | `{}` |\n",
                d.name,
                d.version,
                r.buckets.proven,
                r.buckets.tested,
                r.buckets.trusted,
                r.buckets.unverified,
                r.outside_receipt,
                r.receipt_sha256
            );
        }
    }
    out
}
