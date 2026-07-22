// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::env::var_os;

use clap::Parser;

use log::trace;
use miette::{Context, Result};

use skribi::execute;

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Log more information. Fine-grained control.
    ///
    /// The RUST_LOG variable can also be used.
    /// This variable overrides the argument.
    ///
    /// Possible values: off, error, warn, info, debug, trace
    #[arg(short, long, default_value = "warn")]
    verbose: log::LevelFilter,
}

/// Launch the interpreter
fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut logger = env_logger::Builder::from_default_env();
    if let None = var_os("RUST_LOG") {
        logger.filter_level(args.verbose);
    }
    logger.init();

    trace!("Logger initialised, entenring main");

    execute().context("Failed to execute your file.")
}
