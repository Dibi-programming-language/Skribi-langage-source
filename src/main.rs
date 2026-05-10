// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::env;
use std::process::ExitCode;
use clap::Parser;

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
fn main() -> ExitCode {
    let new_args = Arguments::parse();
    if new_args.clear {
        clear();
    }



    // generic parameters
    let args = env::args().collect::<Vec<_>>(); // get the command line arguments

    match new_execute(args, new_args.verbose) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("\n{err}");
            ExitCode::FAILURE
        }
    }
}
