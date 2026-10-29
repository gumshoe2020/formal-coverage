use crate::Report;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs,
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Escape {
    pub unit: String,
    #[serde(default)]
    pub affected_units: Vec<String>,
    pub file: String,
    pub line: usize,
    pub kind: String,
    pub security: bool,
    pub first_party: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedItem {
    pub file: String,
    pub line: usize,
    pub kind: String,
    pub affected_units: Vec<String>,
    pub sloc: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub crate_name: String,
    pub version: String,
    pub operations: String,
    pub property: String,
    pub tool: String,
    pub source: String,
    pub confirmation: String,
    #[serde(default)]
    pub whole_crate: bool,
}
#[derive(Deserialize)]
struct Registry {
    claims: Vec<Claim>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    #[serde(default)]
    pub cfg: Option<Vec<String>>,
    pub tool: String,
    pub tool_version: String,
    pub command: Vec<String>,
    pub exit_code: i32,
    pub files: BTreeMap<String, String>,
    pub log_sha256: String,
    #[serde(default)]
    pub harnesses: Vec<String>,
    #[serde(default)]
    pub covered_units: BTreeMap<String, String>,
}
pub fn sha256(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}

/// A receipt's own validation and attribution, before unioning by unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRun {
    pub tool: String,
    pub log: String,
    pub accepted: bool,
    pub reasons: Vec<String>,
    pub credited_units: Vec<String>,
    #[serde(default)]
    pub verified_files: Vec<String>,
    #[serde(default)]
    pub cfg: Vec<String>,
    #[serde(default)]
    pub trusted_sites: Vec<(String, usize, String)>,
    #[serde(default)]
    pub escape_sites: Vec<(String, usize, String)>,
    #[serde(default)]
    pub trusted_sloc: usize,
}

pub fn apply_evidence(
    report: &mut Report,
    root: &Path,
    verus: Option<&Path>,
    kani: Option<&Path>,
) -> Result<()> {
    apply_evidence_many(
        report,
        root,
        &verus.into_iter().map(Path::to_path_buf).collect::<Vec<_>>(),
        &kani.into_iter().map(Path::to_path_buf).collect::<Vec<_>>(),
    )
}

