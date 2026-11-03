use crate::{Category, Report, STATUSES};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Totals {
    pub sloc: usize,
    pub statuses: BTreeMap<String, usize>,
    pub unknown_dependency_sizes: usize,
}
impl Default for Totals {
    fn default() -> Self {
        Self {
            sloc: 0,
            statuses: STATUSES.iter().map(|s| (s.to_string(), 0)).collect(),
            unknown_dependency_sizes: 0,
        }
    }
}
impl Totals {
    pub fn percent(&self, status: &str) -> f64 {
        if self.sloc == 0 {
            return 0.;
        }
        *self.statuses.get(status).unwrap_or(&0) as f64 * 100. / self.sloc as f64
    }
    fn add(&mut self, sloc: usize, status: &str, unknown: bool) {
        self.sloc += sloc;
        *self.statuses.entry(status.into()).or_default() += sloc;
        self.unknown_dependency_sizes += usize::from(unknown);
    }
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Split {
    pub security_relevant: Totals,
    pub everything_else: Totals,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProofTotals {
    pub sloc: usize,
    pub units: usize,
    pub verified: usize,
    pub unverified: usize,
    pub hazard_units: usize,
}
impl ProofTotals {
    fn add(&mut self, u: &crate::Unit) {
        self.sloc += u.sloc;
        self.units += 1;
        self.verified += usize::from(u.status == "verified");
        self.unverified += usize::from(u.status != "verified");
        self.hazard_units += usize::from(u.status == "hazard");
    }
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProofCode {
    pub total: ProofTotals,
    pub security_relevant: ProofTotals,
    pub everything_else: ProofTotals,
}
/// First-party structural test code, excluded from runtime totals.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestTotals {
    pub sloc: usize,
    pub units: usize,
    pub security_reachable_sloc: usize,
    pub security_reachable_units: usize,
}
impl TestTotals {
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Summary {
    pub security_first_party_hazard_units: usize,
    pub trusted_items: usize,
    pub trusted_sloc: usize,
    pub escape_sites: usize,
    pub proof_code: ProofCode,
    #[serde(default, skip_serializing_if = "TestTotals::is_empty")]
    pub test_code: TestTotals,
    pub all: Split,
    pub first_party: Split,
    pub third_party: Split,
}
fn side(split: &mut Split, security: bool) -> &mut Totals {
    if security {
        &mut split.security_relevant
    } else {
        &mut split.everything_else
    }
}
pub fn summarize(r: &Report) -> Summary {
    summarize_rules(r, false)
}
/// The same figures under default rules only: every explicit input undone, as if the
/// config had no `[security.*]`, `[dependencies.*]` or schema-2 override entries.
pub fn summarize_default_rules(r: &Report) -> Summary {
    summarize_rules(r, true)
}
fn summarize_rules(r: &Report, defaults: bool) -> Summary {
    let mut summary = Summary {
        trusted_items: r.trusted_items.len(),
        trusted_sloc: r.trusted_items.iter().map(|t| t.sloc).sum(),
        escape_sites: r.escape_hatches.len(),
        ..Summary::default()
    };
    for u in &r.units {
        let security = if defaults {
            u.default_security.unwrap_or(u.security)
        } else {
            u.security
        };
        summary.security_first_party_hazard_units += usize::from(security && u.status == "hazard");
        if u.category == Category::ProofCode {
            summary.proof_code.total.add(u);
            if security {
                summary.proof_code.security_relevant.add(u);
            } else {
                summary.proof_code.everything_else.add(u);
            }
            continue;
        }
        if u.category == Category::Test {
            summary.test_code.sloc += u.sloc;
            summary.test_code.units += 1;
            if security {
                summary.test_code.security_reachable_sloc += u.sloc;
                summary.test_code.security_reachable_units += 1;
            }
            continue;
        }
        side(&mut summary.all, security).add(u.sloc, &u.status, false);
        side(&mut summary.first_party, security).add(u.sloc, &u.status, false);
    }
    for d in &r.dependencies {
        let (security, sloc, known) = if defaults {
            (
                d.default_security.unwrap_or(d.security),
                d.default_sloc.unwrap_or(d.sloc),
                d.size_known && !d.default_size_unknown,
            )
        } else {
            (d.security, d.sloc, d.size_known)
        };
        side(&mut summary.all, security).add(sloc, &d.status, !known);
        side(&mut summary.third_party, security).add(sloc, &d.status, !known);
    }
    summary
}
fn headline(label: &str, totals: &Totals, trust: &str) -> String {
    headline_unit(label, totals, trust, "SLoC")
}
fn headline_unit(label: &str, totals: &Totals, trust: &str, unit: &str) -> String {
    format!(
        "{label}  {} measured {unit}   {}{}; {trust}",
        totals.sloc,
        STATUSES
            .iter()
            .map(|s| format!("{s} {:.2}%", totals.percent(s)))
            .collect::<Vec<_>>()
            .join("  "),
        if totals.unknown_dependency_sizes > 0 {
            format!(
                "  (INCOMPLETE: {} dependency sizes unknown)",
                totals.unknown_dependency_sizes
            )
        } else {
            String::new()
        }
    )
}
fn linked(r: &Report) -> bool {
    r.size_mode == crate::SizeMode::LinkedLines.as_str()
}
/// First Markdown line: how runtime dependencies were sized.
fn size_line(r: &Report) -> String {
    format!(
        "Size mode: `{}`; target: `{}`; profile: `{}`; binary sha256: {}.{}",
        cell(&r.size_mode),
        cell(&r.target),
        cell(&r.size_profile),
        r.binary_sha256
            .as_deref()
            .map(|s| format!("`{s}`"))
            .unwrap_or_else(|| "n/a".into()),
        if linked(r) {
            " Third-party sizes are linked lines (distinct DWARF file:line pairs), not SLoC."
        } else {
            ""
        }
    )
}
fn fallback_block(r: &Report) -> String {
    if r.size_fallbacks.is_empty() {
        return String::new();
    }
    format!(
        "**SIZE FALLBACKS: {} runtime dependencies keep whole-crate sizes because {} evidence was missing:** {}\n\n",
        r.size_fallbacks.len(),
        cell(&r.size_mode),
        r.size_fallbacks
            .iter()
            .map(|f| format!("{}@{} ({})", cell(&f.name), cell(&f.version), cell(&f.reason)))
            .collect::<Vec<_>>()
            .join("; ")
    )
}
fn dependency_sizes(r: &Report) -> String {
    if r.size_mode.is_empty() || r.size_mode == crate::SizeMode::WholeCrate.as_str() {
        return String::new();
    }
    let unit = if linked(r) { "linked lines" } else { "SLoC" };
    let mut out = format!("## Dependency sizes\n\nRuntime dependencies measured in `{}` mode. Size is in {unit}; whole-crate SLoC is the v1.4.2 measure, kept for comparison. Fallbacks keep the whole-crate size.\n\n| Crate | Size mode | Size ({unit}) | Whole-crate SLoC | Note |\n| --- | --- | ---: | ---: | --- |\n", cell(&r.size_mode));
    for d in &r.dependencies {
        out.push_str(&format!(
            "| {}@{} | {} | {} | {} | {} |\n",
            cell(&d.name),
            cell(&d.version),
            cell(&d.size_mode),
            if d.size_known {
                d.sloc.to_string()
            } else {
                "unknown".into()
            },
            d.whole_crate_sloc,
            cell(d.size_reason.as_deref().unwrap_or(""))
        ));
    }
    out.push('\n');
    out
}
pub fn markdown(r: &Report) -> String {
    let s = summarize(r);
    let trust = format!(
        "proven, relying on {} trusted items ({} SLoC) and {} escapes",
        s.trusted_items, s.trusted_sloc, s.escape_sites
    );
    let trust = if r.evidence_runs.iter().any(|run| run.tool == "hax-fstar") {
        format!("{trust}; proven-extracted relies on trusted hax extraction")
    } else {
        trust
    };
    let (third_unit, combined_unit) = if linked(r) {
        ("linked lines", "SLoC + linked lines")
    } else {
        ("SLoC", "SLoC")
    };
    let d = summarize_default_rules(r);
    let mut lines = Vec::new();
    // Each security figure twice: with explicit inputs, then under default rules only.
    for (origin, unit, with, without) in [
        (
            "First-party",
            "SLoC",
            &s.first_party.security_relevant,
            &d.first_party.security_relevant,
        ),
        (
            "Third-party (runtime)",
            third_unit,
            &s.third_party.security_relevant,
            &d.third_party.security_relevant,
        ),
        (
            "Combined",
            combined_unit,
            &s.all.security_relevant,
            &d.all.security_relevant,
        ),
    ] {
        lines.push(headline_unit(
            &format!("SECURITY-RELEVANT {origin} (with inputs)"),
            with,
            &trust,
            unit,
        ));
        lines.push(headline_unit(
            &format!("SECURITY-RELEVANT {origin} (default rules only)"),
            without,
            &trust,
            unit,
        ));
    }
    lines.push(headline(
        "EVERYTHING ELSE First-party",
        &s.first_party.everything_else,
        &trust,
    ));
    lines.push(headline_unit(
        "EVERYTHING ELSE Third-party (runtime)",
        &s.third_party.everything_else,
        &trust,
        third_unit,
    ));
    lines.push(headline_unit(
        "EVERYTHING ELSE Combined",
        &s.all.everything_else,
        &trust,
        combined_unit,
    ));
    let mut out = format!("# Formal coverage\n\n```text\n{}\n```\n\nPercentages are SLoC-weighted, mutually exclusive statuses over measured runtime code. First-party, third-party runtime and combined figures are also in JSON summary.first_party, summary.third_party and summary.all for each block. Headline trust/escape counts conservatively list the whole runtime inventory, including outside verified scope; each receipt lists its cfg-active scope separately. Unknown sizes are excluded and prominently flagged; gates fail closed when security dependency sizes are unknown.\n\n{}\n\n", lines.join("\n"), r.approximation);
    if s.escape_sites > 0 {
        out = out.replacen("# Formal coverage\n\n", &format!("# Formal coverage\n\n**ESCAPES: {} distinct sites, including outside verified scope; all remain gate-visible.**\n\n",s.escape_sites),1);
    }
    if !r.dependency_inventory_complete {
        out = out.replacen("# Formal coverage\n\n", "# Formal coverage\n\n**INCOMPLETE: declared dependencies are missing from the lockfile inventory; percentages cover measured code only.**\n\n",1);
    }
    if s.security_first_party_hazard_units > 0 {
        out = out.replacen("# Formal coverage\n\n", &format!("# Formal coverage\n\n**SECURITY HAZARD: {} first-party security-relevant unit(s) weaken guarantees.** See file:line details below.\n\n",s.security_first_party_hazard_units),1);
    }
    out.push_str(&format!(
        "Target: `{}`; Cargo: `{}`.\n\nDev-dependencies excluded: {} packages{}.\n\n",
        cell(&r.target),
        cell(&r.cargo_version),
        r.dev_dependencies_excluded,
        if r.closure == "lockfile (fallback)" {
            " (exclusions unconfirmed on fallback)"
        } else {
            ""
        }
    ));
    out.push_str(&inputs_section(r, &s, &d));
    out.push_str("## Build-time dependencies\n\nBuild-time code can affect the binary and is trusted, not measured by runtime coverage. Build scripts, proc macros and their dependencies are excluded from runtime denominators; their source SLoC and unsafe counts are listed separately. A package used at runtime and build time appears in both inventories.\n\n| Crate | SLoC | Unsafe sites |\n| --- | ---: | ---: |\n");
    for d in &r.build_dependencies {
        out.push_str(&format!(
            "| {}@{} | {} | {} |\n",
            cell(&d.name),
            cell(&d.version),
            if d.size_known {
                d.sloc.to_string()
            } else {
                "unknown".into()
            },
            d.unsafe_sites
                .map(|n| n.to_string())
                .unwrap_or_else(|| "unconfirmed".into())
        ));
    }
    out.push_str(&dependency_sizes(r));
    out.push_str(&format!("Combined runtime security: {}/{} proven {combined_unit} ({:.2}%); {trust}; proof-code: {} SLoC alongside, excluded from both runtime denominators.\n\n## PROOF CODE\n\n| Classification | SLoC | Units | verified | unverified (including hazards, trusted and bounded) | Hazard units |\n| --- | ---: | ---: | ---: | ---: | ---: |\n",
        s.all.security_relevant.statuses["proven"], s.all.security_relevant.sloc,
        s.all.security_relevant.percent("proven"), s.proof_code.total.sloc));
    for (label, totals) in [
        ("Total", &s.proof_code.total),
        ("SECURITY-RELEVANT", &s.proof_code.security_relevant),
        ("EVERYTHING ELSE", &s.proof_code.everything_else),
    ] {
        out.push_str(&format!(
            "| {label} | {} | {} | {} | {} | {} |\n",
            totals.sloc, totals.units, totals.verified, totals.unverified, totals.hazard_units
        ));
    }
    if !s.test_code.is_empty() {
        out.push_str(&format!("\nTEST CODE: {} SLoC in {} first-party units ({} SLoC in {} units reachable from security entry points), detected structurally and excluded from both runtime denominators.\n",
            s.test_code.sloc, s.test_code.units, s.test_code.security_reachable_sloc, s.test_code.security_reachable_units));
    }
    if r.escape_hatches
        .iter()
        .any(|e| e.first_party && e.unit.ends_with("::<file-scope>"))
    {
        out = out.replacen("# Formal coverage\n\n", "# Formal coverage\n\n**FILE-SCOPE HAZARD: unattributed escapes can invalidate runtime proofs and block gates.**\n\n", 1);
    }
    if s.proof_code.total.hazard_units > 0 {
        out = out.replacen("# Formal coverage\n\n", "# Formal coverage\n\n**PROOF-CODE HAZARD: proof-side escapes can poison runtime proofs and block gates, regardless of security classification.**\n\n", 1);
    }
    out.push_str("\n| Origin | Security SLoC | Security proven | Everything else SLoC | Everything else proven | Trust / escapes |\n| --- | ---: | ---: | ---: | ---: | --- |\n");
    for (label, split) in [
        ("First-party", &s.first_party),
        ("Third-party", &s.third_party),
    ] {
        out.push_str(&format!(
            "| {label} | {} | {:.2}% | {} | {:.2}% | {trust} |\n",
            split.security_relevant.sloc,
            split.security_relevant.percent("proven"),
            split.everything_else.sloc,
            split.everything_else.percent("proven")
        ));
    }
    out.push_str(&crate::ratio::markdown(r));
    out.push_str("\n## Units\n\n| Function / crate | Location | SLoC | Classification | Status | Proof scope | Third-party unsafe sites |\n| --- | --- | ---: | --- | --- | --- | ---: |\n");
    for u in &r.units {
        out.push_str(&format!(
            "| {} | {}:{} | {} | {} | {} | {} | n/a |\n",
            cell(&u.id),
            cell(&u.file),
            u.line,
            u.sloc,
            match u.category {
                Category::ProofCode => "proof-code",
                Category::Test => "test-code",
                Category::Runtime => label(u.security),
            },
            u.status,
            cell(u.proof_scope.as_deref().unwrap_or("none"))
        ));
    }
    for d in &r.dependencies {
        out.push_str(&format!(
            "| {} {} | third-party crate | {} | {} | {} | none | {} |\n",
            cell(&d.name),
            cell(&d.version),
            if d.size_known {
                d.sloc.to_string()
            } else {
                "unknown".into()
            },
            label(d.security),
            d.status,
            d.unsafe_sites
                .map(|n| n.to_string())
                .unwrap_or_else(|| "unconfirmed".into())
        ));
    }
    out.push_str("\n## Trusted items\n\n");
    for t in &r.trusted_items {
        out.push_str(&format!(
            "- {}:{} — {} — {} SLoC — affected units: {}\n",
            cell(&t.file),
            t.line,
            cell(&t.kind),
            t.sloc,
            t.affected_units
                .iter()
                .map(|s| cell(s))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    out.push_str("\n## Escape hatches\n\n");
    if r.escape_hatches.is_empty() {
        out.push_str("None detected.\n");
    }
    for e in &r.escape_hatches {
        out.push_str(&format!(
            "- {}:{} — {} — {} — {}\n",
            cell(&e.file),
            e.line,
            cell(&e.kind),
            cell(&if e.affected_units.is_empty() {
                e.unit.clone()
            } else {
                e.affected_units.join(", ")
            }),
            label(e.security)
        ));
    }
    out.push_str("\n## Explicit overrides\n\n");
    if r.overrides.is_empty() {
        out.push_str("None.\n");
    }
    for o in &r.overrides {
        out.push_str(&format!(
            "- `{}` → {}: {}\n",
            cell(&o.pattern),
            label(o.security),
            o.matched
                .iter()
                .map(|s| cell(s))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    out.push_str("\n## Upstream claims\n\n");
    if r.upstream_claims.is_empty() {
        out.push_str("None matched.\n");
    }
    for c in &r.upstream_claims {
        out.push_str(&format!(
            "- {} {} — **{}** — {}. {}. Tool: {}. [Primary source]({}). {}\n",
            cell(&c.crate_name),
            cell(&c.version),
            cell(&c.confirmation),
            cell(&c.operations),
            cell(&c.property),
            cell(&c.tool),
            c.source,
            if c.whole_crate && c.confirmation == "confirmed" {
                "Confirmed whole-crate upstream scope."
            } else {
                "Whole crate remains unverified."
            }
        ));
    }
    let models: Vec<_> = r
        .warnings
        .iter()
        .filter(|w| w.starts_with("Upstream model (uncredited):"))
        .collect();
    if !models.is_empty() {
        out.push_str("\n## Upstream claims, uncredited\n\nProVerif and SSProve artifacts model the protocol/key schedule; this run gives them no Rust line credit.\n\n");
        for model in models {
            out.push_str(&format!("- {}\n", cell(model)));
        }
    }
    if !r.fstar_definitions.is_empty() {
        out.push_str("\n## hax/F* definitions\n\nSelf-consistency is a heuristic: consistent with its own parser; not checked against an external spec. Postconditions are shown verbatim from checked interfaces. Patch differences conservatively trust the whole module.\n\n| Definition | F* location | Rust unit | Classification | Contract kind | Self-consistency heuristic | Postcondition |\n| --- | --- | --- | --- | --- | --- | --- |\n");
        for d in &r.fstar_definitions {
            out.push_str(&format!(
                "| {} | {}:{} | {} | {} | {} | {} | {} |\n",
                cell(&d.name),
                cell(&d.file),
                d.line,
                cell(d.rust_unit.as_deref().unwrap_or("unmapped")),
                cell(&d.classification),
                cell(&d.contract_kind),
                d.self_consistency_heuristic,
                cell(&d.postcondition)
            ));
        }
    }
    out.push_str("\n## Run receipts\n\n");
    for run in &r.evidence_runs {
        out.push_str(&format!(
            "- {} `{}`: {}. Credited units: {}. {}\n",
            cell(&run.tool),
            cell(&run.log),
            if run.accepted { "accepted" } else { "rejected" },
            run.credited_units
                .iter()
                .map(|u| cell(u))
                .collect::<Vec<_>>()
                .join(", "),
            run.reasons
                .iter()
                .map(|r| cell(r))
                .collect::<Vec<_>>()
                .join("; ")
        ));
        out.push_str(&format!("  Scope: {} files; cfg: {}. Proven, relying on {} trusted items ({} SLoC) and {} escapes in this run. Trusted sites: {}. Escape sites: {}.\n",
            run.verified_files.len(), cell(&run.cfg.join(", ")), run.trusted_sites.len(),run.trusted_sloc,run.escape_sites.len(),
            run.trusted_sites.iter().map(|(f,l,k)|format!("{}:{l} {}",cell(f),cell(k))).collect::<Vec<_>>().join("; "),
            run.escape_sites.iter().map(|(f,l,k)|format!("{}:{l} {}",cell(f),cell(k))).collect::<Vec<_>>().join("; ")));
    }
    out.push_str("\n## Evidence and limitations\n\n");
    for w in &r.warnings {
        out.push_str(&format!("- {}\n", cell(w)));
    }
    out = out.replacen(
        "# Formal coverage\n\n",
        &format!(
            "# Formal coverage\n\nclosure: {}{}\n\n",
            cell(&r.closure),
            r.closure_reason
                .as_ref()
                .map(|reason| format!(" — {}", cell(reason)))
                .unwrap_or_default()
        ),
        1,
    );
    format!(
        "{} Ghost lines: `{}`;\n\n{}{}\n\n{out}",
        size_line(r),
        r.ghost_lines.as_str(),
        fallback_block(r),
        lines[0]
    )
}
fn inputs_section(r: &Report, s: &Summary, d: &Summary) -> String {
    let (a, b) = (
        &s.first_party.security_relevant,
        &d.first_party.security_relevant,
    );
    let mut out = format!(
        "## Inputs\n\nConfig schema {}. Every explicit input is listed with its reason. The \"default rules only\" headline figures drop all of them: the same analysis with no `[security.*]`, `[dependencies.*]` or schema-2 override entries. First-party security with inputs {}/{} proven SLoC ({:.2}%); default rules only {}/{} ({:.2}%); shift {:.2} percentage points. Runtime SLoC moved is credited to the input that decided each unit (the last match; excludes apply after includes). A dependency input moves that dependency's size.\n\n",
        if r.config_schema == 0 {
            "none (no config file)".to_string()
        } else {
            r.config_schema.to_string()
        },
        a.statuses["proven"],
        a.sloc,
        a.percent("proven"),
        b.statuses["proven"],
        b.sloc,
        b.percent("proven"),
        (a.percent("proven") - b.percent("proven")).abs()
    );
    if r.inputs.is_empty() {
        out.push_str("None.\n\n");
        return out;
    }
    out.push_str("| Input | Pattern / key | Direction | Reason | Matched | SLoC into security | SLoC out of security | Size change |\n| --- | --- | --- | --- | --- | ---: | ---: | ---: |\n");
    for i in &r.inputs {
        out.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | {} | {} |\n",
            cell(&i.kind),
            cell(&i.key),
            cell(&i.direction),
            i.reason
                .as_deref()
                .map(cell)
                .unwrap_or_else(|| "**none (schema 2)**".into()),
            if i.kind == "dependencies" || i.kind == "proof_ratio" {
                cell(&i.matched.join(", "))
            } else {
                format!("{} units", i.matched.len())
            },
            i.sloc_into_security,
            i.sloc_out_of_security,
            i.size_change
                .map(|c| format!("{c:+}"))
                .unwrap_or_else(|| "n/a".into())
        ));
    }
    out.push('\n');
    out
}
fn cell(s: &str) -> String {
    s.replace('|', "\\|").replace(['\n', '\r'], " ")
}
fn label(security: bool) -> &'static str {
    if security {
        "SECURITY-RELEVANT"
    } else {
        "EVERYTHING ELSE"
    }
}

/// Fail when explicit inputs move first-party security proven by more than `max` points.
pub fn gate_override_shift(r: &Report, max: f64) -> Result<Vec<String>> {
    anyhow::ensure!(
        max.is_finite() && (0.0..=100.).contains(&max),
        "override shift must be finite and within 0..100"
    );
    validate(r)?;
    let with = summarize(r).first_party.security_relevant.percent("proven");
    let without = summarize_default_rules(r)
        .first_party
        .security_relevant
        .percent("proven");
    let shift = (with - without).abs();
    if shift > max {
        Ok(vec![format!("inputs shift first-party security proven by {shift:.4} percentage points (with inputs {with:.4}%, default rules only {without:.4}%), above --max-override-shift {max:.4}")])
    } else {
        Ok(vec![])
    }
}

/// Apply an independent first-party threshold; the combined gate still runs separately.
pub fn gate_first_party_security(r: &Report, min: f64) -> Result<Vec<String>> {
    anyhow::ensure!(
        min.is_finite() && (0.0..=100.).contains(&min),
        "first-party threshold must be finite and within 0..100"
    );
    validate(r)?;
    let s = summarize(r);
    let pct = s.first_party.security_relevant.percent("proven");
    if pct < min {
        Ok(vec![format!("first-party-security-proven {pct:.4}% is below {min:.4}%; proven, relying on {} trusted items ({} SLoC) and {} escapes", s.trusted_items, s.trusted_sloc, s.escape_sites)])
    } else {
        Ok(vec![])
    }
}

pub fn gate(r: &Report, min: f64, baseline: Option<&Report>, no_new: bool) -> Result<Vec<String>> {
    gate_with_schema_change(r, min, baseline, no_new, false)
}

pub fn gate_with_schema_change(
    r: &Report,
    min: f64,
    baseline: Option<&Report>,
    no_new: bool,
    allow_schema_change: bool,
) -> Result<Vec<String>> {
    anyhow::ensure!(
        min.is_finite() && (0.0..=100.).contains(&min),
        "threshold must be finite and within 0..100"
    );
    anyhow::ensure!(
        !no_new || baseline.is_some(),
        "--no-new-escape-hatches requires --baseline"
    );
    validate(r)?;
    let mut failures = crate::ratio::gate(r);
    let s = summarize(r);
    if !r.dependency_inventory_complete {
        failures.push("security coverage incomplete: dependency inventory unavailable".into());
    }
    if s.all.security_relevant.unknown_dependency_sizes > 0 {
        failures.push("security coverage incomplete: unknown dependency SLoC".into());
    }
    if r.escape_hatches
        .iter()
        .any(|e| e.first_party && e.unit.ends_with("::<file-scope>"))
    {
        failures.push("unattributed file-scope hazard can invalidate runtime proofs".into());
    }
    if s.proof_code.total.hazard_units > 0 {
        failures.push("proof-code hazard can invalidate runtime proofs".into());
    }
    for input in &r.inputs {
        if input.reason.as_deref().is_some_and(crate::todo_reason) {
            failures.push(format!(
                "input {} {:?} has a TODO reason; replace it with the justification",
                input.kind, input.key
            ));
        }
    }
    let pct = s.all.security_relevant.percent("proven");
    if pct < min {
        failures.push(format!("security-proven {pct:.4}% is below {min:.4}%; proven, relying on {} trusted items ({} SLoC) and {} escapes",s.trusted_items,s.trusted_sloc,s.escape_sites));
    }
    if let Some(b) = baseline {
        anyhow::ensure!(b.schema == r.schema || allow_schema_change,
            "baseline schema {} differs from report schema {}; runtime denominators changed; use --allow-schema-change to acknowledge (coverage percentages will not be compared)", b.schema, r.schema);
        anyhow::ensure!(
            (1..=crate::REPORT_SCHEMA).contains(&b.schema),
            "unsupported baseline schema"
        );
        validate_statuses(b)?;
        let previous = summarize(b);
        if !b.dependency_inventory_complete {
            failures.push("baseline dependency inventory incomplete".into());
        }
        if previous.all.security_relevant.unknown_dependency_sizes > 0 {
            failures.push("baseline security coverage incomplete: unknown dependency SLoC".into());
        }
        if b.schema == r.schema && pct < previous.all.security_relevant.percent("proven") {
            failures.push("security-proven percentage dropped against baseline".into());
        }
        if no_new {
            fn counts(r: &Report) -> BTreeMap<(&str, usize, &str), usize> {
                let mut counts = BTreeMap::new();
                for e in &r.escape_hatches {
                    counts
                        .entry((e.file.as_str(), e.line, e.kind.as_str()))
                        .or_insert(1);
                }
                counts
            }
            let previous = counts(b);
            if counts(r)
                .iter()
                .any(|(key, count)| *count > *previous.get(key).unwrap_or(&0))
            {
                failures.push("new escape hatch appeared".into());
            }
        }
    }
    Ok(failures)
}
fn validate(r: &Report) -> Result<()> {
    anyhow::ensure!(
        r.schema == crate::REPORT_SCHEMA,
        "unsupported report schema"
    );
    validate_statuses(r)
}
fn validate_statuses(r: &Report) -> Result<()> {
    anyhow::ensure!(
        r.units
            .iter()
            .all(|u| if u.category == Category::ProofCode {
                [
                    "verified",
                    "unverified",
                    "hazard",
                    "trusted",
                    "proven-bounded",
                ]
                .contains(&u.status.as_str())
            } else {
                STATUSES.contains(&u.status.as_str())
            })
            && r.dependencies
                .iter()
                .chain(&r.build_dependencies)
                .all(|d| STATUSES.contains(&d.status.as_str())),
        "unknown status in report"
    );
    Ok(())
}
