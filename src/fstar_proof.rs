//! Configured hand-written F* cost, bound to strictly verified receipt modules.
use crate::fstar::FstarReceipt;
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FstarConfig {
    #[serde(default)]
    pub proof: ProofConfig,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofConfig {
    #[serde(default)]
    pub patches: Vec<String>,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub models: Vec<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Input {
    pub file: String,
    pub kind: String,
    pub target_module: String,
    pub crate_name: String,
    pub module: String,
    pub security: bool,
    pub sloc: usize,
    pub reason: Option<String>,
    /// Patch line of the hunk header, absent for standalone files/models.
    pub hunk_line: Option<usize>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcludedHunk {
    pub file: String,
    pub target_module: String,
    pub target_file: String,
    pub hunk_line: usize,
    pub reason: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProofState {
    pub inputs: Vec<Input>,
    pub excluded_patch_hunks: Vec<ExcludedHunk>,
    #[serde(skip)]
    pub config: ProofConfig,
    #[serde(skip)]
    pub strict: BTreeSet<String>,
    #[serde(skip)]
    pub rejected: BTreeMap<String, String>,
    #[serde(skip)]
    pub checked_hashes: BTreeMap<String, BTreeSet<String>>,
    #[serde(skip)]
    pub extraction: BTreeSet<String>,
    #[serde(skip)]
    pub strict_hashes: BTreeMap<String, BTreeSet<String>>,
}
fn basename(file: &str) -> String {
    Path::new(file)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into()
}
fn module(file: &str) -> String {
    Path::new(file)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into()
}
fn expand(root: &Path, patterns: &[String], extensions: &[&str]) -> Result<Vec<PathBuf>> {
    let mut files = BTreeSet::new();
    let root = root.canonicalize()?;
    for pattern in patterns {
        ensure!(
            !Path::new(pattern).is_absolute(),
            "F* proof glob must be relative: {pattern}"
        );
        let mut count = 0;
        for entry in glob::glob(root.join(pattern).to_str().context("non-UTF8 proof glob")?)? {
            let path = entry?.canonicalize()?;
            ensure!(
                path.starts_with(&root) && path.is_file(),
                "F* proof input outside workspace or not a file: {pattern}"
            );
            ensure!(
                path.extension()
                    .and_then(|s| s.to_str())
                    .is_some_and(|ext| extensions.contains(&ext)),
                "unsupported F* proof input extension: {}",
                path.display()
            );
            files.insert(path);
            count += 1;
        }
        ensure!(count > 0, "F* proof glob matches nothing: {pattern}");
    }
    Ok(files.into_iter().collect())
}
/// Comments are masked, strings retained: a string literal is code even when it
/// contains comment-looking text. Nested block comments retain cross-line state.
fn code_lines(text: &str) -> Result<BTreeSet<usize>> {
    let bytes = text.as_bytes();
    let (mut i, mut line, mut depth, mut string) = (0, 1, 0usize, false);
    let mut lines = BTreeSet::new();
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            line += 1;
            i += 1;
            continue;
        }
        if !string && bytes.get(i..i + 2) == Some(b"(*") {
            depth += 1;
            i += 2;
            continue;
        }
        if depth > 0 {
            if bytes.get(i..i + 2) == Some(b"*)") {
                depth -= 1;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        if !string && bytes.get(i..i + 2) == Some(b"//") {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if string && bytes[i] == b'\\' {
            lines.insert(line);
            i += 1;
            if i < bytes.len() {
                line += usize::from(bytes[i] == b'\n');
                i += 1;
            }
            continue;
        }
        if bytes[i] == b'"' {
            string = !string;
        }
        if string || !bytes[i].is_ascii_whitespace() {
            lines.insert(line);
        }
        i += 1;
    }
    ensure!(depth == 0 && !string, "unclosed F* proof comment/string");
    Ok(lines)
}
#[derive(Debug)]
struct Hunk {
    target: String,
    line: usize,
    added: String,
    added_lines: BTreeSet<usize>,
}
fn hunks(text: &str) -> Result<Vec<Hunk>> {
    let mut target = String::new();
    let mut result: Vec<Hunk> = vec![];
    // Keep context/new-side text as well as additions, so a comment opened by
    // context or another added line cannot make comment-only additions count.
    for (i, line) in text.lines().enumerate() {
        if let Some(path) = line.strip_prefix("+++ ") {
            target = path.split('\t').next().unwrap_or_default().to_string();
        } else if line.starts_with("@@ ") {
            ensure!(
                !target.is_empty(),
                "patch hunk has no target at line {}",
                i + 1
            );
            result.push(Hunk {
                target: target.clone(),
                line: i + 1,
                added: String::new(),
                added_lines: BTreeSet::new(),
            });
        } else if let Some(hunk) = result.last_mut() {
            if let Some(added) = line.strip_prefix('+') {
                hunk.added_lines.insert(hunk.added.lines().count() + 1);
                hunk.added.push_str(added);
                hunk.added.push('\n');
            } else if let Some(context) = line.strip_prefix(' ') {
                hunk.added.push_str(context);
                hunk.added.push('\n');
            }
        }
    }
    Ok(result)
}
pub(crate) fn configure(
    report: &mut crate::Report,
    root: &Path,
    config: ProofConfig,
) -> Result<()> {
    // Validate even without evidence. Missing globs must never silently disappear.
    expand(root, &config.patches, &["patch", "diff"])?;
    expand(root, &config.files, &["fst", "fsti"])?;
    expand(root, &config.models, &["fst", "fsti"])?;
    report.fstar_proof.config = config;
    refresh(report, root)
}
/// Only called after the existing receipt validator accepted the complete run.
pub(crate) fn accepted(
    report: &mut crate::Report,
    base: &Path,
    receipt: &FstarReceipt,
) -> Result<()> {
    let state = &mut report.fstar_proof;
    state
        .extraction
        .extend(receipt.fresh.keys().map(|f| basename(f)));
    for (file, hash) in &receipt.checked {
        let name = basename(file);
        state
            .checked_hashes
            .entry(name.clone())
            .or_default()
            .insert(hash.clone());
        let implementation = Path::new(file)
            .with_extension("fst")
            .to_string_lossy()
            .into_owned();
        let interface = Path::new(file)
            .with_extension("fsti")
            .to_string_lossy()
            .into_owned();
        let reason = if receipt
            .checked
            .keys()
            .filter(|p| basename(p) == name)
            .count()
            != 1
        {
            Some("ambiguous module basename in accepted receipt")
        } else if !receipt.checked.contains_key(&implementation) {
            Some("interface-only module")
        } else if !receipt
            .modules
            .iter()
            .any(|m| m.file == implementation && m.outcome == "success")
        {
            Some("module not verified successfully")
        } else {
            let mut hazard = false;
            for sibling in [&implementation, &interface] {
                if receipt.checked.contains_key(sibling) {
                    let raw = fs::read_to_string(base.join(sibling))?;
                    let masked = match crate::fstar::mask(&raw) {
                        Ok(masked) => masked,
                        Err(_) => {
                            hazard = true;
                            continue;
                        }
                    };
                    hazard |= masked
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .any(|w| ["assume", "admit", "admitP"].contains(&w))
                        || crate::fstar::rejected_option(&raw).is_some();
                    hazard |= !masked
                        .lines()
                        .any(|l| l == format!("module {}", module(sibling)));
                }
            }
            hazard.then_some("hazard, admission, lax, unsupported option or module-name mismatch")
        };
        if let Some(reason) = reason {
            state.rejected.insert(name, reason.into());
        } else {
            state
                .strict_hashes
                .entry(name.clone())
                .or_default()
                .insert(hash.clone());
            state.strict.insert(name);
        }
    }
    Ok(())
}
fn scope(report: &crate::Report, root: &Path, target: &str) -> (String, String, bool) {
    let parts: Vec<_> = target.split('.').map(str::to_lowercase).collect();
    let krate = parts.first().cloned().unwrap_or_default();
    let module = parts[1..].join("::");
    let matching: Vec<_> = report
        .units
        .iter()
        .filter(|u| {
            u.first_party
                && u.category != crate::Category::Test
                && u.crate_name == krate
                && u.module == module
        })
        .collect();
    if !matching.is_empty() {
        return (krate, module, matching.iter().any(|u| u.security));
    }
    // New standalone lemmas and assumption libraries can lack Rust unit mappings.
    // Keep them in the root crate's unattributed row, conservatively security relevant.
    let manifest: Option<toml::Value> = fs::read_to_string(root.join("Cargo.toml"))
        .ok()
        .and_then(|s| s.parse().ok());
    let primary = manifest
        .as_ref()
        .and_then(|v| {
            v.get("lib")
                .and_then(|v| v.get("name"))
                .or_else(|| v.get("package").and_then(|v| v.get("name")))
        })
        .and_then(|v| v.as_str())
        .map(|s| s.replace('-', "_"))
        .unwrap_or_else(|| "<unattributed>".into());
    (primary, String::new(), true)
}
fn exclusion(state: &ProofState, target: &str) -> Option<String> {
    if state.strict.contains(&basename(target)) {
        return None;
    }
    Some(
        state
            .rejected
            .get(&basename(target))
            .cloned()
            .unwrap_or_else(|| {
                if state.checked_hashes.is_empty() {
                    "no accepted F* receipt".into()
                } else {
                    "module absent from accepted receipt".into()
                }
            }),
    )
}
pub(crate) fn refresh(report: &mut crate::Report, root: &Path) -> Result<()> {
    let root = root.canonicalize()?;
    let state = &report.fstar_proof;
    let config = state.config.clone();
    let models = expand(&root, &config.models, &["fst", "fsti"])?;
    let files = expand(&root, &config.files, &["fst", "fsti"])?;
    let patches = expand(&root, &config.patches, &["patch", "diff"])?;
    let mut inputs = vec![];
    let mut excluded = vec![];
    for (kind, paths) in [("model", models.clone()), ("file", files)] {
        for path in paths {
            let target = module(&path.to_string_lossy());
            let (crate_name, module, security) = scope(report, &root, &target);
            let raw = fs::read_to_string(&path)?;
            let reason = if kind == "model" {
                None
            } else if models.contains(&path) {
                Some("configured model/assumption".into())
            } else if state
                .extraction
                .contains(&basename(&path.to_string_lossy()))
            {
                Some("generated extraction".into())
            } else if let Some(reason) = exclusion(state, &path.to_string_lossy()) {
                Some(reason)
            } else if !state
                .strict_hashes
                .get(&basename(&path.to_string_lossy()))
                .is_some_and(|hashes| hashes.contains(&crate::sha256(raw.as_bytes())))
            {
                Some("file differs from accepted receipt".into())
            } else {
                None
            };
            let sloc = if reason.is_none() {
                code_lines(&raw)?.len()
            } else {
                0
            };
            inputs.push(Input {
                file: path.strip_prefix(&root)?.to_string_lossy().into(),
                kind: kind.into(),
                target_module: target,
                crate_name,
                module,
                security,
                sloc,
                reason,
                hunk_line: None,
            });
        }
    }
    for path in patches {
        let file = path.strip_prefix(&root)?.to_string_lossy().into_owned();
        for hunk in hunks(&fs::read_to_string(&path)?)? {
            let target = module(&hunk.target);
            let (crate_name, module, security) = scope(report, &root, &target);
            let reason = if models
                .iter()
                .any(|p| basename(&p.to_string_lossy()) == basename(&hunk.target))
            {
                Some("configured model/assumption".into())
            } else {
                exclusion(state, &hunk.target)
            };
            let sloc = if reason.is_none() {
                code_lines(&hunk.added)?
                    .intersection(&hunk.added_lines)
                    .count()
            } else {
                0
            };
            if let Some(reason) = &reason {
                excluded.push(ExcludedHunk {
                    file: file.clone(),
                    target_module: target.clone(),
                    target_file: hunk.target.clone(),
                    hunk_line: hunk.line,
                    reason: reason.clone(),
                });
            }
            inputs.push(Input {
                file: file.clone(),
                kind: "patch".into(),
                target_module: target,
                crate_name,
                module,
                security,
                sloc,
                reason,
                hunk_line: Some(hunk.line),
            });
        }
    }
    report.fstar_proof.inputs = inputs;
    report.fstar_proof.excluded_patch_hunks = excluded;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nested_comments_and_strings_count_only_code() {
        assert_eq!(
            code_lines(
                "(* a\n (* b *)\n*)\nlet x = \"// (*\" // comment\n// only\n\nlet y = 2 (*end*)\n"
            )
            .unwrap(),
            BTreeSet::from([4, 7])
        );
    }
    #[test]
    fn patch_counts_added_code_and_not_context_or_headers() {
        let h = hunks("--- a/X.fst\n+++ b/X.fst\n@@ -1,2 +1,6 @@\n (* context opens\n+comment\n+*)\n+let x = 1\n+//comment\n+\n").unwrap();
        assert_eq!(h.len(), 1);
        assert_eq!(
            code_lines(&h[0].added)
                .unwrap()
                .intersection(&h[0].added_lines)
                .count(),
            1
        );
    }
}
