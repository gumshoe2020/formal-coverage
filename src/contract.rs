//! Contract depth (v1.11.0): classify an F* `ensures` clause as a safety fact (panic freedom,
//! overflow, bounds: lengths and integer ranges) or as a value fact. Any doubt about reading a
//! clause lowers it to `safety-only` with reason `unparsed`; nothing is ever rounded up.
use std::collections::HashMap;

pub(crate) struct Contract {
    pub kind: String,
    pub reason: &'static str,
    pub post: String,
    /// Functions the postcondition calls, for the self-consistency check.
    pub calls: Vec<String>,
}

pub(crate) fn classify(decl: &str) -> Contract {
    if let Some((_, post)) = decl.split_once("(ensures") {
        return postcondition(decl, post.trim());
    }
    if decl.contains("Prims.Pure") && decl.contains("(fun _ -> Prims.l_True)")
        || decl.contains("Tot ")
    {
        return contract(
            "safety-only",
            "no-postcondition",
            "True (or absent for Tot)",
        );
    }
    contract("unclassified", "unclassified", decl)
}

fn contract(kind: &str, reason: &'static str, post: &str) -> Contract {
    Contract {
        kind: kind.into(),
        reason,
        post: post.into(),
        calls: vec![],
    }
}

fn postcondition(decl: &str, post: &str) -> Contract {
    let Some(analysis) = analyze(decl) else {
        return contract("safety-only", "unparsed", post);
    };
    let (kind, reason) = if analysis.atoms.iter().all(|a| *a == Atom::True) {
        ("safety-only", "trivial")
    } else if analysis.atoms.iter().all(|a| *a != Atom::Value) {
        ("safety-only", "size-only")
    } else {
        ("functional", "value")
    };
    Contract {
        calls: analysis.calls,
        ..contract(kind, reason, post)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Node {
    Word(String),
    Op(String),
    Hash,
    Group(char, Vec<Node>),
}

#[derive(PartialEq)]
enum Atom {
    True,
    Size,
    Value,
}

struct Analysis {
    atoms: Vec<Atom>,
    calls: Vec<String>,
}

type Types = HashMap<String, Vec<Node>>;

const KEYWORDS: [&str; 10] = [
    "let", "if", "match", "fun", "forall", "exists", "then", "else", "with", "in",
];
const CONNECTIVES: [&str; 5] = ["&&", "/\\", "||", "\\/", "==>"];
const COMPARISONS: [&str; 12] = [
    "=.", "<.", "<=.", ">.", ">=.", "<>.", "=", "<", "<=", ">", ">=", "<>",
];
const ARITHMETIC: [&str; 10] = ["+", "-", "*", "/", "%", "+!", "-!", "*!", "/!", "%!"];
const INTEGERS: [&str; 12] = [
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
];
const TRUE_WORDS: [&str; 4] = ["true", "True", "l_True", "Prims.l_True"];

fn analyze(decl: &str) -> Option<Analysis> {
    let tree = parse_tree(decl)?;
    let ensures = find_ensures(&tree)?;
    let mut analyzer = Analyzer {
        vars: parameter_types(&tree),
        signature_type: signature_type(&tree),
        atoms: vec![],
        calls: vec![],
    };
    analyzer.formula(&strip(ensures))?;
    analyzer.calls.sort();
    analyzer.calls.dedup();
    Some(Analysis {
        atoms: analyzer.atoms,
        calls: analyzer.calls,
    })
}

// ---- tokens and trees ------------------------------------------------------------------

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\''
}

fn is_symbol_char(c: char) -> bool {
    "=<>!+-*/%&|:\\~^$.@?;".contains(c)
}

fn closes(open: char, close: char) -> bool {
    matches!((open, close), ('(', ')') | ('[', ']') | ('{', '}'))
}

fn parse_tree(text: &str) -> Option<Vec<Node>> {
    let chars: Vec<char> = text.chars().collect();
    let mut stack: Vec<(char, Vec<Node>)> = vec![('\0', vec![])];
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '#' {
            push(&mut stack, Node::Hash);
            i += 1;
        } else if c == ',' {
            push(&mut stack, Node::Op(",".into()));
            i += 1;
        } else if matches!(c, '(' | '[' | '{') {
            stack.push((c, vec![]));
            i += 1;
        } else if matches!(c, ')' | ']' | '}') {
            let (open, items) = stack.pop().filter(|(open, _)| closes(*open, c))?;
            stack.last_mut()?.1.push(Node::Group(open, items));
            i += 1;
        } else if c == '"' {
            let end = chars[i + 1..].iter().position(|&q| q == '"')? + i + 2;
            push(&mut stack, Node::Word(chars[i..end].iter().collect()));
            i = end;
        } else if is_word_char(c) {
            let end = word_end(&chars, i);
            push(&mut stack, Node::Word(chars[i..end].iter().collect()));
            i = end;
        } else if is_symbol_char(c) {
            let end = i + chars[i..]
                .iter()
                .take_while(|&&q| is_symbol_char(q))
                .count();
            push(&mut stack, Node::Op(chars[i..end].iter().collect()));
            i = end;
        } else {
            return None;
        }
    }
    (stack.len() == 1).then(|| stack.pop().unwrap().1)
}

