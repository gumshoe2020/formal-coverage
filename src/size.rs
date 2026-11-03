//! Runtime dependency size modes. Missing evidence never shrinks a count: a dependency without
//! readable dep-info or debug info keeps its whole-crate size and is listed as a fallback.
use crate::{Dependency, Options, Report};
use anyhow::{Context, Result};
use object::{Object, ObjectSection, ObjectSymbol};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SizeMode {
    /// Every `.rs` file under the package source directory (v1.4.2 behaviour).
    WholeCrate,
    /// Only the `.rs` files rustc read (cargo dep-info), without spec/proof functions.
    #[default]
    CompiledFiles,
    /// Distinct (file, line) pairs in the binary's DWARF line table.
    LinkedLines,
}

impl SizeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WholeCrate => "whole-crate",
            Self::CompiledFiles => "compiled-files",
            Self::LinkedLines => "linked-lines",
        }
    }
    pub(crate) fn describe(self) -> &'static str {
        match self {
            Self::WholeCrate => "every .rs file under each package source directory, including tests, benches, examples, cfg-disabled and spec/proof code.",
            Self::CompiledFiles => "only the .rs files rustc read for each package under cargo check (dep-info, including generated OUT_DIR files), minus spec/proof functions; inline cfg-disabled items in compiled files still count.",
            Self::LinkedLines => "distinct (file, line) pairs in the supplied binary's DWARF line table under each package source directory; the unit is linked lines, not SLoC.",
        }
    }
}

impl std::str::FromStr for SizeMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "whole-crate" => Ok(Self::WholeCrate),
            "compiled-files" => Ok(Self::CompiledFiles),
            "linked-lines" => Ok(Self::LinkedLines),
            _ => anyhow::bail!(
                "unknown size mode {s:?}: expected whole-crate, compiled-files or linked-lines"
            ),
        }
    }
}

/// A runtime dependency measured whole-crate because the selected mode lacked evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SizeFallback {
    pub name: String,
    pub version: String,
    pub reason: String,
}

pub(crate) const FALLBACK: &str = "whole-crate (fallback)";

fn fall_back(d: &mut Dependency, reason: String, fallbacks: &mut Vec<SizeFallback>) {
    // d.sloc already holds the whole-crate (or explicitly configured source) measurement.
    d.size_mode = FALLBACK.into();
    d.size_reason = Some(reason.clone());
    fallbacks.push(SizeFallback {
        name: d.name.clone(),
        version: d.version.clone(),
        reason,
    });
}

pub(crate) fn apply(root: &Path, options: &Options, report: &mut Report) -> Result<()> {
    report.size_mode = options.size_mode.as_str().into();
    match options.size_mode {
        SizeMode::WholeCrate => {
            report.size_profile = "n/a (source directories)".into();
            Ok(())
        }
        SizeMode::CompiledFiles => {
            compiled_files(root, options, report);
            Ok(())
        }
        SizeMode::LinkedLines => linked_lines(root, options, report),
    }
}

struct Check {
    /// Package id to dep-info path.
    artifacts: BTreeMap<String, PathBuf>,
    error: Option<String>,
}

