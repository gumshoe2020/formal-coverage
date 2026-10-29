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
    #[arg(long)]
    target: Option<String>,
    #[arg(long)]
    config: Option<PathBuf>,
    #[arg(long)]
    verus_log: Vec<PathBuf>,
    #[arg(long)]
    kani_log: Vec<PathBuf>,
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
    },
}
fn run(input: &Input) -> anyhow::Result<formal_coverage::Report> {
    let mut report = formal_coverage::analyze_target(
        &input.workspace_dir,
        input.config.as_deref(),
        input.target.as_deref(),
    )?;
    formal_coverage::apply_evidence_many(
        &mut report,
        &input.workspace_dir,
        &input.verus_log,
        &input.kani_log,
    )?;
    let json = serde_json::to_value(&report)?;
    let mut document = json.as_object().unwrap().clone();
    document.insert(
        "summary".into(),
        serde_json::to_value(formal_coverage::summarize(&report))?,
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
        Commands::Gate {
            input,
            min_security_proven,
            min_first_party_security_proven,
            baseline,
            no_new_escape_hatches,
            allow_schema_change,
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