fn push(stack: &mut [(char, Vec<Node>)], node: Node) {
    stack.last_mut().unwrap().1.push(node);
}

fn word_end(chars: &[char], start: usize) -> usize {
    let mut end = start;
    while end < chars.len()
        && (is_word_char(chars[end])
            || (chars[end] == '.' && chars.get(end + 1).is_some_and(|&c| is_word_char(c))))
    {
        end += 1;
    }
    end
}

fn find_ensures(nodes: &[Node]) -> Option<&[Node]> {
    for node in nodes {
        let Node::Group(open, inner) = node else {
            continue;
        };
        if *open == '(' && inner.first() == Some(&Node::Word("ensures".into())) {
            return Some(&inner[1..]);
        }
        if let Some(found) = find_ensures(inner) {
            return Some(found);
        }
    }
    None
}

/// R1: drop `#T` implicit arguments, `<: T` ascriptions and `b2t`.
fn strip(nodes: &[Node]) -> Vec<Node> {
    let mut out = vec![];
    let mut i = 0;
    while i < nodes.len() {
        match &nodes[i] {
            Node::Hash => i += 2,
            Node::Word(w) if w == "b2t" || w.ends_with(".b2t") => i += 1,
            Node::Op(o) if o == "<:" => {
                i += 1;
                while i < nodes.len() && !ends_ascription(&nodes[i]) {
                    i += 1;
                }
            }
            Node::Group(open, inner) => {
                out.push(Node::Group(*open, strip(inner)));
                i += 1;
            }
            other => {
                out.push(other.clone());
                i += 1;
            }
        }
    }
    out
}

fn ends_ascription(node: &Node) -> bool {
    match node {
        Node::Word(w) => ["then", "else", "with", "in"].contains(&w.as_str()),
        Node::Op(o) => CONNECTIVES.contains(&o.as_str()),
        _ => false,
    }
}

fn is_op(node: &Node, ops: &[&str]) -> bool {
    matches!(node, Node::Op(o) if ops.contains(&o.as_str()))
}

fn is_word(node: &Node, word: &str) -> bool {
    matches!(node, Node::Word(w) if w == word)
}

fn position(nodes: &[Node], matches: impl Fn(&Node) -> bool) -> Option<usize> {
    nodes.iter().position(matches)
}

fn split_on<'a>(nodes: &'a [Node], separators: &[&str]) -> Vec<&'a [Node]> {
    let mut parts = vec![];
    let mut start = 0;
    for (i, node) in nodes.iter().enumerate() {
        if is_op(node, separators) {
            parts.push(&nodes[start..i]);
            start = i + 1;
        }
    }
    parts.push(&nodes[start..]);
    parts
}

// ---- signature: parameter and result types ----------------------------------------------

fn parameter_types(tree: &[Node]) -> Types {
    let mut types = Types::new();
    for node in tree.iter().take_while(|n| !is_op(n, &[":"])) {
        let Node::Group('(', inner) = node else {
            continue;
        };
        let Some(colon) = position(inner, |n| is_op(n, &[":"])) else {
            continue;
        };
        let ty = strip(&inner[colon + 1..]);
        for name in strip(&inner[..colon]) {
            if let Node::Word(name) = name {
                types.insert(name, ty.clone());
            }
        }
    }
    types
}

