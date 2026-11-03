//! Conservative token ownership for inline Verus ghost syntax. Groups are traversed
//! with their delimiters, so a shared runtime/ghost line always remains runtime.
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GhostLines {
    #[default]
    Separate,
    Runtime,
}
impl GhostLines {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Separate => "separate",
            Self::Runtime => "runtime",
        }
    }
}
impl std::str::FromStr for GhostLines {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "separate" => Ok(Self::Separate),
            "runtime" => Ok(Self::Runtime),
            _ => anyhow::bail!("ghost lines must be separate or runtime"),
        }
    }
}
fn is(t: &TokenTree, word: &str) -> bool {
    matches!(t, TokenTree::Ident(i) if i == word)
}
fn brace(t: &TokenTree) -> bool {
    matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace)
}
fn clause(t: &TokenTree) -> bool {
    ["requires", "ensures", "recommends", "decreases"]
        .iter()
        .any(|s| is(t, s))
}
#[derive(Default)]
struct Lines {
    ghost: BTreeSet<usize>,
    runtime: BTreeSet<usize>,
}
impl Lines {
    fn span(&mut self, s: Span, ghost: bool) {
        let set = if ghost {
            &mut self.ghost
        } else {
            &mut self.runtime
        };
        set.extend(s.start().line..=s.end().line);
    }
    fn token(&mut self, t: &TokenTree, ghost: bool) {
        if let TokenTree::Group(g) = t {
            self.span(g.span_open(), ghost);
            for t in g.stream() {
                self.token(&t, ghost);
            }
            self.span(g.span_close(), ghost);
        } else {
            self.span(t.span(), ghost);
        }
    }
    fn all(&mut self, ts: &[TokenTree], ghost: bool) {
        for t in ts {
            self.token(t, ghost);
        }
    }
    fn body(&mut self, ts: &[TokenTree]) {
        let mut i = 0;
        while i < ts.len() {
            let statement = i == 0 || ts[i - 1].to_string() == ";" || brace(&ts[i - 1]);
            // Only complete statements qualify. Unknown/macro token grammars remain runtime.
            if statement {
                if is(&ts[i], "proof") && ts.get(i + 1).is_some_and(brace) {
                    let end =
                        i + 2 + usize::from(ts.get(i + 2).is_some_and(|t| t.to_string() == ";"));
                    self.all(&ts[i..end], true);
                    i = end;
                    continue;
                }
                let binding = is(&ts[i], "let")
                    && ts
                        .get(i + 1)
                        .is_some_and(|t| is(t, "ghost") || is(t, "tracked"));
                if binding {
                    if let Some(n) = ts[i + 1..].iter().position(|t| t.to_string() == ";") {
                        let end = i + 1 + n + 1;
                        self.all(&ts[i..end], true);
                        i = end;
                        continue;
                    }
                }
                let assertion = is(&ts[i], "assert") || is(&ts[i], "assume");
                if assertion {
                    let mut end = i + 1;
                    if ts.get(end).is_some_and(|t| matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis)) {
                        end += 1;
                    } else if is(&ts[i], "assert") && ts.get(end).is_some_and(|t| is(t,"forall")) {
                        while end < ts.len() && !is(&ts[end], "by") && ts[end].to_string() != ";" { end += 1; }
                    }
                    if ts.get(end).is_some_and(|t| is(t, "by"))
                        && ts.get(end + 1).is_some_and(brace)
                    {
                        end += 2;
                        end += usize::from(ts.get(end).is_some_and(|t| t.to_string() == ";"));
                        self.all(&ts[i..end], true);
                        i = end;
                        continue;
                    }
                    if end > i + 1 && ts.get(end).is_some_and(|t| t.to_string() == ";") {
                        end += 1;
                        self.all(&ts[i..end], true);
                        i = end;
                        continue;
                    }
                }
            }
            if ["while", "loop", "for"].iter().any(|s| is(&ts[i], s))
                && (i == 0 || ![".", ":"].contains(&ts[i - 1].to_string().as_str()))
            {
                // A loop clause ends at its body brace. If a clause contains a brace
                // expression, only a brace immediately followed by another brace or a
                // clause/operator can be skipped; ambiguity retains runtime ownership.
                let mut j = i + 1;
                let mut start = None;
                while j < ts.len() && ts[j].to_string() != ";" {
                    if clause(&ts[j])
                        || is(&ts[j], "invariant")
                        || is(&ts[j], "invariant_except_break")
                    {
                        start.get_or_insert(j);
                    }
                    if brace(&ts[j]) {
                        let continuation = ts.get(j + 1).is_some_and(|t| {
                            brace(t)
                                || clause(t)
                                || [",", "&", "|", "=", ">", "<", "+", "-", "*", "/", "."]
                                    .contains(&t.to_string().as_str())
                        });
                        if !continuation {
                            break;
                        }
                    }
                    j += 1;
                }
                if j < ts.len() && brace(&ts[j]) {
                    let s = start.unwrap_or(j);
                    self.all(&ts[i..s], false);
                    self.all(&ts[s..j], true);
                    self.group(&ts[j]);
                    i = j + 1;
                    continue;
                }
            }
            // Do not interpret arbitrary macro inputs, nested function signatures,
            // or parentheses as statement grammars.
            if brace(&ts[i]) && (i == 0 || ts[i - 1].to_string() != "!") {
                self.group(&ts[i]);
            } else {
                self.token(&ts[i], false);
            }
            i += 1;
        }
    }
    fn group(&mut self, t: &TokenTree) {
        if let TokenTree::Group(g) = t {
            self.span(g.span_open(), false);
            self.body(&g.stream().into_iter().collect::<Vec<_>>());
            self.span(g.span_close(), false);
        }
    }
}
/// Called only for runtime functions inside a verus! scope. Last token is the
/// function body (or a trait declaration semicolon), supplied by the inventory.
pub(crate) fn inline_sloc(stream: TokenStream, source_lines: &[&str]) -> usize {
    let ts: Vec<_> = stream.into_iter().collect();
    let mut lines = Lines::default();
    if let Some(last) = ts.last().filter(|t| brace(t)) {
        let end = ts.len() - 1;
        let params = ts[..end]
            .iter()
            .position(
                |t| matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis),
            )
            .map_or(end, |i| i + 1);
        let start = ts[params..end]
            .iter()
            .position(clause)
            .map_or(end, |i| params + i);
        lines.all(&ts[..start], false);
        lines.all(&ts[start..end], true);
        lines.group(last);
    } else {
        lines.all(&ts, false);
    }
    lines
        .ghost
        .difference(&lines.runtime)
        .filter(|line| {
            source_lines
                .get(**line - 1)
                .is_some_and(|s| !s.trim().is_empty())
        })
        .count()
}

pub(crate) fn apply(report: &mut crate::Report, mode: GhostLines) {
    report.ghost_lines = mode;
    for u in &mut report.units {
        if u.first_party && u.category == crate::Category::Runtime {
            u.inline_proof_sloc = if mode == GhostLines::Separate {
                u.ghost_sloc
            } else {
                0
            };
            u.sloc -= u.inline_proof_sloc;
            u.runtime_sloc = u.sloc;
        } else {
            u.runtime_sloc = 0;
            u.inline_proof_sloc = 0;
        }
    }
}
