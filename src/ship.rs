//! Optional first-party scope. Only successful build evidence may remove files.
use crate::{Options, Report, Ship};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// An excluded source file and its units' would-be classifications, outside every total.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotShipped {
    pub file: String,
    pub sloc: usize,
    pub units: Vec<crate::Unit>,
}

type CompiledFiles = BTreeMap<String, std::result::Result<BTreeSet<PathBuf>, String>>;

fn check(
    root: &Path,
    target: &str,
    name: &str,
    library: bool,
    ship: &Ship,
    options: &Options,
) -> Result<CompiledFiles> {
    let dir = match &options.check_target_dir {
        Some(dir) => std::path::absolute(dir)?,
        None => std::path::absolute(root.join("target/formal-coverage-ship"))?,
    };
    let mut command = Command::new("cargo");
    command
        .args([
            "check",
            "--frozen",
            "-p",
            name,
            "--bins",
            "--message-format=json",
            "--target",
            target,
            "--target-dir",
        ])
        .arg(dir)
        .current_dir(root);
    if library {
        command.arg("--lib");
    }
    if !ship.features.iter().any(|f| f == "default") {
        command.arg("--no-default-features");
    }
    let features: Vec<_> = ship.features.iter().filter(|f| *f != "default").collect();
    if !features.is_empty() {
        command.arg("--features").arg(
            features
                .iter()
                .map(|f| f.as_str())
                .collect::<Vec<_>>()
                .join(","),
        );
    }
    let output = command.output().context("running cargo check")?;
    anyhow::ensure!(
        output.status.success(),
        "cargo check exited with {}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr).trim()
    );
    let mut artifacts: BTreeMap<String, Vec<Option<PathBuf>>> = BTreeMap::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if message["reason"] != "compiler-artifact"
            || message["target"]["kind"]
                .as_array()
                .is_some_and(|k| k.iter().any(|k| k == "custom-build" || k == "proc-macro"))
        {
            continue;
        }
        if let Some(id) = message["package_id"].as_str() {
            artifacts
                .entry(id.into())
                .or_default()
                .push(crate::size::artifact_dep_info(&message));
        }
    }
    let mut files = BTreeMap::new();
    for (id, paths) in artifacts {
        // A partially evidenced package must keep its whole inventory, even if another
        // target (e.g. its library) has readable dep-info.
        let read = paths
            .into_iter()
            .map(|path| {
                let path = path.context("no dep-info artifact")?;
                let paths = crate::size::read_dep_info(&path, root)?;
                anyhow::ensure!(
                    paths
                        .iter()
                        .any(|p| p.extension().is_some_and(|e| e == "rs")),
                    "dep-info lists no Rust sources"
                );
                Ok(paths
                    .into_iter()
                    .map(|p| p.canonicalize().unwrap_or(p))
                    .collect::<BTreeSet<_>>())
            })
            .collect::<Result<Vec<_>>>();
        files.insert(
            id,
            read.map(|read| read.into_iter().flatten().collect())
                .map_err(|error| format!("{error:#}")),
        );
    }
    Ok(files)
}

