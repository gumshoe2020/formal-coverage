//! Target-filtered Cargo graph, with the original lockfile inventory as fallback.
use crate::{Dependency, Report};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<Package>,
    workspace_members: BTreeSet<String>,
    resolve: Resolve,
}
#[derive(Deserialize)]
struct Package {
    id: String,
    name: String,
    version: String,
    manifest_path: PathBuf,
    targets: Vec<Target>,
    dependencies: Vec<ManifestDependency>,
    features: BTreeMap<String, Vec<String>>,
}
#[derive(Deserialize)]
struct ManifestDependency {
    name: String,
    rename: Option<String>,
    optional: bool,
    kind: Option<String>,
}
#[derive(Deserialize)]
struct Target {
    kind: Vec<String>,
}
#[derive(Deserialize)]
struct Resolve {
    nodes: Vec<Node>,
}
#[derive(Deserialize)]
struct Node {
    id: String,
    deps: Vec<Edge>,
    features: BTreeSet<String>,
}
#[derive(Deserialize)]
struct Edge {
    name: String,
    pkg: String,
    dep_kinds: Vec<Kind>,
}
#[derive(Deserialize)]
struct Kind {
    kind: Option<String>,
}

// Metadata may retain weak optional edges (foo?/feature) without enabling foo.
// Only omit a declared optional edge when its activation can be ruled out.
fn enabled(parent: &Package, node: &Node, edge: &Edge, child: &Package, kind: &Kind) -> bool {
    let declarations: Vec<_> = parent
        .dependencies
        .iter()
        .filter(|d| {
            d.name == child.name
                && d.kind == kind.kind
                && d.rename.as_deref().unwrap_or(&d.name).replace('-', "_") == edge.name
        })
        .collect();
    if declarations.is_empty() || declarations.iter().any(|d| !d.optional) {
        return true;
    }
    let aliases: BTreeSet<_> = declarations
        .iter()
        .map(|d| d.rename.as_deref().unwrap_or(&d.name))
        .collect();
    let mut pending: Vec<_> = node.features.iter().map(String::as_str).collect();
    let mut seen = BTreeSet::new();
    while let Some(feature) = pending.pop() {
        if !seen.insert(feature) {
            continue;
        }
        if let Some(dep) = feature.strip_prefix("dep:") {
            if aliases.contains(dep) {
                return true;
            }
        } else if let Some((dep, _)) = feature.split_once('/') {
            if !dep.ends_with('?') && aliases.contains(dep) {
                return true;
            }
        } else {
            if aliases.contains(feature) {
                return true;
            }
            if let Some(values) = parent.features.get(feature) {
                pending.extend(values.iter().map(String::as_str));
            }
        }
    }
    false
}

