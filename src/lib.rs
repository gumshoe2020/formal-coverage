mod report;
pub use report::{
    gate, gate_first_party_security, gate_with_schema_change, markdown, summarize, Summary,
};
mod cfg;
mod evidence;
mod kani;
mod modules;
pub use evidence::{
    apply_evidence, apply_evidence_many, apply_upstream_registry, sha256, Claim, Escape,
    EvidenceRun, Receipt, STATUSES,
};
mod classify;
mod dependencies;
use anyhow::{Context, Result};
pub use classify::{analyze, analyze_target, Config, Dependency, Override};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    #[default]
    Runtime,
    ProofCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: String,
    pub crate_name: String,
    pub module: String,
    pub name: String,
    pub file: String,
    pub line: usize,
    pub end_line: usize,
    pub sloc: usize,
    pub first_party: bool,
    pub security: bool,
    pub status: String,
    #[serde(default)]
    pub category: Category,
    pub verus: bool,
    pub proof_scope: Option<String>,
    #[serde(skip)]
    pub tokens: String,
    #[serde(skip)]
    pub source: String,
    #[serde(skip)]
    pub scope_attributes: String,
    #[serde(skip)]
    pub scope_sites: Vec<(usize, String)>,
    #[serde(skip)]
    pub kani_harness: bool,
    #[serde(skip)]
    pub kani_only: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub schema: u32,
    pub approximation: String,
    pub units: Vec<Unit>,
    #[serde(default)]
    pub source_files: Vec<String>,
    pub security_relevant: usize,
    pub everything_else: usize,
    pub overrides: Vec<Override>,
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub build_dependencies: Vec<Dependency>,
    #[serde(default)]
    pub dev_dependencies_excluded: usize,
    #[serde(default)]
    pub closure: String,
    #[serde(default)]
    pub closure_reason: Option<String>,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub cargo_version: String,
    pub declared_external_dependencies: Vec<String>,
    pub dependency_inventory_complete: bool,
    pub warnings: Vec<String>,
    #[serde(default)]
    pub evidence_runs: Vec<EvidenceRun>,
    pub escape_hatches: Vec<Escape>,
    #[serde(default)]
    pub trusted_items: Vec<evidence::TrustedItem>,
    pub upstream_claims: Vec<Claim>,
}
#[derive(Deserialize)]
struct Metadata {
    packages: Vec<Package>,
    workspace_members: Vec<String>,
}
#[derive(Deserialize)]
struct Package {
    id: String,
    name: String,
    manifest_path: PathBuf,
    targets: Vec<Target>,
    dependencies: Vec<ManifestDependency>,
}
#[derive(Deserialize)]
struct ManifestDependency {
    name: String,
    source: Option<String>,
    path: Option<PathBuf>,
}

#[derive(Deserialize)]
struct Target {
    #[serde(rename = "src_path")]
    _src_path: PathBuf,
}

