//! Caller-supplied hax/F* evidence. Unsupported scope never earns credit.
use crate::{sha256, EvidenceRun, Report};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FstarReceipt {
    pub tool: String,
    pub hax_version: String,
    pub hax_commit: String,
    pub fstar_version: String,
    pub extraction_command: Vec<String>,
    pub extraction_exit_code: i32,
    pub files: BTreeMap<String, String>,
    /// Relative to the log directory, never resolved against the Rust tree.
    pub fresh: BTreeMap<String, String>,
    pub checked: BTreeMap<String, String>,
    pub patches: BTreeMap<String, String>,
    pub log_sha256: String,
    pub modules: Vec<FstarModule>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FstarModule {
    pub file: String,
    pub command: Vec<String>,
    pub exit_code: i32,
    pub outcome: String,
}

fn inside(base: &Path, name: &str) -> Result<PathBuf> {
    ensure!(!Path::new(name).is_absolute(), "absolute receipt path");
    let path = base.join(name).canonicalize()?;
    ensure!(
        path.starts_with(base.canonicalize()?),
        "receipt path outside root"
    );
    Ok(path)
}
fn hashes(base: &Path, files: &BTreeMap<String, String>) -> Result<()> {
    for (name, hash) in files {
        ensure!(
            sha256(&fs::read(inside(base, name)?)?) == *hash,
            "stale hash: {name}"
        );
    }
    Ok(())
}
/// One explicit F* input; includes must be receipt-hashed directories. No unknown
/// switches, lax/admission, response files, code generation or cached proofs.
fn command(args: &[String], input: &str, checked: &BTreeMap<String, String>) -> bool {
    if args.first().map(String::as_str) != Some("fstar.exe") {
        return false;
    }
    let mut i = 1;
    let mut seen = std::collections::BTreeSet::new();
    let mut source = false;
    while i < args.len() {
        match args[i].as_str() {
            "--include" => {
                let Some(dir) = args.get(i + 1) else {
                    return false;
                };
                if !checked
                    .keys()
                    .any(|p| Path::new(p).parent() == Some(Path::new(dir)))
                    || !seen.insert(dir)
                {
                    return false;
                }
                i += 2;
            }
            "--cache_checked_modules" | "--use_hints" | "--record_hints" => return false,
            arg if arg == input && !source && checked.contains_key(arg) => {
                source = true;
                i += 1;
            }
            _ => return false,
        }
    }
    source
}
fn validate(root: &Path, log_path: &Path, report: &Report) -> Result<FstarReceipt> {
    let log = fs::read(log_path)?;
    let receipt: FstarReceipt =
        serde_json::from_slice(&fs::read(format!("{}.evidence.json", log_path.display()))?)?;
    ensure!(receipt.tool == "hax-fstar", "tool mismatch");
    ensure!(
        !receipt.hax_version.trim().is_empty() && !receipt.fstar_version.trim().is_empty(),
        "missing tool version"
    );
    ensure!(
        receipt.hax_commit.len() == 40 && receipt.hax_commit.bytes().all(|c| c.is_ascii_hexdigit()),
        "missing full hax commit"
    );
    ensure!(receipt.log_sha256 == sha256(&log), "log hash mismatch");
    ensure!(
        receipt.extraction_command == ["python3", "hax-driver.py", "extract-fstar"]
            && receipt.extraction_exit_code == 0,
        "unsupported or failed extraction"
    );
    for file in report.source_files.iter().map(String::as_str).chain([
        "Cargo.toml",
        "Cargo.lock",
        "hax-driver.py",
    ]) {
        ensure!(
            receipt.files.contains_key(file),
            "missing source hash: {file}"
        );
    }
    hashes(root, &receipt.files)?;
    let base = log_path.parent().unwrap_or(Path::new("."));
    ensure!(
        !receipt.fresh.is_empty() && !receipt.checked.is_empty() && !receipt.modules.is_empty(),
        "empty extraction/check scope"
    );
    hashes(base, &receipt.fresh)?;
    hashes(base, &receipt.checked)?;
    hashes(base, &receipt.patches)?;
    // Complete sibling source inventory: hidden helpers cannot escape hash binding.
    for name in receipt.fresh.keys().chain(receipt.checked.keys()) {
        ensure!(
            [Some("fst"), Some("fsti")]
                .contains(&Path::new(name).extension().and_then(|s| s.to_str())),
            "non-F* extraction input"
        );
        let dir = inside(base, name)?.parent().unwrap().to_path_buf();
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            ensure!(
                !path.to_string_lossy().ends_with(".checked")
                    && !path.to_string_lossy().ends_with(".checked.lax")
                    && !path.to_string_lossy().ends_with(".hints"),
                "cached F* artifact in receipt scope"
            );
            if [Some("fst"), Some("fsti")].contains(&path.extension().and_then(|s| s.to_str())) {
                ensure!(
                    receipt
                        .fresh
                        .keys()
                        .chain(receipt.checked.keys())
                        .any(|n| inside(base, n).is_ok_and(|p| p == path)),
                    "unhashed F* sibling"
                );
            }
        }
    }
    let mut basenames = std::collections::BTreeSet::new();
    for file in receipt.fresh.keys() {
        ensure!(
            basenames.insert(Path::new(file).file_name()),
            "ambiguous fresh extraction name"
        );
    }
    let mut seen = std::collections::BTreeSet::new();
    for module in &receipt.modules {
        ensure!(seen.insert(&module.file), "duplicate module outcome");
        ensure!(
            command(&module.command, &module.file, &receipt.checked),
            "unsupported F* command"
        );
        ensure!(
            ["success", "failed", "timeout", "skipped"].contains(&module.outcome.as_str()),
            "unknown module outcome"
        );
        ensure!(
            (module.outcome == "success") == (module.exit_code == 0),
            "inconsistent module outcome"
        );
        if module.outcome == "success" {
            let name = Path::new(&module.file)
                .file_stem()
                .unwrap()
                .to_string_lossy();
            ensure!(
                String::from_utf8_lossy(&log)
                    .lines()
                    .any(|l| l.trim() == format!("Verified module: {name}")),
                "missing matching F* success output"
            );
        }
    }
    Ok(receipt)
}

pub fn apply_fstar_evidence(report: &mut Report, root: &Path, logs: &[PathBuf]) -> Result<()> {
    crate::ship::apply_evidence(report, |inventory| {
        apply_fstar_evidence_scoped(inventory, root, logs)
    })
}