fn version(program: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(program).args(args).output()?;
    anyhow::ensure!(
        out.status.success(),
        "{program}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    Ok(String::from_utf8(out.stdout)?.trim().into())
}
fn dependency(name: String, version: String, security: bool) -> Dependency {
    Dependency {
        name,
        version,
        security,
        sloc: 0,
        size_known: false,
        source: None,
        unsafe_sites: None,
        status: "unverified".into(),
    }
}
pub(crate) fn resolve(root: &Path, target: Option<&str>, report: &mut Report) -> Result<()> {
    report.target = match target {
        Some(target) => target.to_string(),
        None => version("rustc", &["-vV"])?
            .lines()
            .find_map(|l| l.strip_prefix("host: "))
            .context("rustc host triple missing")?
            .into(),
    };
    report.cargo_version =
        version("cargo", &["--version"]).unwrap_or_else(|e| format!("unconfirmed: {e}"));
    match metadata(root, &report.target).and_then(|m| closure(m, report)) {
        Ok(()) => {
            report.closure = "cargo metadata (runtime)".into();
            report.dependency_inventory_complete = true;
        }
        Err(error) => {
            report.dependencies.clear();
            report.build_dependencies.clear();
            report.dev_dependencies_excluded = 0;
            report.closure = "lockfile (fallback)".into();
            report.closure_reason = Some(format!("{error:#}"));
            let lock_path = root.join("Cargo.lock");
            if lock_path.exists() {
                let lock: toml::Value = toml::from_str(&fs::read_to_string(lock_path)?)?;
                let first: BTreeSet<_> =
                    report.units.iter().map(|u| u.crate_name.clone()).collect();
                if let Some(packages) = lock.get("package").and_then(|p| p.as_array()) {
                    for p in packages {
                        let name = p["name"].as_str().context("package name")?.to_string();
                        if !first.contains(&name.replace('-', "_")) {
                            report.dependencies.push(dependency(
                                name,
                                p["version"].as_str().unwrap_or("unknown").into(),
                                report.units.iter().any(|u| u.security),
                            ));
                        }
                    }
                }
            }
            report.dependency_inventory_complete = report
                .declared_external_dependencies
                .iter()
                .all(|name| report.dependencies.iter().any(|d| d.name == *name));
            report.warnings.push(format!("closure: lockfile (fallback): {}. Entire prior lockfile closure retained; build/dev/optional exclusions unconfirmed.", report.closure_reason.as_deref().unwrap()));
            if !report.dependency_inventory_complete {
                report.warnings.push(
                    "INCOMPLETE dependency inventory: missing/stale Cargo.lock; gates fail closed."
                        .into(),
                );
            }
        }
    }
    Ok(())
}
fn metadata(root: &Path, target: &str) -> Result<Metadata> {
    let out = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--frozen",
            "--filter-platform",
            target,
        ])
        .current_dir(root)
        .output()
        .context("running cargo metadata")?;
    anyhow::ensure!(
        out.status.success(),
        "cargo metadata: {}",
        String::from_utf8_lossy(&out.stderr).trim()
    );
    serde_json::from_slice(&out.stdout).context("decoding cargo metadata resolve graph")
}
fn closure(m: Metadata, report: &mut Report) -> Result<()> {
    let packages: BTreeMap<_, _> = m.packages.iter().map(|p| (p.id.as_str(), p)).collect();
    let nodes: BTreeMap<_, _> = m.resolve.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let mut runtime = BTreeSet::new();
    let mut build = BTreeSet::new();
    let mut dev = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut pending: Vec<_> = m
        .workspace_members
        .iter()
        .map(|id| (id.as_str(), false))
        .collect();
    while let Some((id, building)) = pending.pop() {
        if !seen.insert((id, building)) {
            continue;
        }
        let node = nodes.get(id).context("missing resolved node")?;
        for edge in &node.deps {
            let package = packages
                .get(edge.pkg.as_str())
                .context("missing resolved package")?;
            anyhow::ensure!(!edge.dep_kinds.is_empty(), "missing dependency kinds");
            for kind in &edge.dep_kinds {
                if !enabled(packages[id], node, edge, package, kind) {
                    continue;
                }
                match kind.kind.as_deref() {
                    Some("dev") => {
                        dev.insert(edge.pkg.as_str());
                    }
                    None | Some("build") => {
                        let next_build = building
                            || kind.kind.as_deref() == Some("build")
                            || package
                                .targets
                                .iter()
                                .any(|t| t.kind.iter().any(|k| k == "proc-macro"));
                        if next_build {
                            build.insert(edge.pkg.as_str());
                        } else {
                            runtime.insert(edge.pkg.as_str());
                        }
                        pending.push((edge.pkg.as_str(), next_build));
                    }
                    Some(other) => anyhow::bail!("unrecognized dependency kind: {other}"),
                }
            }
        }
    }
    // Include normal/build transitives of dev roots in the exclusion note too.
    let mut dev_pending: Vec<_> = dev.iter().copied().collect();
    while let Some(id) = dev_pending.pop() {
        let node = nodes.get(id).context("missing dev resolved node")?;
        for edge in &node.deps {
            if edge.dep_kinds.iter().any(|k| {
                k.kind.as_deref() != Some("dev")
                    && enabled(packages[id], node, edge, packages[edge.pkg.as_str()], k)
            }) && dev.insert(edge.pkg.as_str())
            {
                dev_pending.push(edge.pkg.as_str());
            }
        }
    }
    report.dev_dependencies_excluded = dev
        .difference(&runtime)
        .filter(|id| !build.contains(*id) && !m.workspace_members.contains(**id))
        .count();
    let security = report.units.iter().any(|u| u.security);
    for (ids, output) in [
        (runtime, &mut report.dependencies),
        (build, &mut report.build_dependencies),
    ] {
        for id in ids {
            if m.workspace_members.contains(id) {
                continue;
            }
            let p = packages[id];
            let source = p
                .manifest_path
                .parent()
                .context("dependency manifest parent")?;
            // Missing source is a metadata failure, not permission to shrink the denominator.
            anyhow::ensure!(
                source.join("Cargo.toml").is_file(),
                "missing dependency source: {}",
                source.display()
            );
            let mut files = Vec::new();
            crate::collect_rs(source, &mut files)?;
            let mut d = dependency(p.name.clone(), p.version.clone(), security);
            d.sloc = files
                .iter()
                .map(|f| fs::read_to_string(f).map(|s| crate::sloc(&s)))
                .collect::<std::io::Result<Vec<_>>>()?
                .iter()
                .sum();
            d.size_known = true;
            d.source = Some(source.to_string_lossy().into());
            output.push(d);
        }
        output.sort_by(|a, b| (&a.name, &a.version).cmp(&(&b.name, &b.version)));
    }
    Ok(())
}
