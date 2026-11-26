mod ratio;
pub use ratio::{proof_ratio, ProofDepth, ProofRatio, ProofRatioConfig, ProofRatioRange};
mod ghost;
mod hax;
pub use ghost::GhostLines;
mod report;
pub use report::{
    gate, gate_first_party_security, gate_override_shift, gate_with_schema_change, markdown,
    summarize, summarize_default_rules, Summary,
};
mod cfg;
mod contract;
mod evidence;
mod fstar;
pub use fstar::{apply_fstar_reuse, reuse};
mod fstar_proof;
mod kani;
pub use fstar::{apply_fstar_evidence, FstarModule, FstarReceipt};
mod modules;
pub use evidence::{
    apply_evidence, apply_evidence_many, apply_upstream_registry, sha256, Claim, Escape,
    EvidenceRun, Receipt, STATUSES,
};
mod classify;
mod dependencies;
mod inputs;
mod ship;
mod size;
use anyhow::{Context, Result};
pub use classify::{analyze, analyze_target, analyze_with, Config, Dependency, Options, Override};
pub use inputs::{migrate_config, todo_reason, DependencyRule, InputRecord, Rule, Ship};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use serde::{Deserialize, Serialize};
pub use ship::NotShipped;
pub use size::{dwarf_lines, DwarfLines, SizeFallback, SizeMode};
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
    /// Structural test code: excluded from runtime totals, reported separately.
    Test,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TestEvidence {
    BuiltinAttribute,
    CfgTest,
    TestTarget,
    #[default]
    None,
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
    #[serde(default)]
    pub runtime_sloc: usize,
    #[serde(default)]
    pub inline_proof_sloc: usize,
    #[serde(default)]
    pub ghost_sloc: usize,
    #[serde(default)]
    pub fstar_annotation_sloc: usize,
    /// Extraction-only unit, scoped to a first-party hax crate.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fstar_cfg_hax: bool,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub fstar_cfg_hax_inline_sloc: usize,
    #[serde(skip)]
    pub hax_crate: bool,
    #[serde(skip)]
    pub cfg_hax_scope: bool,
    #[serde(skip)]
    pub cfg_hax_was_runtime: bool,
    pub first_party: bool,
    pub security: bool,
    pub status: String,
    #[serde(default)]
    pub category: Category,
    pub verus: bool,
    pub proof_scope: Option<String>,
    /// Security under default rules, present only when an explicit input moved this unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_security: Option<bool>,
    #[serde(skip)]
    pub tokens: String,
    #[serde(skip)]
    pub source: String,
    #[serde(skip)]
    pub scope_attributes: String,
    #[serde(skip)]
    pub scope_sites: Vec<(usize, String)>,
    #[serde(skip)]
    pub inherent_impl: Option<(usize, usize, usize, usize, String)>,
    #[serde(skip)]
    pub kani_harness: bool,
    #[serde(skip)]
    pub kani_only: bool,
    #[serde(default)]
    pub test_evidence: TestEvidence,
    #[serde(skip)]
    unrecognized_test_attribute: bool,
}
fn is_zero(n: &usize) -> bool {
    *n == 0
}