fn apply_fstar_evidence_scoped(report: &mut Report, root: &Path, logs: &[PathBuf]) -> Result<()> {
    inventory_rust_trust(report, root)?;
    for log in logs {
        let mut run = EvidenceRun {
            tool: "hax-fstar".into(),
            log: log.display().to_string(),
            accepted: false,
            reasons: vec![],
            credited_units: vec![],
            verified_files: vec![],
            cfg: vec![],
            trusted_sites: vec![],
            escape_sites: vec![],
            trusted_sloc: 0,
        };
        match validate(root, log, report).context("F* receipt rejected") {
            Ok(receipt) => {
                run.accepted = true;
                crate::fstar_proof::accepted(
                    report,
                    log.parent().unwrap_or(Path::new(".")),
                    &receipt,
                )?;
                run.verified_files = receipt.checked.keys().cloned().collect();
                run.reasons.push("Accepted caller-supplied evidence, not an execution attestation; hax extraction is trusted".into());
                apply_validated(report, root, log, &receipt, &mut run)?;
            }
            Err(error) => run.reasons.push(format!("{error:#}")),
        }
        run.trusted_sloc = report
            .trusted_items
            .iter()
            .filter(|t| {
                run.trusted_sites.iter().any(|(f, l, k)| {
                    f == &t.file
                        && l == &t.line
                        && (k == &t.kind || k == "hax extraction (trusted)")
                })
            })
            .map(|t| t.sloc)
            .sum();
        report.evidence_runs.push(run);
    }
    if !logs.is_empty() {
        let credited: std::collections::BTreeSet<_> = report
            .units
            .iter()
            .filter(|u| {
                u.status == "proven-extracted" || (u.fstar_cfg_hax && u.status == "verified")
            })
            .map(|u| u.id.clone())
            .collect();
        for run in report
            .evidence_runs
            .iter_mut()
            .filter(|r| r.tool == "hax-fstar")
        {
            run.credited_units.retain(|id| credited.contains(id));
        }
        report
            .trusted_items
            .sort_by(|a, b| (&a.file, a.line, &a.kind).cmp(&(&b.file, b.line, &b.kind)));
        report
            .trusted_items
            .dedup_by(|a, b| a.file == b.file && a.line == b.line && a.kind == b.kind);
        report
            .escape_hatches
            .sort_by(|a, b| (&a.file, a.line, &a.kind).cmp(&(&b.file, b.line, &b.kind)));
        report
            .escape_hatches
            .dedup_by(|a, b| a.file == b.file && a.line == b.line && a.kind == b.kind);
    }
    crate::fstar_proof::refresh(report, root)?;
    Ok(())
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FstarDefinition {
    pub name: String,
    pub file: String,
    pub line: usize,
    pub rust_unit: Option<String>,
    pub classification: String,
    pub contract_kind: String,
    pub postcondition: String,
    pub self_consistency_heuristic: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping_reason: Option<String>,
}

/// Declared extraction context is caller supplied, not inferred from cfg evaluation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtractionConfig {
    #[serde(default)]
    pub inactive_features: Vec<String>,
    pub reason: Option<String>,
}
impl ExtractionConfig {
    pub(crate) fn is_empty(&self) -> bool {
        self.inactive_features.is_empty() && self.reason.is_none()
    }
    pub(crate) fn validate(&self) -> Result<()> {
        ensure!(
            self.inactive_features.is_empty()
                || self.reason.as_ref().is_some_and(|r| !r.trim().is_empty()),
            "fstar.extraction.inactive_features requires a non-empty reason"
        );
        Ok(())
    }
}

pub(crate) fn trust_identifiers(stream: proc_macro2::TokenStream) -> Vec<(usize, String)> {
    let mut found = vec![];
    for token in stream {
        match token {
            proc_macro2::TokenTree::Ident(id)
                if ["opaque", "replace", "replace_body"].contains(&id.to_string().as_str()) =>
            {
                found.push((id.span().start().line, id.to_string()))
            }
            proc_macro2::TokenTree::Group(g) => found.extend(trust_identifiers(g.stream())),
            _ => {}
        }
    }
    found
}

