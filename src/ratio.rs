//! First-party proof cost, independent of evidence acceptance and dependency sizing.
use crate::{Category, GhostLines, Report, Unit};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofRatioConfig {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRatioRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub source: String,
    pub reason: Option<String>,
}
impl Default for ProofRatioRange {
    fn default() -> Self {
        Self {
            min: Some(1.),
            max: Some(10.),
            source: "default".into(),
            reason: None,
        }
    }
}
impl ProofRatioRange {
    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(
            self.min.is_some() || self.max.is_some(),
            "proof ratio range needs at least one bound"
        );
        anyhow::ensure!(
            self.min
                .into_iter()
                .chain(self.max)
                .all(|n| n.is_finite() && n >= 0.),
            "proof ratio bounds must be finite and nonnegative"
        );
        anyhow::ensure!(
            !matches!((self.min,self.max), (Some(a),Some(b)) if a > b),
            "proof ratio min exceeds max"
        );
        Ok(())
    }
    fn status(&self, ratio: Option<f64>) -> String {
        match ratio {
            None => "n/a",
            Some(r) if self.min.is_some_and(|min| r < min) => "below",
            Some(r) if self.max.is_some_and(|max| r > max) => "above",
            _ => "within",
        }
        .into()
    }
}
impl std::str::FromStr for ProofRatioRange {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let (min, max) = s
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("proof ratio range must be MIN:MAX"))?;
        let bound = |s: &str| -> Result<Option<f64>> {
            if s.trim().is_empty() {
                Ok(None)
            } else {
                Ok(Some(s.trim().parse()?))
            }
        };
        let range = Self {
            min: bound(min)?,
            max: bound(max)?,
            source: "cli".into(),
            reason: None,
        };
        range.validate()?;
        Ok(range)
    }
}
impl ProofRatioConfig {
    pub(crate) fn range(&self) -> Result<ProofRatioRange> {
        anyhow::ensure!(
            !self.reason.trim().is_empty(),
            "proof_ratio needs a non-empty reason"
        );
        let r = ProofRatioRange {
            min: self.min,
            max: self.max,
            source: "config".into(),
            reason: Some(self.reason.clone()),
        };
        r.validate()?;
        Ok(r)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatioMetrics {
    pub standalone_proof_sloc: usize,
    pub inline_proof_sloc: usize,
    pub proof_sloc: usize,
    pub fstar_annotation_sloc: usize,
    pub fstar_cfg_hax_sloc: usize,
    pub fstar_cfg_hax_inline_sloc: usize,
    pub fstar_patch_sloc: usize,
    pub fstar_file_sloc: usize,
    pub fstar_model_sloc: usize,
    pub excluded_patch_hunks: Vec<crate::fstar_proof::ExcludedHunk>,
    pub proven_runtime_sloc: usize,
    pub proven_extracted_runtime_sloc: usize,
    pub proven_bounded_runtime_sloc: usize,
    pub runtime_sloc: usize,
    pub ratio: Option<f64>,
    pub status: String,
}
fn metrics<'a>(
    units: impl Iterator<Item = &'a Unit>,
    mode: GhostLines,
    range: &ProofRatioRange,
    current: GhostLines,
    inputs: Vec<&crate::fstar_proof::Input>,
    excluded: Vec<crate::fstar_proof::ExcludedHunk>,
) -> RatioMetrics {
    let mut m = RatioMetrics {
        standalone_proof_sloc: 0,
        inline_proof_sloc: 0,
        proof_sloc: 0,
        fstar_annotation_sloc: 0,
        fstar_cfg_hax_sloc: 0,
        fstar_cfg_hax_inline_sloc: 0,
        fstar_patch_sloc: 0,
        fstar_file_sloc: 0,
        fstar_model_sloc: 0,
        excluded_patch_hunks: excluded,
        proven_runtime_sloc: 0,
        proven_extracted_runtime_sloc: 0,
        proven_bounded_runtime_sloc: 0,
        runtime_sloc: 0,
        ratio: None,
        status: String::new(),
    };
    for u in units.filter(|u| u.first_party) {
        match u.category {
            Category::ProofCode => {
                m.standalone_proof_sloc += u.sloc;
                if u.fstar_cfg_hax {
                    m.fstar_cfg_hax_sloc += u.sloc;
                }
            }
            Category::Runtime => {
                let (runtime, inline) = runtime_lines(u, mode, current);
                m.inline_proof_sloc += inline;
                if mode == GhostLines::Separate {
                    m.fstar_annotation_sloc += u.fstar_annotation_sloc;
                    m.fstar_cfg_hax_inline_sloc += u.fstar_cfg_hax_inline_sloc;
                }
                m.runtime_sloc += runtime;
                if u.status == "proven" {
                    m.proven_runtime_sloc += runtime;
                }
                if u.status == "proven-extracted" {
                    m.proven_extracted_runtime_sloc += runtime;
                }
                if u.status == "proven-bounded" {
                    m.proven_bounded_runtime_sloc += runtime;
                }
            }
            Category::Test => {}
        }
    }
    for input in inputs {
        match input.kind.as_str() {
            "patch" => m.fstar_patch_sloc += input.sloc,
            "file" => m.fstar_file_sloc += input.sloc,
            "model" => m.fstar_model_sloc += input.sloc,
            _ => {}
        }
    }
    m.proof_sloc =
        m.standalone_proof_sloc + m.inline_proof_sloc + m.fstar_patch_sloc + m.fstar_file_sloc;
    let denominator = m.proven_runtime_sloc + m.proven_extracted_runtime_sloc;
    m.ratio = (denominator > 0).then(|| m.proof_sloc as f64 / denominator as f64);
    m.status = range.status(m.ratio);
    m
}
/// Runtime SLoC of a unit under a ghost-line mode, and the inline proof lines taken out of it.
fn runtime_lines(u: &Unit, mode: GhostLines, current: GhostLines) -> (usize, usize) {
    let legacy = u.sloc
        + if current == GhostLines::Separate {
            u.inline_proof_sloc
        } else {
            0
        };
    let inline = if mode == GhostLines::Separate {
        u.ghost_sloc
    } else {
        0
    };
    (legacy - inline, inline)
}
/// How much of the proven runtime code is proven only to be free of panics, overflows and
/// out-of-bounds access, and how much has a postcondition about values.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofDepth {
    pub safety_only_sloc: usize,
    pub functional_own_code_only_sloc: usize,
    pub functional_sloc: usize,
}
impl ProofDepth {
    pub fn total(&self) -> usize {
        self.safety_only_sloc + self.functional_own_code_only_sloc + self.functional_sloc
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Depth {
    SafetyOnly,
    SelfConsistentOnly,
    Functional,
}
/// Depth of each credited F* unit: the lowest of its credited definitions, never rounded up.
fn extracted_depths(r: &Report) -> std::collections::HashMap<&str, Depth> {
    let mut depths: std::collections::HashMap<&str, Depth> = Default::default();
    for d in r
        .fstar_definitions
        .iter()
        .filter(|d| d.classification == "proven-extracted")
    {
        let Some(id) = d.rust_unit.as_deref() else {
            continue;
        };
        let depth = match (d.contract_kind.as_str(), d.self_consistency_heuristic) {
            ("functional", true) => Depth::SelfConsistentOnly,
            ("functional", false) => Depth::Functional,
            _ => Depth::SafetyOnly,
        };
        let lowest = depths.entry(id).or_insert(depth);
        *lowest = (*lowest).min(depth);
    }
    depths
}
/// Proof depth over proven and proven-extracted first-party runtime SLoC of `units`.
/// Verus-proven units carry no F* contract here, so they count as safety-only (never rounded up).
pub(crate) fn proof_depth<'a>(r: &Report, units: impl Iterator<Item = &'a Unit>) -> ProofDepth {
    let depths = extracted_depths(r);
    let mut total = ProofDepth::default();
    for u in units.filter(|u| u.first_party && u.category == Category::Runtime) {
        if !["proven", "proven-extracted"].contains(&u.status.as_str()) {
            continue;
        }
        let (runtime, _) = runtime_lines(u, r.ghost_lines, r.ghost_lines);
        let depth = depths
            .get(u.id.as_str())
            .copied()
            .unwrap_or(Depth::SafetyOnly);
        match (u.status.as_str(), depth) {
            ("proven-extracted", Depth::Functional) => total.functional_sloc += runtime,
            ("proven-extracted", Depth::SelfConsistentOnly) => {
                total.functional_own_code_only_sloc += runtime
            }
            _ => total.safety_only_sloc += runtime,
        }
    }
    total
}
fn depth_markdown(r: &Report) -> String {
    let depth = proof_depth(r, r.units.iter().filter(|u| u.security));
    let total = depth.total();
    let mut out = String::from("\nProof depth (first-party security-relevant, proven + proven-extracted runtime SLoC):\n\n| Contract depth | Lines | Share of proven lines |\n| --- | ---: | ---: |\n");
    for (label, lines) in [
        ("safety-only", depth.safety_only_sloc),
        (
            "functional, checked only against own code",
            depth.functional_own_code_only_sloc,
        ),
        ("functional", depth.functional_sloc),
    ] {
        let share = if total == 0 {
            0.
        } else {
            lines as f64 * 100. / total as f64
        };
        out.push_str(&format!("| {label} | {lines} | {share:.2}% |\n"));
    }
    out.push_str("\nSafety-only means proven free of panics, overflows and out-of-bounds access, including length and range facts; it says nothing about what the code computes.\n");
    out
}
#[derive(Debug, Serialize)]
pub struct RatioRow {
    pub crate_name: String,
    /// Empty source module is explicitly listed as <unattributed>.
    pub module: Option<String>,
    #[serde(flatten)]
    pub metrics: RatioMetrics,
}
#[derive(Debug, Serialize)]
pub struct ModeFigures {
    pub security_relevant: RatioMetrics,
    pub all_first_party: RatioMetrics,
}
#[derive(Debug, Serialize)]
pub struct BothModes {
    pub separate: ModeFigures,
    pub runtime: ModeFigures,
}
#[derive(Debug, Serialize)]
pub struct ProofRatio {
    /// Headline is first-party security-relevant units only.
    #[serde(flatten)]
    pub security_relevant: RatioMetrics,
    pub unknown_hax_paths: Vec<String>,
    pub range: ProofRatioRange,
    pub gate_enabled: bool,
    pub all_first_party: RatioMetrics,
    pub crates: Vec<RatioRow>,
    pub modules: Vec<RatioRow>,
    pub runtime_proven_modes: BothModes,
}
pub fn proof_ratio(r: &Report) -> ProofRatio {
    let range = &r.proof_ratio_range;
    let get =
        |units: Vec<&Unit>, mode, security: bool, krate: Option<&str>, module: Option<&str>| {
            let inputs: Vec<_> = r
                .fstar_proof
                .inputs
                .iter()
                .filter(|i| {
                    (!security || i.security)
                        && krate.is_none_or(|k| i.crate_name == k)
                        && module.is_none_or(|m| i.module == m)
                })
                .collect();
            let excluded = r
                .fstar_proof
                .excluded_patch_hunks
                .iter()
                .filter(|h| {
                    inputs
                        .iter()
                        .any(|i| i.file == h.file && i.hunk_line == Some(h.hunk_line))
                })
                .cloned()
                .collect();
            metrics(
                units.into_iter(),
                mode,
                range,
                r.ghost_lines,
                inputs,
                excluded,
            )
        };
    let all = || r.units.iter().collect::<Vec<_>>();
    let security = || r.units.iter().filter(|u| u.security).collect::<Vec<_>>();
    let names: BTreeSet<_> = r
        .units
        .iter()
        .filter(|u| u.first_party)
        .map(|u| u.crate_name.clone())
        .chain(r.fstar_proof.inputs.iter().map(|i| i.crate_name.clone()))
        .collect();
    let modules: BTreeSet<_> = r
        .units
        .iter()
        .filter(|u| u.first_party)
        .map(|u| (u.crate_name.clone(), u.module.clone()))
        .chain(
            r.fstar_proof
                .inputs
                .iter()
                .map(|i| (i.crate_name.clone(), i.module.clone())),
        )
        .collect();
    let mode = |m| ModeFigures {
        security_relevant: get(security(), m, true, None, None),
        all_first_party: get(all(), m, false, None, None),
    };
    ProofRatio {
        security_relevant: get(security(), r.ghost_lines, true, None, None),
        unknown_hax_paths: r.unknown_hax_paths.clone(),
        range: range.clone(),
        gate_enabled: !r.no_proof_ratio_gate,
        all_first_party: get(all(), r.ghost_lines, false, None, None),
        crates: names
            .into_iter()
            .map(|name| RatioRow {
                metrics: get(
                    r.units.iter().filter(|u| u.crate_name == name).collect(),
                    r.ghost_lines,
                    false,
                    Some(&name),
                    None,
                ),
                crate_name: name,
                module: None,
            })
            .collect(),
        modules: modules
            .into_iter()
            .map(|(name, module)| RatioRow {
                metrics: get(
                    r.units
                        .iter()
                        .filter(|u| u.crate_name == name && u.module == module)
                        .collect(),
                    r.ghost_lines,
                    false,
                    Some(&name),
                    Some(&module),
                ),
                crate_name: name,
                module: Some(if module.is_empty() {
                    "<unattributed>".into()
                } else {
                    module
                }),
            })
            .collect(),
        runtime_proven_modes: BothModes {
            separate: mode(GhostLines::Separate),
            runtime: mode(GhostLines::Runtime),
        },
    }
}
pub(crate) fn gate(r: &Report) -> Vec<String> {
    let p = proof_ratio(r);
    if p.gate_enabled && ["below", "above"].contains(&p.security_relevant.status.as_str()) {
        vec![format!(
            "first-party security proof ratio {:.2} is {} the acceptance range",
            p.security_relevant.ratio.unwrap(),
            p.security_relevant.status
        )]
    } else {
        vec![]
    }
}
fn number(n: Option<f64>) -> String {
    n.map(|n| format!("{n:.2}")).unwrap_or_else(|| "n/a".into())
}
pub(crate) fn markdown(r: &Report) -> String {
    let p = proof_ratio(r);
    let mut out = format!("## Proof ratio\n\nGhost lines: `{}`. Range {}:{}; source `{}`{}; ratio gate {}. Cost signal, not proof quality. Module rows outside the range are warnings only.\n\n", r.ghost_lines.as_str(), p.range.min.map(|n|format!("{n:.2}")).unwrap_or_default(),p.range.max.map(|n|format!("{n:.2}")).unwrap_or_default(),p.range.source,p.range.reason.as_ref().map(|s|format!(" (reason: {})",cell(s))).unwrap_or_default(),if p.gate_enabled { "enabled" } else { "disabled (--no-proof-ratio-gate)" });
    out.push_str("| Scope | Standalone proof | Inline proof | Total proof | Proven runtime | Proven-extracted runtime | Proven-bounded runtime | Ratio | Status |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |\n");
    let row = |label: &str, m: &RatioMetrics| {
        format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            cell(label),
            m.standalone_proof_sloc,
            m.inline_proof_sloc,
            m.proof_sloc,
            m.proven_runtime_sloc,
            m.proven_extracted_runtime_sloc,
            m.proven_bounded_runtime_sloc,
            number(m.ratio),
            m.status
        )
    };
    out.push_str(&row(
        "First-party security-relevant (headline)",
        &p.security_relevant,
    ));
    out.push_str(&row("All first-party", &p.all_first_party));
    for r in &p.crates {
        out.push_str(&row(&format!("Crate {}", r.crate_name), &r.metrics));
    }
    for r in &p.modules {
        out.push_str(&row(
            &format!("Module {}::{}", r.crate_name, r.module.as_deref().unwrap()),
            &r.metrics,
        ));
    }
    for r in &p.modules {
        if ["below", "above"].contains(&r.metrics.status.as_str()) {
            out.push_str(&format!(
                "\nWarning: module {}::{} ratio {} is {} range (does not fail gate).\n",
                cell(&r.crate_name),
                cell(r.module.as_deref().unwrap()),
                number(r.metrics.ratio),
                r.metrics.status
            ));
        }
    }
    out.push_str(&depth_markdown(r));
    out.push_str("\nF* proof components (annotations are already included in inline proof; models are trusted assumptions):\n\n| Scope | Annotations | cfg(hax) standalone | cfg(hax) inline | Patch additions | Hand-written F* | Models (excluded) |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for (scope, metrics) in [
        ("Security", &p.security_relevant),
        ("All first-party", &p.all_first_party),
    ] {
        out.push_str(&format!(
            "| {scope} | {} | {} | {} | {} | {} | {} |\n",
            metrics.fstar_annotation_sloc,
            metrics.fstar_cfg_hax_sloc,
            metrics.fstar_cfg_hax_inline_sloc,
            metrics.fstar_patch_sloc,
            metrics.fstar_file_sloc,
            metrics.fstar_model_sloc
        ));
    }
    if !p.all_first_party.excluded_patch_hunks.is_empty() {
        out.push_str("\nExcluded F* patch hunks:\n\n| Patch:hunk line | Target module | Reason |\n| --- | --- | --- |\n");
        for h in &p.all_first_party.excluded_patch_hunks {
            out.push_str(&format!(
                "| {}:{} | {} | {} |\n",
                cell(&h.file),
                h.hunk_line,
                cell(&h.target_module),
                cell(&h.reason)
            ));
        }
    }
    out.push_str("\nRuntime proven figures under both ghost modes (same accepted evidence):\n\n| Scope | separate proven / runtime SLoC | runtime proven / runtime SLoC |\n| --- | ---: | ---: |\n");
    for (label, a, b) in [
        (
            "First-party security-relevant",
            &p.runtime_proven_modes.separate.security_relevant,
            &p.runtime_proven_modes.runtime.security_relevant,
        ),
        (
            "All first-party",
            &p.runtime_proven_modes.separate.all_first_party,
            &p.runtime_proven_modes.runtime.all_first_party,
        ),
    ] {
        out.push_str(&format!(
            "| {label} | {} / {} | {} / {} |\n",
            a.proven_runtime_sloc, a.runtime_sloc, b.proven_runtime_sloc, b.runtime_sloc
        ));
    }
    out.push_str(&format!(
        "\nF* annotation SLoC: {} (security), {} (all first-party). Unknown hax paths: {}.\n",
        p.security_relevant.fstar_annotation_sloc,
        p.all_first_party.fstar_annotation_sloc,
        p.unknown_hax_paths.join(", ")
    ));
    out.push('\n');
    out
}
fn cell(s: &str) -> String {
    s.replace('|', "\\|").replace(['\n', '\r'], " ")
}
