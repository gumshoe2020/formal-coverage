//! Locate the file behind every out-of-line `mod` declaration the way rustc
//! does (rustc_expand/src/module.rs), including `#[path = "..."]`.
//!
//! The inventory names modules after their file paths. That is right for
//! default module files, but a `#[path]` module's file can sit anywhere, so
//! the attributes on its `mod` item (`#[cfg(test)]` above all) were lost.
//! This walk records, for each file reached through a `#[path]` declaration
//! (directly or below one), the attributes in scope at every declaration
//! that loads it.
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

/// One declaration that loads `file` into a crate.
#[derive(Debug, Clone)]
pub(crate) struct Claim {
    /// Canonical path of the module file.
    pub file: PathBuf,
    /// Attributes of every enclosing `mod` item and file, outermost first.
    pub attrs: String,
    /// The crate root is a test or bench target (compiled with `--test`).
    pub test_crate: bool,
    /// This declaration, or one enclosing it, used `#[path]`.
    pub via_path: bool,
}

/// A `#[path]` declaration the inventory cannot follow faithfully.
#[derive(Debug, Clone)]
pub(crate) struct Hazard {
    /// Canonical path of the file holding the `mod` item.
    pub declaring_file: PathBuf,
    pub line: usize,
    pub reason: String,
    /// Canonical module file, when it exists.
    pub target: Option<PathBuf>,
}

#[derive(Default)]
pub(crate) struct Resolution {
    pub claims: Vec<Claim>,
    pub hazards: Vec<Hazard>,
}

/// A crate root and whether it is compiled as a test harness.
pub(crate) struct CrateRoot<'a> {
    pub file: &'a Path,
    pub test_crate: bool,
}

/// Follow module declarations from each crate root. `inventoried` holds the
/// canonical paths of the files the inventory measures.
pub(crate) fn resolve(roots: &[CrateRoot], inventoried: &BTreeSet<PathBuf>) -> Resolution {
    let mut resolution = Resolution::default();
    for root in roots {
        let Ok(file) = root.file.canonicalize() else {
            continue;
        };
        let mut walker = Walker {
            inventoried,
            test_crate: root.test_crate,
            stack: vec![],
            claimed: BTreeMap::new(),
            out: &mut resolution,
        };
        let dir = Location {
            dir: file.parent().unwrap_or(&file).to_path_buf(),
            relative: None,
        };
        walker.claim(&file, "", false);
        walker.file(&file, dir, "", false);
    }
    resolution
}

/// rustc's `ModuleData::dir_path` plus `DirOwnership::Owned { relative }`:
/// a non-mod-rs file `foo.rs` looks for its children under `foo/`.
#[derive(Clone)]
struct Location {
    dir: PathBuf,
    relative: Option<String>,
}

/// The module item currently being declared, and where it sits.
struct Declaration<'a> {
    declaring_file: &'a Path,
    location: &'a Location,
    /// Attributes in scope: enclosing ones followed by the item's own.
    attrs: String,
    /// The item's own `#[path]`, if any.
    path: PathAttribute,
    via_path: bool,
}

struct Walker<'a> {
    inventoried: &'a BTreeSet<PathBuf>,
    test_crate: bool,
    /// Files being walked, for circular `#[path]` detection.
    stack: Vec<PathBuf>,
    /// Per file in this crate: declarations loading it, and whether any used `#[path]`.
    claimed: BTreeMap<PathBuf, (usize, bool)>,
    out: &'a mut Resolution,
}