fn sites(
    stream: proc_macro2::TokenStream,
    config: &ExtractionConfig,
    found: &mut Vec<(usize, String, Option<String>)>,
) {
    use proc_macro2::TokenTree;
    use syn::{parse::Parser, punctuated::Punctuated, Meta, Token};
    // Record exceptions by token position. Anything not positively identified keeps trust.
    fn mark(
        stream: proc_macro2::TokenStream,
        reason: &str,
        exceptions: &mut BTreeMap<(usize, usize), String>,
    ) {
        for token in stream {
            match token {
                TokenTree::Ident(id)
                    if ["opaque", "replace", "replace_body"].contains(&id.to_string().as_str()) =>
                {
                    let pos = id.span().start();
                    exceptions
                        .entry((pos.line, pos.column))
                        .or_insert_with(|| reason.into());
                }
                TokenTree::Group(g) => mark(g.stream(), reason, exceptions),
                _ => {}
            }
        }
    }
    fn inspect_meta(
        meta: &Meta,
        config: &ExtractionConfig,
        exceptions: &mut BTreeMap<(usize, usize), String>,
    ) {
        let path = meta.path();
        let segments: Vec<_> = path.segments.iter().collect();
        if let [.., backend, trust] = segments.as_slice() {
            let backend = backend.ident.to_string();
            if ["proverif", "coq", "ssprove"].contains(&backend.as_str())
                && ["opaque", "replace", "replace_body"].contains(&trust.ident.to_string().as_str())
            {
                let pos = trust.ident.span().start();
                exceptions.insert((pos.line, pos.column), backend);
            }
        }
        let Meta::List(list) = meta else { return };
        if !path.is_ident("cfg_attr") {
            return;
        }
        let Ok(args) = Punctuated::<Meta, Token![,]>::parse_terminated.parse2(list.tokens.clone())
        else {
            return;
        };
        let mut args = args.iter();
        let predicate = args.next();
        let inactive = match predicate {
            Some(Meta::NameValue(nv)) if nv.path.is_ident("feature") => match &nv.value {
                syn::Expr::Lit(lit) => match &lit.lit {
                    syn::Lit::Str(feature)
                        if config.inactive_features.contains(&feature.value()) =>
                    {
                        Some(feature.value())
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        };
        for attr in args {
            inspect_meta(attr, config, exceptions);
            if let Some(feature) = &inactive {
                // The plain predicate scopes all tokens of each enclosed attribute.
                // Preserve original token positions, including nested groups.
                mark_meta(attr, &format!("inactive feature {feature}"), exceptions);
            }
        }
    }
    fn mark_meta(meta: &Meta, reason: &str, exceptions: &mut BTreeMap<(usize, usize), String>) {
        for segment in &meta.path().segments {
            let id = &segment.ident;
            if ["opaque", "replace", "replace_body"].contains(&id.to_string().as_str()) {
                let pos = id.span().start();
                exceptions
                    .entry((pos.line, pos.column))
                    .or_insert_with(|| reason.into());
            }
        }
        if let Meta::List(list) = meta {
            mark(list.tokens.clone(), reason, exceptions);
        }
    }
    fn attrs(
        stream: proc_macro2::TokenStream,
        config: &ExtractionConfig,
        exceptions: &mut BTreeMap<(usize, usize), String>,
    ) {
        let tokens: Vec<_> = stream.into_iter().collect();
        for (i, token) in tokens.iter().enumerate() {
            if token.to_string() == "#" {
                let offset = usize::from(tokens.get(i + 1).is_some_and(|t| t.to_string() == "!"));
                if let Some(TokenTree::Group(g)) = tokens.get(i + 1 + offset) {
                    if g.delimiter() == proc_macro2::Delimiter::Bracket {
                        if let Ok(attr) = syn::parse2::<Meta>(g.stream()) {
                            inspect_meta(&attr, config, exceptions);
                        }
                    }
                }
            }
            if let TokenTree::Group(g) = token {
                attrs(g.stream(), config, exceptions);
            }
        }
    }
    fn collect(
        stream: proc_macro2::TokenStream,
        exceptions: &BTreeMap<(usize, usize), String>,
        found: &mut Vec<(usize, String, Option<String>)>,
    ) {
        for token in stream {
            match token {
                TokenTree::Ident(id)
                    if ["opaque", "replace", "replace_body"].contains(&id.to_string().as_str()) =>
                {
                    let pos = id.span().start();
                    found.push((
                        pos.line,
                        id.to_string(),
                        exceptions.get(&(pos.line, pos.column)).cloned(),
                    ));
                }
                TokenTree::Group(g) => collect(g.stream(), exceptions, found),
                _ => {}
            }
        }
    }
    let mut exceptions = BTreeMap::new();
    attrs(stream.clone(), config, &mut exceptions);
    collect(stream, &exceptions, found);
}

fn inventory_rust_trust(report: &mut Report, root: &Path) -> Result<()> {
    fn model_files(dir: &Path, root: &Path, warnings: &mut Vec<String>) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_symlink() {
                continue;
            }
            if path.is_dir() {
                model_files(&path, root, warnings)?;
            } else if [Some("pv"), Some("pvl"), Some("v")]
                .contains(&path.extension().and_then(|s| s.to_str()))
            {
                warnings.push(format!("Upstream model (uncredited): {} — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof",path.strip_prefix(root)?.display()));
            }
        }
        Ok(())
    }
    model_files(&root.join("proofs/proverif"), root, &mut report.warnings)?;
    model_files(&root.join("proofs/ssprove"), root, &mut report.warnings)?;

    for file in &report.source_files {
        let source = fs::read_to_string(root.join(file))?;
        // Scope only hax-annotated sources; conservative token inventory includes cfg-disabled sites.
        if !["hax_lib", "proverif", "coq", "ssprove"]
            .iter()
            .any(|marker| source.contains(marker))
        {
            continue;
        }
        let mut found = vec![];
        if let Ok(stream) = source.parse() {
            sites(stream, &report.fstar_extraction, &mut found);
        }
        for (line, kind, backend) in found {
            let affected: Vec<_> = report
                .units
                .iter()
                .filter(|u| {
                    u.file == *file
                        && (u.line <= line && line <= u.end_line
                            || u.scope_sites.contains(&(line, kind.clone())))
                })
                .map(|u| u.id.clone())
                .collect();
            let item = crate::evidence::TrustedItem {
                file: file.clone(),
                line,
                kind: backend.as_ref().map_or_else(
                    || format!("hax {kind} (syntactic, includes inactive annotations)"),
                    |backend| format!("hax {kind} (non-F* trust: {backend})"),
                ),
                affected_units: affected.clone(),
                sloc: report
                    .units
                    .iter()
                    .filter(|u| affected.contains(&u.id))
                    .map(|u| u.sloc)
                    .sum(),
            };
            if backend.is_some() {
                report.non_fstar_trust.push(item);
                continue;
            }
            report.trusted_items.push(item);
            for u in report.units.iter_mut().filter(|u| affected.contains(&u.id)) {
                if u.status != "hazard" {
                    u.status = "trusted".into();
                }
            }
        }
    }
    Ok(())
}

/// Mask nested comments and strings without changing byte offsets or line numbers.
pub(crate) fn mask(text: &str) -> Result<String> {
    let mut out = text.as_bytes().to_vec();
    let bytes = text.as_bytes();
    let (mut i, mut depth, mut string) = (0, 0usize, false);
    while i < bytes.len() {
        if depth == 0 && !string && bytes.get(i..i + 2) == Some(b"//") {
            while i < bytes.len() && bytes[i] != b'\n' {
                out[i] = b' ';
                i += 1;
            }
            continue;
        }
        if !string && bytes.get(i..i + 2) == Some(b"(*") {
            depth += 1;
            out[i] = b' ';
            out[i + 1] = b' ';
            i += 2;
            continue;
        }
        if depth > 0 && bytes.get(i..i + 2) == Some(b"*)") {
            depth -= 1;
            out[i] = b' ';
            out[i + 1] = b' ';
            i += 2;
            continue;
        }
        if depth == 0 && bytes[i] == b'"' {
            string = !string;
            out[i] = b' ';
            i += 1;
            continue;
        }
        if string && bytes[i] == b'\\' {
            out[i] = b' ';
            i += 1;
            if i < bytes.len() {
                if bytes[i] != b'\n' {
                    out[i] = b' ';
                }
                i += 1;
            }
            continue;
        }
        if (depth > 0 || string) && bytes[i] != b'\n' {
            out[i] = b' ';
        }
        i += 1;
    }
    ensure!(depth == 0 && !string, "unclosed F* comment/string");
    Ok(String::from_utf8(out)?)
}
fn words(s: &str) -> impl Iterator<Item = &str> {
    s.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|w| !w.is_empty())
}
struct Decl {
    name: String,
    line: usize,
    kind: String,
    text: String,
}
fn identifier_prefix(s: &str) -> &str {
    s.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()
        .unwrap_or("")
}
// hax emits declarations at column zero; `and` continues the previous `let rec`/`type` group.
fn declaration_start(line: &str, previous_kind: Option<&str>) -> Option<(String, String)> {
    for prefix in ["let rec ", "let ", "val ", "assume val ", "type "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            let kind = if prefix == "let rec " {
                "let"
            } else {
                prefix.trim()
            };
            return Some((identifier_prefix(rest).to_string(), kind.to_string()));
        }
    }
    let rest = line.strip_prefix("and ")?;
    let kind = previous_kind.filter(|k| ["let", "type"].contains(k))?;
    Some((identifier_prefix(rest).to_string(), kind.to_string()))
}
fn declarations(text: &str) -> Vec<Decl> {
    let lines: Vec<_> = text.lines().collect();
    let mut starts: Vec<(usize, String, String)> = vec![];
    for (i, line) in lines.iter().enumerate() {
        let previous_kind = starts.last().map(|s| s.2.as_str());
        if let Some((name, kind)) = declaration_start(line, previous_kind) {
            if !name.is_empty() {
                starts.push((i, name, kind));
            }
        }
    }
    starts
        .iter()
        .enumerate()
        .map(|(j, (i, name, kind))| Decl {
            name: name.clone(),
            line: i + 1,
            kind: kind.clone(),
            text: lines[*i..starts.get(j + 1).map(|x| x.0).unwrap_or(lines.len())].join("\n"),
        })
        .collect()
}
fn mapped<'a>(module: &str, name: &str, report: &'a Report) -> Option<&'a crate::Unit> {
    let module = module
        .split('.')
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join("::");
    let direct = format!("{module}::{name}");
    let mut units = report.units.iter().filter(|u| u.id == direct);
    if let Some(first) = units.next() {
        return units.next().is_none().then_some(first);
    }
    let inherent = name
        .strip_prefix("impl_")
        .and_then(|rest| rest.split_once("__"))
        .filter(|(ty, method)| !ty.is_empty() && !method.is_empty());
    if let Some((ty, method)) = inherent {
        let id = format!("{module}::{ty}::{method}");
        let mut units = report.units.iter().filter(|u| u.id == id);
        if let Some(first) = units.next() {
            return units.next().is_none().then_some(first);
        }
    }
    // R1: only uppercase-initial hax value names; never guess other escapes.
    fn strip_value(value: &str) -> Option<&str> {
        value
            .strip_prefix("v_")
            .filter(|rest| rest.as_bytes().first().is_some_and(u8::is_ascii_uppercase))
    }
    let id = if let Some((ty, method)) = inherent {
        format!("{module}::{ty}::{}", strip_value(method)?)
    } else {
        format!("{module}::{}", strip_value(name)?)
    };
    let mut units = report.units.iter().filter(|u| u.id == id);
    let first = units.next()?;
    units.next().is_none().then_some(first)
}
fn numbered_name(name: &str) -> Option<(&str, &str)> {
    let (number, method) = name.strip_prefix("impl_")?.split_once("__")?;
    (!number.is_empty() && number.bytes().all(|c| c.is_ascii_digit()) && !method.is_empty())
        .then_some((number, method))
}
fn numbered_method(method: &str) -> &str {
    method
        .strip_prefix("v_")
        .filter(|rest| rest.as_bytes().first().is_some_and(u8::is_ascii_uppercase))
        .unwrap_or(method)
}

