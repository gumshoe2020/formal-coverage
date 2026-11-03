use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser)]
#[command(version, about = "Conservative Rust formal coverage")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Args)]
struct Input {
    workspace_dir: PathBuf,
    #[arg(long, default_value = "separate")]
    ghost_lines: formal_coverage::GhostLines,
    #[arg(long)]
    proof_ratio_range: Option<formal_coverage::ProofRatioRange>,
    #[arg(long)]
    no_proof_ratio_gate: bool,
    #[arg(long)]
    target: Option<String>,
    #[arg(long)]
    config: Option<PathBuf>,
    /// Runtime dependency sizing: whole-crate, compiled-files or linked-lines.
    #[arg(long, default_value = "compiled-files", value_parser = size_mode)]
    size_mode: formal_coverage::SizeMode,
    /// Binary whose DWARF line table linked-lines mode reads.
    #[arg(long, required_if_eq("size_mode", "linked-lines"))]
    binary: Option<PathBuf>,
    /// Target directory for compiled-files `cargo check` (default <workspace>/target/formal-coverage).
    #[arg(long)]
    check_target_dir: Option<PathBuf>,
    #[arg(long)]
    verus_log: Vec<PathBuf>,
    #[arg(long)]
    kani_log: Vec<PathBuf>,
    #[arg(long)]
    fstar_log: Vec<PathBuf>,
    #[arg(long)]
    json: Option<PathBuf>,
    #[arg(long)]
    markdown: Option<PathBuf>,
}
#[derive(Subcommand)]
enum Commands {
    Report {
        #[command(flatten)]
        input: Input,
    },
    Gate {
        #[command(flatten)]
        input: Input,
        #[arg(long, required_unless_present = "min_first_party_security_proven")]
        min_security_proven: Option<f64>,
        #[arg(long, required_unless_present = "min_security_proven")]
        min_first_party_security_proven: Option<f64>,
        #[arg(long)]
        baseline: Option<PathBuf>,
        #[arg(long, requires = "baseline")]
        no_new_escape_hatches: bool,
        #[arg(long, requires = "baseline")]
        allow_schema_change: bool,
        /// Fail when first-party security proven with inputs and under default rules only
        /// differ by more than this many percentage points (off unless given).
        #[arg(long)]
        max_override_shift: Option<f64>,
    },
    /// Rewrite a schema-2 config (bare globs) as schema 3 with `reason = "TODO"`.
    MigrateConfig {
        config: PathBuf,
        /// Write here instead of stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}
fn size_mode(s: &str) -> Result<formal_coverage::SizeMode, String> {
    s.parse().map_err(|e: anyhow::Error| e.to_string())
}
fn run(input: &Input) -> anyhow::Result<formal_coverage::Report> {
    let mut report = formal_coverage::analyze_with(
        &input.workspace_dir,
        &formal_coverage::Options {
            ghost_lines: input.ghost_lines,
            proof_ratio_range: input.proof_ratio_range.clone(),
            no_proof_ratio_gate: input.no_proof_ratio_gate,
            config: input.config.clone(),
            target: input.target.clone(),
            size_mode: input.size_mode,
            binary: input.binary.clone(),
            check_target_dir: input.check_target_dir.clone(),
        },
    )?;
    formal_coverage::apply_evidence_many(
        &mut report,
        &input.workspace_dir,
        &input.verus_log,
        &input.kani_log,
    )?;
    formal_coverage::apply_fstar_evidence(&mut report, &input.workspace_dir, &input.fstar_log)?;
    let json = serde_json::to_value(&report)?;
    let mut document = json.as_object().unwrap().clone();
    document.insert(
        "proof_ratio".into(),
        serde_json::to_value(formal_coverage::proof_ratio(&report))?,
    );
    document.insert(
        "summary".into(),
        serde_json::to_value(formal_coverage::summarize(&report))?,
    );
    document.insert(
        "summary_default_rules".into(),
        serde_json::to_value(formal_coverage::summarize_default_rules(&report))?,
    );
    if let Some(path) = &input.json {
        std::fs::write(path, serde_json::to_string_pretty(&document)?)?;
    }
    let md = formal_coverage::markdown(&report);
    if let Some(path) = &input.markdown {
        std::fs::write(path, &md)?;
    }
    println!("{md}");
    Ok(report)
}
fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Commands::Report { input } => {
            run(&input)?;
        }
        Commands::MigrateConfig { config, output } => {
            let migrated = formal_coverage::migrate_config(&std::fs::read_to_string(&config)?)?;
            match output {
                Some(path) => std::fs::write(path, migrated)?,
                None => print!("{migrated}"),
            }
        }
        Commands::Gate {
            input,
            min_security_proven,
            min_first_party_security_proven,
            baseline,
            no_new_escape_hatches,
            allow_schema_change,
            max_override_shift,
        } => {
            let report = run(&input)?;
            let previous = baseline
                .map(|p| -> anyhow::Result<formal_coverage::Report> {
                    Ok(serde_json::from_slice(&std::fs::read(p)?)?)
                })
                .transpose()?;
            if allow_schema_change && previous.as_ref().is_some_and(|b| b.schema != report.schema) {
                eprintln!("Schema change acknowledged: skipping baseline percentage comparison; threshold, completeness and escape checks still apply.");
            }
            let mut failures = formal_coverage::gate_with_schema_change(
                &report,
                min_security_proven.unwrap_or(0.),
                previous.as_ref(),
                no_new_escape_hatches,
                allow_schema_change,
            )?;
            if let Some(min) = min_first_party_security_proven {
                failures.extend(formal_coverage::gate_first_party_security(&report, min)?);
            }
            if let Some(max) = max_override_shift {
                failures.extend(formal_coverage::gate_override_shift(&report, max)?);
            }
            for failure in &failures {
                eprintln!("GATE FAILED: {failure}");
            }
            if !failures.is_empty() {
                std::process::exit(1);
            }
            println!("Gate passed.");
        }
    }
    Ok(())
}