/// Without the target's standard library `cargo check` cannot succeed; skip it rather than
/// run build scripts for nothing.
fn missing_target_std(root: &Path, target: &str) -> Option<String> {
    let output = Command::new("rustc")
        .args(["--print", "target-libdir", "--target", target])
        .current_dir(root)
        .output();
    let output = match output {
        Ok(output) if output.status.success() => output,
        Ok(output) => {
            return Some(format!(
                "rustc cannot target {target}: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ))
        }
        Err(error) => return Some(format!("running rustc: {error}")),
    };
    let dir = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
    let populated = fs::read_dir(&dir).is_ok_and(|mut entries| entries.next().is_some());
    (!populated).then(|| {
        format!(
            "the active toolchain has no standard library for {target} ({} is missing or empty); cargo check was not run",
            dir.display()
        )
    })
}

fn cargo_check(root: &Path, target: &str, options: &Options) -> Check {
    if let Some(error) = missing_target_std(root, target) {
        return Check {
            artifacts: BTreeMap::new(),
            error: Some(error),
        };
    }
    // cargo resolves a relative --target-dir against the workspace, not our cwd.
    let dir = match &options.check_target_dir {
        Some(dir) => std::path::absolute(dir),
        None => std::path::absolute(root.join("target/formal-coverage")),
    };
    let dir = match dir {
        Ok(dir) => dir,
        Err(error) => {
            return Check {
                artifacts: BTreeMap::new(),
                error: Some(format!("check target directory: {error}")),
            }
        }
    };
    let output = Command::new("cargo")
        .args([
            "check",
            "--frozen",
            "--workspace",
            "--keep-going",
            "--message-format=json",
            "--target",
            target,
            "--target-dir",
        ])
        .arg(&dir)
        .current_dir(root)
        .output();
    let output = match output {
        Ok(output) => output,
        Err(error) => {
            return Check {
                artifacts: BTreeMap::new(),
                error: Some(format!("running cargo check: {error}")),
            }
        }
    };
    let mut artifacts = BTreeMap::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if message["reason"] != "compiler-artifact" {
            continue;
        }
        let library = message["target"]["kind"].as_array().is_some_and(|kinds| {
            kinds.iter().any(|k| {
                ["lib", "rlib", "dylib", "cdylib", "staticlib"].contains(&k.as_str().unwrap_or(""))
            })
        });
        let (Some(id), Some(files)) = (
            message["package_id"].as_str(),
            message["filenames"].as_array(),
        ) else {
            continue;
        };
        if !library {
            continue;
        }
        let rmeta = files
            .iter()
            .filter_map(|f| f.as_str())
            .map(PathBuf::from)
            .find(|f| f.extension().is_some_and(|e| e == "rmeta" || e == "rlib"));
        if let Some(rmeta) = rmeta {
            let stem = rmeta.file_stem().unwrap_or_default().to_string_lossy();
            let stem = stem.strip_prefix("lib").unwrap_or(&stem);
            artifacts.insert(id.to_string(), rmeta.with_file_name(format!("{stem}.d")));
        }
    }
    let error = (!output.status.success()).then(|| {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let errors: Vec<_> = stderr
            .lines()
            .filter(|l| l.starts_with("error"))
            .take(3)
            .collect();
        format!(
            "cargo check --target {target} exited with {}: {}",
            output.status,
            if errors.is_empty() {
                stderr.lines().last().unwrap_or("no output").to_string()
            } else {
                errors.join("; ")
            }
        )
    });
    Check { artifacts, error }
}

/// Source files listed in a Makefile-style dep-info file, in order and without duplicates.
pub(crate) fn read_dep_info(path: &Path, root: &Path) -> Result<Vec<PathBuf>> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut files = Vec::new();
    let mut seen = BTreeSet::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        // Split at the first unescaped ':' that ends the rule target.
        let bytes: Vec<char> = line.chars().collect();
        let mut i = 0;
        let mut colon = None;
        while i < bytes.len() {
            if bytes[i] == '\\' {
                i += 2;
                continue;
            }
            if bytes[i] == ':' && bytes.get(i + 1).is_none_or(|c| *c == ' ') {
                colon = Some(i);
                break;
            }
            i += 1;
        }
        let Some(colon) = colon else {
            continue;
        };
        let mut current = String::new();
        let mut tokens = Vec::new();
        let mut chars = bytes[colon + 1..].iter().peekable();
        while let Some(&c) = chars.next() {
            match c {
                '\\' if chars.peek().is_some_and(|n| **n == ' ' || **n == '\\') => {
                    current.push(*chars.next().unwrap())
                }
                ' ' => {
                    if !current.is_empty() {
                        tokens.push(std::mem::take(&mut current));
                    }
                }
                c => current.push(c),
            }
        }
        if !current.is_empty() {
            tokens.push(current);
        }
        for token in tokens {
            let path = PathBuf::from(&token);
            let path = if path.is_absolute() {
                path
            } else {
                root.join(path)
            };
            if seen.insert(path.clone()) {
                files.push(path);
            }
        }
    }
    Ok(files)
}

fn compiled_size(
    d: &Dependency,
    check: &Check,
    root: &Path,
    closure: &str,
) -> std::result::Result<usize, String> {
    let id = d
        .package_id
        .as_ref()
        .ok_or_else(|| format!("no cargo package id (dependency closure: {closure})"))?;
    let dep_info = check.artifacts.get(id).ok_or_else(|| match &check.error {
        Some(error) => format!("no library artifact from cargo check ({error})"),
        None => "no library artifact from cargo check".to_string(),
    })?;
    if !dep_info.is_file() {
        return Err(format!("dep-info file missing: {}", dep_info.display()));
    }
    let files = read_dep_info(dep_info, root)
        .map_err(|e| format!("dep-info unreadable: {e:#}"))?
        .into_iter()
        .filter(|f| f.extension().is_some_and(|e| e == "rs"))
        .collect::<Vec<_>>();
    if files.is_empty() {
        return Err(format!(
            "dep-info lists no .rs files: {}",
            dep_info.display()
        ));
    }
    let mut total = 0;
    for file in files {
        let source = fs::read_to_string(&file)
            .map_err(|e| format!("dep-info source unreadable: {}: {e}", file.display()))?;
        total += crate::runtime_sloc(&source, &file.to_string_lossy());
    }
    Ok(total)
}