/// Decisions are made for whole groups, before any unit receives credit.
fn numbered_mappings(
    module: &str,
    decls: &[Decl],
    report: &Report,
) -> BTreeMap<String, std::result::Result<String, String>> {
    let rust_module = module
        .split('.')
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join("::");
    let mut groups: BTreeMap<&str, Vec<&Decl>> = BTreeMap::new();
    for decl in decls {
        if let Some((n, _)) = numbered_name(&decl.name) {
            groups.entry(n).or_default().push(decl);
        }
    }
    let mut blocks: BTreeMap<_, Vec<&crate::Unit>> = BTreeMap::new();
    for unit in &report.units {
        if let Some((start, start_column, end, end_column, ty)) = &unit.inherent_impl {
            if unit.id == format!("{rust_module}::{ty}::{}", unit.name) {
                blocks
                    .entry((
                        unit.file.clone(),
                        *start,
                        *start_column,
                        *end,
                        *end_column,
                        ty.clone(),
                    ))
                    .or_default()
                    .push(unit);
            }
        }
    }
    let mut decisions = BTreeMap::new();
    for (n, group) in &groups {
        // The shared declaration parser recognizes an identifier prefix. An F* apostrophe
        // continuation is unsupported here and must not turn `method'` into `method`.
        if group
            .iter()
            .any(|d| d.text.contains(&format!("{}'", d.name)))
        {
            decisions.insert(
                *n,
                Err(format!(
                    "numbered impl_{n}: unsupported F* identifier continuation"
                )),
            );
            continue;
        }
        let candidates: Vec<_> = blocks
            .iter()
            .filter(|((_, _, _, _, _, ty), units)| {
                group.iter().all(|d| {
                    units
                        .iter()
                        .any(|u| u.name == numbered_method(numbered_name(&d.name).unwrap().1))
                }) && group.iter().any(|d| {
                    d.text
                        .split(|c: char| !c.is_alphanumeric() && c != '_' && c != '\'')
                        .any(|w| w == format!("t_{ty}"))
                })
            })
            .map(|(key, _)| key.clone())
            .collect();
        decisions.insert(*n, if candidates.len() == 1 { Ok(candidates[0].clone()) }
            else { Err(format!("numbered impl_{n}: {} qualifying inherent blocks (whole method set and type-token guard; missing/unsupported Rust syntax has no candidates)", candidates.len())) });
    }
    let mut result = BTreeMap::new();
    for (n, group) in groups {
        let decision = &decisions[n];
        let collision = decision.as_ref().is_ok_and(|block| {
            decisions
                .values()
                .filter(|d| d.as_ref().ok() == Some(block))
                .count()
                > 1
        });
        for decl in group {
            let mapped = match decision {
                Ok(_) if collision => Err(format!("numbered impl_{n}: injectivity conflict; multiple groups select the same block")),
                Ok(block) => {
                    let method = numbered_method(numbered_name(&decl.name).unwrap().1);
                    let units: Vec<_> = blocks[block].iter().filter(|u| u.name == method).collect();
                    if units.len() == 1 && report.units.iter().filter(|u| u.id == units[0].id).count() == 1 {
                        Ok(units[0].id.clone())
                    } else { Err(format!("numbered impl_{n}: ambiguous Rust unit identity")) }
                }
                Err(reason) => Err(reason.clone()),
            };
            result.insert(decl.name.clone(), mapped);
        }
    }
    result
}

