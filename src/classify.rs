use crate::{inventory, Report, Unit};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
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
}

pub fn analyze(root: &Path, config_path: Option<&Path>) -> Result<Report> {
    analyze_target(root, config_path, None)
}

pub fn analyze_target(
    root: &Path,
    config_path: Option<&Path>,
    target: Option<&str>,
) -> Result<Report> {
    let mut report = inventory(root)?;
    let path = config_path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| root.join("formal-coverage.toml"));
    let config: Config = if path.exists() {
        toml::from_str(&fs::read_to_string(path)?)?
    } else {
        Config::default()
    };
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
    for (patterns, security) in [(&config.security, true), (&config.not_security, false)] {
        for pattern in patterns {
            let mut matched = Vec::new();
            for u in &mut report.units {
                if matches(pattern, u)? {
                    u.security = security;
                    matched.push(u.id.clone());
                }
            }
            report.overrides.push(Override {
                pattern: pattern.clone(),
                security,
                matched,
            });
        }
    }
    crate::dependencies::resolve(root, target, &mut report)?;
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
            d.source = Some(path.to_string_lossy().to_string());
        }
    }
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
    Ok(report)
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