impl Walker<'_> {
    fn file(&mut self, file: &Path, location: Location, inherited: &str, via_path: bool) {
        let parsed = fs::read_to_string(file).map(|source| source.parse::<TokenStream>());
        let Ok(Ok(stream)) = parsed else {
            // Unparseable files are already whole-file hazards in the inventory.
            return;
        };
        self.stack.push(file.to_path_buf());
        self.items(stream, file, &location, inherited, via_path);
        self.stack.pop();
    }

    fn items(
        &mut self,
        stream: TokenStream,
        file: &Path,
        location: &Location,
        inherited: &str,
        via_path: bool,
    ) {
        let inherited = format!(
            "{inherited} {}",
            crate::cfg::attributes(stream.clone(), true)
        );
        let tokens: Vec<_> = stream.into_iter().collect();
        let mut start = 0;
        for i in 0..tokens.len() {
            let prefix = &tokens[start..i];
            let declaration = || {
                let own = crate::cfg::attributes(prefix.iter().cloned().collect(), false);
                Declaration {
                    declaring_file: file,
                    location,
                    attrs: format!("{inherited} {own}"),
                    path: path_attribute(own),
                    via_path,
                }
            };
            match &tokens[i] {
                TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                    if let Some((keyword, name)) = module_name(prefix) {
                        self.inline_module(g.stream(), keyword, name, declaration());
                    } else if is_item_macro(prefix) {
                        // Item macros such as verus! keep the enclosing module.
                        self.items(g.stream(), file, location, &inherited, via_path);
                    }
                    start = i + 1;
                }
                t if t.to_string() == ";" => {
                    if let Some((keyword, name)) = module_name(prefix) {
                        self.out_of_line_module(keyword, &name, declaration());
                    }
                    start = i + 1;
                }
                _ => {}
            }
        }
    }

    fn inline_module(
        &mut self,
        body: TokenStream,
        keyword: &TokenTree,
        name: String,
        declaration: Declaration,
    ) {
        let Declaration {
            declaring_file,
            location,
            attrs,
            path,
            via_path,
        } = declaration;
        // For an inline module, `#[path]` names a directory (rustc keeps it
        // for historical reasons) relative to the enclosing module directory.
        let followed = !matches!(path, PathAttribute::None);
        let inner = match path {
            PathAttribute::None => {
                let mut dir = location.dir.clone();
                if let Some(relative) = &location.relative {
                    dir.push(relative);
                }
                dir.push(&name);
                Location {
                    dir,
                    relative: None,
                }
            }
            PathAttribute::Literal(path) => Location {
                dir: location.dir.join(path),
                relative: None,
            },
            PathAttribute::Unsupported(reason) => {
                self.hazard(declaring_file, keyword, reason, None);
                return;
            }
        };
        self.items(body, declaring_file, &inner, &attrs, via_path || followed);
    }

    fn out_of_line_module(&mut self, keyword: &TokenTree, name: &str, declaration: Declaration) {
        let Declaration {
            declaring_file,
            location,
            attrs,
            path,
            via_path,
        } = declaration;
        let (file, child, via_path) = match path {
            PathAttribute::None => match default_module_file(location, name) {
                Some(found) => (found.0, found.1, via_path),
                // rustc rejects a missing or ambiguous default module file
                // (E0583, E0761). Outside a `#[path]` chain the inventory
                // keeps its file-path module names, so this stays silent.
                None => return,
            },
            // rustc: `mod_file_path_from_attr` joins the literal to the
            // module directory, and every `#[path]` file acts as a mod.rs.
            PathAttribute::Literal(path) => (location.dir.join(path), None, true),
            PathAttribute::Unsupported(reason) => {
                self.hazard(declaring_file, keyword, reason, None);
                return;
            }
        };
        let Some(file) = self.check(declaring_file, keyword, &file, via_path) else {
            return;
        };
        let location = Location {
            dir: file.parent().unwrap_or(&file).to_path_buf(),
            relative: child,
        };
        self.claim(&file, &attrs, via_path);
        self.file(&file, location, &attrs, via_path);
    }

    /// The canonical module file, or None after recording (inside a `#[path]`
    /// chain) why it can't be followed.
    fn check(
        &mut self,
        declaring_file: &Path,
        keyword: &TokenTree,
        file: &Path,
        via_path: bool,
    ) -> Option<PathBuf> {
        let canonical = match self.locate(file) {
            Ok(canonical) => canonical,
            Err((reason, target)) => {
                if via_path {
                    self.hazard(declaring_file, keyword, reason, target);
                }
                return None;
            }
        };
        let earlier_via_path = self.claimed.get(&canonical).map(|(_, v)| *v);
        if earlier_via_path.is_some_and(|earlier| via_path || earlier) {
            self.hazard(declaring_file, keyword, DUPLICATE, Some(canonical.clone()));
        }
        Some(canonical)
    }

    fn locate(&self, file: &Path) -> Result<PathBuf, (&'static str, Option<PathBuf>)> {
        let canonical = file.canonicalize().map_err(|_| (MISSING, None))?;
        if fs::read_to_string(&canonical).is_err() {
            return Err((MISSING, Some(canonical)));
        }
        if self.stack.contains(&canonical) {
            return Err((CIRCULAR, Some(canonical)));
        }
        if !self.inventoried.contains(&canonical) {
            return Err((OUTSIDE, Some(canonical)));
        }
        Ok(canonical)
    }

    fn claim(&mut self, file: &Path, attrs: &str, via_path: bool) {
        let entry = self.claimed.entry(file.to_path_buf()).or_default();
        entry.0 += 1;
        entry.1 |= via_path;
        self.out.claims.push(Claim {
            file: file.to_path_buf(),
            attrs: attrs.to_string(),
            test_crate: self.test_crate,
            via_path,
        });
    }

    fn hazard(
        &mut self,
        declaring_file: &Path,
        keyword: &TokenTree,
        reason: &str,
        target: Option<PathBuf>,
    ) {
        self.out.hazards.push(Hazard {
            declaring_file: declaring_file.to_path_buf(),
            line: keyword.span().start().line,
            reason: reason.to_string(),
            target,
        });
    }
}

