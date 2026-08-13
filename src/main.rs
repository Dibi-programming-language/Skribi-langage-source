// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

/// This module is used to store ast structs
mod ast;
/// Arguments of the main program
mod cli;
/// This module handles reading from inputs
mod file;
/// The global interner mutex
mod interner;
/// Used to lex the files
mod lexer;
/// To parse the tokens into an AST
mod parse;
/// This module handles multi sources
mod source;

use clap::Parser;

use env_logger::{Builder, Env};
use log::trace;
use miette::{Result, set_hook, set_panic_hook};

use cli::Arguments;

/// Launch the interpreter
fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut logger = Builder::from_env(
        Env::default()
            .filter_or("SKRIBI_C_LOG", "warn")
            .write_style("SKRIBI_C_LOG_STYLE"),
    );

    // Allows to render panics using miette
    // Allows an uniform representation of errors
    set_panic_hook();

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

    set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .show_related_errors_as_nested()
                .build(),
        )
    }))?;

    args.cmd.execute()
}
