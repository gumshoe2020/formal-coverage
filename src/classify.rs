use crate::{inventory, Report, SizeMode, Unit};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Default, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub entry_points: Vec<String>,
    #[serde(default)]
    pub security: Vec<String>,
    #[serde(default)]
    pub not_security: Vec<String>,
    #[serde(default)]
    pub dependency_sources: BTreeMap<String, std::path::PathBuf>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Override {
    pub pattern: String,
    pub security: bool,
    pub matched: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub security: bool,
    pub sloc: usize,
    pub size_known: bool,
    pub source: Option<String>,
    pub unsafe_sites: Option<usize>,
    pub status: String,
    /// `whole-crate`, `compiled-files`, `linked-lines` or `whole-crate (fallback)`.
    #[serde(default)]
    pub size_mode: String,
    #[serde(default)]
    pub size_reason: Option<String>,
    /// Every `.rs` file under the package source directory (the v1.4.2 measure).
    #[serde(default)]
    pub whole_crate_sloc: usize,
    /// Security under default rules, when a `[dependencies.*]` input or the units' inputs changed it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_security: Option<bool>,
    /// Measured size before a configured `sloc` input replaced it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_sloc: Option<usize>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub default_size_unknown: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_reason: Option<String>,
    #[serde(skip)]
    pub package_id: Option<String>,
    #[serde(skip)]
    pub lib_name: Option<String>,
}

/// Analysis inputs beyond the workspace root.
#[derive(Debug, Clone, Default)]
pub struct Options {
    pub ghost_lines: crate::GhostLines,
    pub proof_ratio_range: Option<crate::ProofRatioRange>,
    pub no_proof_ratio_gate: bool,
    pub config: Option<PathBuf>,
    pub target: Option<String>,
    pub size_mode: SizeMode,
    /// Linked-lines mode: the binary whose DWARF line table is read.
    pub binary: Option<PathBuf>,
    /// Compiled-files mode: where `cargo check` writes; default `<root>/target/formal-coverage`.
    pub check_target_dir: Option<PathBuf>,
}

pub fn analyze(root: &Path, config_path: Option<&Path>) -> Result<Report> {
    analyze_target(root, config_path, None)
}

pub fn analyze_target(
    root: &Path,
    config_path: Option<&Path>,
    target: Option<&str>,
) -> Result<Report> {
    analyze_with(
        root,
        &Options {
            config: config_path.map(Path::to_path_buf),
            target: target.map(str::to_string),
            ..Options::default()
        },
    )
}

