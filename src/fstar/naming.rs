//! Conservative v1.9.1 fallbacks. No suffix/name-only trait matching.
use super::{mapped, numbered_mappings, words, Decl};
use crate::{Report, Unit};
use std::{collections::BTreeMap, fs, path::Path};

type Decision = Result<Vec<String>, String>;

pub(super) fn trailing(name: &str) -> Option<&str> {
    let plain = name.strip_suffix('_')?;
    let (_, digits) = plain.rsplit_once('_')?;
    (!digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())).then_some(plain)
}

fn rust_module(module: &str) -> String {
    module
        .split('.')
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join("::")
}

pub(super) fn instance(name: &str) -> bool {
    name == "impl"
        || name
            .strip_prefix("impl_")
            .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

// A small type grammar: qualified atoms and parenthesized applications only.
#[derive(Debug, PartialEq, Eq)]
struct Ty(String, Vec<Ty>);
fn fstar_types(text: &str, module: &str) -> Option<Vec<Ty>> {
    fn parse(tokens: &[String], i: &mut usize, module: &str, nested: bool) -> Option<Vec<Ty>> {
        let mut out = vec![];
        while *i < tokens.len() && tokens[*i] != ")" {
            if tokens[*i] == "(" {
                *i += 1;
                let mut app = parse(tokens, i, module, true)?;
                if tokens.get(*i)? != ")" || app.is_empty() {
                    return None;
                }
                *i += 1;
                let mut head = app.remove(0);
                head.1.extend(app);
                out.push(head);
            } else {
                let atom = &tokens[*i];
                if !atom
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
                {
                    return None;
                }
                let (path, name) = atom.rsplit_once('.').unwrap_or(("", atom));
                let name = name.strip_prefix("t_").unwrap_or(name);
                let canonical = match (path, name) {
                    ("Core_models.Convert" | "", "From" | "TryFrom") => {
                        format!("core::convert::{name}")
                    }
                    ("Core_models.Clone" | "", "Clone") => "core::clone::Clone".into(),
                    ("Core_models.Ops.Index", "Index") => "core::ops::Index".into(),
                    ("Alloc.Vec", "Vec") => "alloc::vec::Vec".into(),
                    ("Alloc.Alloc", "Global") => "alloc::alloc::Global".into(),
                    ("", "u8" | "u16" | "u32" | "u64" | "usize" | "bool") => name.into(),
                    ("", _) => format!("{}::{name}", rust_module(module)),
                    (_, _)
                        if path
                            .split('.')
                            .next()?
                            .eq_ignore_ascii_case(module.split('.').next()?) =>
                    {
                        format!("{}::{name}", rust_module(path))
                    }
                    _ => return None,
                };
                out.push(Ty(canonical, vec![]));
                *i += 1;
            }
        }
        if !nested && *i != tokens.len() {
            return None;
        }
        Some(out)
    }
    let spaced = text.replace('(', " ( ").replace(')', " ) ");
    parse(
        &spaced
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>(),
        &mut 0,
        module,
        false,
    )
}

fn imports(tree: &syn::UseTree, prefix: &str, out: &mut BTreeMap<String, Vec<String>>) {
    match tree {
        syn::UseTree::Path(p) => imports(&p.tree, &format!("{prefix}{}::", p.ident), out),
        syn::UseTree::Name(n) => out
            .entry(n.ident.to_string())
            .or_default()
            .push(format!("{prefix}{}", n.ident)),
        syn::UseTree::Rename(n) => out
            .entry(n.rename.to_string())
            .or_default()
            .push(format!("{prefix}{}", n.ident)),
        syn::UseTree::Group(g) => {
            for t in &g.items {
                imports(t, prefix, out);
            }
        }
        syn::UseTree::Glob(_) => {} // Never infer a symbol from a wildcard.
    }
}
fn absolute(path: &str, module: &str) -> Option<String> {
    let krate = module.split("::").next()?;
    if let Some(rest) = path.strip_prefix("crate::") {
        return Some(format!("{krate}::{rest}"));
    }
    if let Some(rest) = path.strip_prefix("self::") {
        return Some(format!("{module}::{rest}"));
    }
    if let Some(rest) = path.strip_prefix("super::") {
        return absolute(&format!("self::{rest}"), module.rsplit_once("::")?.0);
    }
    if path.starts_with("core::") || path.starts_with("alloc::") {
        return Some(path.into());
    }
    Some(format!("{module}::{path}"))
}
fn rust_type(ty: &syn::Type, module: &str, uses: &BTreeMap<String, Vec<String>>) -> Option<Ty> {
    let syn::Type::Path(p) = ty else {
        return None;
    };
    if p.qself.is_some() {
        return None;
    }
    let mut segments = p.path.segments.iter();
    let first = segments.next()?;
    let mut name = first.ident.to_string();
    let external = ["core", "alloc"].contains(&name.as_str());
    if let Some(paths) = uses.get(&name) {
        if paths.len() != 1 {
            return None;
        }
        name = absolute(&paths[0], module)?;
    } else if ["From", "TryFrom"].contains(&name.as_str()) {
        name = format!("core::convert::{name}");
    } else if name == "Clone" {
        name = "core::clone::Clone".into();
    } else if !external && !["u8", "u16", "u32", "u64", "usize", "bool"].contains(&name.as_str()) {
        name = absolute(&name, module)?;
    }
    let mut args = vec![];
    for (index, segment) in std::iter::once(first).chain(segments).enumerate() {
        if index > 0 {
            name.push_str(&format!("::{}", segment.ident));
        }
        match &segment.arguments {
            syn::PathArguments::None => {}
            syn::PathArguments::AngleBracketed(a) => {
                for arg in &a.args {
                    let syn::GenericArgument::Type(t) = arg else {
                        return None;
                    };
                    args.push(rust_type(t, module, uses)?);
                }
            }
            _ => return None,
        }
    }
    Some(Ty(name, args))
}

fn fields(text: &str) -> Option<Vec<String>> {
    // Lex top-level record assignments, ignoring nested method bodies and pre/post fields.
    let text = text.trim();
    let text = text.strip_prefix('{')?.trim_end().strip_suffix('}')?;
    let mut depth = 0i32;
    let mut start = 0;
    let mut names = vec![];
    let mut pieces = vec![];
    for (i, c) in text.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ';' if depth == 0 => {
                pieces.push(&text[start..i]);
                start = i + 1;
            }
            _ => {}
        }
        if depth < 0 {
            return None;
        }
    }
    if depth != 0 {
        return None;
    }
    pieces.push(&text[start..]);
    for piece in pieces.into_iter().filter(|p| !p.trim().is_empty()) {
        let (key, body) = piece.split_once('=')?;
        let key = key.trim();
        if body.trim().is_empty() || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return None;
        }
        if let Some(method) = key.strip_prefix("f_") {
            if !method.ends_with("_pre")
                && !method.ends_with("_post")
                && method
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_lowercase)
            {
                if names.iter().any(|n| n == method) {
                    return None;
                }
                names.push(method.into());
            }
        }
    }
    Some(names)
}