/// Schema 4 adds F* proof cost and moves hax annotations out of runtime in separate mode.
/// Configuration schemas remain 2/3. Cross-schema coverage comparisons need acknowledgement.
pub const REPORT_SCHEMA: u32 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ship: Option<inputs::Ship>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_shipped: Vec<ship::NotShipped>,
    pub schema: u32,
    #[serde(default)]
    pub ghost_lines: GhostLines,
    #[serde(default)]
    pub proof_ratio_range: ProofRatioRange,
    #[serde(default)]
    pub no_proof_ratio_gate: bool,
    #[serde(default)]
    pub unparseable_files: Vec<String>,
    #[serde(default)]
    pub unknown_hax_paths: Vec<String>,
    #[serde(default)]
    pub size_mode: String,
    #[serde(default)]
    pub size_profile: String,
    #[serde(default)]
    pub binary_sha256: Option<String>,
    #[serde(default)]
    pub size_fallbacks: Vec<SizeFallback>,
    pub approximation: String,
    pub units: Vec<Unit>,
    #[serde(default)]
    pub source_files: Vec<String>,
    pub security_relevant: usize,
    pub everything_else: usize,
    pub overrides: Vec<Override>,
    /// Config schema: 2 (bare globs), 3 (explicit inputs with reasons), 0 (no config file).
    #[serde(default)]
    pub config_schema: u32,
    /// Every explicit input with its reason and the SLoC it moved.
    #[serde(default)]
    pub inputs: Vec<InputRecord>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fstar_definitions: Vec<fstar::FstarDefinition>,
    #[serde(default)]
    pub fstar_proof: fstar_proof::ProofState,
    #[serde(default, skip_serializing_if = "fstar::ExtractionConfig::is_empty")]
    pub fstar_extraction: fstar::ExtractionConfig,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub non_fstar_trust: Vec<evidence::TrustedItem>,
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
    src_path: PathBuf,
    kind: Vec<String>,
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
    let mut unparseable_files = Vec::new();
    let mut source_files = std::collections::BTreeSet::new();
    for package in metadata
        .packages
        .iter()
        .filter(|p| metadata.workspace_members.contains(&p.id))
    {
        let dir = package.manifest_path.parent().unwrap();
        let mut files = Vec::new();
        collect_rs(dir, &mut files)?;
        // Integration tests and benches are compiled with --test, so cfg(test) holds throughout.
        let test_targets: Vec<_> = package
            .targets
            .iter()
            .filter(|t| t.kind.iter().any(|k| k == "test" || k == "bench"))
            .map(|t| t.src_path.clone())
            .collect();
        let first_unit = units.len();
        let mut file_modules = Vec::new();
        let mut test_modules = Vec::new();
        let mut hax_crate = false;
        // Include all source files conservatively; test/build files are identified by their module paths.
        for path in files {
            let source = fs::read_to_string(&path)?;
            hax_crate |= source.contains("hax_lib");
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
            let Ok(stream) = source.parse::<TokenStream>() else {
                unparseable_files.push(relative.clone());
                units.push(Unit {
                    id: format!(
                        "{}::{module}::<unparseable>",
                        package.name.replace('-', "_")
                    ),
                    crate_name: package.name.replace('-', "_"),
                    module,
                    name: "<unparseable>".into(),
                    file: relative,
                    line: 1,
                    end_line: source_lines.len(),
                    sloc: sloc(&source),
                    runtime_sloc: sloc(&source),
                    inline_proof_sloc: 0,
                    ghost_sloc: 0,
                    fstar_annotation_sloc: 0,
                    fstar_cfg_hax: false,
                    fstar_cfg_hax_inline_sloc: 0,
                    hax_crate: false,
                    cfg_hax_scope: false,
                    cfg_hax_was_runtime: false,
                    first_party: true,
                    security: false,
                    status: "hazard".into(),
                    category: Category::Runtime,
                    verus: false,
                    proof_scope: None,
                    default_security: None,
                    tokens: String::new(),
                    source,
                    scope_attributes: String::new(),
                    scope_sites: vec![],
                    inherent_impl: None,
                    kani_harness: false,
                    kani_only: false,
                    test_evidence: TestEvidence::None,
                    unrecognized_test_attribute: false,
                });
                continue;
            };
            module_decls(stream.clone(), &module, "", &mut test_modules);
            file_modules.push((
                relative.clone(),
                module.clone(),
                local.starts_with("tests/")
                    || local.starts_with("benches/")
                    || test_targets.contains(&path),
            ));
            let file_start = units.len();
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
            inventory_impls(&source, &mut units[file_start..]);
        }
        for u in &mut units[first_unit..] {
            u.hax_crate = hax_crate;
            if u.name == "<unparseable>" {
                continue;
            }
            let (_, module, test_target) = file_modules
                .iter()
                .find(|(file, _, _)| *file == u.file)
                .expect("unit file inventoried");
            let declared_test = test_modules.iter().any(|(declared, attrs)| {
                (module == declared || module.starts_with(&format!("{declared}::")))
                    && cfg_implies_test(attrs)
            });
            if u.test_evidence == TestEvidence::None {
                if *test_target {
                    u.test_evidence = TestEvidence::TestTarget;
                } else if declared_test {
                    u.test_evidence = TestEvidence::CfgTest;
                }
            }
            for (_, attrs) in test_modules.iter().filter(|(declared, _)| {
                module == declared || module.starts_with(&format!("{declared}::"))
            }) {
                u.cfg_hax_scope |= cfg_implies_hax(attrs);
            }
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
    Ok(Report { ship: None, not_shipped: vec![], proof_ratio_range: ProofRatioRange::default(), no_proof_ratio_gate: false, unparseable_files, unknown_hax_paths: vec![], ghost_lines: GhostLines::default(), schema: REPORT_SCHEMA, size_mode: String::new(), size_profile: String::new(), binary_sha256: None, size_fallbacks: vec![], approximation: "Syntactic token inventory; includes cfg-disabled functions and Verus ghost/spec code. Security classification pending.".into(), units, source_files: source_files.into_iter().collect(), security_relevant: 0, everything_else: 0, overrides: vec![], config_schema: 0, inputs: vec![], dependencies: vec![], build_dependencies: vec![], dev_dependencies_excluded: 0, closure: String::new(), closure_reason: None, target: String::new(), cargo_version: String::new(), declared_external_dependencies, dependency_inventory_complete, warnings: vec![], evidence_runs: vec![], fstar_definitions: vec![], fstar_proof: fstar_proof::ProofState::default(), fstar_extraction: fstar::ExtractionConfig::default(), non_fstar_trust: vec![], escape_hatches: vec![], trusted_items: vec![], upstream_claims: vec![] })
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
/// Out-of-line `mod name;` declarations with the attributes in scope at each one.
fn module_decls(
    stream: TokenStream,
    module: &str,
    inherited: &str,
    out: &mut Vec<(String, String)>,
) {
    let inherited = format!("{inherited} {}", cfg::attributes(stream.clone(), true));
    let tokens: Vec<_> = stream.into_iter().collect();
    let child = |name: &TokenTree| {
        if module.is_empty() {
            name.to_string()
        } else {
            format!("{module}::{name}")
        }
    };
    let mut start = 0;
    for i in 0..tokens.len() {
        let prefix = &tokens[start..i];
        let attrs = || {
            format!(
                "{inherited} {}",
                cfg::attributes(prefix.iter().cloned().collect(), false)
            )
        };
        match &tokens[i] {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                let declared = prefix
                    .iter()
                    .position(|t| t.to_string() == "mod")
                    .and_then(|n| prefix.get(n + 1));
                if let Some(name) = declared {
                    module_decls(g.stream(), &child(name), &attrs(), out);
                } else if prefix.last().is_some_and(|t| t.to_string() == "!")
                    || prefix
                        .get(prefix.len().wrapping_sub(2))
                        .is_some_and(|t| t.to_string() == "!")
                {
                    // Item macros such as verus! keep the enclosing module.
                    module_decls(g.stream(), module, &attrs(), out);
                }
                start = i + 1;
            }
            t if t.to_string() == ";" => {
                if let [.., keyword, name] = prefix {
                    if keyword.to_string() == "mod" {
                        out.push((child(name), attrs()));
                    }
                }
                start = i + 1;
            }
            _ => {}
        }
    }
}

fn test_attribute(attrs: TokenStream) -> (TestEvidence, bool) {
    let mut unrecognized = false;
    for t in attrs {
        let TokenTree::Group(g) = t else { continue };
        let path: Vec<String> = g
            .stream()
            .into_iter()
            .take_while(|t| matches!(t, TokenTree::Ident(_)) || t.to_string() == ":")
            .filter_map(|t| match t {
                TokenTree::Ident(i) => Some(i.to_string()),
                _ => None,
            })
            .collect();
        let path: Vec<_> = path.iter().map(String::as_str).collect();
        match path.as_slice() {
            ["test" | "bench"]
            | ["std" | "core", "prelude", "v1" | "rust_2015" | "rust_2018" | "rust_2021" | "rust_2024", "test"] => {
                return (TestEvidence::BuiltinAttribute, false)
            }
            [.., "test"] => unrecognized = true,
            _ => {}
        }
    }
    (TestEvidence::None, unrecognized)
}

/// True when some `#[cfg(...)]` in `attrs` can only hold under `cfg(test)`.
fn cfg_implies_test(attrs: &str) -> bool {
    cfg_implies(attrs, "test")
}
pub(crate) fn cfg_implies_hax(attrs: &str) -> bool {
    cfg_implies(attrs, "hax")
}
fn cfg_implies(attrs: &str, atom: &str) -> bool {
    fn implies(stream: TokenStream, atom: &str) -> bool {
        let tokens: Vec<_> = stream.into_iter().collect();
        match tokens.as_slice() {
            [TokenTree::Ident(name)] => name == atom,
            [TokenTree::Ident(op), TokenTree::Group(g)]
                if g.delimiter() == Delimiter::Parenthesis =>
            {
                let mut args = vec![TokenStream::new()];
                for t in g.stream() {
                    if t.to_string() == "," {
                        args.push(TokenStream::new());
                    } else {
                        args.last_mut().unwrap().extend([t]);
                    }
                }
                args.retain(|a| !a.is_empty());
                match op.to_string().as_str() {
                    "all" => args.into_iter().any(|a| implies(a, atom)),
                    "any" => !args.is_empty() && args.into_iter().all(|a| implies(a, atom)),
                    _ => false,
                }
            }
            _ => false,
        }
    }
    let Ok(stream) = attrs.parse::<TokenStream>() else {
        return false;
    };
    cfg::attributes(stream, false).into_iter().any(|t| {
        let TokenTree::Group(g) = t else {
            return false;
        };
        let tokens: Vec<_> = g.stream().into_iter().collect();
        matches!(tokens.as_slice(), [TokenTree::Ident(cfg), TokenTree::Group(p)]
            if cfg == "cfg" && p.delimiter() == Delimiter::Parenthesis && implies(p.stream(), atom))
    })
}

/// Preserve exact block identity only when Rust syntax can be parsed reliably.
fn inventory_impls(source: &str, units: &mut [Unit]) {
    use syn::spanned::Spanned;
    fn visit(items: &[syn::Item], units: &mut [Unit]) {
        for item in items {
            match item {
                syn::Item::Mod(m) => {
                    if let Some((_, items)) = &m.content {
                        visit(items, units);
                    }
                }
                syn::Item::Impl(block) if block.trait_.is_none() => {
                    let syn::Type::Path(ty) = &*block.self_ty else {
                        continue;
                    };
                    // Qualified/generic self types need additional hax naming rules.
                    if ty.qself.is_some() || ty.path.segments.len() != 1 {
                        continue;
                    }
                    let ty = ty.path.segments[0].ident.to_string();
                    let span = block.span();
                    for item in &block.items {
                        let syn::ImplItem::Fn(method) = item else {
                            continue;
                        };
                        let line = method.sig.fn_token.span.start().line;
                        for unit in units.iter_mut().filter(|u| {
                            method.sig.ident == u.name && u.line <= line && line <= u.end_line
                        }) {
                            unit.inherent_impl = Some((
                                span.start().line,
                                span.start().column,
                                span.end().line,
                                span.end().column,
                                ty.clone(),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if let Ok(file) = syn::parse_file(source) {
        visit(&file.items, units);
    }
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
                    let attrs = cfg::attributes(tokens[start..i].iter().cloned().collect(), false);
                    let own_attributes = attrs.to_string();
                    let scope_attributes = format!("{} {}", inherited_attrs, own_attributes);
                    let (mut test_evidence, unrecognized_test_attribute) = test_attribute(attrs);
                    if test_evidence == TestEvidence::None && cfg_implies_test(&scope_attributes) {
                        test_evidence = TestEvidence::CfgTest;
                    }
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
                        runtime_sloc: sloc(&snippet),
                        inline_proof_sloc: 0,
                        ghost_sloc: if verus { ghost::inline_sloc(text.clone(), source_lines) } else { 0 },
                        fstar_annotation_sloc: 0,
                        fstar_cfg_hax: false,
                        fstar_cfg_hax_inline_sloc: 0,
                        hax_crate: false,
                        cfg_hax_scope: false,
                        cfg_hax_was_runtime: false,
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
                        default_security: None,
                        tokens: text.to_string(),
                        source: snippet,
                        scope_sites: inherited_sites.iter().cloned().chain(evidence::attribute_sites(cfg::attributes(tokens[start..i].iter().cloned().collect(),false))).collect(),
                        inherent_impl: None,
                        scope_attributes,
                        kani_harness,
                        kani_only,
                        test_evidence,
                        unrecognized_test_attribute,
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
    sloc_lines(source).len()
}

/// SLoC of one dependency source file without spec/proof functions (the first-party
/// `Category::ProofCode` rule). Unparseable files keep every nonblank line.
pub(crate) fn runtime_sloc(source: &str, file: &str) -> usize {
    let lines = sloc_lines(source);
    let Ok(stream) = source.parse::<TokenStream>() else {
        return lines.len();
    };
    let source_lines: Vec<_> = source.lines().collect();
    let mut units = Vec::new();
    walk(
        stream,
        "",
        SourceContext {
            krate: "dependency",
            file,
            source_lines: &source_lines,
        },
        false,
        "",
        &[],
        &mut units,
    );
    let proof: Vec<_> = units
        .iter()
        .filter(|u| u.category == Category::ProofCode)
        .map(|u| u.line..=u.end_line)
        .collect();
    lines
        .iter()
        .filter(|line| !proof.iter().any(|span| span.contains(line)))
        .count()
}

/// Skip both sugared doc comments and explicit doc attributes at this token level.
/// Other attributes and literal contents keep their original spans and ownership.
pub(crate) fn non_doc_tokens(stream: TokenStream) -> impl Iterator<Item = TokenTree> {
    let mut tokens = stream.into_iter();
    std::iter::from_fn(move || loop {
        let token = tokens.next()?;
        if matches!(&token, TokenTree::Punct(p) if p.as_char() == '#') {
            let mut lookahead = tokens.clone();
            let mut next = lookahead.next();
            if matches!(&next, Some(TokenTree::Punct(p)) if p.as_char() == '!') {
                next = lookahead.next();
            }
            if matches!(next, Some(TokenTree::Group(g))
                if g.delimiter() == Delimiter::Bracket
                    && matches!(g.stream().into_iter().next(), Some(TokenTree::Ident(i)) if i == "doc"))
            {
                tokens = lookahead;
                continue;
            }
        }
        return Some(token);
    })
}

fn sloc_lines(source: &str) -> std::collections::BTreeSet<usize> {
    fn visit(stream: TokenStream, lines: &mut std::collections::BTreeSet<usize>) {
        for token in non_doc_tokens(stream) {
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
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty())
            .map(|(i, _)| i + 1)
            .collect();
    };
    let mut lines = std::collections::BTreeSet::new();
    visit(stream, &mut lines);
    source
        .lines()
        .enumerate()
        .filter(|(i, line)| lines.contains(&(i + 1)) && !line.trim().is_empty())
        .map(|(i, _)| i + 1)
        .collect()
}

#[cfg(test)]
mod cfg_implication_tests {
    #[test]
    fn shared_atom_algebra() {
        for atom in ["test", "hax"] {
            for (condition, expected) in [
                (atom.to_string(), true),
                (format!("all({atom}, unix)"), true),
                (format!("any({atom}, all({atom}, windows))"), true),
                (format!("any({atom}, unix)"), false),
                (format!("not({atom})"), false),
                (format!("feature = \"{atom}\""), false),
                (format!("{atom}_pv"), false),
                ("any()".into(), false),
                ("all()".into(), false),
            ] {
                assert_eq!(
                    super::cfg_implies(&format!("#[cfg({condition})]"), atom),
                    expected,
                    "{condition}"
                );
            }
            assert!(!super::cfg_implies(
                &format!("#[cfg_attr({atom}, inline)]"),
                atom
            ));
            assert!(super::cfg_implies(
                &format!("#[cfg(unix)] #[cfg({atom})]"),
                atom
            ));
        }
    }
}