fn contract(decl: &str, names: &[String]) -> (String, String, bool) {
    if let Some((_, post)) = decl.split_once("(ensures") {
        let post = post.trim().to_string();
        let meaningful = words(&post)
            .any(|w| !["fun", "result", "True", "true", "Prims", "l_True"].contains(&w));
        if meaningful {
            // Heuristic: same-module function reference and no known external spec namespace.
            let own = words(&post).any(|w| names.iter().any(|n| n == w));
            let external = post.split_whitespace().any(|w| {
                w.contains('.')
                    && !w.starts_with("Core.")
                    && !w.starts_with("Prims.")
                    && !w.starts_with("Bertie.")
            });
            return ("functional".into(), post, own && !external);
        }
        return ("safety-only".into(), post, false);
    }
    if decl.contains("Prims.Pure") && decl.contains("(fun _ -> Prims.l_True)")
        || decl.contains("Tot ")
    {
        return (
            "safety-only".into(),
            "True (or absent for Tot)".into(),
            false,
        );
    }
    ("unclassified".into(), decl.into(), false)
}
fn pragma_name(line: &str) -> Option<&str> {
    let word = line.trim_start().strip_prefix('#')?;
    let end = word
        .find(|c: char| !(c.is_ascii_lowercase() || c == '-'))
        .unwrap_or(word.len());
    word[..end].contains('-').then_some(&word[..end])
}
const SPLIT_QUERIES_VALUES: [&str; 3] = ["always", "on_failure", "no"];
fn option_is_accepted(name: &str, value: &str) -> bool {
    match name {
        "--fuel" | "--ifuel" | "--z3rlimit" => value.parse::<usize>().is_ok(),
        "--split_queries" => SPLIT_QUERIES_VALUES.contains(&value),
        _ => false,
    }
}
fn first_rejected_in_options(options: &str) -> Option<String> {
    let args: Vec<_> = options.split_whitespace().collect();
    for (i, name) in args.iter().enumerate().step_by(2) {
        let Some(value) = args.get(i + 1) else {
            return Some(name.to_string());
        };
        if !option_is_accepted(name, value) {
            let shown = if value.starts_with("--") {
                name.to_string()
            } else {
                format!("{name} {value}")
            };
            return Some(shown);
        }
    }
    None
}
fn rejected_option_in_line(line: &str) -> Option<String> {
    if !line.trim_start().starts_with('#')
        || matches!(
            pragma_name(line),
            None | Some("pop-options") | Some("restart-solver")
        )
    {
        return None;
    }
    let Some((_, tail)) = line.split_once('"') else {
        return Some(line.trim().to_string());
    };
    let Some((options, _)) = tail.rsplit_once('"') else {
        return Some(line.trim().to_string());
    };
    first_rejected_in_options(options)
}
pub(crate) fn rejected_option(raw: &str) -> Option<String> {
    raw.lines().find_map(rejected_option_in_line)
}
fn apply_validated(
    report: &mut Report,
    _root: &Path,
    log: &Path,
    receipt: &FstarReceipt,
    run: &mut EvidenceRun,
) -> Result<()> {
    let base = log.parent().unwrap_or(Path::new("."));
    run.trusted_sites.push((
        log.display().to_string(),
        1,
        "hax extraction (trusted)".into(),
    ));
    report.trusted_items.push(crate::evidence::TrustedItem {
        file: log.display().to_string(),
        line: 1,
        kind: format!(
            "hax extraction {} {}",
            receipt.hax_version, receipt.hax_commit
        ),
        affected_units: vec![],
        sloc: 0,
    });
    let mut module_decls: BTreeMap<String, Vec<Decl>> = BTreeMap::new();
    let mut parse_errors = BTreeMap::new();
    for file in receipt.checked.keys() {
        let module = Path::new(file)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let raw = fs::read_to_string(inside(base, file)?)?;
        match mask(&raw) {
            Ok(text) if text.lines().any(|l| l == format!("module {module}")) => module_decls
                .entry(module)
                .or_default()
                .extend(declarations(&text)),
            _ => {
                parse_errors.insert(
                    module,
                    "numbered mapping: sibling F* parse/module failure".to_string(),
                );
            }
        }
    }
    let numbered: BTreeMap<_, _> = module_decls
        .iter()
        .map(|(module, decls)| (module.clone(), numbered_mappings(module, decls, report)))
        .collect();
    for (file, hash) in &receipt.checked {
        let raw = fs::read_to_string(inside(base, file)?)?;
        let text = match mask(&raw) {
            Ok(t) => t,
            Err(e) => {
                run.reasons.push(format!("{file}: {e}; no credit"));
                continue;
            }
        };
        let module = Path::new(file).file_stem().unwrap().to_string_lossy();
        if !text.lines().any(|l| l == format!("module {module}")) {
            run.reasons
                .push(format!("{file}: module name mismatch; unmapped"));
            continue;
        }
        let decls = declarations(&text);
        let names: Vec<_> = decls.iter().map(|d| d.name.clone()).collect();
        let hazards: Vec<_> = text
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                words(l)
                    .filter(|w| ["assume", "admit", "admitP"].contains(w))
                    .map(move |w| (i + 1, w.to_string()))
            })
            .chain(raw.lines().enumerate().filter_map(|(i, l)| {
                rejected_option_in_line(l).map(|o| (i + 1, format!("unsupported F* option: {o}")))
            }))
            .collect();
        for (line, kind) in &hazards {
            report.escape_hatches.push(crate::Escape {
                unit: format!("{file}::<file-scope>"),
                affected_units: vec![],
                file: file.clone(),
                line: *line,
                kind: kind.clone(),
                security: true,
                first_party: true,
            });
            run.escape_sites.push((file.clone(), *line, kind.clone()));
        }
        let sibling = |ext: &str| {
            Path::new(file)
                .with_extension(ext)
                .to_string_lossy()
                .to_string()
        };
        let implementation = sibling("fst");
        let interface = sibling("fsti");
        let interface_hazard = if receipt.checked.contains_key(&interface) {
            let spec = fs::read_to_string(inside(base, &interface)?)?;
            words(&mask(&spec)?).any(|w| ["assume", "admit", "admitP"].contains(&w))
                || spec.contains("--admit_")
                || spec.contains("--lax")
                || rejected_option(&spec).is_some()
        } else {
            false
        };
        let success = receipt
            .modules
            .iter()
            .any(|m| m.file == implementation && m.outcome == "success");
        let identical = |p: &str| {
            receipt.checked.get(p).is_some_and(|h| {
                receipt
                    .fresh
                    .iter()
                    .any(|(f, v)| Path::new(f).file_name() == Path::new(p).file_name() && v == h)
            })
        };
        let fresh = identical(file)
            && (!receipt.checked.contains_key(&interface) || identical(&interface))
            && (!receipt.checked.contains_key(&implementation) || identical(&implementation));
        let is_interface = file.ends_with(".fsti");
        for decl in decls.iter().filter(|d| d.kind != "type" && d.name != "_") {
            let mut unit = mapped(&module, &decl.name, report).cloned();
            let mut mapping_reason = None;
            if unit.is_none() && numbered_name(&decl.name).is_some() {
                let direct = format!(
                    "{}::{}",
                    module
                        .split('.')
                        .map(str::to_lowercase)
                        .collect::<Vec<_>>()
                        .join("::"),
                    decl.name
                );
                if report.units.iter().filter(|u| u.id == direct).count() > 1 {
                    mapping_reason =
                        Some("numbered mapping: ambiguous direct Rust identity".into());
                } else if let Some(reason) = parse_errors.get(module.as_ref()) {
                    mapping_reason = Some(reason.clone());
                } else if let Some(decision) = numbered
                    .get(module.as_ref())
                    .and_then(|m| m.get(&decl.name))
                {
                    match decision {
                        Ok(id) => unit = report.units.iter().find(|u| &u.id == id).cloned(),
                        Err(reason) => mapping_reason = Some(reason.clone()),
                    }
                }
            }
            let has_body = if is_interface {
                receipt.checked.contains_key(&implementation)
                    && declarations(&mask(&fs::read_to_string(inside(base, &implementation)?)?)?)
                        .iter()
                        .any(|d| d.kind == "let" && d.name == decl.name)
            } else {
                decl.kind == "let"
            };
            let specification = if is_interface {
                decl.text.clone()
            } else if receipt.checked.contains_key(&interface) {
                declarations(&mask(&fs::read_to_string(inside(base, &interface)?)?)?)
                    .into_iter()
                    .find(|d| d.name == decl.name)
                    .map(|d| d.text)
                    .unwrap_or_default()
            } else {
                decl.text.clone()
            };
            let (kind, post, self_consistency) = contract(&specification, &names);
            let rust_trusted = unit.as_ref().is_some_and(|u| u.status == "trusted");
            let trusted = rust_trusted
                || !fresh
                || !has_body
                || (is_interface && !success)
                || decl.kind == "assume val";
            let classification = if trusted {
                "trusted"
            } else if !hazards.is_empty() || interface_hazard {
                "hazard"
            } else if unit.is_none() {
                "unmapped"
            } else if success && kind != "unclassified" {
                "proven-extracted"
            } else {
                "unverified"
            };
            if trusted {
                let reason = if rust_trusted {
                    "Rust hax opaque/replacement trust"
                } else if !fresh {
                    "patch-altered or non-fresh F* module (whole-module conservative rule)"
                } else {
                    "interface-only val / assume val"
                };
                report.trusted_items.push(crate::evidence::TrustedItem {
                    file: file.clone(),
                    line: decl.line,
                    kind: reason.into(),
                    affected_units: unit.iter().map(|u| u.id.clone()).collect(),
                    sloc: unit.as_ref().map_or(0, |u| u.sloc),
                });
                run.trusted_sites
                    .push((file.clone(), decl.line, reason.into()));
            }
            if !is_interface || trusted {
                if let Some(u) = &unit {
                    if classification == "proven-extracted"
                        && (u.category == crate::Category::Runtime
                            || (u.fstar_cfg_hax && u.cfg_hax_was_runtime))
                        && !["hazard", "trusted", "proven", "proven-bounded"]
                            .contains(&u.status.as_str())
                    {
                        let target = report.units.iter_mut().find(|v| v.id == u.id).unwrap();
                        target.status = if target.fstar_cfg_hax {
                            "verified"
                        } else {
                            classification
                        }
                        .into();
                        target.proof_scope = Some(format!("hax/F*; trusted hax extraction; contract {kind}; postcondition: {post}; self-consistency heuristic={self_consistency}{}", if self_consistency { "; consistent with its own parser; not checked against an external spec" } else { "" }));
                        run.credited_units.push(u.id.clone());
                    }
                    if trusted {
                        let target = report.units.iter_mut().find(|v| v.id == u.id).unwrap();
                        if target.status != "hazard" {
                            target.status = "trusted".into();
                        }
                    }
                }
            }
            report.fstar_definitions.push(FstarDefinition {
                name: format!("{module}.{}", decl.name),
                file: file.clone(),
                line: decl.line,
                rust_unit: unit.map(|u| u.id),
                classification: classification.into(),
                contract_kind: kind,
                postcondition: post,
                self_consistency_heuristic: self_consistency,
                mapping_reason,
            });
        }
        let _ = hash;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn implicit_argument_lines_are_not_file_options_but_real_pragmas_still_are() {
        let benign = "f\n  #u8\n  #FStar.Tactics.Typeclasses.solve\n  #(t_Slice u8)\n#pop-options\n#restart-solver\n#push-options \"--fuel 0 --z3rlimit 100\"\n";
        assert_eq!(rejected_option(benign), None);
        for hazard in [
            "#push-options \"--admit_smt_queries true\"",
            "#set-options \"--lax\"",
            "#reset-options",
            "#push-options \"--z3rlimit\"",
            "#push-options \"--fuel x\"",
            "#print-effects-graph",
        ] {
            assert!(
                rejected_option(&format!("let x = 1\n{hazard}\n")).is_some(),
                "{hazard}"
            );
        }
    }
    #[test]
    fn split_queries_is_the_only_added_option_and_only_with_a_known_value() {
        for value in ["always", "on_failure", "no"] {
            for extra in ["", " --fuel 0 --ifuel 1", " --z3rlimit 100"] {
                let pragma = format!("#push-options \"--split_queries {value}{extra}\"\n");
                assert_eq!(rejected_option(&pragma), None, "{pragma}");
                let leading = format!("#push-options \"--z3rlimit 5 --split_queries {value}\"\n");
                assert_eq!(rejected_option(&leading), None, "{leading}");
            }
        }
        for (pragma, shown) in [
            ("#push-options \"--split_queries\"", "--split_queries"),
            (
                "#push-options \"--split_queries maybe\"",
                "--split_queries maybe",
            ),
            (
                "#push-options \"--split_queries Always\"",
                "--split_queries Always",
            ),
            (
                "#push-options \"--split_queries --fuel 1\"",
                "--split_queries",
            ),
            (
                "#push-options \"--split_queries always --admit_smt_queries true\"",
                "--admit_smt_queries true",
            ),
            ("#push-options \"--split_queries always --lax\"", "--lax"),
            ("#push-options \"--lax --split_queries always\"", "--lax"),
            (
                "#push-options \"--split_queries always --foo bar\"",
                "--foo bar",
            ),
            ("#push-options \"--z3rlimit\"", "--z3rlimit"),
            ("#push-options \"--fuel x\"", "--fuel x"),
            ("#reset-options", "#reset-options"),
        ] {
            assert_eq!(
                rejected_option(&format!("{pragma}\n")).as_deref(),
                Some(shown),
                "{pragma}"
            );
        }
    }
    #[test]
    fn contracts_and_comment_masking() {
        let names = vec!["parse_certificate_verify".into()];
        let (kind,_,own) = contract("val certificate_verify : Prims.Pure result True (ensures fun result -> parse_certificate_verify result = x)",&names);
        assert_eq!(kind, "functional");
        assert!(own);
        assert_eq!(
            contract(
                "val ecdsa_signature : Prims.Pure result Prims.l_True (fun _ -> Prims.l_True)",
                &names
            )
            .0,
            "safety-only"
        );
        assert!(
            !contract(
                "val f : result (ensures fun result -> External.Spec.valid result)",
                &names
            )
            .2
        );
        let text = "module Test\n(* admit (* assume *) *)\n// admitP\nlet f x = \"admit\"\n";
        let masked = mask(text).unwrap();
        assert_eq!(text.lines().count(), masked.lines().count());
        assert!(!words(&masked).any(|w| ["admit", "assume", "admitP"].contains(&w)));
        assert!(mask("(* missing").is_err());
        assert_eq!(declarations(&masked)[0].name, "f");
    }
    #[test]
    fn let_rec_declarations_are_named_after_the_function() {
        let text = "module Bertie.Der\n\nlet two_pow_256_ (_: Prims.unit) =\n  1\n\nlet rec be_value_prefix (c: t_Slice u8) (n: usize) =\n  if n =. mk_usize 0\n  then Rust_primitives.Hax.Int.from_machine (mk_u8 0)\n  else\n    ((be_value_prefix c (n -! mk_usize 1 <: usize) <: Hax_lib.Int.t_Int) *\n\nlet rec pow256 (n: usize) =\n  if n =. mk_usize 0\n  then Rust_primitives.Hax.Int.from_machine (mk_u8 1)\n\n#push-options \"--fuel 2 --z3rlimit 100\"\n\nlet rec skip_zeros (buf: t_Slice u8) (i: usize) =\n  if i <. mk_usize 32\n\n#pop-options\n\nlet rec pow256_is_pow2 (n: usize) =\n  if n >. mk_usize 0\n";
        let decls = declarations(text);
        let named: Vec<_> = decls
            .iter()
            .map(|d| (d.name.as_str(), d.kind.as_str()))
            .collect();
        assert_eq!(
            named,
            [
                ("two_pow_256_", "let"),
                ("be_value_prefix", "let"),
                ("pow256", "let"),
                ("skip_zeros", "let"),
                ("pow256_is_pow2", "let")
            ]
        );
        assert_eq!(decls[1].line, 6);
        assert!(decls[1].text.starts_with("let rec be_value_prefix"));
        assert!(!decls.iter().any(|d| d.name == "rec"));
    }
    #[test]
    fn mutually_recursive_groups_name_every_member() {
        let text = "module M\nlet rec even (n: usize) =\n  if n =. mk_usize 0 then true else odd n\n\nand odd (n: usize) =\n  if n =. mk_usize 0 then false else even n\n\nlet after (x: u8) = x\ntype t = | A\nand u = | B\nval v : u8\nand stray = 1\n";
        let named: Vec<_> = declarations(text)
            .into_iter()
            .map(|d| (d.name, d.kind))
            .collect();
        let expected = [
            ("even", "let"),
            ("odd", "let"),
            ("after", "let"),
            ("t", "type"),
            ("u", "type"),
            ("v", "val"),
        ];
        assert_eq!(named, expected.map(|(n, k)| (n.to_string(), k.to_string())));
    }
    #[test]
    fn inherent_mapping_is_unique_and_direct_names_take_precedence() {
        let mut report = crate::inventory(Path::new("fixtures/hax")).unwrap();
        let mut unit = report.units[0].clone();
        unit.id = "crate::m::Foo::bar".into();
        report.units = vec![unit.clone()];
        assert_eq!(
            mapped("Crate.M", "impl_Foo__bar", &report).unwrap().id,
            unit.id
        );
        for name in [
            "impl_1",
            "impl_1'",
            "impl",
            "impl_Foo__missing",
            "impl___bar",
            "impl_Foo__",
        ] {
            assert!(mapped("Crate.M", name, &report).is_none(), "{name}");
        }
        report.units.push(unit);
        assert!(mapped("Crate.M", "impl_Foo__bar", &report).is_none());
        report.units.pop();
        let mut direct = report.units[0].clone();
        direct.id = "crate::m::impl_Foo__bar".into();
        report.units.push(direct.clone());
        assert_eq!(
            mapped("Crate.M", "impl_Foo__bar", &report).unwrap().id,
            direct.id
        );
        report.units.push(direct);
        // An ambiguous direct id must never fall back to a unique inherent id.
        assert!(mapped("Crate.M", "impl_Foo__bar", &report).is_none());
    }
    #[test]
    fn uppercase_value_mapping_is_unique_and_preserves_existing_precedence() {
        let mut report = crate::inventory(Path::new("fixtures/hax")).unwrap();
        let mut unit = report.units[0].clone();
        unit.id = "crate::m::XTR".into();
        report.units = vec![unit.clone()];
        assert_eq!(mapped("Crate.M", "v_XTR", &report).unwrap().id, unit.id);
        for rest in ["xtr", "", "1A", "_A", "É", "v_XTR"] {
            report.units[0].id = format!("crate::m::{rest}");
            assert!(mapped("Crate.M", &format!("v_{rest}"), &report).is_none());
        }
        unit.id = "crate::m::Foo".into();
        report.units = vec![unit.clone(), unit.clone()];
        assert!(mapped("Crate.M", "v_Foo", &report).is_none());
        report.units.pop();
        let mut direct = unit.clone();
        direct.id = "crate::m::v_Foo".into();
        report.units.push(direct.clone());
        assert_eq!(mapped("Crate.M", "v_Foo", &report).unwrap().id, direct.id);
        report.units.push(direct);
        assert!(mapped("Crate.M", "v_Foo", &report).is_none());

        unit.id = "crate::m::Foo::Bar".into();
        report.units = vec![unit.clone()];
        assert_eq!(
            mapped("Crate.M", "impl_Foo__v_Bar", &report).unwrap().id,
            unit.id
        );
        report.units.push(unit.clone());
        assert!(mapped("Crate.M", "impl_Foo__v_Bar", &report).is_none());
        report.units.pop();
        for rest in ["bar", "", "1A", "_A", "É"] {
            report.units[0].id = format!("crate::m::Foo::{rest}");
            assert!(mapped("Crate.M", &format!("impl_Foo__v_{rest}"), &report).is_none());
        }
        report.units[0] = unit.clone();
        let mut existing = unit.clone();
        existing.id = "crate::m::Foo::v_Bar".into();
        report.units.push(existing.clone());
        assert_eq!(
            mapped("Crate.M", "impl_Foo__v_Bar", &report).unwrap().id,
            existing.id
        );
        report.units.push(existing);
        assert!(mapped("Crate.M", "impl_Foo__v_Bar", &report).is_none());
        let mut direct = unit;
        direct.id = "crate::m::impl_Foo__v_Bar".into();
        report.units.push(direct.clone());
        assert_eq!(
            mapped("Crate.M", "impl_Foo__v_Bar", &report).unwrap().id,
            direct.id
        );
    }
    fn numbered_report(source: &str) -> Report {
        let mut report = crate::inventory(Path::new("fixtures/hax")).unwrap();
        report.units.clear();
        let lines: Vec<_> = source.lines().collect();
        crate::walk(
            source.parse().unwrap(),
            "m",
            crate::SourceContext {
                krate: "crate",
                file: "src/m.rs",
                source_lines: &lines,
            },
            false,
            "",
            &[],
            &mut report.units,
        );
        crate::inventory_impls(source, &mut report.units);
        report
    }
    #[test]
    fn numbered_groups_use_blocks_not_type_or_source_order() {
        let report =
            numbered_report("struct Foo;\nimpl Foo { fn a() {} }\nimpl Foo { fn B() {} }\n");
        let decls = declarations("let impl_91__a (x: t_Foo) = x\nlet impl_2__v_B (x: t_Foo) = x\n");
        let result = numbered_mappings("Crate.M", &decls, &report);
        assert_eq!(result["impl_91__a"].as_deref(), Ok("crate::m::Foo::a"));
        assert_eq!(result["impl_2__v_B"].as_deref(), Ok("crate::m::Foo::B"));
    }
    #[test]
    fn numbered_groups_fail_closed_for_ambiguity_missing_methods_traits_and_type_guards() {
        for (source, text) in [
            (
                "struct Foo; impl Foo { fn a() {} }",
                "let impl_1__a (x: t_Foo') = x",
            ),
            (
                "struct Foo; impl Foo { fn a() {} }",
                "let impl_1__a' (x: t_Foo) = x",
            ),
            (
                "struct Foo; impl Foo { fn a() {} } impl Foo { fn a() {} }",
                "let impl_1__a (x: t_Foo) = x",
            ),
            (
                "struct Foo; struct Bar;\nimpl Foo { fn a() {} }\nimpl Bar { fn a() {} }",
                "let impl_1__a (x: t_Foo) (y: t_Bar) = x",
            ),
            (
                "struct Foo;\nimpl Foo { fn a() {} }",
                "let impl_1__missing (x: t_Foo) = x",
            ),
            (
                "struct Foo;\nimpl Trait for Foo { fn a() {} }",
                "let impl_1__a (x: t_Foo) = x",
            ),
            (
                "struct Foo;\nimpl Foo { fn a() {} }",
                "let impl_1__a (x: t_FooExtra) = x",
            ),
            (
                "struct Foo;\nimpl Foo { fn a() {} }",
                "let impl_1__a (x: t_Bar) = x",
            ),
            (
                "struct Foo;\nimpl Foo { fn a() {} }\nimpl Foo { fn b() {} }",
                "let impl_1__a (x: t_Foo) = x\nlet impl_1__b (x: t_Foo) = x",
            ),
        ] {
            let result =
                numbered_mappings("Crate.M", &declarations(text), &numbered_report(source));
            assert!(!result.is_empty());
            assert!(result.values().all(|r| r.is_err()), "{text}");
        }
    }
    #[test]
    fn numbered_groups_are_injective_and_digits_are_ascii() {
        let report = numbered_report("struct Foo;\nimpl Foo { fn a() {} fn b() {} }");
        let result = numbered_mappings(
            "Crate.M",
            &declarations("let impl_2__a (x: t_Foo) = x\nlet impl_9__b (x: t_Foo) = x"),
            &report,
        );
        assert!(result
            .values()
            .all(|r| r.as_ref().unwrap_err().contains("injectivity")));
        for name in ["impl_１２__a", "impl_1__", "impl___a", "impl_1x__a"] {
            assert!(numbered_name(name).is_none());
        }
    }
    #[test]
    fn closed_command() {
        let checked = BTreeMap::from([("fresh/Test.fst".into(), "hash".into())]);
        let good = vec!["fstar.exe".into(), "fresh/Test.fst".into()];
        assert!(command(&good, "fresh/Test.fst", &checked));
        for flag in [
            "--lax",
            "--admit_smt_queries",
            "--admit_except",
            "--admit",
            "--unknown",
            "@flags",
            "--cache_checked_modules",
            "--use_hints",
            "--record_hints",
        ] {
            let mut args = good.clone();
            args.push(flag.into());
            assert!(!command(&args, "fresh/Test.fst", &checked), "{flag}");
        }
        assert!(!command(&["fstar.exe".into()], "fresh/Test.fst", &checked));
        assert!(!command(
            &[
                "fstar.exe".into(),
                "--include".into(),
                "unhashed".into(),
                "fresh/Test.fst".into()
            ],
            "fresh/Test.fst",
            &checked
        ));
    }
}
