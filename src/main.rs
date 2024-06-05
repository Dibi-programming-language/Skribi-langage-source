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
mod interpret;
mod parse;
mod skr_errors;
mod tokenize;
mod tokens;
mod utils;
#[cfg(test)]
mod tests;

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

    if let Ok(content) = get_content(args, extension.clone()) {
        // Read the file
        let lines = content;

        // Remove the comments and split the code into instructions
        match tokenize(lines) {
            Ok(tokens) => {
                let tokens_deque = tokens.into_iter().collect();
                let _nodes = parse::main(tokens_deque);
                // TODO
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    } else {
        panic!("Error while getting the content of the file. Check the file extension and the file path. Valid file extensions : {:?}", extension.clone());
    }
}
