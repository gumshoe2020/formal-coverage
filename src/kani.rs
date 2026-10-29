//! Closed serial Kani receipt grammar and scope from hash-validated attributes.
use crate::Unit;
use anyhow::{bail, ensure, Result};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::BTreeSet;

pub(crate) struct Command<'a> {
    pub harness: &'a str,
    unwind: Option<usize>,
    features: BTreeSet<&'a str>,
}

fn identifier_path(value: &str) -> bool {
    value.split("::").all(|part| {
        let mut chars = part.chars();
        chars.next().is_some_and(|c| c == '_' || c.is_alphabetic())
            && chars.all(|c| c == '_' || c.is_alphanumeric())
    })
}

pub(crate) fn command(args: &[String]) -> Option<Command<'_>> {
    if args.first()? != "cargo" || args.get(1)? != "kani" {
        return None;
    }
    let mut harness = None;
    let mut unwind = None;
    let mut features = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut i = 2;
    while i < args.len() {
        let arg = args[i].as_str();
        if let Some(value) = arg.strip_prefix("--output-format=") {
            if !seen.insert("--output-format") || !["regular", "terse"].contains(&value) {
                return None;
            }
            i += 1;
            continue;
        }
        let value = args.get(i + 1)?.as_str();
        match arg {
            "-Z" => {
                if ![
                    "unstable-options",
                    "stubbing",
                    "function-contracts",
                    "restrict-vtable",
                ]
                .contains(&value)
                    || !features.insert(value)
                {
                    return None;
                }
            }
            "--harness" if seen.insert("--harness") && identifier_path(value) => {
                harness = Some(value)
            }
            "--unwind" if seen.insert("--unwind") => {
                let bound = value.parse::<usize>().ok()?;
                if bound == 0 {
                    return None;
                }
                unwind = Some(bound);
            }
            "--output-format"
                if seen.insert("--output-format") && ["regular", "terse"].contains(&value) => {}
            "--package" | "-p"
                if seen.insert("--package")
                    && !value.is_empty()
                    && !value.starts_with('-')
                    && value
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_' || c == '-') => {}
            "--harness-timeout" if seen.insert("--harness-timeout") => {
                let number = value.trim_end_matches(['s', 'm', 'h']);
                if number.is_empty()
                    || value.len() - number.len() > 1
                    || !number.bytes().all(|c| c.is_ascii_digit())
                    || number.parse::<u64>().ok()? == 0
                {
                    return None;
                }
            }
            _ => return None,
        }
        i += 2;
    }
    Some(Command {
        harness: harness?,
        unwind,
        features,
    })
}

fn attributes(text: &str) -> Result<Vec<(String, Option<TokenStream>)>> {
    let stream: TokenStream = text
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid harness attributes"))?;
    let mut result = Vec::new();
    for token in crate::cfg::attributes(stream, false) {
        let TokenTree::Group(group) = token else {
            continue;
        };
        if group.delimiter() != Delimiter::Bracket {
            continue;
        }
        let ts: Vec<_> = group.stream().into_iter().collect();
        let normalized = group.stream().to_string().replace(' ', "");
        ensure!(
            !(normalized.starts_with("cfg_attr(") && normalized.contains("kani::")),
            "conditional Kani scope is unconfirmed"
        );
        if ts.len() < 4
            || ts[0].to_string() != "kani"
            || ts[1].to_string() != ":"
            || ts[2].to_string() != ":"
        {
            continue;
        }
        let args = match &ts[4..] {
            [] => None,
            [TokenTree::Group(g)] if g.delimiter() == Delimiter::Parenthesis => Some(g.stream()),
            _ => bail!("unsupported Kani attribute syntax"),
        };
        result.push((ts[3].to_string(), args));
    }
    Ok(result)
}

pub(crate) fn is_harness(text: &str) -> bool {
    attributes(text).is_ok_and(|attrs| {
        attrs
            .iter()
            .any(|(name, _)| ["proof", "proof_for_contract"].contains(&name.as_str()))
    })
}

/// Resolve the logged pretty name or legacy crate-qualified name, without
/// allowing substring filters that also select a different source harness.
pub(crate) fn harness<'a>(id: &str, units: &'a [Unit]) -> Option<&'a Unit> {
    let candidates: Vec<_> = units
        .iter()
        .filter(|u| u.kani_harness)
        .filter(|u| {
            u.id.contains(id)
                || u.id
                    .strip_prefix(&format!("{}::", u.crate_name))
                    .is_some_and(|s| s.contains(id))
        })
        .collect();
    if candidates.len() != 1 {
        return None;
    }
    let unit = candidates[0];
    (unit.id == id || unit.id.strip_prefix(&format!("{}::", unit.crate_name)) == Some(id))
        .then_some(unit)
}

pub(crate) fn scope(args: &[String], attrs: &str, id: &str) -> Result<String> {
    let cmd = command(args).ok_or_else(|| anyhow::anyhow!("unsupported Kani command"))?;
    let mut source_unwind = None;
    let mut assumptions = Vec::new();
    let mut contract = None;
    for (name, params) in attributes(attrs)? {
        if name == "proof" || name == "solver" {
            continue;
        }
        ensure!(
            name != "should_panic",
            "expected-panic harness is not safety evidence"
        );
        let params = params.ok_or_else(|| anyhow::anyhow!("missing Kani attribute arguments"))?;
        let mut parts = vec![TokenStream::new()];
        for t in params {
            if t.to_string() == "," {
                parts.push(TokenStream::new());
            } else {
                parts.last_mut().unwrap().extend([t]);
            }
        }
        let parts: Vec<_> = parts
            .iter()
            .map(|s| s.to_string().replace(' ', ""))
            .collect();
        ensure!(
            parts.iter().all(|s| !s.is_empty()),
            "empty Kani scope argument"
        );
        match (name.as_str(), parts.as_slice()) {
            ("unwind", [n]) => {
                ensure!(source_unwind.is_none(), "duplicate source unwind");
                source_unwind = Some(n.parse::<usize>()?);
            }
            ("stub", [a, b]) => {
                ensure!(
                    identifier_path(a) && identifier_path(b),
                    "unsupported Kani stub path"
                );
                assumptions.push(format!("{a} replaced by stub {b}"));
            }
            ("stub_verified", [f]) => {
                ensure!(identifier_path(f), "unsupported verified-stub path");
                assumptions.push(format!("{f} assumed to meet its contract"));
            }
            ("proof_for_contract", [f]) => {
                ensure!(identifier_path(f), "unsupported contract function path");
                ensure!(contract.is_none(), "duplicate contract harness scope");
                contract = Some(format!("{f}'s contract (requires/ensures)"));
            }
            _ => bail!("unsupported Kani scope attribute {name}"),
        }
    }
    let bound = match cmd.unwind.or(source_unwind) {
        Some(n) => format!("unwind bound {n}"),
        None => "Kani default: no unwind limit, unwinding assertions on".into(),
    };
    let mut scope =
        format!("Kani harness {id}: {bound}; assertions under the harness input model only");
    if let Some(contract) = contract {
        scope.push_str(&format!("; scope: {contract}"));
    }
    if cmd.features.contains("restrict-vtable") {
        assumptions.push("dyn calls restricted to type-compatible vtable targets".into());
    }
    for assumption in assumptions {
        scope.push_str(&format!("; {assumption}"));
    }
    Ok(scope)
}
