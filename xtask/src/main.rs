mod config;
mod policy;

use std::path::PathBuf;

use clap::Parser;
use schemagen::diagnostics::Sink;
use schemagen::driver::{Source, emit, finish};
use schemagen::{ConfigUsage, Module, generate, render_modules};

use policy::I3sPolicy;

/// The profiles, each generated as one module. `cmn` comes first because the
/// others borrow its types.
const PROFILES: [&str; 4] = ["cmn", "bld", "psl", "pcsl"];

/// Generates `src/generated.rs` from the I3S schemas.
#[derive(Debug, Parser)]
#[command(name = "xtask")]
struct Cli {
    /// The directory holding one subdirectory of schemas per profile.
    #[arg(long, default_value = "extern/i3s-schema/schema")]
    schema: PathBuf,
    /// The Rust file to write.
    #[arg(long, default_value = "src/generated.rs")]
    output: PathBuf,
    /// Verify that the file on disk is up to date, without writing.
    #[arg(long)]
    check: bool,
    /// Treat warnings as errors.
    #[arg(long)]
    deny_warnings: bool,
}

fn main() {
    if let Err(error) = run(&Cli::parse()) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run(cli: &Cli) -> schemagen::Result<()> {
    let schema_dir = cli
        .schema
        .canonicalize()
        .map_err(|error| schemagen::Error::io(&cli.schema, error))?;

    let mut usage = ConfigUsage::default();
    let mut generated = Vec::new();
    for profile in PROFILES {
        let config = config::config(profile)?;
        let policy = I3sPolicy { profile };
        let source = Source {
            base: Some(schema_dir.clone()),
            search_paths: vec![schema_dir.clone()],
            inputs: vec![schema_dir.join(profile)],
        };
        let mut loaded = source.load(&config)?;
        let (definitions, used) = generate(&mut loaded.graph, &loaded.roots, &config, &policy)?;
        usage.merge(&used);
        finish(loaded.graph.sink(), cli.deny_warnings)?;
        generated.push((profile, definitions, config, policy));
    }

    // Each entry is used by whichever profile it applies to, so unused ones
    // are judged across all of them.
    let mut sink = Sink::new();
    usage.report_unused(&config::config("cmn")?, &mut sink);
    finish(&sink, cli.deny_warnings)?;

    let modules: Vec<Module<'_>> = generated
        .iter()
        .map(|(profile, definitions, config, policy)| Module {
            name: profile,
            generated: definitions,
            config,
            policy,
        })
        .collect();
    let source = render_modules("Auto-generated i3s types. Do not edit manually.", &modules)?;
    emit(&cli.output, &source, cli.check)?;
    if cli.check {
        println!("up to date: {}", cli.output.display());
    } else {
        println!("wrote {}", cli.output.display());
    }
    Ok(())
}
