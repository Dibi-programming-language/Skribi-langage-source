// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::env;

use get_file_content::get_content;

// Import
use crate::tokens::tokenize;
use crate::utils::clear;

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
    // parameters
    let extension: Vec<String> = vec!["skrb".to_string(), "skribi".to_string()];

    // generic parameters
    let args = env::args().collect::<Vec<_>>(); // get the command line arguments

    // clear the shell for the user
    if !args.contains(&format!("{FLAG_CHAR}compiler-debug")) {
        clear();
    }

    match get_content(args, extension.clone()) {
        Ok(content) => {
            // Read the file
            let lines = content;

            // Remove the comments and split the code into instructions
            match tokenize(lines) {
                Ok(tokens) => {
                    let tokens_deque = tokens.into_iter().collect();
                    let _nodes = parse::parse(tokens_deque);
                    // TODO
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
