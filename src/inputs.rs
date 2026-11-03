//! Configuration schemas 2 (bare override globs) and 3 (explicit inputs with reasons).
use crate::Config;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

/// `[[security.include]]` / `[[security.exclude]]`: a unit pattern and why it moves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub pattern: String,
    pub reason: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct SecurityRules {
    #[serde(default)]
    include: Vec<Rule>,
    #[serde(default)]
    exclude: Vec<Rule>,
}

/// `[dependencies."name"]` or `[dependencies."name@version"]`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyRule {
    #[serde(default)]
    pub security: Option<bool>,
    #[serde(default)]
    pub sloc: Option<usize>,
    pub reason: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigV3 {
    proof_ratio: Option<crate::ProofRatioConfig>,
    #[serde(default)]
    entry_points: Vec<String>,
    #[serde(default)]
    security: SecurityRules,
    #[serde(default)]
    dependencies: BTreeMap<String, DependencyRule>,
    #[serde(default)]
    dependency_sources: BTreeMap<String, PathBuf>,
}

/// Either schema, normalized. Schema-2 overrides carry no reason.
#[derive(Debug, Default)]
pub(crate) struct Inputs {
    pub proof_ratio: Option<crate::ProofRatioConfig>,
    /// 0 when no config file exists.
    pub schema: u32,
    pub entry_points: Vec<String>,
    pub include: Vec<(String, Option<String>)>,
    pub exclude: Vec<(String, Option<String>)>,
    pub dependencies: Vec<(String, DependencyRule)>,
    pub dependency_sources: BTreeMap<String, PathBuf>,
}

/// One explicit input as applied, for the report's Inputs section.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputRecord {
    /// `security.include`, `security.exclude`, `security` / `not_security` (schema 2) or `dependencies`.
    pub kind: String,
    /// Unit pattern or dependency key.
    pub key: String,
    pub direction: String,
    pub reason: Option<String>,
    /// Unit ids, or `name@version` for a dependency.
    pub matched: Vec<String>,
    /// Runtime SLoC this input moved into the security side (default rules → with inputs).
    #[serde(default)]
    pub sloc_into_security: usize,
    /// Runtime SLoC this input moved out of the security side.
    #[serde(default)]
    pub sloc_out_of_security: usize,
    /// Change in a dependency's measured size (configured minus measured), if any.
    #[serde(default)]
    pub size_change: Option<i64>,
}

/// A schema-2 file uses flat `security = [...]` / `not_security = [...]` arrays.
fn is_schema2(value: &toml::Value) -> bool {
    value.get("not_security").is_some() || value.get("security").is_some_and(|s| s.is_array())
}

/// True if a reason still contains the placeholder written by `migrate-config`.
pub fn todo_reason(reason: &str) -> bool {
    reason
        .split(|c: char| !c.is_alphanumeric())
        .any(|word| word.eq_ignore_ascii_case("todo"))
}

pub(crate) fn load(path: &Path) -> Result<Inputs> {
    if !path.exists() {
        return Ok(Inputs::default());
    }
    let text = fs::read_to_string(path)?;
    let value: toml::Value =
        toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    if is_schema2(&value) {
        let config: Config = toml::from_str(&text)?;
        return Ok(Inputs {
            proof_ratio: None,
            schema: 2,
            entry_points: config.entry_points,
            include: config.security.into_iter().map(|p| (p, None)).collect(),
            exclude: config.not_security.into_iter().map(|p| (p, None)).collect(),
            dependencies: vec![],
            dependency_sources: config.dependency_sources,
        });
    }
    let config: ConfigV3 =
        toml::from_str(&text).with_context(|| format!("{}: schema-3 config", path.display()))?;
    if let Some(ratio) = &config.proof_ratio {
        ratio.range()?;
    }
    for rule in config
        .security
        .include
        .iter()
        .chain(&config.security.exclude)
    {
        anyhow::ensure!(
            !rule.reason.trim().is_empty(),
            "override {:?} has an empty reason; every input needs one",
            rule.pattern
        );
    }
    for (key, rule) in &config.dependencies {
        anyhow::ensure!(
            !rule.reason.trim().is_empty(),
            "dependency override {key:?} has an empty reason; every input needs one"
        );
        anyhow::ensure!(
            rule.security.is_some() || rule.sloc.is_some(),
            "dependency override {key:?} sets neither security nor sloc"
        );
    }
    Ok(Inputs {
        proof_ratio: config.proof_ratio,
        schema: 3,
        entry_points: config.entry_points,
        include: config
            .security
            .include
            .into_iter()
            .map(|r| (r.pattern, Some(r.reason)))
            .collect(),
        exclude: config
            .security
            .exclude
            .into_iter()
            .map(|r| (r.pattern, Some(r.reason)))
            .collect(),
        dependencies: config.dependencies.into_iter().collect(),
        dependency_sources: config.dependency_sources,
    })
}

fn quoted(s: &str) -> String {
    toml::Value::String(s.into()).to_string()
}

/// Rewrite a schema-2 config as schema 3 with `reason = "TODO"` on every override.
/// Comments are not carried over; `gate` fails until each TODO is replaced.
pub fn migrate_config(text: &str) -> Result<String> {
    let value: toml::Value = toml::from_str(text)?;
    anyhow::ensure!(
        is_schema2(&value),
        "not a schema-2 config (no flat security/not_security arrays)"
    );
    let config: Config = toml::from_str(text)?;
    let mut out = String::from("# Migrated from schema 2 by `formal-coverage migrate-config`. Replace every\n# TODO reason with the justification; `gate` fails while any reason is TODO.\n");
    out.push_str(&format!(
        "entry_points = [{}]\n",
        config
            .entry_points
            .iter()
            .map(|e| quoted(e))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    for (table, patterns) in [
        ("security.include", &config.security),
        ("security.exclude", &config.not_security),
    ] {
        for pattern in patterns {
            out.push_str(&format!(
                "\n[[{table}]]\npattern = {}\nreason = \"TODO\"\n",
                quoted(pattern)
            ));
        }
    }
    if !config.dependency_sources.is_empty() {
        out.push_str("\n[dependency_sources]\n");
        for (key, path) in &config.dependency_sources {
            out.push_str(&format!(
                "{} = {}\n",
                quoted(key),
                quoted(&path.to_string_lossy())
            ));
        }
    }
    Ok(out)
}