pub fn analyze_with(root: &Path, options: &Options) -> Result<Report> {
    let target = options.target.as_deref();
    let mut report = inventory(root)?;
    // whole-crate reproduces v1.4.2, which kept test code in runtime categories.
    if options.size_mode != SizeMode::WholeCrate {
        for u in &mut report.units {
            if u.test_code && u.category == crate::Category::Runtime {
                u.category = crate::Category::Test;
            }
        }
    }
    crate::ghost::apply(&mut report, options.ghost_lines);
    let path = options
        .config
        .clone()
        .unwrap_or_else(|| root.join("formal-coverage.toml"));
    let config = crate::inputs::load(&path)?;
    report.config_schema = config.schema;
    if let Some(ratio) = &config.proof_ratio {
        report.proof_ratio_range = ratio.range()?;
        report.inputs.push(crate::InputRecord {
            kind: "proof_ratio".into(),
            key: "range".into(),
            direction: "acceptance range".into(),
            reason: Some(ratio.reason.clone()),
            matched: vec!["first-party security-relevant".into()],
            sloc_into_security: 0,
            sloc_out_of_security: 0,
            size_change: None,
        });
    }
    if let Some(range) = &options.proof_ratio_range {
        range.validate()?;
        report.proof_ratio_range = range.clone();
    }
    report.no_proof_ratio_gate = options.no_proof_ratio_gate;
    let mut reachable = BTreeSet::new();
    if config.entry_points.is_empty() {
        report.warnings.push(
            "No entry points: all first-party functions conservatively security-relevant.".into(),
        );
        reachable.extend(0..report.units.len());
    }
    for entry in &config.entry_points {
        let found: Vec<_> = report
            .units
            .iter()
            .enumerate()
            .filter(|(_, u)| matches(entry, u).unwrap_or(false))
            .map(|(i, _)| i)
            .collect();
        anyhow::ensure!(
            !found.is_empty(),
            "entry point {entry:?} matches no function"
        );
        reachable.extend(found);
    }
    // Identifiers overapproximate calls, function pointers, aliases, dispatch and macro inputs.
    // Unknown call targets can hide arbitrary edges: include every function in that case.
    // Once every unit is reachable, rescanning edges cannot grow the set.
    while reachable.len() < report.units.len() {
        let mut next = reachable.clone();
        for &i in &reachable {
            let text = &report.units[i].tokens;
            let identifiers = words(text);
            for (j, u) in report.units.iter().enumerate() {
                if identifiers.contains(&u.name.as_str()) {
                    next.insert(j);
                }
            }
            if unresolved_call(text, &report.units) {
                next.extend(0..report.units.len());
            }
        }
        if next == reachable {
            break;
        }
        reachable = next;
    }
    for (i, u) in report.units.iter_mut().enumerate() {
        u.security = reachable.contains(&i);
    }
    // Default rules, then includes, then excludes: the last matching input decides.
    let defaults: Vec<bool> = report.units.iter().map(|u| u.security).collect();
    let mut decided: Vec<Option<usize>> = vec![None; report.units.len()];
    for (rules, security) in [(&config.include, true), (&config.exclude, false)] {
        for (pattern, reason) in rules {
            let record = report.inputs.len();
            let mut matched = Vec::new();
            for (i, u) in report.units.iter_mut().enumerate() {
                if matches(pattern, u)? {
                    u.security = security;
                    matched.push(u.id.clone());
                    decided[i] = Some(record);
                }
            }
            let (kind, legacy) = match (config.schema, security) {
                (2, true) => ("security", true),
                (2, false) => ("not_security", true),
                (_, true) => ("security.include", false),
                (_, false) => ("security.exclude", false),
            };
            if legacy {
                report.warnings.push(format!("override has no reason: {kind} {pattern:?} (schema-2 config; `formal-coverage migrate-config` rewrites it with TODO reasons)"));
            } else {
                anyhow::ensure!(
                    !matched.is_empty(),
                    "{kind} pattern {pattern:?} matches no function"
                );
            }
            report.overrides.push(Override {
                pattern: pattern.clone(),
                security,
                matched: matched.clone(),
            });
            report.inputs.push(crate::InputRecord {
                kind: kind.into(),
                key: pattern.clone(),
                direction: if security {
                    "into security"
                } else {
                    "out of security"
                }
                .into(),
                reason: reason.clone(),
                matched,
                sloc_into_security: 0,
                sloc_out_of_security: 0,
                size_change: None,
            });
        }
    }
    for (i, u) in report.units.iter_mut().enumerate() {
        if u.security != defaults[i] {
            u.default_security = Some(defaults[i]);
            if u.category == crate::Category::Runtime {
                let record = &mut report.inputs[decided[i].expect("moved units were matched")];
                if u.security {
                    record.sloc_into_security += u.sloc;
                } else {
                    record.sloc_out_of_security += u.sloc;
                }
            }
        }
    }
    crate::dependencies::resolve(root, target, &mut report)?;
    // Default-rules dependency security mirrors resolve(): any first-party security unit.
    let default_dependency_security = defaults.iter().any(|s| *s);
    for d in &mut report.dependencies {
        if d.security != default_dependency_security {
            d.default_security = Some(default_dependency_security);
        }
    }
    for d in report
        .dependencies
        .iter_mut()
        .chain(&mut report.build_dependencies)
    {
        if let Some(path) = config
            .dependency_sources
            .get(&format!("{}@{}", d.name, d.version))
        {
            let source = root.join(path).canonicalize()?;
            anyhow::ensure!(
                source.starts_with(root.canonicalize()?),
                "dependency source must be inside analyzed workspace"
            );
            let manifest: toml::Value =
                toml::from_str(&fs::read_to_string(source.join("Cargo.toml"))?)?;
            anyhow::ensure!(
                manifest
                    .get("package")
                    .and_then(|p| p.get("name"))
                    .and_then(|v| v.as_str())
                    == Some(d.name.as_str())
                    && manifest
                        .get("package")
                        .and_then(|p| p.get("version"))
                        .and_then(|v| v.as_str())
                        == Some(d.version.as_str()),
                "dependency source manifest does not match {}@{}",
                d.name,
                d.version
            );
            let mut files = Vec::new();
            crate::collect_rs(&source, &mut files)?;
            d.sloc = files
                .iter()
                .map(|p| fs::read_to_string(p).map(|s| crate::sloc(&s)))
                .collect::<std::io::Result<Vec<_>>>()?
                .iter()
                .sum();
            d.size_known = true;
            d.whole_crate_sloc = d.sloc;
            d.source = Some(path.to_string_lossy().to_string());
        }
    }
    crate::size::apply(root, options, &mut report)?;
    apply_dependency_rules(&config.dependencies, &mut report)?;
    report.security_relevant = report
        .units
        .iter()
        .filter(|u| u.security && u.category == crate::Category::Runtime)
        .map(|u| u.sloc)
        .sum::<usize>()
        + report
            .dependencies
            .iter()
            .filter(|d| d.security)
            .map(|d| d.sloc)
            .sum::<usize>();
    report.everything_else = report
        .units
        .iter()
        .filter(|u| !u.security && u.category == crate::Category::Runtime)
        .map(|u| u.sloc)
        .sum::<usize>()
        + report
            .dependencies
            .iter()
            .filter(|d| !d.security)
            .map(|d| d.sloc)
            .sum::<usize>();
    report.approximation = "Approximate syntactic identifier graph: ambiguous names include all matching functions; unresolved calls and macros include all first-party functions. No entry points includes all. cfg/test code included; spec/proof functions inventoried separately as proof-code, excluded from runtime totals. Overrides apply after reachability (not_security last). Dependency closure and target are recorded separately; package source inventories conservatively include cfg/test code.".into();
    if options.size_mode != SizeMode::WholeCrate {
        report.approximation = report.approximation.replace(
            "cfg/test code included;",
            "structural test code (test attributes, items under cfg(test) at any depth, integration-test and bench targets) inventoried separately as test-code, excluded from runtime totals; other cfg-disabled code included;",
        );
        report.approximation = report.approximation.replace(
            "package source inventories conservatively include cfg/test code.",
            &format!(
                "runtime dependency sizes use {}: {}",
                options.size_mode.as_str(),
                options.size_mode.describe()
            ),
        );
    }
    Ok(report)
}
/// `[dependencies."name"]` / `[dependencies."name@version"]` inputs on runtime dependencies.
fn apply_dependency_rules(
    rules: &[(String, crate::inputs::DependencyRule)],
    report: &mut Report,
) -> Result<()> {
    for (key, rule) in rules {
        let (name, version) = match key.split_once('@') {
            Some((name, version)) => (name, Some(version)),
            None => (key.as_str(), None),
        };
        let hits: Vec<usize> = report
            .dependencies
            .iter()
            .enumerate()
            .filter(|(_, d)| d.name == name && version.is_none_or(|v| d.version == v))
            .map(|(i, _)| i)
            .collect();
        anyhow::ensure!(
            !hits.is_empty(),
            "dependency key {key:?} matches no runtime dependency"
        );
        anyhow::ensure!(
            hits.len() == 1,
            "dependency key {key:?} is ambiguous: {}; use name@version",
            hits.iter()
                .map(|&i| format!(
                    "{}@{}",
                    report.dependencies[i].name, report.dependencies[i].version
                ))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let d = &mut report.dependencies[hits[0]];
        let mut record = crate::InputRecord {
            kind: "dependencies".into(),
            key: key.clone(),
            direction: String::new(),
            reason: Some(rule.reason.clone()),
            matched: vec![format!("{}@{}", d.name, d.version)],
            sloc_into_security: 0,
            sloc_out_of_security: 0,
            size_change: None,
        };
        let mut directions = Vec::new();
        let default_security = d.default_security.unwrap_or(d.security);
        if let Some(sloc) = rule.sloc {
            directions.push(format!("size {sloc}"));
            if !d.size_known || d.sloc != sloc {
                d.default_sloc = Some(d.sloc);
                d.default_size_unknown = !d.size_known;
                record.size_change = Some(sloc as i64 - d.sloc as i64);
            }
            d.sloc = sloc;
            d.size_known = true;
            d.size_mode = "configured".into();
            d.size_reason = Some(rule.reason.clone());
        }
        if let Some(security) = rule.security {
            directions.push(if security { "security" } else { "not security" }.into());
            d.security = security;
        }
        d.default_security = (d.security != default_security).then_some(default_security);
        d.override_reason = Some(rule.reason.clone());
        let default_sloc = d.default_sloc.unwrap_or(d.sloc);
        match (default_security, d.security) {
            (false, true) => record.sloc_into_security = d.sloc,
            (true, false) => record.sloc_out_of_security = default_sloc,
            _ => {}
        }
        record.direction = directions.join(", ");
        report.inputs.push(record);
    }
    Ok(())
}

fn matches(pattern: &str, u: &Unit) -> Result<bool> {
    let glob = glob::Pattern::new(pattern)?;
    Ok(glob.matches(&u.id) || glob.matches(&u.file))
}
fn words(text: &str) -> BTreeSet<&str> {
    text.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|s| !s.is_empty())
        .collect()
}
fn unresolved_call(text: &str, units: &[Unit]) -> bool {
    fn visit(stream: proc_macro2::TokenStream, units: &[Unit]) -> bool {
        use proc_macro2::{Delimiter, TokenTree};
        let tokens: Vec<_> = stream.into_iter().collect();
        for (i, token) in tokens.iter().enumerate() {
            if let TokenTree::Group(group) = token {
                if group.delimiter() == Delimiter::Parenthesis && i > 0 {
                    match &tokens[i - 1] {
                        TokenTree::Ident(name) => {
                            let name = name.to_string();
                            if ![
                                "if",
                                "while",
                                "match",
                                "fn",
                                "requires",
                                "ensures",
                                "recommends",
                                "decreases",
                                "invariant",
                            ]
                            .contains(&name.as_str())
                                && !units.iter().any(|u| u.name == name)
                            {
                                return true;
                            }
                        }
                        TokenTree::Punct(p) if p.as_char() == '>' => return true,
                        TokenTree::Group(_) => return true,
                        _ => {}
                    }
                }
                if i > 0 && tokens[i - 1].to_string() == "!" {
                    return true;
                }
                if visit(group.stream(), units) {
                    return true;
                }
            }
        }
        false
    }
    text.parse()
        .map(|stream| visit(stream, units))
        .unwrap_or(true)
}
