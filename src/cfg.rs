//! Receipt-local cfg evaluation. Unknown syntax excludes only its owner.
use crate::Receipt;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::BTreeSet;

fn normalized(s: &str) -> String {
    s.parse::<TokenStream>()
        .map(|s| s.into_iter().map(|t| t.to_string()).collect::<String>())
        .unwrap_or_else(|_| s.to_string())
}
pub(crate) fn values(r: &Receipt) -> BTreeSet<String> {
    let mut values: BTreeSet<_> = r
        .cfg
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|s| normalized(s))
        .collect();
    for pair in r.command.windows(2) {
        if pair[0] == "--cfg" {
            values.insert(normalized(&pair[1]));
        }
    }
    for arg in &r.command {
        if let Some(s) = arg.strip_prefix("--cfg=") {
            values.insert(normalized(s));
        }
    }
    // Pinned d17fbd0 source/rust_verify/src/driver.rs:289-291.
    if r.tool == "verus" {
        values
            .extend(["verus_only", "verus_keep_ghost", "verus_keep_ghost_body"].map(String::from));
    } else if r.tool == "kani" {
        values.insert("kani".into());
    }
    values
}
fn parts(stream: TokenStream) -> Vec<TokenStream> {
    let mut out = vec![TokenStream::new()];
    for t in stream {
        if t.to_string() == "," {
            out.push(TokenStream::new());
        } else {
            out.last_mut().unwrap().extend([t]);
        }
    }
    if out.last().is_some_and(TokenStream::is_empty) {
        out.pop();
    }
    out
}
fn predicate(stream: TokenStream, set: &BTreeSet<String>) -> Option<bool> {
    let ts: Vec<_> = stream.clone().into_iter().collect();
    match ts.as_slice() {
        [TokenTree::Ident(name)] => {
            let name = name.to_string();
            if (name.starts_with("verus_")
                && !["verus_only", "verus_keep_ghost", "verus_keep_ghost_body"]
                    .contains(&name.as_str()))
                || ["unix", "windows", "debug_assertions", "proc_macro"].contains(&name.as_str())
            {
                if set.contains(&name) {
                    Some(true)
                } else {
                    None
                }
            } else {
                Some(set.contains(&name))
            }
        }
        [TokenTree::Ident(name), TokenTree::Punct(eq), TokenTree::Literal(_)]
            if eq.as_char() == '=' =>
        {
            let key = normalized(&stream.to_string());
            if name == "feature" || set.contains(&key) {
                Some(set.contains(&key))
            } else {
                None
            }
        }
        [TokenTree::Ident(op), TokenTree::Group(g)] if g.delimiter() == Delimiter::Parenthesis => {
            let args: Vec<_> = parts(g.stream())
                .into_iter()
                .map(|p| predicate(p, set))
                .collect();
            match op.to_string().as_str() {
                "all" => {
                    if args.contains(&Some(false)) {
                        Some(false)
                    } else if args.iter().all(Option::is_some) {
                        Some(true)
                    } else {
                        None
                    }
                }
                "any" => {
                    if args.contains(&Some(true)) {
                        Some(true)
                    } else if args.iter().all(Option::is_some) {
                        Some(false)
                    } else {
                        None
                    }
                }
                "not" if args.len() == 1 => args[0].map(|b| !b),
                _ => None,
            }
        }
        _ => None,
    }
}
pub(crate) fn attributes(stream: TokenStream, inner_only: bool) -> TokenStream {
    let ts: Vec<_> = stream.into_iter().collect();
    let mut out = TokenStream::new();
    for i in 0..ts.len() {
        if ts[i].to_string() != "#" {
            continue;
        }
        let inner = ts.get(i + 1).is_some_and(|t| t.to_string() == "!");
        if inner_only && !inner {
            continue;
        }
        if let Some(TokenTree::Group(g)) = ts.get(i + 1 + usize::from(inner)) {
            if g.delimiter() == Delimiter::Bracket {
                out.extend([
                    TokenTree::Punct(proc_macro2::Punct::new('#', proc_macro2::Spacing::Alone)),
                    TokenTree::Group(g.clone()),
                ]);
            }
        }
    }
    out
}
fn attribute(stream: TokenStream, set: &BTreeSet<String>) -> Option<bool> {
    let ts: Vec<_> = stream.into_iter().collect();
    let Some(TokenTree::Ident(name)) = ts.first() else {
        return Some(true);
    };
    if name != "cfg" && name != "cfg_attr" {
        return Some(true);
    }
    let Some(TokenTree::Group(g)) = ts.get(1) else {
        return None;
    };
    if name == "cfg" {
        return predicate(g.stream(), set);
    }
    let mut args = parts(g.stream()).into_iter();
    let pred = predicate(args.next()?, set);
    let mut result = Some(true);
    for added in args {
        // These attributes cannot change which items exist, irrespective of the predicate.
        let first = added.clone().into_iter().next()?.to_string();
        if ["no_std", "feature", "allow", "warn", "deny", "doc"].contains(&first.as_str()) {
            continue;
        }
        let active = match pred {
            Some(false) => Some(true),
            Some(true) if ["cfg", "cfg_attr"].contains(&first.as_str()) => attribute(added, set),
            None if ["cfg", "cfg_attr"].contains(&first.as_str())
                && attribute(added, set) == Some(true) =>
            {
                Some(true)
            }
            _ => None,
        };
        result = match (result, active) {
            (Some(false), _) | (_, Some(false)) => Some(false),
            (Some(true), Some(true)) => Some(true),
            _ => None,
        };
    }
    result
}
pub(crate) fn enabled(attrs: &str, r: &Receipt, owner: &str, warnings: &mut Vec<String>) -> bool {
    let Ok(stream) = attrs.parse::<TokenStream>() else {
        warnings.push(format!("{owner}: cfg syntax unconfirmed; unverified"));
        return false;
    };
    let set = values(r);
    for t in attributes(stream, false) {
        if let TokenTree::Group(g) = t {
            match attribute(g.stream(), &set) {
                Some(true) => {}
                Some(false) => return false,
                None => {
                    warnings.push(format!("{owner}: unevaluable cfg predicate/attribute {}; scope unconfirmed; unverified",g.stream()));
                    return false;
                }
            }
        }
    }
    true
}

