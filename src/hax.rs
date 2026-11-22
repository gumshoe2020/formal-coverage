//! Closed hax/F* path inventory and conservative token-line ownership.
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::{BTreeMap, BTreeSet};

// Exact paths after the crate spelling. Never recognize a namespace by prefix.
const PATHS: &[&str] = &[
    "requires",
    "ensures",
    "decreases",
    "loop_invariant",
    "fstar",
    "fstar::before",
    "fstar::after",
    "fstar::replace",
    "fstar::replace_body",
    "fstar::options",
    "fstar::expr",
    "fstar::prop",
    "fstar::unsafe_expr",
    "opaque",
    "attributes",
    "lemma",
    "assert",
    "assume",
    "implies",
    "forall",
    "exists",
    "int",
    "prop",
    "int::Int",
    "int::Int::new",
    "int::Int::pow2",
    "prop::Prop",
    "prop::Prop::from_bool",
    "prop::constructors::from_bool",
    "prop::constructors::and",
    "prop::constructors::or",
    "prop::constructors::not",
    "prop::constructors::eq",
    "prop::constructors::ne",
    "prop::constructors::implies",
    "prop::constructors::forall",
    "prop::constructors::exists",
];
fn path(ts: &[TokenTree]) -> (String, usize) {
    let mut n = 0;
    let mut parts = vec![];
    while let Some(TokenTree::Ident(id)) = ts.get(n) {
        parts.push(id.to_string());
        n += 1;
        if ts.get(n).is_some_and(|t| t.to_string() == ":")
            && ts.get(n + 1).is_some_and(|t| t.to_string() == ":")
        {
            n += 2;
        } else {
            break;
        }
    }
    (parts.join("::"), n)
}
#[derive(Default)]
pub(crate) struct Scan {
    proof: BTreeSet<usize>,
    runtime: BTreeSet<usize>,
    cfg_proof: BTreeSet<usize>,
    pub unknown: BTreeSet<String>,
}
impl Scan {
    fn token(&mut self, t: &TokenTree, proof: bool) {
        let spans = match t {
            TokenTree::Group(g) => {
                for t in crate::non_doc_tokens(g.stream()) {
                    self.token(&t, proof);
                }
                vec![g.span_open(), g.span_close()]
            }
            _ => vec![t.span()],
        };
        for span in spans {
            let dest = if proof {
                &mut self.proof
            } else {
                &mut self.runtime
            };
            dest.extend(span.start().line..=span.end().line);
        }
    }
    fn recognized(&mut self, name: &str, aliases: &BTreeMap<String, String>) -> bool {
        let Some((root, rest)) = name.split_once("::") else {
            return false;
        };
        let canonical = if ["hax_lib", "hax"].contains(&root) {
            rest.to_string()
        } else if let Some(prefix) = aliases.get(root) {
            format!("{prefix}{rest}")
        } else {
            return false;
        };
        if PATHS.contains(&canonical.as_str()) {
            true
        } else {
            self.unknown.insert(format!("hax_lib::{canonical}"));
            false
        }
    }
    fn unknowns(&mut self, stream: TokenStream, aliases: &BTreeMap<String, String>) {
        let ts: Vec<_> = crate::non_doc_tokens(stream).collect();
        let mut i = 0;
        while i < ts.len() {
            let (name, n) = path(&ts[i..]);
            if n > 0 {
                self.recognized(&name, aliases);
                i += n;
            } else {
                if let TokenTree::Group(g) = &ts[i] {
                    self.unknowns(g.stream(), aliases);
                }
                i += 1;
            }
        }
    }
    fn attribute(&mut self, ts: &[TokenTree], aliases: &BTreeMap<String, String>) -> bool {
        let (name, n) = path(ts);
        if self.recognized(&name, aliases) {
            return true;
        }
        if name == "cfg_attr" {
            if let Some(TokenTree::Group(g)) = ts.get(n) {
                let inner: Vec<_> = g.stream().into_iter().collect();
                // The condition is not proof. The wrapper belongs to proof only if
                // every applied attribute is a recognized hax annotation.
                if let Some(comma) = inner.iter().position(|t| t.to_string() == ",") {
                    let mut start = comma + 1;
                    let mut all = start < inner.len();
                    for end in (start..inner.len())
                        .filter(|i| inner[*i].to_string() == ",")
                        .chain(std::iter::once(inner.len()))
                    {
                        if start < end {
                            all &= self.attribute(&inner[start..end], aliases);
                        }
                        start = end + 1;
                    }
                    return all;
                }
            }
        }
        false
    }
    fn walk(&mut self, stream: TokenStream, inherited: &BTreeMap<String, String>, cfg_hax: bool) {
        let ts: Vec<_> = crate::non_doc_tokens(stream).collect();
        let mut aliases = inherited.clone();
        // Rust use items apply throughout their lexical scope, including before the use.
        for i in 0..ts.len() {
            if ts[i].to_string() != "use" {
                continue;
            }
            let (name, n) = path(&ts[i + 1..]);
            if ["hax_lib", "hax"].contains(&name.as_str())
                && ts.get(i + 1 + n).is_some_and(|t| t.to_string() == "as")
            {
                if let Some(TokenTree::Ident(alias)) = ts.get(i + 2 + n) {
                    aliases.insert(alias.to_string(), String::new());
                }
            }
        }
        let mut i = 0;
        while i < ts.len() {
            if cfg_hax && ts[i].to_string() == "#" {
                let mut attrs_end = i;
                while ts.get(attrs_end).is_some_and(|t| t.to_string() == "#") {
                    if !matches!(ts.get(attrs_end + 1), Some(TokenTree::Group(g))
                        if g.delimiter() == Delimiter::Bracket)
                    {
                        break;
                    }
                    attrs_end += 2;
                }
                let attrs: TokenStream = ts[i..attrs_end].iter().cloned().collect();
                if crate::cfg_implies_hax(&attrs.to_string()) && attrs_end < ts.len() {
                    let Some(end) = statement_end(&ts, i) else {
                        // Unsupported non-Rust grammars retain runtime ownership.
                        self.token(&ts[i], false);
                        i += 1;
                        continue;
                    };
                    for t in &ts[i..end] {
                        let mut owned = Scan::default();
                        owned.token(t, true);
                        self.cfg_proof.extend(owned.proof.iter().copied());
                        self.proof.extend(owned.proof);
                    }
                    self.unknowns(ts[i..end].iter().cloned().collect(), &aliases);
                    i = end;
                    continue;
                }
            }
            if ts[i].to_string() == "#" {
                let j = i + 1 + usize::from(ts.get(i + 1).is_some_and(|t| t.to_string() == "!"));
                if let Some(TokenTree::Group(g)) = ts.get(j) {
                    if g.delimiter() == Delimiter::Bracket
                        && self.attribute(&g.stream().into_iter().collect::<Vec<_>>(), &aliases)
                    {
                        for t in &ts[i..=j] {
                            if let TokenTree::Group(g) = t {
                                self.unknowns(g.stream(), &aliases);
                            }
                            self.token(t, true);
                        }
                        i = j + 1;
                        continue;
                    }
                    if g.delimiter() == Delimiter::Bracket {
                        self.unknowns(g.stream(), &aliases);
                        for t in &ts[i..=j] {
                            self.token(t, false);
                        }
                        i = j + 1;
                        continue;
                    }
                }
            }
            let (name, n) = path(&ts[i..]);
            if n > 0 && self.recognized(&name, &aliases) {
                let mut end = i + n;
                let is_macro = ts.get(end).is_some_and(|t| t.to_string() == "!");
                end += usize::from(is_macro);
                if let Some(TokenTree::Group(g)) = ts.get(end) {
                    if is_macro || g.delimiter() == Delimiter::Parenthesis {
                        end += 1;
                        end += usize::from(ts.get(end).is_some_and(|t| t.to_string() == ";"));
                        for t in &ts[i..end] {
                            if let TokenTree::Group(g) = t {
                                self.unknowns(g.stream(), &aliases);
                            }
                            self.token(t, true);
                        }
                        i = end;
                        continue;
                    }
                }
            }
            // Unknown macro payloads are not Rust expression/attribute grammars.
            // In particular, ProVerif/SSProve cannot smuggle proof cost through
            // nested helper-looking tokens.
            if n > 0 && name != "verus" && ts.get(i + n).is_some_and(|t| t.to_string() == "!") {
                if let Some(TokenTree::Group(g)) = ts.get(i + n + 1) {
                    self.unknowns(g.stream(), &aliases);
                    for t in &ts[i..=i + n + 1] {
                        self.token(t, false);
                    }
                    i += n + 2;
                    continue;
                }
            }
            match &ts[i] {
                TokenTree::Group(g) => {
                    // Delimiters outside a proof construct always retain runtime ownership.
                    self.runtime
                        .extend([g.span_open().start().line, g.span_close().start().line]);
                    self.walk(g.stream(), &aliases, cfg_hax);
                }
                t => self.token(t, false),
            }
            i += 1;
        }
    }
    pub fn lines(&self, start: usize, end: usize) -> Vec<usize> {
        self.proof
            .difference(&self.runtime)
            .copied()
            .filter(|n| (start..=end).contains(n))
            .collect()
    }
}
// Parse one Rust statement, retaining the original token spans for ownership.
// syn is already in the dependency closure; enabling its statement grammar avoids
// guessing where if/else, closures, struct expressions and let-else statements end.
fn statement_end(ts: &[TokenTree], start: usize) -> Option<usize> {
    use syn::parse::Parser;
    let parser = |input: syn::parse::ParseStream<'_>| {
        input.parse::<syn::Stmt>()?;
        let rest: TokenStream = input.parse()?;
        Ok(ts.len() - rest.into_iter().count())
    };
    parser.parse2(ts[start..].iter().cloned().collect()).ok()
}
pub(crate) fn scan(source: &str) -> Option<Scan> {
    scan_with_cfg(source, false)
}
fn scan_with_cfg(source: &str, cfg_hax: bool) -> Option<Scan> {
    let mut s = Scan::default();
    s.walk(source.parse().ok()?, &BTreeMap::new(), cfg_hax);
    Some(s)
}
pub(crate) fn apply(report: &mut crate::Report, root: &std::path::Path) -> anyhow::Result<()> {
    for u in &mut report.units {
        // Existing test classification takes precedence over extraction-only proof.
        u.fstar_cfg_hax = u.category != crate::Category::Test
            && u.first_party
            && u.hax_crate
            && (u.cfg_hax_scope || crate::cfg_implies_hax(&u.scope_attributes));
        u.cfg_hax_was_runtime = u.category == crate::Category::Runtime;
        if u.fstar_cfg_hax {
            u.category = crate::Category::ProofCode;
        }
    }
    for file in &report.source_files {
        let source = std::fs::read_to_string(root.join(file))?;
        let Some(scan) = scan(&source) else {
            continue;
        };
        report
            .unknown_hax_paths
            .extend(scan.unknown.iter().cloned());
        let combined = scan_with_cfg(&source, true).expect("source already tokenized");
        let standalone: Vec<_> = report
            .units
            .iter()
            .filter(|u| u.first_party && u.file == *file && u.fstar_cfg_hax)
            .map(|u| u.line..=u.end_line)
            .collect();
        for u in report
            .units
            .iter_mut()
            .filter(|u| u.first_party && u.file == *file && u.category == crate::Category::Runtime)
        {
            let ownership = if u.hax_crate { &combined } else { &scan };
            u.fstar_annotation_sloc = scan
                .proof
                .difference(&ownership.runtime)
                .filter(|n| (u.line..=u.end_line).contains(n))
                .filter(|n| !standalone.iter().any(|span| span.contains(n)))
                .count();
            u.fstar_cfg_hax_inline_sloc = ownership
                .cfg_proof
                .difference(&ownership.runtime)
                .filter(|n| (u.line..=u.end_line).contains(n))
                .count();
            let hax_lines = ownership.lines(u.line, u.end_line);
            if u.verus {
                let padded = format!("{}{}", "\n".repeat(u.line - 1), u.source);
                if let Ok(stream) = padded.parse() {
                    let mut lines =
                        crate::ghost::inline_lines(stream, &source.lines().collect::<Vec<_>>());
                    lines.extend(hax_lines);
                    u.ghost_sloc = lines.len();
                }
            } else {
                u.ghost_sloc = hax_lines.len();
            }
        }
    }
    report.unknown_hax_paths.sort();
    report.unknown_hax_paths.dedup();
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! entries {
        ($($name:ident: $path:literal),* $(,)?) => { $(#[test] fn $name() {
            for root in ["hax_lib", "hax"] {
                let source = format!("#[{root}::{}(true)]\nfn f() {{\n {root}::{}!(true);\n}}\n", $path, $path);
                assert_eq!(scan(&source).unwrap().lines(1, 4), [1, 3], "{source}");
            }
        })* #[test] fn every_whitelist_entry_has_a_test() {
            assert_eq!(PATHS, &[$($path),*]);
        }};
    }
    entries! {
        requires:"requires", ensures:"ensures", decreases:"decreases", invariant:"loop_invariant", fstar:"fstar",
        before:"fstar::before", after:"fstar::after", replace:"fstar::replace", replace_body:"fstar::replace_body",
        options:"fstar::options", expr:"fstar::expr", prop_expr:"fstar::prop", unsafe_expr:"fstar::unsafe_expr",
        opaque:"opaque", attributes:"attributes", lemma:"lemma", assertion:"assert", assumption:"assume", implies:"implies", forall:"forall", exists:"exists",
        int:"int", prop:"prop", int_type:"int::Int", int_new:"int::Int::new", int_pow:"int::Int::pow2",
        prop_type:"prop::Prop", prop_bool:"prop::Prop::from_bool", bool_constructor:"prop::constructors::from_bool",
        and:"prop::constructors::and", or:"prop::constructors::or", not:"prop::constructors::not",
        eq:"prop::constructors::eq", ne:"prop::constructors::ne", prop_implies:"prop::constructors::implies",
        prop_forall:"prop::constructors::forall", prop_exists:"prop::constructors::exists",
    }
    #[test]
    fn aliases_wrappers_docs_mixed_lines_and_unknowns() {
        let s = scan("use hax_lib as hx;\n#[cfg_attr(hax, hx::requires(true))]\n/// doc\n#[doc = \"also doc\"]\nfn f() {\n hx::assert!(true);\n let x = 1; hx::assert!(true);\n hx::future!(true);\n}\n").unwrap();
        assert_eq!(s.lines(1, 9), [2, 6]);
        assert!(s.unknown.contains("hax_lib::future"));
        assert!(
            scan("mod a {use hax_lib as hx;}\nfn f() {\n hx::assert!(true);\n}\n")
                .unwrap()
                .lines(1, 4)
                .is_empty()
        );
    }
    #[test]
    fn other_backends_and_unknown_subpaths_are_runtime() {
        let s = scan("#[hax_lib::proverif::before(\"x\")]\n#[hax_lib::pv_constructor]\nfn f() {\n hax_lib::ssprove!(x);\n hax_lib::fstar::future!(x);\n}\n").unwrap();
        assert!(s.lines(1, 6).is_empty());
        assert_eq!(s.unknown.len(), 4);
    }
    #[test]
    fn backend_and_unknown_macro_payloads_do_not_become_proof() {
        for outer in ["hax_lib::proverif", "hax_lib::ssprove", "other"] {
            let source = format!("fn f() {{\n{outer}! {{\nhax_lib::assert!(true);\n}};\n}}\n");
            assert!(scan(&source).unwrap().lines(1, 6).is_empty());
        }
        let source = "#[hax_lib::proverif::before(\n hax_lib::assert!(true)\n)]\nfn f() {}\n";
        assert!(scan(source).unwrap().lines(1, 5).is_empty());
    }
}