// Recover the file namespace from an inventory-bound function token and its
// syntactic enclosing scopes. Unit.module also contains trait/inherent impl headers.
fn file_module(source: &str, file: &str, report: &Report) -> Option<String> {
    fn visit(
        stream: proc_macro2::TokenStream,
        scopes: &[String],
        file: &str,
        report: &Report,
        roots: &mut std::collections::BTreeSet<String>,
    ) {
        use proc_macro2::{Delimiter, TokenTree};
        let tokens: Vec<_> = stream.into_iter().collect();
        let mut start = 0;
        for (i, token) in tokens.iter().enumerate() {
            if token.to_string() == "fn" {
                if let Some(TokenTree::Ident(name)) = tokens.get(i + 1) {
                    for u in report.units.iter().filter(|u| {
                        u.file == file
                            && *name == u.name
                            && u.line <= token.span().start().line
                            && token.span().start().line <= u.end_line
                    }) {
                        let namespace = format!(
                            "{}{}{}",
                            u.crate_name,
                            if u.module.is_empty() { "" } else { "::" },
                            u.module
                        );
                        if scopes.is_empty() {
                            roots.insert(namespace);
                        } else if let Some(base) =
                            namespace.strip_suffix(&format!("::{}", scopes.join("::")))
                        {
                            roots.insert(base.into());
                        }
                    }
                }
            }
            if let TokenTree::Group(g) = token {
                if g.delimiter() != Delimiter::Brace {
                    continue;
                }
                let prefix = &tokens[start..i];
                let mut next = scopes.to_vec();
                if let Some(n) = prefix.iter().position(|t| t.to_string() == "impl") {
                    next.push(
                        prefix[n + 1..]
                            .iter()
                            .map(ToString::to_string)
                            .collect::<String>(),
                    );
                } else if let Some(n) = prefix
                    .iter()
                    .position(|t| ["mod", "trait"].contains(&t.to_string().as_str()))
                {
                    if let Some(name) = prefix.get(n + 1) {
                        next.push(name.to_string());
                    }
                } else {
                    start = i + 1;
                    continue;
                }
                visit(g.stream(), &next, file, report, roots);
                start = i + 1;
            }
            if token.to_string() == ";" {
                start = i + 1;
            }
        }
    }
    let mut roots = std::collections::BTreeSet::new();
    visit(source.parse().ok()?, &[], file, report, &mut roots);
    (roots.len() == 1).then(|| roots.into_iter().next().unwrap())
}