pub fn apply_evidence_many(
    report: &mut Report,
    root: &Path,
    verus: &[std::path::PathBuf],
    kani: &[std::path::PathBuf],
) -> Result<()> {
    let registry: Registry = toml::from_str(include_str!("../registry/upstream.toml"))?;
    apply_upstream_registry(report, registry.claims)?;
    let files: BTreeSet<_> = report.source_files.iter().cloned().collect();
    for file in files {
        let source = fs::read_to_string(root.join(&file))?;
        let stream: proc_macro2::TokenStream =
            source.parse().map_err(|e| anyhow::anyhow!("{e}"))?;
        let mut found = Vec::new();
        escapes(stream, &mut found);
        found.sort();
        found.dedup();
        for (line, kind) in found {
            let mut affected: Vec<_> = report
                .units
                .iter()
                .filter(|u| {
                    u.file == file
                        && ((u.line <= line && line <= u.end_line)
                            || u.scope_sites.contains(&(line, kind.clone())))
                })
                .collect();
            let trusted = ["external_body", "assume_specification"].contains(&kind.as_str());
            // Kani assumptions constrain harness inputs. Keep other assumptions visible.
            if kind == "kani::assume" && affected.iter().any(|u| u.kani_harness || u.kani_only) {
                continue;
            }
            if affected.is_empty() && !trusted {
                affected.extend(report.units.iter().filter(|u| u.file == file));
            }
            let ids: Vec<_> = affected.iter().map(|u| u.id.clone()).collect();
            if trusted {
                let sloc = if affected.is_empty() {
                    // Standalone assume_specification, or an attributed non-function trusted item.
                    trusted_item_sloc(&source, line)
                } else {
                    affected.iter().map(|u| u.sloc).sum()
                };
                report.trusted_items.push(TrustedItem {
                    file: file.clone(),
                    line,
                    kind,
                    affected_units: ids,
                    sloc,
                });
            } else {
                report.escape_hatches.push(Escape {
                    unit: ids
                        .first()
                        .cloned()
                        .unwrap_or_else(|| format!("{file}::<file-scope>")),
                    affected_units: ids,
                    file: file.clone(),
                    line,
                    kind,
                    security: affected.is_empty() || affected.iter().any(|u| u.security),
                    first_party: true,
                });
            }
        }
    }
    let runtime_count = report.dependencies.len();
    for (index, dependency) in report
        .dependencies
        .iter_mut()
        .chain(&mut report.build_dependencies)
        .enumerate()
    {
        let Some(source) = &dependency.source else {
            continue;
        };
        let mut files = Vec::new();
        crate::collect_rs(&root.join(source), &mut files)?;
        let mut unsafe_sites = 0;
        let mut scan_confirmed = true;
        for file in files {
            let text = fs::read_to_string(&file)?;
            let Ok(stream) = text.parse::<proc_macro2::TokenStream>() else {
                report.warnings.push(format!(
                    "{}: dependency escape scan unconfirmed (tokenization failed)",
                    file.display()
                ));
                scan_confirmed = false;
                continue;
            };
            let mut found = Vec::new();
            escapes(stream, &mut found);
            found.sort();
            found.dedup();
            unsafe_sites += found.iter().filter(|(_, kind)| kind == "unsafe").count();
            for (line, kind) in found {
                if index >= runtime_count {
                    continue;
                }
                report.escape_hatches.push(Escape {
                    unit: format!("{}@{}", dependency.name, dependency.version),
                    affected_units: vec![],
                    file: file
                        .strip_prefix(root)
                        .unwrap_or(&file)
                        .to_string_lossy()
                        .to_string(),
                    line,
                    kind,
                    security: dependency.security,
                    first_party: false,
                });
            }
        }
        dependency.unsafe_sites = scan_confirmed.then_some(unsafe_sites);
    }
    let mut runs = Vec::new();
    for (paths, tool) in [(verus, "verus"), (kani, "kani")] {
        for path in paths {
            let mut reasons = Vec::new();
            let files = match evidence_files(root, Some(path), tool, &mut reasons) {
                Ok(files) => files,
                Err(e) => {
                    reasons.push(format!("{tool} evidence unconfirmed: {e}"));
                    BTreeSet::new()
                }
            };
            let accepted = !files.is_empty();
            let mut receipt = if accepted {
                Some(serde_json::from_slice::<Receipt>(&fs::read(format!(
                    "{}.evidence.json",
                    path.display()
                ))?)?)
            } else {
                None
            };
            if tool == "kani" {
                if let Some(r) = &mut receipt {
                    let raw = r.harnesses[0].clone();
                    if let Some(unit) = crate::kani::harness(&raw, &report.units) {
                        r.harnesses = vec![unit.id.clone()];
                        for id in r.covered_units.values_mut() {
                            if *id == raw {
                                *id = unit.id.clone();
                            }
                        }
                    }
                }
            }
            report
                .warnings
                .extend(reasons.iter().map(|r| format!("{}: {r}", path.display())));
            let scope_files: BTreeSet<String> = if tool == "verus" {
                files.clone()
            } else {
                report
                    .units
                    .iter()
                    .filter(|u| {
                        receipt.as_ref().is_some_and(|r| {
                            r.harnesses.contains(&u.id) || r.covered_units.contains_key(&u.id)
                        })
                    })
                    .map(|u| u.file.clone())
                    .collect()
            };
            let mut scope_warnings = vec![];
            let mut in_scope = |file: &String, line: usize, kind: &String, affected: &[String]| {
                if !scope_files.contains(file) {
                    return false;
                }
                let Some(receipt) = &receipt else {
                    return false;
                };
                let owners: Vec<_> = report
                    .units
                    .iter()
                    .filter(|u| {
                        &u.file == file
                            && affected.contains(&u.id)
                            && ((u.line <= line && line <= u.end_line)
                                || u.scope_sites.contains(&(line, kind.clone())))
                    })
                    .collect();
                if owners.is_empty() {
                    let Ok(source) = fs::read_to_string(root.join(file)) else {
                        scope_warnings.push(format!("{file}:{line}: source scope unconfirmed"));
                        return false;
                    };
                    let attrs = crate::cfg::site_attributes(&source, line, kind);
                    crate::cfg::enabled(
                        &attrs,
                        receipt,
                        &format!("{file}:{line}"),
                        &mut scope_warnings,
                    )
                } else {
                    owners.iter().any(|u| {
                        crate::cfg::enabled(
                            &u.scope_attributes,
                            receipt,
                            &u.id,
                            &mut scope_warnings,
                        )
                    })
                }
            };
            let trusted: Vec<_> = report
                .trusted_items
                .iter()
                .filter(|t| in_scope(&t.file, t.line, &t.kind, &t.affected_units))
                .collect();
            let escape_sites = report
                .escape_hatches
                .iter()
                .filter(|e| e.first_party && in_scope(&e.file, e.line, &e.kind, &e.affected_units))
                .map(|e| (e.file.clone(), e.line, e.kind.clone()))
                .collect();
            report.warnings.extend(scope_warnings);
            report.evidence_runs.push(EvidenceRun {
                tool: tool.into(),
                log: path.display().to_string(),
                accepted,
                reasons,
                credited_units: vec![],
                verified_files: scope_files.into_iter().collect(),
                cfg: receipt
                    .as_ref()
                    .map(crate::cfg::values)
                    .unwrap_or_default()
                    .into_iter()
                    .collect(),
                trusted_sites: trusted
                    .iter()
                    .map(|t| (t.file.clone(), t.line, t.kind.clone()))
                    .collect(),
                escape_sites,
                trusted_sloc: trusted.iter().map(|t| t.sloc).sum(),
            });
            runs.push((tool, files, receipt));
        }
    }
    let tests: Vec<_> = report
        .units
        .iter()
        .filter(|u| u.tokens.contains("[test]"))
        .map(|u| u.tokens.clone())
        .collect();
    let test_identifiers: HashSet<_> = tests.iter().flat_map(|t| identifiers(t)).collect();
    // Keep each unit's hazards in their original order, including the legacy
    // primary owner, without adding the same escape twice for that owner.
    let mut hazards_by_unit: BTreeMap<&str, Vec<&Escape>> = BTreeMap::new();
    for escape in report.escape_hatches.iter().filter(|e| e.first_party) {
        let owners: BTreeSet<_> = escape
            .affected_units
            .iter()
            .map(String::as_str)
            .chain(std::iter::once(escape.unit.as_str()))
            .collect();
        for owner in owners {
            hazards_by_unit.entry(owner).or_default().push(escape);
        }
    }
    let trusted_units: HashSet<_> = report
        .trusted_items
        .iter()
        .flat_map(|t| t.affected_units.iter().map(String::as_str))
        .collect();
    // Escapes remain gate-visible globally; attribution is receipt-local.
    let mut name_counts = BTreeMap::new();
    let mut id_counts = BTreeMap::new();
    for u in &report.units {
        *name_counts.entry(u.name.clone()).or_insert(0usize) += 1;
        *id_counts.entry(u.id.clone()).or_insert(0usize) += 1;
    }
    let kani_harnesses: Vec<BTreeMap<_, _>> = runs
        .iter()
        .map(|(tool, files, _)| {
            report
                .units
                .iter()
                .filter(|u| {
                    *tool == "kani"
                        && u.kani_harness
                        && id_counts.get(&u.id) == Some(&1)
                        && files.contains(&format!("{}#{}", u.file, u.id))
                })
                .map(|u| (u.id.clone(), u.tokens.clone()))
                .collect()
        })
        .collect();
    for u in &mut report.units {
        if u.category == crate::Category::Runtime && test_identifiers.contains(u.name.as_str()) {
            u.status = "tested".into();
        }
        let contracted = u.verus && {
            let names = identifiers(&u.tokens);
            names.contains("requires") || names.contains("ensures")
        };
        let hazards = hazards_by_unit
            .get(u.id.as_str())
            .map(Vec::as_slice)
            .unwrap_or_default();
        let mut clean_verus_run = false;
        for (index, (tool, files, receipt)) in runs.iter().enumerate() {
            if *tool != "verus" || !files.contains(&u.file) {
                continue;
            }
            let Some(receipt) = receipt else {
                continue;
            };
            if !(contracted || (u.verus && u.category == crate::Category::ProofCode))
                || !crate::cfg::enabled(
                    &u.scope_attributes,
                    receipt,
                    &format!("{}:{} {}", u.file, u.line, u.id),
                    &mut report.warnings,
                )
            {
                continue;
            }
            let run = &report.evidence_runs[index];
            if hazards.iter().any(|e| {
                run.escape_sites
                    .contains(&(e.file.clone(), e.line, e.kind.clone()))
            }) {
                continue;
            }
            clean_verus_run = true;
            u.status = if u.category == crate::Category::ProofCode {
                "verified"
            } else {
                "proven"
            }
            .into();
            let run = &report.evidence_runs[index];
            u.proof_scope = Some(format!("Verus complete run; relying on {} trusted items ({} SLoC) and {} escapes in this run; specification correctness and toolchain are outside this proof",run.trusted_sites.len(),run.trusted_sloc,run.escape_sites.len()));
            report.evidence_runs[index]
                .credited_units
                .push(u.id.clone());
        }
        let unit_trusted = trusted_units.contains(u.id.as_str());
        if unit_trusted {
            u.status = "trusted".into();
            u.proof_scope = Some("Unchecked trusted specification/body; never proven".into());
        }
        if !hazards.is_empty() && !clean_verus_run {
            u.status = "hazard".into();
            u.proof_scope = None;
        }
        for (index, (tool, _, receipt)) in runs.iter().enumerate() {
            if *tool != "kani" {
                continue;
            }
            let Some(receipt) = receipt else {
                continue;
            };
            let harnesses = &kani_harnesses[index];
            let harness_id = if harnesses.contains_key(&u.id) {
                Some(&u.id)
            } else {
                receipt.covered_units.get(&u.id)
            };
            let covered = harness_id
                .and_then(|id| harnesses.get(id))
                .is_some_and(|text| {
                    harnesses.contains_key(&u.id)
                        || (name_counts.get(&u.name) == Some(&1) && direct_call(text, &u.name))
                });
            let scope = harness_id.and_then(|id| {
                harnesses
                    .get(id)
                    .and_then(|text| crate::kani::scope(&receipt.command, text, id).ok())
            });
            if scope.is_some()
                && !unit_trusted
                && report.evidence_runs[index].trusted_sites.is_empty()
                && report.evidence_runs[index]
                    .escape_sites
                    .iter()
                    .all(|(_, _, kind)| kind == "unsafe")
                && covered
                && hazards.iter().all(|e| e.kind == "unsafe")
            {
                report.evidence_runs[index]
                    .credited_units
                    .push(u.id.clone());
                if u.status != "proven" {
                    u.status = "proven-bounded".into();
                    u.proof_scope = scope;
                }
            }
        }
    }
    let proven_ids: HashSet<_> = report
        .units
        .iter()
        .filter(|u| ["proven", "verified", "proven-bounded"].contains(&u.status.as_str()))
        .map(|u| u.id.as_str())
        .collect();
    for run in &mut report.evidence_runs {
        run.credited_units
            .retain(|id| proven_ids.contains(id.as_str()));
        run.credited_units.sort();
        run.credited_units.dedup();
    }
    report
        .escape_hatches
        .sort_by(|a, b| (&a.file, a.line, &a.kind).cmp(&(&b.file, b.line, &b.kind)));
    report
        .escape_hatches
        .dedup_by(|a, b| a.file == b.file && a.line == b.line && a.kind == b.kind);
    Ok(())
}
fn identifiers(text: &str) -> BTreeSet<&str> {
    text.split(|c: char| !c.is_alphanumeric() && c != '_')
        .collect()
}
fn escapes(stream: proc_macro2::TokenStream, found: &mut Vec<(usize, String)>) {
    use proc_macro2::TokenTree;
    let tokens: Vec<_> = stream.into_iter().collect();
    for (i, t) in tokens.iter().enumerate() {
        if let TokenTree::Ident(id) = t {
            let name = id.to_string();
            if [
                "assume",
                "admit",
                "unsafe",
                "external_body",
                "external",
                "assume_specification",
            ]
            .contains(&name.as_str())
            {
                let kind = if name == "assume"
                    && i >= 3
                    && tokens[i - 3].to_string() == "kani"
                    && tokens[i - 2].to_string() == ":"
                    && tokens[i - 1].to_string() == ":"
                {
                    "kani::assume".into()
                } else {
                    name
                };
                found.push((id.span().start().line, kind));
            } else if name == "allow"
                && tokens.get(i + 1).is_some_and(|g| {
                    let text = g.to_string();
                    text.contains("verifier") || text.contains("verus") || text.contains("kani")
                })
            {
                found.push((id.span().start().line, "allow(verification lint)".into()));
            }
        }
        if let TokenTree::Group(g) = t {
            escapes(g.stream(), found);
        }
    }
}
fn evidence_files(
    root: &Path,
    path: Option<&Path>,
    tool: &str,
    warnings: &mut Vec<String>,
) -> Result<BTreeSet<String>> {
    let mut covered = BTreeSet::new();
    let Some(path) = path else {
        return Ok(covered);
    };
    let log = fs::read(path)?;
    let sidecar = std::path::PathBuf::from(format!("{}.evidence.json", path.display()));
    if !sidecar.exists() {
        warnings.push(format!(
            "{}: missing source-bound run receipt {}; evidence unconfirmed",
            tool,
            sidecar.display()
        ));
        return Ok(covered);
    }
    let receipt: Receipt = serde_json::from_slice(&fs::read(sidecar)?)?;
    let text = String::from_utf8_lossy(&log);
    let fail = |warnings: &mut Vec<String>, why: &str| {
        warnings.push(format!("{tool} evidence unconfirmed: {why}"));
    };
    let mismatch = if receipt.tool != tool {
        Some("tool mismatch")
    } else if receipt.tool_version.trim().is_empty() {
        Some("missing tool version")
    } else if receipt.exit_code != 0 {
        Some("nonzero exit status")
    } else if receipt.log_sha256 != sha256(&log) {
        Some("log hash mismatch")
    } else {
        None
    };
    if let Some(reason) = mismatch {
        fail(warnings, reason);
        return Ok(covered);
    }
    if tool == "verus" {
        // Supported complete-file invocation only; filtered, imported or verify-skipping runs fail closed.
        if verus_root(&receipt).is_none() {
            fail(warnings, "unsupported or partial verifier command");
            return Ok(covered);
        }
        let results: Vec<_> = text
            .lines()
            .filter(|l| l.starts_with("verification results:: "))
            .collect();
        if results.len() != 1
            || !verus_success(results[0])
            || text
                .lines()
                .any(|l| l.starts_with("error") || l.contains("partial verification"))
        {
            fail(
                warnings,
                "missing, failing, zero-count or partial Verus summary",
            );
            return Ok(covered);
        }
    } else {
        let Some(command) = crate::kani::command(&receipt.command) else {
            fail(
                warnings,
                "unsupported Kani command (one explicit harness per serial log required)",
            );
            return Ok(covered);
        };
        if receipt.harnesses != [command.harness] {
            fail(warnings, "Kani command/receipt harness mismatch");
            return Ok(covered);
        }
        let checking: Vec<_> = text
            .lines()
            .filter_map(|l| l.strip_prefix("Checking harness "))
            .map(|s| s.trim_end_matches("...").trim())
            .collect();
        let successes = text
            .lines()
            .filter(|l| l.trim() == "VERIFICATION:- SUCCESSFUL")
            .count();
        if checking != [command.harness]
            || successes != 1
            || text.contains("FAILED")
            || text.contains("expected")
            || text.contains("error:")
        {
            fail(warnings, "Kani harness result missing, ambiguous or failed");
            return Ok(covered);
        }
    }
    if receipt.files.is_empty() {
        fail(warnings, "no source files");
        return Ok(covered);
    }
    // Require hashes for every first-party Rust source: module or imported helper edits invalidate evidence.
    let current = crate::inventory(root)?;
    if current
        .source_files
        .iter()
        .any(|file| !receipt.files.contains_key(file))
    {
        fail(
            warnings,
            &format!(
                "receipt must hash all inventoried first-party source files; missing: {}",
                current
                    .source_files
                    .iter()
                    .filter(|file| !receipt.files.contains_key(*file))
                    .cloned()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
        return Ok(covered);
    }
    for (file, hash) in &receipt.files {
        let path = root.join(file).canonicalize()?;
        anyhow::ensure!(
            path.starts_with(root.canonicalize()?),
            "receipt source outside workspace"
        );
        if sha256(&fs::read(path)?) != *hash {
            fail(warnings, "stale source hash");
            return Ok(BTreeSet::new());
        }
    }
    if tool == "verus" {
        covered = crate::modules::closure(root, &receipt, warnings)?;
    } else {
        let Some(unit) = crate::kani::harness(&receipt.harnesses[0], &current.units) else {
            fail(
                warnings,
                "Kani harness is missing or filter is ambiguous in hashed source",
            );
            return Ok(covered);
        };
        if let Err(error) = crate::kani::scope(&receipt.command, &unit.tokens, &unit.id) {
            fail(
                warnings,
                &format!("unsupported Kani harness scope: {error}"),
            );
            return Ok(covered);
        }
        covered.insert(format!("{}#{}", unit.file, unit.id));
    }
    warnings.push(format!("{tool}: accepted caller-supplied run receipt {}; receipts are evidence, not cryptographic attestation of execution",path.display()));
    Ok(covered)
}
fn verus_success(line: &str) -> bool {
    let Some(rest) = line.strip_prefix("verification results:: ") else {
        return false;
    };
    let Some((count, errors)) = rest.split_once(" verified, ") else {
        return false;
    };
    count.parse::<usize>().is_ok_and(|n| n > 0) && errors == "0 errors"
}

// Keep the status vocabulary explicit: operation-only upstream claims are never whole-crate proof.
pub const STATUSES: [&str; 7] = [
    "proven",
    "proven-bounded",
    "proven-upstream",
    "tested",
    "unverified",
    "hazard",
    "trusted",
];

fn direct_call(text: &str, name: &str) -> bool {
    fn visit(stream: proc_macro2::TokenStream, name: &str) -> bool {
        let tokens: Vec<_> = stream.into_iter().collect();
        tokens.iter().enumerate().any(|(i,t)| {
            if t.to_string() == name && matches!(tokens.get(i+1),Some(proc_macro2::TokenTree::Group(g)) if g.delimiter() == proc_macro2::Delimiter::Parenthesis) {return true;}
            if let proc_macro2::TokenTree::Group(g)=t { return visit(g.stream(),name); }
            false
        })
    }
    text.parse().is_ok_and(|stream| visit(stream, name))
}

/// Apply explicit registry claims. Partial or unconfirmed claims never promote whole crates.
pub fn apply_upstream_registry(report: &mut Report, claims: Vec<Claim>) -> Result<()> {
    for claim in claims {
        anyhow::ensure!(
            ["confirmed", "unconfirmed"].contains(&claim.confirmation.as_str()),
            "invalid upstream confirmation"
        );
        anyhow::ensure!(
            claim.source.starts_with("https://")
                && !claim.operations.trim().is_empty()
                && !claim.property.trim().is_empty()
                && !claim.tool.trim().is_empty(),
            "upstream claim needs a primary source and explicit scope"
        );
        let requirement = semver::VersionReq::parse(&claim.version)?;
        let mut matched = false;
        for dependency in &mut report.dependencies {
            if dependency.name == claim.crate_name
                && semver::Version::parse(&dependency.version)
                    .is_ok_and(|v| requirement.matches(&v))
            {
                matched = true;
                if claim.whole_crate && claim.confirmation == "confirmed" {
                    dependency.status = "proven-upstream".into();
                }
            }
        }
        if matched {
            report.upstream_claims.push(claim);
        }
    }
    Ok(())
}

/// Complete direct invocations only; harmless build cfg switches are explicitly parsed.
pub(crate) fn verus_root(r: &Receipt) -> Option<&str> {
    if !r.command.first()?.ends_with("verus") {
        return None;
    }
    let mut root = None;
    let mut library = false;
    let mut i = 1;
    while i < r.command.len() {
        let arg = &r.command[i];
        match arg.as_str() {
            "--crate-type" if r.command.get(i + 1)?.as_str() == "lib" => {
                library = true;
                i += 2;
            }
            "--crate-type=lib" => {
                library = true;
                i += 1;
            }
            "--cfg" => {
                r.command.get(i + 1)?;
                i += 2;
            }
            _ if arg.starts_with("--cfg=") || arg.starts_with("--edition=") => {
                i += 1;
            }
            _ if !arg.starts_with('-') && r.files.contains_key(arg) && root.is_none() => {
                root = Some(arg.as_str());
                i += 1;
            }
            _ => return None,
        }
    }
    if library {
        root
    } else {
        None
    }
}

pub(crate) fn attribute_sites(stream: proc_macro2::TokenStream) -> Vec<(usize, String)> {
    let mut found = vec![];
    escapes(stream, &mut found);
    found
}
fn trusted_item_sloc(source: &str, line: usize) -> usize {
    fn end(stream: proc_macro2::TokenStream, line: usize) -> Option<usize> {
        let tokens: Vec<_> = stream.into_iter().collect();
        for (i, t) in tokens.iter().enumerate() {
            if t.span().start().line == line
                && (t.to_string() == "#" || t.to_string() == "assume_specification")
            {
                for next in &tokens[i + 1..] {
                    if next.to_string() == ";"
                        || matches!(next,proc_macro2::TokenTree::Group(g) if g.delimiter()==proc_macro2::Delimiter::Brace)
                    {
                        return Some(next.span().end().line);
                    }
                }
            }
            if let proc_macro2::TokenTree::Group(g) = t {
                if let Some(e) = end(g.stream(), line) {
                    return Some(e);
                }
            }
        }
        None
    }
    let last = source
        .parse()
        .ok()
        .and_then(|s| end(s, line))
        .unwrap_or(line);
    crate::sloc(
        &source
            .lines()
            .skip(line - 1)
            .take(last - line + 1)
            .collect::<Vec<_>>()
            .join("\n"),
    )
}