fn compiled_files(root: &Path, options: &Options, report: &mut Report) {
    report.size_profile = "dev (cargo check)".into();
    let pending = report.dependencies.iter().any(|d| d.package_id.is_some());
    let check = if pending {
        cargo_check(root, &report.target, options)
    } else {
        Check {
            artifacts: BTreeMap::new(),
            error: None,
        }
    };
    if let Some(error) = &check.error {
        report.warnings.push(format!(
            "compiled-files: {error}; packages without dep-info keep whole-crate sizes."
        ));
    }
    let closure = report.closure.clone();
    let mut fallbacks = Vec::new();
    for d in &mut report.dependencies {
        match compiled_size(d, &check, root, &closure) {
            Ok(sloc) => {
                d.sloc = sloc;
                d.size_known = true;
                d.size_mode = SizeMode::CompiledFiles.as_str().into();
            }
            Err(reason) => fall_back(d, reason, &mut fallbacks),
        }
    }
    report.size_fallbacks = fallbacks;
}

/// DWARF line rows and symbol-table crate names of one binary.
#[derive(Debug, Clone, Default)]
pub struct DwarfLines {
    pub sha256: String,
    /// Distinct lines (> 0) per source path as recorded in the line table.
    pub lines: BTreeMap<String, BTreeSet<u64>>,
    /// First path segments of demangled Rust symbols defined in the binary.
    pub symbol_crates: BTreeSet<String>,
    pub defined_symbols: usize,
    /// Which objects supplied the DWARF (the binary, an adjacent or a fresh dSYM).
    pub debug_sources: Vec<String>,
}

fn add_lines(data: &[u8], out: &mut BTreeMap<String, BTreeSet<u64>>) -> Result<bool> {
    let file = object::File::parse(data)?;
    if file.section_by_name(".debug_line").is_none() {
        return Ok(false);
    }
    let endian = if file.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };
    let load = |id: gimli::SectionId| -> std::result::Result<Cow<[u8]>, gimli::Error> {
        match file.section_by_name(id.name()) {
            Some(section) => section.uncompressed_data().map_err(|_| gimli::Error::Io),
            None => Ok(Cow::Borrowed(&[][..])),
        }
    };
    let sections = gimli::DwarfSections::load(load)?;
    let dwarf = sections.borrow(|section| gimli::EndianSlice::new(section, endian));
    let mut units = dwarf.units();
    let mut any = false;
    while let Some(header) = units.next()? {
        let unit = dwarf.unit(header)?;
        let Some(program) = unit.line_program.clone() else {
            continue;
        };
        let mut names: HashMap<u64, String> = HashMap::new();
        let mut rows = program.rows();
        while let Some((header, row)) = rows.next_row()? {
            if row.end_sequence() {
                continue;
            }
            let Some(line) = row.line() else {
                continue;
            };
            let index = row.file_index();
            if let std::collections::hash_map::Entry::Vacant(slot) = names.entry(index) {
                let Some(entry) = row.file(header) else {
                    continue;
                };
                let mut path = PathBuf::new();
                if let Some(dir) = unit.comp_dir {
                    path.push(dir.to_string_lossy().as_ref());
                }
                // Directory index 0 is the compilation directory.
                if entry.directory_index() != 0 {
                    if let Some(dir) = entry.directory(header) {
                        path.push(dwarf.attr_string(&unit, dir)?.to_string_lossy().as_ref());
                    }
                }
                path.push(
                    dwarf
                        .attr_string(&unit, entry.path_name())?
                        .to_string_lossy()
                        .as_ref(),
                );
                slot.insert(path.to_string_lossy().into_owned());
            }
            out.entry(names[&index].clone())
                .or_default()
                .insert(line.get());
            any = true;
        }
    }
    Ok(any)
}

