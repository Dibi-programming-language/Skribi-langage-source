// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use clap::Parser;

use log::trace;
use miette::{Context, Result};

use skribi::execute;

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Log more information, set the level to INFO.
    /// For fine-grained control over log levels, use the RUST_LOG variable.
    #[arg(short, long)]
    verbose: bool,
    /// Log all information, set the level to TRACE.
    /// For fine-grained control over log levels, use the RUST_LOG variable.
    #[arg(long)]
    very_verbose: bool,
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

    execute()
        .context("Failed to execute your file.")
}
