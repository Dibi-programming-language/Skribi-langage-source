// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::env::var_os;

use clap::Parser;

use log::{LevelFilter, trace};
use miette::{Context, Result};

use skribi::execute;

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Log more information. Fine-grained control.
    ///
    /// The RUST_LOG variable can also be used.
    /// The variable is overrided by the argument.
    ///
    /// With nothing set, defaults to warn.
    ///
    /// Possible values: off, error, warn, info, debug, trace
    #[arg(short, long)]
    verbose: Option<LevelFilter>,
}

/// Launch the interpreter
fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut logger = env_logger::Builder::from_default_env();
    if let Some(level) = args.verbose {
        logger.filter_level(level);
    } else if let None = var_os("RUST_LOG") {
        logger.filter_level(LevelFilter::Warn);
    }
    logger.init();

    trace!("Logger initialised, entenring main");

    execute().context("Failed to execute your file.")
}
