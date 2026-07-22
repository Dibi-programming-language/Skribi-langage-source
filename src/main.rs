// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::fs::create_dir_all;

use clap::Parser;

use log::{info, trace};
use miette::{Context, IntoDiagnostic, Result};

use skribi::{file::File, source::SourceManager};

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// The source file to use. Defaults to STDIN.
    /// STDIN is currently not supported.
    source: Option<String>,
    /// Sets the path of the compilation folder.
    /// Defaults to `.skribi`.
    #[arg(short, long, default_value = ".skribi")]
    compile_path: String,
    /// Log more information, set the level to INFO.
    /// For fine-grained control over log levels, use the RUST_LOG variable.
    #[arg(short, long)]
    verbose: bool,
    /// Log all information, set the level to TRACE.
    /// For fine-grained control over log levels, use the RUST_LOG variable.
    #[arg(long)]
    very_verbose: bool,
    /// Run the code instead of compiling it.
    #[arg(short, long)]
    run: bool,
}

/// Creates a folder to store everything
fn create_skribi_directory(path: &str) -> Result<()> {
    trace!("About to create hidden directory `{}`", path);
    create_dir_all(path).into_diagnostic().context(format!(
        "While creating hidden `{}` directory to store compiled files",
        path
    ))?;
    info!("Hidden directory `{}` created for compiled files", path);
    Ok(())
}

/// Launch the interpreter
fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut logger = env_logger::Builder::from_default_env();
    if args.verbose {
        logger.filter_level(log::LevelFilter::Info);
    } else if args.very_verbose {
        logger.filter_level(log::LevelFilter::Trace);
    }

    logger.init();
    trace!("Logger initialised, entenring main");

    create_skribi_directory(&args.compile_path)?;

    if let Some(path) = args.source {
        let file = File::from_file(&path).context("While reading file passed as argument")?;
        let mut manager = SourceManager::empty();
        manager.add_file(&file)?;

        if args.run {
            manager.execute()
        } else {
            manager.compile()
        }
    } else {
        todo!("STDIN is currently not supported")
    }
}