pub fn inventory(root: &Path) -> Result<Report> {
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
    anyhow::ensure!(
        output.status.success(),
        "cargo metadata: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: Metadata = serde_json::from_slice(&output.stdout)?;
    let root = root.canonicalize()?;
    let mut units = Vec::new();
    let mut source_files = std::collections::BTreeSet::new();
    for package in metadata
        .packages
        .iter()
        .filter(|p| metadata.workspace_members.contains(&p.id))
    {
        let dir = package.manifest_path.parent().unwrap();
        let mut files = Vec::new();
        collect_rs(dir, &mut files)?;
        // Include all source files conservatively; test/build files are identified by their module paths.
        for path in files {
            let source = fs::read_to_string(&path)?;
            let stream: TokenStream = source
                .parse()
                .map_err(|e| anyhow::anyhow!("{}: {e}", path.display()))?;
            let relative = path
                .strip_prefix(&root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            source_files.insert(relative.clone());
            let local = path
                .strip_prefix(dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            let module = module_path(&local, &package.targets);
            let source_lines: Vec<_> = source.lines().collect();
            walk(
                stream,
                &module,
                SourceContext {
                    krate: &package.name.replace('-', "_"),
                    file: &relative,
                    source_lines: &source_lines,
                },
                false,
                "",
                &[],
                &mut units,
            );
        }
    }
    let workspace_dirs: Vec<PathBuf> = metadata
        .packages
        .iter()
        .filter(|p| metadata.workspace_members.contains(&p.id))
        .filter_map(|p| p.manifest_path.parent())
        .map(|p| p.canonicalize().unwrap_or_else(|_| p.to_path_buf()))
        .collect();
    let declared_external_dependencies: Vec<String> = metadata
        .packages
        .iter()
        .filter(|p| metadata.workspace_members.contains(&p.id))
        .flat_map(|p| p.dependencies.iter())
        .filter(|d| {
            d.source.is_some()
                || !d.path.as_ref().is_some_and(|p| {
                    workspace_dirs.contains(&p.canonicalize().unwrap_or_else(|_| p.clone()))
                })
        })
        .map(|d| d.name.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let dependency_inventory_complete = declared_external_dependencies.is_empty();
    units.sort_by(|a, b| (&a.file, a.line, &a.id).cmp(&(&b.file, b.line, &b.id)));
    Ok(Report { schema: 2, approximation: "Syntactic token inventory; includes cfg-disabled functions and Verus ghost/spec code. Security classification pending.".into(), units, source_files: source_files.into_iter().collect(), security_relevant: 0, everything_else: 0, overrides: vec![], dependencies: vec![], build_dependencies: vec![], dev_dependencies_excluded: 0, closure: String::new(), closure_reason: None, target: String::new(), cargo_version: String::new(), declared_external_dependencies, dependency_inventory_complete, warnings: vec![], evidence_runs: vec![], escape_hatches: vec![], trusted_items: vec![], upstream_claims: vec![] })
}
fn module_path(local: &str, targets: &[Target]) -> String {
    let _ = targets;
    let s = local
        .strip_prefix("src/")
        .unwrap_or(local)
        .trim_end_matches(".rs");
    s.split('/')
        .filter(|x| !["lib", "main", "mod"].contains(x))
        .collect::<Vec<_>>()
        .join("::")
}
fn collect_rs(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_symlink() {
            let excluded = ["target", ".git", ".tools", ".cargo-home", "logs", "result"]
                .contains(&entry.file_name().to_string_lossy().as_ref());
            anyhow::ensure!(
                excluded || (!path.is_dir() && !path.extension().is_some_and(|x| x == "rs")),
                "source symlink cannot be inventoried safely: {}",
                path.display()
            );
            continue;
        }
        if path.is_dir() {
            if !["target", ".git", ".tools", ".cargo-home", "logs"]
                .contains(&entry.file_name().to_string_lossy().as_ref())
            {
                // Nested Cargo packages are inventoried independently.
                if !path.join("Cargo.toml").exists() {
                    collect_rs(&path, files)?;
                }
            }
        } else if path.extension().is_some_and(|x| x == "rs") {
            files.push(path);
        }
    }
    Ok(())
}
#[derive(Clone, Copy)]
struct SourceContext<'a> {
    krate: &'a str,
    file: &'a str,
    source_lines: &'a [&'a str],
}
fn walk(
    stream: TokenStream,
    module: &str,
    context: SourceContext<'_>,
    verus: bool,
    inherited_attrs: &str,
    inherited_sites: &[(usize, String)],
    units: &mut Vec<Unit>,
) {
    let SourceContext {
        krate,
        file,
        source_lines,
    } = context;
    let inner_tokens = cfg::attributes(stream.clone(), true);
    let mut inherited_sites = inherited_sites.to_vec();
    inherited_sites.extend(evidence::attribute_sites(inner_tokens.clone()));
    let inner = inner_tokens.to_string();
    let inherited_attrs = format!("{inherited_attrs} {inner}");
    let tokens: Vec<_> = stream.into_iter().collect();
    let mut i = 0;
    let mut start = 0;
    while i < tokens.len() {
        if tokens[i].to_string() == "fn" {
            if let Some(TokenTree::Ident(name)) = tokens.get(i + 1) {
                let mut end = i + 2;
                while end < tokens.len() {
                    if tokens[end].to_string() == ";" {
                        break;
                    }
                    if matches!(&tokens[end], TokenTree::Group(g) if g.delimiter() == Delimiter::Brace)
                    {
                        let next = tokens.get(end + 1).map(ToString::to_string);
                        let continuation = next.as_deref().is_some_and(|n| {
                            [
                                ",",
                                "else",
                                "decreases",
                                "ensures",
                                "requires",
                                "recommends",
                                "&",
                                "|",
                                "=",
                                ">",
                                "<",
                                "+",
                                "-",
                                "*",
                                "/",
                                ".",
                            ]
                            .contains(&n)
                        }) || matches!(tokens.get(end+1), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace);
                        if !continuation {
                            break;
                        }
                    }
                    end += 1;
                }
                if let Some(last_token) = tokens.get(end) {
                    let line = tokens[start].span().start().line;
                    let last = last_token.span().end().line;
                    let snippet = source_lines
                        .get(line - 1..)
                        .unwrap_or_default()
                        .iter()
                        .take(last - line + 1)
                        .copied()
                        .collect::<Vec<_>>()
                        .join("\n");
                    let text: TokenStream = tokens[start..=end].iter().cloned().collect();
                    let name = name.to_string();
                    let own_attributes =
                        cfg::attributes(tokens[start..i].iter().cloned().collect(), false)
                            .to_string();
                    let scope_attributes = format!("{} {}", inherited_attrs, own_attributes);
                    let kani_harness = kani::is_harness(&own_attributes);
                    let kani_only = cfg::has_attribute(&scope_attributes, "cfg (kani)")
                        || (!kani_harness && kani::is_harness(&scope_attributes));
                    units.push(Unit {
                        id: [krate, module, &name]
                            .into_iter()
                            .filter(|x| !x.is_empty())
                            .collect::<Vec<_>>()
                            .join("::"),
                        crate_name: krate.into(),
                        module: module.into(),
                        name: name.clone(),
                        file: file.into(),
                        line,
                        end_line: last,
                        sloc: sloc(&snippet),
                        first_party: true,
                        security: false,
                        status: "unverified".into(),
                        category: if kani_only || kani_harness || (i > 0 && matches!(&tokens[i - 1], TokenTree::Ident(mode) if mode == "spec" || mode == "proof")) {
                            Category::ProofCode
                        } else {
                            Category::Runtime
                        },
                        verus,
                        proof_scope: None,
                        tokens: text.to_string(),
                        source: snippet,
                        scope_sites: inherited_sites.iter().cloned().chain(evidence::attribute_sites(cfg::attributes(tokens[start..i].iter().cloned().collect(),false))).collect(),
                        scope_attributes,
                        kani_harness,
                        kani_only,
                    });
                    // Local functions also count as units; trait declarations have no body.
                    if let TokenTree::Group(body) = last_token {
                        walk(
                            body.stream(),
                            &format!("{module}::{name}"),
                            context,
                            verus,
                            &format!(
                                "{} {}",
                                inherited_attrs,
                                cfg::attributes(tokens[start..i].iter().cloned().collect(), false)
                            ),
                            &inherited_sites
                                .iter()
                                .cloned()
                                .chain(evidence::attribute_sites(cfg::attributes(
                                    tokens[start..i].iter().cloned().collect(),
                                    false,
                                )))
                                .collect::<Vec<_>>(),
                            units,
                        );
                    }
                    i = end + 1;
                    start = i;
                    continue;
                }
            }
        }
        if let TokenTree::Group(g) = &tokens[i] {
            if g.delimiter() == Delimiter::Brace {
                let prefix = &tokens[start..i];
                let child = prefix
                    .iter()
                    .position(|t| t.to_string() == "mod")
                    .and_then(|n| prefix.get(n + 1))
                    .map(ToString::to_string);
                let impl_name = prefix
                    .iter()
                    .position(|t| t.to_string() == "impl")
                    .map(|n| {
                        prefix[n + 1..]
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join("")
                    });
                let trait_name = prefix
                    .iter()
                    .position(|t| t.to_string() == "trait")
                    .and_then(|n| prefix.get(n + 1))
                    .map(ToString::to_string);
                let extra = child.or(impl_name).or(trait_name);
                let next = extra
                    .map(|x| {
                        if module.is_empty() {
                            x
                        } else {
                            format!("{module}::{x}")
                        }
                    })
                    .unwrap_or_else(|| module.into());
                walk(
                    g.stream(),
                    &next,
                    context,
                    verus || prefix.iter().any(|t| t.to_string() == "verus"),
                    &format!(
                        "{} {}",
                        inherited_attrs,
                        cfg::attributes(prefix.iter().cloned().collect(), false)
                    ),
                    &inherited_sites
                        .iter()
                        .cloned()
                        .chain(evidence::attribute_sites(cfg::attributes(
                            prefix.iter().cloned().collect(),
                            false,
                        )))
                        .collect::<Vec<_>>(),
                    units,
                );
                start = i + 1;
            }
        }
        if tokens[i].to_string() == ";" {
            start = i + 1;
        }
        i += 1;
    }
}
/// Counts nonblank lines touched by non-comment Rust tokens (including multiline literals).
pub fn sloc(source: &str) -> usize {
    fn visit(stream: TokenStream, lines: &mut std::collections::BTreeSet<usize>) {
        for token in stream {
            if let TokenTree::Group(group) = token {
                lines.insert(group.span_open().start().line);
                lines.insert(group.span_close().end().line);
                visit(group.stream(), lines);
            } else {
                lines.extend(token.span().start().line..=token.span().end().line);
            }
        }
    }
    let Ok(stream) = source.parse::<TokenStream>() else {
        // Malformed dependency files cannot safely be lexed; count all nonblank lines,
        // conservatively retaining comments instead of understating unverified code size.
        return source
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count();
    };
    let mut lines = std::collections::BTreeSet::new();
    visit(stream, &mut lines);
    source
        .lines()
        .enumerate()
        .filter(|(i, line)| lines.contains(&(i + 1)) && !line.trim().is_empty())
        .count()
}
