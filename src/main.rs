// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use clap::Parser;

use log::trace;
use miette::{Context, Result};

use skribi::{file::File, source::SourceManager};

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// The source file to use. Defaults to STDIN.
    /// STDIN is currently not supported.
    source: Option<String>,
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

    if let Some(path) = args.source {
        let file = File::from_file(&path).context("While reading file passed as argument")?;
        let mut manager = SourceManager::empty();
        manager.add_file(file);

        if args.run {
            manager.execute()
        } else {
            manager.compile()
        }
    } else {
        todo!("STDIN is currently not supported")
    }
}
