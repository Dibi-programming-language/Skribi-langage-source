// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::env;

use colored::Colorize;
use get_file_content::get_content;

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
fn main() {
    // generic parameters
    let args = env::args().collect::<Vec<_>>(); // get the command line arguments

    execute(args);
}

pub fn execute(args: Vec<String>) {
    // parameters
    let extension: Vec<String> = vec!["skrb".to_string(), "skribi".to_string()];

    // clear the shell for the user
    if !args.contains(&format!("{FLAG_CHAR}compiler-debug")) {
        clear();
    }

    let show_ast = args.contains(&format!("{FLAG_CHAR}show-ast"));

    // Read the file
    match get_content(args, extension.clone()) {
        Ok(content) => {
            println!("{}", "Reading...".italic());
            // Remove the comments and split the code into instructions
            match tokenize(content) {
                Ok(tokens) => {
                    println!("{}", "Analysing...".italic());
                    let nodes = parse::parse(tokens);
                    if let Ok(Some(ast)) = nodes {
                        println!("{}", "Executing...".italic());
                        if show_ast {
                            println!("{:?}", ast);
                        }
                        let result = ast.execute(&mut ExecutionContext::new());
                        if let Err(err) = result {
                            println!();
                            err.show();
                            panic!(
                                "{}",
                                "--- Your program stopped in a unexpected way ---".red()
                            );
                        } else {
                            println!();
                            println!("{}", "Program's end with no error".bold());
                        }
                    } else if let Err(err) = nodes {
                        panic!("{} {:?}", "--- The code is wrong ---\n".red(), err)
                    } else {
                        panic!(
                            "{}",
                            "--- This file does not have any executable content ---".red()
                        );
                    }
                }
                Err(err) => {
                    panic!("{:?}", err);
                }
            }
        }
        Err(err) => {
            panic!("Error while getting the content of the file. Check the file extension and the file path. Valid file extensions : {:?}. Error message : {:?}", extension.clone(), err);
        }
    }
}