fn signature_type(tree: &[Node]) -> Vec<Node> {
    let pure = position(tree, |n| is_word(n, "Prims.Pure"));
    pure.and_then(|i| tree.get(i + 1))
        .map(|ty| strip(std::slice::from_ref(ty)))
        .unwrap_or_default()
}

fn unwrap_parens(ty: &[Node]) -> &[Node] {
    match ty {
        [Node::Group('(', inner)] => unwrap_parens(inner),
        _ => ty,
    }
}

fn is_integer_type(ty: &[Node]) -> bool {
    let word = match unwrap_parens(ty) {
        [Node::Word(w)] | [Node::Word(w), Node::Group('{', _)] => w,
        _ => return false,
    };
    matches!(word.as_str(), "usize" | "isize" | "Hax_lib.Int.t_Int")
}

fn last_segment(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

// ---- formulas ---------------------------------------------------------------------------

struct Analyzer {
    vars: Types,
    signature_type: Vec<Node>,
    atoms: Vec<Atom>,
    calls: Vec<String>,
}

impl Analyzer {
    fn formula(&mut self, nodes: &[Node]) -> Option<()> {
        match nodes.first()? {
            Node::Word(w) if w == "fun" => self.function(nodes),
            Node::Word(w) if w == "let" => self.rebinding(nodes),
            Node::Word(w) if w == "if" => self.conditional(nodes),
            Node::Word(w) if w == "match" => self.matching(nodes),
            _ => self.connected(nodes),
        }
    }

    fn function(&mut self, nodes: &[Node]) -> Option<()> {
        let arrow = position(nodes, |n| is_op(n, &["->"]))?;
        let [_, Node::Word(binder)] = &nodes[..arrow] else {
            return None;
        };
        self.vars
            .insert(binder.clone(), self.signature_type.clone());
        self.formula(&nodes[arrow + 1..])
    }

    /// `let x:T = x in BODY`: only rebinding of an existing variable is scaffolding.
    fn rebinding(&mut self, nodes: &[Node]) -> Option<()> {
        let end = position(nodes, |n| is_word(n, "in"))?;
        let equals = position(&nodes[..end], |n| is_op(n, &["="]))?;
        let Node::Word(name) = nodes.get(1)? else {
            return None;
        };
        let [Node::Word(value)] = &nodes[equals + 1..end] else {
            return None;
        };
        let inherited = self.vars.get(value)?.clone();
        let declared = match &nodes[2..equals] {
            [] => inherited,
            [colon, ty @ ..] if is_op(colon, &[":"]) => ty.to_vec(),
            _ => return None,
        };
        self.vars.insert(name.clone(), declared);
        self.formula(&nodes[end + 1..])
    }

    fn conditional(&mut self, nodes: &[Node]) -> Option<()> {
        let then = position(nodes, |n| is_word(n, "then"))?;
        let otherwise = matching_else(nodes, then)?;
        self.formula(&nodes[1..then])?;
        self.formula(&nodes[then + 1..otherwise])?;
        self.formula(&nodes[otherwise + 1..])
    }

    fn matching(&mut self, nodes: &[Node]) -> Option<()> {
        let with = position(nodes, |n| is_word(n, "with"))?;
        let scrutinee = &nodes[1..with];
        let scrutinee_type = match scrutinee {
            [Node::Word(w)] => Some(self.vars.get(w)?.clone()),
            _ => {
                self.atom(scrutinee)?;
                None
            }
        };
        for arm in split_on(&nodes[with + 1..], &["|"]).into_iter().skip(1) {
            let arrow = position(arm, |n| is_op(n, &["->"]))?;
            self.bind_pattern(&arm[..arrow], scrutinee_type.as_deref())?;
            self.formula(&arm[arrow + 1..])?;
        }
        Some(())
    }

    fn connected(&mut self, nodes: &[Node]) -> Option<()> {
        let parts = split_on(nodes, &CONNECTIVES);
        if parts.len() > 1 {
            return parts.into_iter().try_for_each(|p| self.formula(p));
        }
        match nodes {
            [Node::Group('(', inner)] => self.formula(inner),
            _ => self.atom(nodes),
        }
    }

    // ---- match patterns ----

    fn bind_pattern(&mut self, pattern: &[Node], scrutinee_type: Option<&[Node]>) -> Option<()> {
        match pattern {
            [Node::Word(_)] => Some(()),
            [Node::Word(ctor), payload] => {
                let payload_type = scrutinee_type.and_then(|t| payload_type(ctor, t));
                self.bind_payload(payload, payload_type)
            }
            _ => None,
        }
    }

    fn bind_payload(&mut self, payload: &Node, ty: Option<Vec<Node>>) -> Option<()> {
        let names = match payload {
            Node::Word(name) => vec![vec![Node::Word(name.clone())]],
            Node::Group('(', inner) if inner.is_empty() => vec![],
            Node::Group('(', inner) => split_on(inner, &[","]).iter().map(|p| p.to_vec()).collect(),
            _ => return None,
        };
        let types = component_types(ty, names.len())?;
        for (name, ty) in names.iter().zip(types) {
            let [Node::Word(name)] = name.as_slice() else {
                return None;
            };
            self.vars.insert(name.clone(), ty);
        }
        Some(())
    }

    // ---- atoms ----

    fn atom(&mut self, nodes: &[Node]) -> Option<()> {
        if contains_keyword(nodes) {
            return None;
        }
        let atom = match nodes {
            [Node::Word(w)] if TRUE_WORDS.contains(&w.as_str()) => Atom::True,
            _ if self.is_size_comparison(nodes) => Atom::Size,
            _ => Atom::Value,
        };
        self.atoms.push(atom);
        self.collect_calls(nodes);
        Some(())
    }

    fn collect_calls(&mut self, nodes: &[Node]) {
        for node in nodes {
            match node {
                Node::Group(_, inner) => self.collect_calls(inner),
                Node::Word(w) if self.is_function_name(w) => self.calls.push(w.clone()),
                _ => {}
            }
        }
    }

    fn is_function_name(&self, word: &str) -> bool {
        let first = word.split('.').next().unwrap_or(word);
        !(is_number(word)
            || word.starts_with('"')
            || self.vars.contains_key(first)
            || TRUE_WORDS.contains(&word)
            || word == "false"
            || is_length_function(word)
            || is_arithmetic_function(word))
    }

    fn is_size_comparison(&self, nodes: &[Node]) -> bool {
        let at: Vec<_> = (0..nodes.len())
            .filter(|&i| is_op(&nodes[i], &COMPARISONS))
            .collect();
        let [at] = at.as_slice() else {
            return false;
        };
        self.is_size_expression(&nodes[..*at]) && self.is_size_expression(&nodes[at + 1..])
    }

    fn is_size_expression(&self, nodes: &[Node]) -> bool {
        split_on(nodes, &ARITHMETIC)
            .iter()
            .all(|term| self.is_size_term(term))
    }

    fn is_size_term(&self, nodes: &[Node]) -> bool {
        match nodes {
            [Node::Group('(', inner)] => self.is_size_expression(inner),
            [Node::Word(w)] => {
                is_number(w) || self.is_integer_variable(w) || self.is_range_field(w)
            }
            [Node::Word(head), args @ ..] if is_length_function(head) => {
                args.iter().all(|a| self.is_length_argument(a))
            }
            [Node::Word(head), arg] if is_machine_literal(head) => is_number_node(arg),
            [Node::Word(head), args @ ..] if last_segment(head) == "from_machine" => {
                args.len() == 1 && self.is_size_term(args)
            }
            _ => false,
        }
    }

    fn is_integer_variable(&self, word: &str) -> bool {
        self.vars.get(word).is_some_and(|ty| is_integer_type(ty))
    }

    fn is_range_field(&self, word: &str) -> bool {
        word.rsplit_once('.').is_some_and(|(owner, field)| {
            (field == "f_start" || field == "f_end") && self.vars.contains_key(owner)
        })
    }

    fn is_length_argument(&self, node: &Node) -> bool {
        match node {
            Node::Word(w) => self.vars.contains_key(w.split('.').next().unwrap_or(w)),
            Node::Group('(', inner) => {
                matches!(inner.as_slice(), [n] if self.is_length_argument(n))
            }
            _ => false,
        }
    }
}

fn matching_else(nodes: &[Node], then: usize) -> Option<usize> {
    let mut nested = 0;
    for (i, node) in nodes.iter().enumerate().skip(then + 1) {
        if is_word(node, "if") {
            nested += 1;
        } else if is_word(node, "else") && nested == 0 {
            return Some(i);
        } else if is_word(node, "else") {
            nested -= 1;
        }
    }
    None
}

fn contains_keyword(nodes: &[Node]) -> bool {
    nodes.iter().any(|node| match node {
        Node::Word(w) => KEYWORDS.contains(&w.as_str()),
        Node::Group(_, inner) => contains_keyword(inner),
        _ => false,
    })
}

fn payload_type(constructor: &str, ty: &[Node]) -> Option<Vec<Node>> {
    let [Node::Word(head), args @ ..] = unwrap_parens(ty) else {
        return None;
    };
    let index = match (last_segment(constructor), last_segment(head)) {
        ("Result_Ok", "t_Result") | ("Option_Some" | "Some", "t_Option") => 0,
        ("Result_Err", "t_Result") => 1,
        _ => return None,
    };
    args.get(index).map(|arg| vec![arg.clone()])
}

/// One type per binder: a tuple type `A & B & C` splits on `&`; unknown types stay unknown.
fn component_types(ty: Option<Vec<Node>>, count: usize) -> Option<Vec<Vec<Node>>> {
    let Some(ty) = ty else {
        return Some(vec![vec![]; count]);
    };
    if count == 0 {
        return Some(vec![]);
    }
    if count == 1 {
        return Some(vec![ty]);
    }
    let parts: Vec<_> = split_on(unwrap_parens(&ty), &["&"])
        .iter()
        .map(|p| p.to_vec())
        .collect();
    (parts.len() == count).then_some(parts)
}

fn is_number(word: &str) -> bool {
    word.starts_with(|c: char| c.is_ascii_digit())
}

fn is_number_node(node: &Node) -> bool {
    matches!(node, Node::Word(w) if is_number(w))
}

fn is_machine_literal(word: &str) -> bool {
    last_segment(word)
        .strip_prefix("mk_")
        .is_some_and(|ty| ty == "isize" || INTEGERS.contains(&ty))
}

fn is_length_function(word: &str) -> bool {
    let name = last_segment(word);
    name == "len" || name == "length" || (name.starts_with("impl_") && name.ends_with("__len"))
}

fn is_arithmetic_function(word: &str) -> bool {
    is_machine_literal(word) || last_segment(word) == "from_machine"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sig(params: &str, ret: &str, ensures: &str) -> String {
        format!("val f {params}\n    : Prims.Pure {ret}\n      Prims.l_True\n      (ensures\n        {ensures})")
    }

    fn kind_and_reason(decl: &str) -> (String, &'static str) {
        let c = classify(decl);
        (c.kind, c.reason)
    }

    #[test]
    fn declarations_without_a_postcondition() {
        let none = "val f : Prims.Pure result Prims.l_True (fun _ -> Prims.l_True)";
        assert_eq!(
            kind_and_reason(none),
            ("safety-only".into(), "no-postcondition")
        );
        assert_eq!(classify("val g : x -> Tot y").reason, "no-postcondition");
        assert_eq!(classify("val g : Foo").kind, "unclassified");
    }

    #[test]
    fn trivially_true_postcondition() {
        let decl = sig("(x: u8)", "usize", "fun _ -> Prims.l_True");
        assert_eq!(kind_and_reason(&decl), ("safety-only".into(), "trivial"));
        let decl = sig(
            "(x: u8)",
            "usize",
            "fun temp_0_ -> let _:Prims.unit = temp_0_ in true",
        );
        assert_eq!(kind_and_reason(&decl), ("safety-only".into(), "trivial"));
    }

    #[test]
    fn rule_one_normalization_strips_scaffolding() {
        let decl = sig(
            "(b: t_Bytes)",
            "(Core.Result.t_Result (t_Bytes & usize) u8)",
            "fun result ->\n let result:Core.Result.t_Result (t_Bytes & usize) u8 = result in\n \
             match result <: Core.Result.t_Result (t_Bytes & usize) u8 with\n \
             | Core.Result.Result_Ok (out, n) -> b2t (n <=. (impl_Bytes__len #u8 b <: usize) <: bool)\n \
             | Core.Result.Result_Err _ -> true",
        );
        assert_eq!(kind_and_reason(&decl), ("safety-only".into(), "size-only"));
    }

    #[test]
    fn rule_two_length_and_range_facts_are_size_atoms() {
        for ensures in [
            "fun result -> let result:usize = result in result <=. mk_usize 64",
            "fun result -> let result:t_Bytes = result in (impl_Bytes__len result <: usize) =. mk_usize 2",
            "fun result -> let result:t_Bytes = result in (impl_Bytes__len result <: usize) -! mk_usize 2 =. (impl_Bytes__len b <: usize)",
            "fun result -> let result:Hax_lib.Int.t_Int = result in result >= (Rust_primitives.Hax.Int.from_machine (mk_u8 1) <: Hax_lib.Int.t_Int)",
            "fun result -> let result:t_Range = result in result.f_start <=. result.f_end",
            "fun result -> let result:usize = result in if result <. mk_usize 32 then result >=. mk_usize 1 else true",
        ] {
            let decl = sig("(b: t_Bytes)", "usize", ensures);
            assert_eq!(kind_and_reason(&decl), ("safety-only".into(), "size-only"), "{ensures}");
        }
    }

    #[test]
    fn rule_three_value_facts_are_functional() {
        for ensures in [
            "fun result -> let result:t_Bytes = result in result =. b",
            "fun result -> let result:usize = result in result <=. mk_usize 4 && (b.[ result ] <: u8) <>. mk_u8 0",
            "fun result -> let result:usize = result in result <=. v_LIMIT",
            "fun result -> let result:usize = result in is_valid b result",
            "fun result -> let result:usize = result in (impl_Bytes__len (concat b b) <: usize) =. result",
        ] {
            let decl = sig("(b: t_Bytes)", "usize", ensures);
            assert_eq!(kind_and_reason(&decl), ("functional".into(), "value"), "{ensures}");
        }
    }

    #[test]
    fn variables_are_size_only_when_integer_typed() {
        let decl = sig(
            "(n: usize) (m: usize{v m < 4}) (b: t_Bytes)",
            "usize",
            "fun r -> let r:usize = r in r <=. n +! m",
        );
        assert_eq!(classify(&decl).reason, "size-only");
        let decl = sig(
            "(b: t_Bytes) (c: t_Bytes)",
            "usize",
            "fun r -> let r:usize = r in b =. c",
        );
        assert_eq!(classify(&decl).reason, "value");
        let decl = sig(
            "(b1: u8) (b2: u8)",
            "(Core.Result.t_Result Prims.unit u8)",
            "fun result -> match result with | Core.Result.Result_Ok _ -> b1 =. b2 | _ -> true",
        );
        assert_eq!(kind_and_reason(&decl), ("functional".into(), "value"));
    }

    #[test]
    fn match_arm_binders_are_typed_from_the_result_type() {
        let decl = sig(
            "(b: t_Bytes)",
            "(Core.Result.t_Result (t_Bytes & usize) u8)",
            "fun result -> match result with | Core.Result.Result_Ok (out, n) -> out =. b | _ -> true",
        );
        assert_eq!(classify(&decl).reason, "value");
        let decl = sig(
            "(b: t_Bytes)",
            "(Core.Result.t_Result (t_Bytes & usize) u8)",
            "fun result -> match result with | Core.Result.Result_Ok (out, n) -> n <=. mk_usize 3 | _ -> true",
        );
        assert_eq!(classify(&decl).reason, "size-only");
    }

    #[test]
    fn calls_are_collected_for_self_consistency() {
        let decl = sig(
            "(b: t_Bytes)",
            "(Core.Result.t_Result t_Bytes u8)",
            "fun result -> match result with | Core.Result.Result_Ok v -> parse_it b v | _ -> true",
        );
        assert_eq!(classify(&decl).calls, vec!["parse_it"]);
    }

    #[test]
    fn unreadable_postconditions_are_unparsed_never_functional() {
        for ensures in [
            "fun result -> forall (i: nat). i < 3 ==> true",
            "fun result -> let y = f result in y =. result",
            "fun result -> match result with | Some (Some x) -> true | _ -> true",
            "fun result -> if result then true",
            "fun result -> result =. `",
        ] {
            let decl = sig("(b: t_Bytes)", "usize", ensures);
            assert_eq!(
                kind_and_reason(&decl),
                ("safety-only".into(), "unparsed"),
                "{ensures}"
            );
        }
        assert_eq!(
            classify("val f : Prims.Pure x (ensures fun r -> (r").reason,
            "unparsed"
        );
    }
}
