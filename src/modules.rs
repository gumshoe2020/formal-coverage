//! Conservative source module closure for a complete Verus invocation.
use crate::Receipt;
use anyhow::Result;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::{collections::BTreeSet, fs, path::Path};

pub(crate) fn closure(
    root: &Path,
    receipt: &Receipt,
    warnings: &mut Vec<String>,
) -> Result<BTreeSet<String>> {
    let mut covered = BTreeSet::new();
    let file = Path::new(crate::evidence::verus_root(receipt).unwrap());
    visit(
        root,
        file,
        file.parent().unwrap(),
        receipt,
        &mut covered,
        warnings,
    )?;
    Ok(covered)
}
fn visit(
    root: &Path,
    file: &Path,
    directory: &Path,
    receipt: &Receipt,
    covered: &mut BTreeSet<String>,
    warnings: &mut Vec<String>,
) -> Result<()> {
    let name = file.to_string_lossy().to_string();
    if !receipt.files.contains_key(&name) {
        warnings.push(format!(
            "module {}: missing receipt hash; unverified",
            file.display()
        ));
        return Ok(());
    }
    if covered.contains(&name) {
        return Ok(());
    }
    let source = fs::read_to_string(root.join(file))?;
    let stream: TokenStream = source.parse().map_err(|e| anyhow::anyhow!("{e}"))?;
    if !crate::cfg::enabled(
        &crate::cfg::attributes(stream.clone(), true).to_string(),
        receipt,
        &name,
        warnings,
    ) {
        return Ok(());
    }
    covered.insert(name);
    declarations(root, stream, directory, receipt, covered, warnings)
}
fn declarations(
    root: &Path,
    stream: TokenStream,
    directory: &Path,
    receipt: &Receipt,
    covered: &mut BTreeSet<String>,
    warnings: &mut Vec<String>,
) -> Result<()> {
    if !crate::cfg::enabled(
        &crate::cfg::attributes(stream.clone(), true).to_string(),
        receipt,
        &directory.display().to_string(),
        warnings,
    ) {
        return Ok(());
    }
    let tokens: Vec<_> = stream.into_iter().collect();
    let mut start = 0;
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i].to_string() == "mod" {
            if let (Some(TokenTree::Ident(name)), Some(body)) =
                (tokens.get(i + 1), tokens.get(i + 2))
            {
                let name = name.to_string().trim_start_matches("r#").to_string();
                let prefix = tokens[start..i]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                let words: BTreeSet<_> = prefix
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .collect();
                if ["path", "external", "external_body"]
                    .iter()
                    .any(|a| words.contains(a))
                    || !crate::cfg::enabled(
                        &prefix,
                        receipt,
                        &format!("module {}::{name}", directory.display()),
                        warnings,
                    )
                {
                    warnings.push(format!(
                        "module {}::{}: excluded path/cfg/verifier scope; unverified",
                        directory.display(),
                        name
                    ));
                } else if body.to_string() == ";" {
                    let flat = directory.join(format!("{name}.rs"));
                    let nested = directory.join(&name).join("mod.rs");
                    let options: Vec<_> = [flat, nested]
                        .into_iter()
                        .filter(|p| root.join(p).is_file())
                        .collect();
                    if options.len() == 1 {
                        visit(
                            root,
                            &options[0],
                            &directory.join(&name),
                            receipt,
                            covered,
                            warnings,
                        )?;
                    } else {
                        warnings.push(format!(
                            "module {}::{}: missing or ambiguous Rust module file; unverified",
                            directory.display(),
                            name
                        ));
                    }
                } else if let TokenTree::Group(g) = body {
                    if g.delimiter() == Delimiter::Brace {
                        declarations(
                            root,
                            g.stream(),
                            &directory.join(&name),
                            receipt,
                            covered,
                            warnings,
                        )?;
                    }
                }
                i += 3;
                start = i;
                continue;
            }
        }
        if let TokenTree::Group(g) = &tokens[i] {
            // Verus item macros are transparent; arbitrary macros/functions aren't module declarations.
            if g.delimiter() == Delimiter::Brace
                && i >= 2
                && tokens[i - 1].to_string() == "!"
                && tokens[i - 2].to_string() == "verus"
                && crate::cfg::enabled(
                    &tokens[start..i]
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                    receipt,
                    &directory.display().to_string(),
                    warnings,
                )
            {
                declarations(root, g.stream(), directory, receipt, covered, warnings)?;
            }
            if g.delimiter() == Delimiter::Brace {
                start = i + 1;
            }
        }
        if tokens[i].to_string() == ";" {
            start = i + 1;
        }
        i += 1;
    }
    Ok(())
}
