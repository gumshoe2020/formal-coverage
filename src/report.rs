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
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Summary {
    pub security_first_party_hazard_units: usize,
    pub trusted_items: usize,
    pub trusted_sloc: usize,
    pub escape_sites: usize,
    pub proof_code: ProofCode,
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
    let mut summary = Summary {
        trusted_items: r.trusted_items.len(),
        trusted_sloc: r.trusted_items.iter().map(|t| t.sloc).sum(),
        escape_sites: r.escape_hatches.len(),
        ..Summary::default()
    };
    for u in &r.units {
        summary.security_first_party_hazard_units +=
            usize::from(u.security && u.status == "hazard");
        if u.category == Category::ProofCode {
            summary.proof_code.total.add(u);
            if u.security {
                summary.proof_code.security_relevant.add(u);
            } else {
                summary.proof_code.everything_else.add(u);
            }
            continue;
        }
        side(&mut summary.all, u.security).add(u.sloc, &u.status, false);
        side(&mut summary.first_party, u.security).add(u.sloc, &u.status, false);
    }
    for d in &r.dependencies {
        side(&mut summary.all, d.security).add(d.sloc, &d.status, !d.size_known);
        side(&mut summary.third_party, d.security).add(d.sloc, &d.status, !d.size_known);
    }
    summary
}
fn headline(label: &str, totals: &Totals, trust: &str) -> String {
    format!(
        "{label}  {} measured SLoC   {}{}; {trust}",
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
pub fn markdown(r: &Report) -> String {
    let s = summarize(r);
    let trust = format!(
        "proven, relying on {} trusted items ({} SLoC) and {} escapes",
        s.trusted_items, s.trusted_sloc, s.escape_sites
    );
    let mut lines = Vec::new();
    for (label, first, third, combined) in [
        (
            "SECURITY-RELEVANT",
            &s.first_party.security_relevant,
            &s.third_party.security_relevant,
            &s.all.security_relevant,
        ),
        (
            "EVERYTHING ELSE",
            &s.first_party.everything_else,
            &s.third_party.everything_else,
            &s.all.everything_else,
        ),
    ] {
        lines.push(headline(&format!("{label} First-party"), first, &trust));
        lines.push(headline(
            &format!("{label} Third-party (runtime)"),
            third,
            &trust,
        ));
        lines.push(headline(&format!("{label} Combined"), combined, &trust));
    }
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
    out.push_str(&format!("Combined runtime security: {}/{} proven SLoC ({:.2}%); {trust}; proof-code: {} SLoC alongside, excluded from both runtime denominators.\n\n## PROOF CODE\n\n| Classification | SLoC | Units | verified | unverified (including hazards, trusted and bounded) | Hazard units |\n| --- | ---: | ---: | ---: | ---: | ---: |\n",
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
    out.push_str("\n## Units\n\n| Function / crate | Location | SLoC | Classification | Status | Proof scope | Third-party unsafe sites |\n| --- | --- | ---: | --- | --- | --- | ---: |\n");
    for u in &r.units {
        out.push_str(&format!(
            "| {} | {}:{} | {} | {} | {} | {} | n/a |\n",
            cell(&u.id),
            cell(&u.file),
            u.line,
            u.sloc,
            if u.category == Category::ProofCode {
                "proof-code"
            } else {
                label(u.security)
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
    format!("{}\n\n{out}", lines[0])
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
    let mut failures = Vec::new();
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
    let pct = s.all.security_relevant.percent("proven");
    if pct < min {
        failures.push(format!("security-proven {pct:.4}% is below {min:.4}%; proven, relying on {} trusted items ({} SLoC) and {} escapes",s.trusted_items,s.trusted_sloc,s.escape_sites));
    }
    if let Some(b) = baseline {
        anyhow::ensure!(b.schema == r.schema || allow_schema_change,
            "baseline schema {} differs from report schema {}; runtime denominators changed; use --allow-schema-change to acknowledge (coverage percentages will not be compared)", b.schema, r.schema);
        anyhow::ensure!(
            b.schema == 1 || b.schema == 2,
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
    anyhow::ensure!(r.schema == 2, "unsupported report schema");
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