pub(crate) fn apply(
    root: &Path,
    options: &Options,
    ship: &Ship,
    report: &mut Report,
) -> Result<()> {
    // Validate roots independently of the resolved graph: missing/offline dependency metadata
    // must not turn a misspelled package into a successful configuration.
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--offline",
        ])
        .current_dir(root)
        .output()?;
    anyhow::ensure!(
        output.status.success(),
        "[ship] cannot inventory workspace packages"
    );
    let inventory: crate::Metadata = serde_json::from_slice(&output.stdout)?;
    for name in &ship.packages {
        anyhow::ensure!(
            inventory
                .packages
                .iter()
                .any(|p| p.name == *name && inventory.workspace_members.contains(&p.id)),
            "[ship] unknown workspace package {name:?}"
        );
    }
    report.ship = Some(ship.clone());
    let packages = match crate::dependencies::ship_packages(root, &report.target, ship) {
        Ok(packages) => packages,
        Err(error) => {
            for p in inventory
                .packages
                .iter()
                .filter(|p| inventory.workspace_members.contains(&p.id))
            {
                report.warnings.push(format!(
                    "ship: package {} kept in scope: dependency graph unavailable: {error:#}",
                    p.name
                ));
            }
            return Ok(());
        }
    };
    let mut compiled = CompiledFiles::new();
    for package in packages.iter().filter(|p| ship.packages.contains(&p.name)) {
        let name = &package.name;
        match check(root, &report.target, name, package.library, ship, options) {
            Ok(files) => {
                for (id, files) in files {
                    compiled
                        .entry(id)
                        .and_modify(|current| match (current, &files) {
                            (Ok(current), Ok(files)) => current.extend(files.iter().cloned()),
                            (current, Err(reason)) => *current = Err(reason.clone()),
                            _ => {}
                        })
                        .or_insert(files);
                }
            }
            Err(error) => {
                // No complete root build means absence from the graph is unconfirmed too.
                for package in &packages {
                    let package = &package.name;
                    report.warnings.push(format!("ship: package {package} kept in scope: shipped package {name} cargo check failed: {error:#}"));
                }
                return Ok(());
            }
        }
    }
    let root = root.canonicalize()?;
    let mut dropped = BTreeSet::new();
    for package in &packages {
        let name = &package.name;
        let files = match compiled.get(&package.id) {
            Some(Ok(files)) => Some(files),
            result if package.reached => {
                let reason = match result {
                    Some(Err(reason)) => reason.as_str(),
                    _ => "no dep-info artifact from cargo check",
                };
                report.warnings.push(format!(
                    "ship: package {name} kept in scope: dep-info unavailable: {reason}"
                ));
                continue;
            }
            _ => None,
        };
        // The deepest package directory owns a source (nested workspace members are separate).
        for file in &report.source_files {
            let path = root.join(file);
            let owner = packages
                .iter()
                .filter(|p| path.starts_with(&p.dir))
                .max_by_key(|p| p.dir.components().count());
            if owner.is_none_or(|owner| owner.id != package.id) {
                continue;
            }
            if package.reached && files.is_some_and(|files| files.contains(&path)) {
                continue;
            }
            dropped.insert(file.clone());
        }
    }
    for file in &dropped {
        let units = report
            .units
            .iter()
            .filter(|u| u.file == *file)
            .cloned()
            .collect();
        let source = fs::read_to_string(root.join(file))?;
        report.not_shipped.push(NotShipped {
            file: file.clone(),
            sloc: crate::sloc(&source),
            units,
        });
    }
    // Input movement is a coverage total too: exclude movements of dropped runtime units.
    for unit in report.units.iter().filter(|u| {
        dropped.contains(&u.file)
            && u.category == crate::Category::Runtime
            && u.default_security.is_some()
    }) {
        if let Some(input) = report.inputs.iter_mut().rev().find(|input| {
            input.matched.contains(&unit.id)
                && [
                    "security",
                    "not_security",
                    "security.include",
                    "security.exclude",
                ]
                .contains(&input.kind.as_str())
        }) {
            if unit.security {
                input.sloc_into_security -= unit.sloc;
            } else {
                input.sloc_out_of_security -= unit.sloc;
            }
        }
    }
    report.units.retain(|u| !dropped.contains(&u.file));
    report.source_files.retain(|f| !dropped.contains(f));
    report.unparseable_files.retain(|f| !dropped.contains(f));
    report.unknown_hax_paths.clear();
    for file in &report.source_files {
        if let Some(scan) = crate::hax::scan(&fs::read_to_string(root.join(file))?) {
            report.unknown_hax_paths.extend(scan.unknown);
        }
    }
    report.unknown_hax_paths.sort();
    report.unknown_hax_paths.dedup();
    Ok(())
}

/// Evaluate evidence against every first-party unit, then project coverage back onto
/// the shipped files. Receipt-local proof scope remains the full evidence scope.
pub(crate) fn apply_evidence(
    report: &mut Report,
    apply: impl FnOnce(&mut Report) -> Result<()>,
) -> Result<()> {
    if report.not_shipped.is_empty() {
        return apply(report);
    }
    let mut inventory = report.clone();
    for file in &report.not_shipped {
        inventory.units.extend(file.units.iter().cloned());
        inventory.source_files.push(file.file.clone());
    }
    inventory.not_shipped.clear();
    inventory
        .units
        .sort_by(|a, b| (&a.file, a.line, &a.id).cmp(&(&b.file, b.line, &b.id)));
    inventory.source_files.sort();
    apply(&mut inventory)?;
    let dropped: BTreeSet<_> = report.not_shipped.iter().map(|f| f.file.clone()).collect();
    for file in &mut report.not_shipped {
        file.units = inventory
            .units
            .iter()
            .filter(|u| u.file == file.file)
            .cloned()
            .collect();
    }
    inventory.not_shipped = report.not_shipped.clone();
    inventory.units.retain(|u| !dropped.contains(&u.file));
    inventory.source_files = report.source_files.clone();
    // These inventories feed coverage/trust totals; excluded source stays outside them.
    // EvidenceRun retains its complete receipt-local scope, used to justify unit tiers.
    inventory
        .trusted_items
        .retain(|t| !dropped.contains(&t.file));
    inventory
        .non_fstar_trust
        .retain(|t| !dropped.contains(&t.file));
    inventory
        .escape_hatches
        .retain(|e| !dropped.contains(&e.file));
    *report = inventory;
    Ok(())
}