fn record(module: &str, decl: &Decl, raw: &str, report: &Report, root: &Path) -> Decision {
    // The shared declaration parser excludes preceding attributes; bind the immediately preceding line.
    let lines: Vec<_> = raw.lines().collect();
    if decl.line < 2 || lines[decl.line - 2].trim() != "[@@ FStar.Tactics.Typeclasses.tcinstance]" {
        return Err("instance: missing adjacent tcinstance attribute".into());
    }
    let rest = decl
        .text
        .strip_prefix("let ")
        .and_then(|s| s.strip_prefix(&decl.name))
        .ok_or("instance: unsupported declaration")?;
    let rest = rest
        .trim_start()
        .strip_prefix(':')
        .ok_or("instance: generic/unsupported binders; correspondence unconfirmed")?;
    let (typ, body) = rest.split_once('=').ok_or("instance: no record body")?;
    let mut ts = fstar_types(typ, module).ok_or("instance: unsupported F* type")?;
    if ts.len() < 2 {
        return Err("instance: missing trait or Self type".into());
    }
    let mut trait_ty = ts.remove(0);
    let self_ty = ts.remove(0);
    trait_ty.1.extend(ts);
    let body = body.split("\n[@@").next().unwrap_or(body);
    let methods = fields(body).ok_or("instance: unsupported record fields")?;
    if methods.is_empty() {
        return Err(
            "instance: no method body fields (pre/post and associated types do not count)".into(),
        );
    }
    let module = rust_module(module);
    let mut candidates: Vec<Vec<String>> = vec![];
    struct Query<'a> {
        wanted_module: &'a str,
        file: &'a str,
        report: &'a Report,
        self_ty: &'a Ty,
        trait_ty: &'a Ty,
        methods: &'a [String],
    }
    for file in &report.source_files {
        if !report
            .units
            .iter()
            .any(|u| u.file == *file && u.id.starts_with(&format!("{module}::")))
        {
            continue;
        }
        let source =
            fs::read_to_string(root.join(file)).map_err(|_| "instance: Rust source unreadable")?;
        let Ok(parsed) = syn::parse_file(&source) else {
            continue;
        };
        fn visit(
            items: &[syn::Item],
            module: &str,
            query: &Query<'_>,
            candidates: &mut Vec<Vec<String>>,
        ) {
            let Query {
                wanted_module,
                file,
                report,
                self_ty,
                trait_ty,
                methods,
            } = *query;
            let mut uses = BTreeMap::new();
            for item in items {
                if let syn::Item::Use(u) = item {
                    let mut found = BTreeMap::new();
                    imports(&u.tree, "", &mut found);
                    for (name, paths) in found {
                        if u.attrs.is_empty() {
                            uses.entry(name).or_insert_with(Vec::new).extend(paths);
                        } else {
                            uses.entry(name)
                                .or_insert_with(Vec::new)
                                .extend(["unconfirmed".into(), "conditional".into()]);
                        }
                    }
                }
                let name = match item {
                    syn::Item::Struct(i) => Some(i.ident.to_string()),
                    syn::Item::Enum(i) => Some(i.ident.to_string()),
                    syn::Item::Trait(i) => Some(i.ident.to_string()),
                    syn::Item::Type(i) => Some(i.ident.to_string()),
                    _ => None,
                };
                if let Some(name) = name {
                    uses.entry(name.clone())
                        .or_insert_with(Vec::new)
                        .push(format!("self::{name}"));
                }
            }
            for item in items {
                if let syn::Item::Mod(m) = item {
                    if let Some((_, items)) = &m.content {
                        visit(items, &format!("{module}::{}", m.ident), query, candidates);
                    }
                }
                let syn::Item::Impl(b) = item else {
                    continue;
                };
                if module != wanted_module
                    || !b.generics.params.is_empty()
                    || b.generics.where_clause.is_some()
                    || b.unsafety.is_some()
                    || b.attrs
                        .iter()
                        .any(|a| a.path().is_ident("cfg") || a.path().is_ident("cfg_attr"))
                {
                    continue;
                }
                let Some((path, _)) = &b.trait_ else {
                    continue;
                };
                let trait_as_type = syn::Type::Path(syn::TypePath {
                    attrs: vec![],
                    qself: None,
                    path: path.clone(),
                });
                if rust_type(&b.self_ty, module, &uses).as_ref() != Some(self_ty)
                    || rust_type(&trait_as_type, module, &uses).as_ref() != Some(trait_ty)
                {
                    continue;
                }
                let mut targets = vec![];
                for name in methods {
                    let matches: Vec<_> = b
                        .items
                        .iter()
                        .filter_map(|i| {
                            if let syn::ImplItem::Fn(m) = i {
                                Some(m)
                            } else {
                                None
                            }
                        })
                        .filter(|m| m.sig.ident == name)
                        .collect();
                    if matches.len() != 1 {
                        break;
                    }
                    if matches[0]
                        .attrs
                        .iter()
                        .any(|a| a.path().is_ident("cfg") || a.path().is_ident("cfg_attr"))
                    {
                        break;
                    }
                    let line = matches[0].sig.fn_token.span.start().line;
                    let units: Vec<_> = report
                        .units
                        .iter()
                        .filter(|u| {
                            u.file == file
                                && u.name == *name
                                && u.line <= line
                                && line <= u.end_line
                        })
                        .collect();
                    if units.len() != 1
                        || words(&units[0].scope_attributes)
                            .any(|w| ["cfg", "cfg_attr"].contains(&w))
                        || report.units.iter().filter(|u| u.id == units[0].id).count() != 1
                    {
                        break;
                    }
                    targets.push(units[0].id.clone());
                }
                if targets.len() == methods.len() {
                    candidates.push(targets);
                }
            }
        }
        if let Some(base) = file_module(&source, file, report) {
            visit(
                &parsed.items,
                &base,
                &Query {
                    wanted_module: &module,
                    file,
                    report,
                    self_ty: &self_ty,
                    trait_ty: &trait_ty,
                    methods: &methods,
                },
                &mut candidates,
            );
        }
    }
    if candidates.len() != 1 {
        return Err(format!("instance: {} matching trait/Self/argument blocks; cfg, aliases, references and generics require confirmed identity",candidates.len()));
    }
    Ok(candidates.remove(0))
}

