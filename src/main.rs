// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use clap::Parser;

use env_logger::{Builder, Env};
use log::{LevelFilter, trace};
use miette::{Context, Result};

use skribi::execute;

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Log more information. Fine-grained control.
    ///
    /// The SKRIBI_C_LOG variable can also be used.
    /// To specify a style, use SKRIBI_C_LOG_STYLE.
    /// The variable is overriden by the argument.
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

    let mut logger = Builder::from_env(
        Env::default()
            .filter_or("SKRIBI_C_LOG", "warn")
            .write_style("SKRIBI_C_LOG_STYLE"),
    );

    // To ignore the env variable in production:
    // #[cfg(not (debug_assertions))]
    // logger.filter_level(LevelFilter::Warn);
    // Or detect special values?
    // Did not add the env to clap as this would be a double with from_env
    // in debug mode.

    if let Some(level) = args.verbose {
        logger.filter_level(level);
    }

    logger.init();

    trace!("Logger initialised, entenring main");

    execute().context("Failed to execute your file.")
}
