// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

mod interpret;
mod parse;
mod pre_run;
mod tokens;

// Import
use crate::tokens::tokenize;
use interpret::main as interpret;
use pre_run::{get_instructions, get_path};
use skribi_language_source::{clear, read};
use std::env;

const FLAG_CHAR: &str = "--";

/// Launch the interpreter
fn main() {
    // parameters
    let extension: Vec<String> = vec!["skrb".to_string(), "skribi".to_string()];

    // generic parameters
    let args = env::args().collect::<Vec<_>>(); // get the command line arguments

    // clear the shell for the user
    if !args.contains(&format!("{FLAG_CHAR}interpret-debug")) {
        clear();
    }

    let path = get_path(args.clone());

    // Check if the file has the right extension
    if !extension.contains(&String::from(path.split('.').last().unwrap())) {
        println!("Not a valid file extension");
        println!("Valid file extensions:");
        for ext in extension {
            println!("\t{}", ext);
        }
        return;
    }

    // Read the file
    let lines = read(&path);

    // Remove the comments and split the code into instructions
    let code = tokenize(lines);
    let nodes = parse::main(code);

    // Parse the code

    // interpret the code
    // interpret(code, args);
}
