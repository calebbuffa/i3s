mod policy;

use anyhow::{Context, Result};
use clap::Parser;
use policy::{I3sConfig, I3sPolicy};
use schemagen::{Config, Graph, generate_types_from_roots, render_module};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "xtask", about = "Generate i3s Rust types from JSON Schema")]
struct Args {
    #[arg(long, default_value = "extern/i3s-schema/schema")]
    schemas: PathBuf,
    #[arg(long, default_value = "xtask/config.json")]
    config: PathBuf,
    #[arg(long, default_value = "src/generated.rs")]
    output: PathBuf,
    #[arg(long)]
    check: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config_text = std::fs::read_to_string(&args.config)
        .with_context(|| format!("reading {}", args.config.display()))?;
    let config: Config = serde_json::from_str(&config_text)
        .with_context(|| format!("parsing {}", args.config.display()))?;
    let policy_config: I3sConfig = serde_json::from_str(&config_text)?;

    let graph_root = args
        .schemas
        .parent()
        .context("schema directory must have a parent")?;
    let graph_root = std::path::Path::new(graph_root)
        .canonicalize()
        .with_context(|| format!("resolving {}", graph_root.display()))?;
    let schema_dir = args
        .schemas
        .canonicalize()
        .with_context(|| format!("resolving {}", args.schemas.display()))?;
    let schema_relative = args
        .schemas
        .file_name()
        .context("schema directory must have a name")?;
    let mut graph = Graph::new(graph_root);
    graph.add_search_path(&schema_dir);
    let mut modules = Vec::new();
    for profile in ["cmn", "bld", "psl", "pcsl"] {
        let roots = graph.load_tree(PathBuf::from(schema_relative).join(profile))?;
        let policy = I3sPolicy {
            profile,
            config: &policy_config,
        };
        let definitions = generate_types_from_roots(&mut graph, roots, &config, &policy)
            .map_err(anyhow::Error::msg)?;
        let body = render_module("", &definitions, &config, &policy).map_err(anyhow::Error::msg)?;
        modules.push(format!("pub mod {profile} {{\n{body}\n}}"));
    }
    let source = format!(
        "//! Auto-generated i3s types. Do not edit manually.\n\n{}",
        modules.join("\n\n")
    );

    let mut stale = false;
    if args.check {
        if std::fs::read_to_string(&args.output).ok().as_deref() != Some(source.as_str()) {
            eprintln!("out of date: {}", args.output.display());
            stale = true;
        }
    } else {
        if let Some(parent) = args.output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&args.output, source)?;
    }
    if stale {
        anyhow::bail!("generated i3s files are out of date");
    }
    if args.check {
        eprintln!("check ok: i3s generated files are up to date");
    } else {
        eprintln!("generated i3s types in {}", args.output.display());
    }
    Ok(())
}
