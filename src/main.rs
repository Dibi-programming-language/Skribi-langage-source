// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::env;
use std::process::ExitCode;

use colored::Colorize;
use get_file_content::get_content;
use skr_errors::{RootError};

use crate::execute::{Execute, ExecutionContext};
// Import
use crate::tokens::tokenize;
use crate::utils::clear;

pub mod execute;
mod get_file_content;
mod parse;
mod skr_errors;
#[cfg(test)]
mod tests;
mod tokens;
mod utils;

const FLAG_CHAR: &str = "--";

/// Launch the interpreter
fn main() -> ExitCode {
    // generic parameters
    let args = env::args().collect::<Vec<_>>(); // get the command line arguments

    match execute(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        },
    }
}

pub fn execute(args: Vec<String>) -> Result<(), RootError> {
    // parameters
    let extension: Vec<String> = vec!["skrb".to_string(), "skribi".to_string()];

    // clear the shell for the user
    if args.contains(&format!("{FLAG_CHAR}clear")) {
        clear();
    }

    let show_ast = args.contains(&format!("{FLAG_CHAR}show-ast"));

    // Read the file
    match get_content(args, extension.clone()) {
        Ok(content) => {
            eprintln!("{}", "Reading...".italic());
            // Remove the comments and split the code into instructions
            let tokens = tokenize(content)?;

            eprintln!("{}", "Analysing...".italic());
            let nodes = parse::parse(tokens)?;
            if let Some(ast) = nodes {
                eprintln!("{}", "Executing...".italic());
                if show_ast {
                    println!("{:?}", ast);
                }
                ast.execute(&mut ExecutionContext::new())?;
                eprintln!("\n{}", "Program's end with no error".bold());
                return Ok(());
            } else {
                Err(RootError::EmptyFile)
            }
        }
        Err(err) => {
            Err(RootError::FileError(extension, err))
        }
    }
}