/// Crate names at path starts in a demangled symbol: `a::..`, `<a::T as b::Tr>::..`.
fn symbol_crates(demangled: &str, out: &mut BTreeSet<String>) {
    let mut starts = vec![0];
    for (i, c) in demangled.char_indices() {
        if c == '<' {
            starts.push(i + 1);
        }
    }
    starts.extend(demangled.match_indices(" as ").map(|(i, _)| i + 4));
    for start in starts {
        let rest = &demangled[start..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !name.is_empty() && rest[name.len()..].starts_with("::") {
            out.insert(name);
        }
    }
}

fn dsym_dwarf(dsym: &Path, binary: &Path) -> PathBuf {
    dsym.join("Contents/Resources/DWARF")
        .join(binary.file_name().unwrap_or_default())
}

/// Read the DWARF line table of `binary`; on Mach-O, also from a UUID-matched dSYM.
pub fn dwarf_lines(binary: &Path) -> Result<DwarfLines> {
    let data = fs::read(binary).with_context(|| format!("reading {}", binary.display()))?;
    let mut result = DwarfLines {
        sha256: crate::sha256(&data),
        ..DwarfLines::default()
    };
    let file = object::File::parse(&*data).context("parsing binary")?;
    for symbol in file.symbols().filter(|s| s.is_definition()) {
        let Ok(name) = symbol.name() else {
            continue;
        };
        result.defined_symbols += 1;
        if let Ok(demangled) = rustc_demangle::try_demangle(name) {
            symbol_crates(&format!("{demangled:#}"), &mut result.symbol_crates);
        }
    }
    if add_lines(&data, &mut result.lines)? {
        result.debug_sources.push("binary".into());
    }
    if file.format() == object::BinaryFormat::MachO {
        let uuid = file.mach_uuid().ok().flatten();
        let fresh = std::env::temp_dir().join(format!(
            "formal-coverage-{}-{}.dSYM",
            std::process::id(),
            &result.sha256[..16]
        ));
        let ran = Command::new("dsymutil")
            .arg(binary)
            .arg("-o")
            .arg(&fresh)
            .output()
            .is_ok_and(|o| o.status.success());
        let mut adjacent = binary.as_os_str().to_owned();
        adjacent.push(".dSYM");
        let candidates = [
            (PathBuf::from(adjacent), "adjacent dSYM"),
            (fresh.clone(), "fresh dsymutil"),
        ];
        for (dsym, label) in candidates {
            if label == "fresh dsymutil" && !ran {
                continue;
            }
            let Ok(bytes) = fs::read(dsym_dwarf(&dsym, binary)) else {
                continue;
            };
            let Ok(parsed) = object::File::parse(&*bytes) else {
                continue;
            };
            // A stale dSYM describes another build: use only a UUID match.
            if uuid.is_none() || parsed.mach_uuid().ok().flatten() != uuid {
                continue;
            }
            if add_lines(&bytes, &mut result.lines)? {
                result.debug_sources.push(label.into());
            }
        }
        let _ = fs::remove_dir_all(&fresh);
    }
    Ok(result)
}

/// Packages whose debug info a root-manifest profile override turns off.
fn debug_disabled(root: &Path) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let Ok(text) = fs::read_to_string(root.join("Cargo.toml")) else {
        return out;
    };
    let Ok(manifest) = toml::from_str::<toml::Value>(&text) else {
        return out;
    };
    for profile in manifest
        .get("profile")
        .and_then(|p| p.as_table())
        .into_iter()
        .flat_map(|t| t.values())
    {
        for (name, settings) in profile
            .get("package")
            .and_then(|p| p.as_table())
            .into_iter()
            .flatten()
        {
            let on = match settings.get("debug") {
                None => true,
                Some(toml::Value::Boolean(b)) => *b,
                Some(toml::Value::Integer(n)) => *n > 0,
                Some(toml::Value::String(s)) => {
                    ["limited", "full", "line-tables-only"].contains(&s.as_str())
                }
                Some(_) => false,
            };
            if !on {
                out.insert(name.clone());
            }
        }
    }
    out
}