const MALFORMED: &str = "#[path] value is not a string literal (rustc: malformed attribute error)";
const CONDITIONAL: &str =
    "#[cfg_attr(.., path = ..)] is not followed (rustc applies it when the cfg holds)";
const CIRCULAR: &str = "circular module declaration (rustc: \"circular modules\" error)";
const OUTSIDE: &str =
    "module file is outside the measured workspace sources (rustc compiles it; its code is not measured)";
const DUPLICATE: &str = "module file is already loaded by another module declaration (rustc compiles it again as a separate module); test only if every declaration is";
const MISSING: &str =
    "module file is missing or unreadable (rustc: \"couldn't find file\" error when the module is cfg-enabled)";

/// rustc's `default_submod_path`: `name.rs` or `name/mod.rs` under the module
/// directory (plus the non-mod-rs prefix). Exactly one must exist.
fn default_module_file(location: &Location, name: &str) -> Option<(PathBuf, Option<String>)> {
    let mut dir = location.dir.clone();
    if let Some(relative) = &location.relative {
        dir.push(relative);
    }
    let flat = dir.join(format!("{name}.rs"));
    let nested = dir.join(name).join("mod.rs");
    match (flat.is_file(), nested.is_file()) {
        (true, false) => Some((flat, Some(name.to_string()))),
        (false, true) => Some((nested, None)),
        _ => None,
    }
}

/// The `mod` keyword and module name ending an item prefix.
fn module_name(prefix: &[TokenTree]) -> Option<(&TokenTree, String)> {
    let at = prefix.iter().rposition(|t| t.to_string() == "mod")?;
    let TokenTree::Ident(name) = prefix.get(at + 1)? else {
        return None;
    };
    (at + 2 == prefix.len()).then(|| {
        let name = name.to_string();
        let name = name.strip_prefix("r#").unwrap_or(&name).to_string();
        (&prefix[at], name)
    })
}

fn is_item_macro(prefix: &[TokenTree]) -> bool {
    prefix.last().is_some_and(|t| t.to_string() == "!")
        || prefix
            .get(prefix.len().wrapping_sub(2))
            .is_some_and(|t| t.to_string() == "!")
}

enum PathAttribute {
    None,
    Literal(String),
    Unsupported(&'static str),
}

/// The first `#[path = "..."]` among a declaration's own attributes, as rustc reads it.
fn path_attribute(own: TokenStream) -> PathAttribute {
    for token in own {
        let TokenTree::Group(group) = token else {
            continue;
        };
        let inner: Vec<_> = group.stream().into_iter().collect();
        match inner.as_slice() {
            [TokenTree::Ident(name), eq, value] if name == "path" && eq.to_string() == "=" => {
                return syn::parse2::<syn::LitStr>(value.clone().into())
                    .map(|lit| PathAttribute::Literal(lit.value()))
                    .unwrap_or_else(|_| PathAttribute::Unsupported(MALFORMED));
            }
            [TokenTree::Ident(name), ..] if name == "path" => {
                return PathAttribute::Unsupported(MALFORMED);
            }
            [TokenTree::Ident(name), TokenTree::Group(args)]
                if name == "cfg_attr" && mentions_path(args.stream()) =>
            {
                return PathAttribute::Unsupported(CONDITIONAL);
            }
            _ => {}
        }
    }
    PathAttribute::None
}

fn mentions_path(stream: TokenStream) -> bool {
    stream
        .into_iter()
        .any(|t| matches!(t, TokenTree::Ident(i) if i == "path"))
}