/// Recover cfg owners for non-function items (notably assume_specification).
pub(crate) fn site_attributes(source: &str, line: usize, kind: &str) -> String {
    fn visit(stream: TokenStream, line: usize, kind: &str, inherited: &str) -> Option<String> {
        let inner = attributes(stream.clone(), true).to_string();
        let inherited = format!("{inherited} {inner}");
        let ts: Vec<_> = stream.into_iter().collect();
        let mut start = 0;
        for (i, t) in ts.iter().enumerate() {
            let attrs = format!(
                "{} {}",
                inherited,
                attributes(ts[start..=i].iter().cloned().collect(), false)
            );
            if matches!(t, TokenTree::Ident(_))
                && t.to_string() == kind
                && t.span().start().line == line
            {
                return Some(attrs);
            }
            if let TokenTree::Group(g) = t {
                if g.span().start().line <= line && line <= g.span().end().line {
                    if let Some(found) = visit(g.stream(), line, kind, &attrs) {
                        return Some(found);
                    }
                }
                if g.delimiter() == Delimiter::Brace {
                    start = i + 1;
                }
            }
            if t.to_string() == ";" {
                start = i + 1;
            }
        }
        None
    }
    source
        .parse()
        .ok()
        .and_then(|s| visit(s, line, kind, ""))
        .unwrap_or_default()
}

pub(crate) fn has_attribute(attrs: &str, expected: &str) -> bool {
    attrs.parse::<TokenStream>().is_ok_and(|s| {
        attributes(s, false)
            .into_iter()
            .any(|t| matches!(t,TokenTree::Group(g) if g.stream().to_string() == expected))
    })
}