pub(super) fn fallbacks(
    module: &str,
    decls: &[Decl],
    siblings: &[Decl],
    raw: &str,
    report: &Report,
    root: &Path,
) -> BTreeMap<String, Decision> {
    let mut out = BTreeMap::new();
    let empty: Vec<_> = siblings
        .iter()
        .filter(|d| d.name.starts_with("impl__"))
        .map(|d| Decl {
            name: d.name.replacen("impl__", "impl_0__", 1),
            line: d.line,
            kind: d.kind.clone(),
            text: d.text.replacen("impl__", "impl_0__", 1),
        })
        .collect();
    let empty = numbered_mappings(module, &empty, report);
    for d in decls {
        if mapped(module, &d.name, report).is_some() {
            continue;
        }
        let module_id = rust_module(module);
        let mut possible = vec![format!("{module_id}::{}", d.name)];
        if let Some((ty, method)) = d
            .name
            .strip_prefix("impl_")
            .and_then(|s| s.split_once("__"))
        {
            possible.push(format!("{module_id}::{ty}::{method}"));
        }
        if report.units.iter().any(|u| possible.contains(&u.id)) {
            out.insert(
                d.name.clone(),
                Err("v191 mapping: existing Rust spelling is ambiguous; no fallback".into()),
            );
            continue;
        }
        let decision = if let Some(plain) = trailing(&d.name) {
            mapped(module, plain, report)
                .map(|u| vec![u.id.clone()])
                .ok_or_else(|| "form A: stripped lookup has zero or several candidates".into())
        } else if d.name.starts_with("impl__") {
            empty
                .get(&d.name.replacen("impl__", "impl_0__", 1))
                .cloned()
                .unwrap_or_else(|| Err("form E: missing group".into()))
                .map(|id| vec![id])
        } else if instance(&d.name) && d.kind == "let" {
            record(module, d, raw, report, root)
        } else if d
            .name
            .strip_suffix('_')
            .is_some_and(|n| n.as_bytes().last().is_some_and(u8::is_ascii_digit))
        {
            Err(
                "form A: suffix is not an underscore-separated decimal segment established by M0"
                    .into(),
            )
        } else {
            continue;
        };
        out.insert(d.name.clone(), decision);
    }
    out
}

pub(super) fn cfg_twins(module: &str, name: &str, report: &Report) -> bool {
    let id = format!("{}::{name}", rust_module(module));
    report.units.iter().filter(|u| u.id == id).count() > 1
}

pub(super) fn ids(decision: &Decision) -> impl Iterator<Item = &String> {
    decision.iter().flatten()
}

pub(super) fn units(decision: &Decision, report: &Report) -> Vec<Unit> {
    decision
        .as_ref()
        .map(|ids| {
            ids.iter()
                .filter_map(|id| report.units.iter().find(|u| u.id == *id).cloned())
                .collect()
        })
        .unwrap_or_default()
}