fn linked_lines(root: &Path, options: &Options, report: &mut Report) -> Result<()> {
    let binary = options
        .binary
        .as_ref()
        .context("--size-mode linked-lines requires --binary <path>")?;
    report.size_profile = format!(
        "{} (directory of the supplied binary)",
        binary
            .parent()
            .and_then(Path::file_name)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".into())
    );
    report.binary_sha256 = fs::read(binary).ok().map(|d| crate::sha256(&d));
    let mut fallbacks = Vec::new();
    let table = match dwarf_lines(binary) {
        Ok(table) => table,
        Err(error) => {
            for d in &mut report.dependencies {
                fall_back(
                    d,
                    format!("binary debug info unreadable: {error:#}"),
                    &mut fallbacks,
                );
            }
            report.size_fallbacks = fallbacks;
            return Ok(());
        }
    };
    let canonical_root = root.canonicalize()?;
    let first_party: BTreeSet<String> = report
        .units
        .iter()
        .flat_map(|u| {
            [
                canonical_root.join(&u.file).to_string_lossy().into_owned(),
                root.join(&u.file).to_string_lossy().into_owned(),
            ]
        })
        .collect();
    if !table.lines.keys().any(|f| first_party.contains(f)) {
        for d in &mut report.dependencies {
            fall_back(
                d,
                "binary has no DWARF line rows for first-party sources (built without debug info?)"
                    .into(),
                &mut fallbacks,
            );
        }
        report.size_fallbacks = fallbacks;
        return Ok(());
    }
    // Each line-table file belongs to the dependency with the longest matching source directory.
    let dirs: Vec<Vec<String>> = report
        .dependencies
        .iter()
        .map(|d| {
            d.source
                .iter()
                .flat_map(|s| {
                    let path = PathBuf::from(s);
                    let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
                    [path, canonical]
                })
                .map(|p| format!("{}/", p.to_string_lossy().trim_end_matches('/')))
                .collect()
        })
        .collect();
    let mut counts = vec![0usize; report.dependencies.len()];
    for (file, lines) in &table.lines {
        let mut best: Option<(usize, usize)> = None;
        for (i, prefixes) in dirs.iter().enumerate() {
            for prefix in prefixes {
                if file.starts_with(prefix.as_str())
                    && best.is_none_or(|(_, len)| prefix.len() > len)
                {
                    best = Some((i, prefix.len()));
                }
            }
        }
        // Binaries built under another CARGO_HOME keep the registry `name-version/` component.
        let owner = best.map(|(i, _)| i).or_else(|| {
            report
                .dependencies
                .iter()
                .position(|d| file.contains(&format!("/{}-{}/", d.name, d.version)))
        });
        if let Some(i) = owner {
            counts[i] += lines.len();
        }
    }
    let disabled = debug_disabled(root);
    for (d, count) in report.dependencies.iter_mut().zip(counts) {
        if disabled.contains(&d.name) || disabled.contains("*") {
            fall_back(
                d,
                "a root-manifest profile override disables debug info for this package".into(),
                &mut fallbacks,
            );
        } else if count > 0 {
            d.sloc = count;
            d.size_known = true;
            d.size_mode = SizeMode::LinkedLines.as_str().into();
        } else if table.defined_symbols == 0 {
            fall_back(
                d,
                "no line rows, and no symbol table to confirm the package is absent".into(),
                &mut fallbacks,
            );
        } else if d
            .lib_name
            .as_ref()
            .is_some_and(|n| table.symbol_crates.contains(n))
        {
            fall_back(
                d,
                "symbols from this package are linked but have no line rows (debug info missing?)"
                    .into(),
                &mut fallbacks,
            );
        } else if d.lib_name.is_none() {
            fall_back(
                d,
                "no line rows and no library target name to check symbols".into(),
                &mut fallbacks,
            );
        } else {
            d.sloc = 0;
            d.size_known = true;
            d.size_mode = SizeMode::LinkedLines.as_str().into();
            d.size_reason =
                Some("no line rows and no symbols from this package in the binary".into());
        }
    }
    report.size_fallbacks = fallbacks;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dep_info_paths_and_symbol_crates() {
        let dir = std::env::temp_dir().join(format!("fc-depinfo-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("x.d");
        fs::write(&path, "/t/libx-1.rmeta: /a/src/lib.rs /a/src/with\\ space.rs gen.rs\n\n/a/src/lib.rs:\n# env-dep:X=1\n").unwrap();
        let files = read_dep_info(&path, Path::new("/root")).unwrap();
        assert_eq!(
            files,
            [
                PathBuf::from("/a/src/lib.rs"),
                PathBuf::from("/a/src/with space.rs"),
                PathBuf::from("/root/gen.rs")
            ]
        );
        fs::remove_dir_all(dir).unwrap();
        let mut crates = BTreeSet::new();
        symbol_crates(
            "<typenum::uint::UInt<U, B> as core::ops::Add>::add",
            &mut crates,
        );
        symbol_crates("libc::unix::write", &mut crates);
        assert_eq!(
            crates.into_iter().collect::<Vec<_>>(),
            ["core", "libc", "typenum"]
        );
    }
}
