// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use clap::Parser;

use miette::{Context, Result};
use skribi::get_file_content::new_get_content;
use skribi::new_execute;
use skribi::utils::clear;

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// The file to operate on, use stdin by default
    file: Option<String>,
    /// Use this to print the AST after parsing
    #[arg(short, long)]
    ast: bool,
    /// Use this to print some debug information
    #[arg(short, long)]
    verbose: bool,
    /// Clear the screen before executing
    #[arg(short, long)]
    clear: bool,
}

/// Launch the interpreter
fn main() -> Result<()> {
    let new_args = Arguments::parse();
    if new_args.clear {
        clear();
    }

    let gotfile = new_get_content(&new_args.file)
        .context("While starting CLI")?;

    new_execute(gotfile, new_args.verbose)
}
