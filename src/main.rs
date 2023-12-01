// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

mod interpret;
mod pre_run;

// Import
use interpret::main as interpret;
use pre_run::{get_instructions, get_path};
use skribi_language_source::{clear, read};
use std::env;

// Main
fn main() {
    // parameters
    let flag_char = "/"; // if it was "-", it would sometimes interfere with cargo's flags
    let extension: Vec<String> = vec!["skrb".to_string(), "skribi".to_string()];

    // generic parameters
    let args: Vec<_> = env::args().collect(); // get the command line arguments

    // clear the shell for the user
    if !args.contains(&String::from(flag_char.to_string() + "interpret-debug")) {
        clear();
    }

    let path = get_path(args.clone(), flag_char);

    // Check if the file has the right extension
    if !extension.contains(&String::from(path.split('.').last().unwrap())) {
        println!("Not a valid file extension");
        return;
    }
    // Read the file
    let lines = read(&path);

    // Remove the comments and split the code into instructions
    let code = get_instructions(lines);

    // interpret the code
    interpret(code, args);
}
