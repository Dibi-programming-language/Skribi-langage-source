// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

mod interpret;
mod parse;
mod pre_run;
mod tokens;
mod get_file_content;
mod tokenize;

// Import
use crate::tokens::tokenize;
use interpret::main as interpret;
use pre_run::{get_instructions, get_path};
use skribi_language_source::{clear, read};
use get_file_content::{get_content};
use std::env;
use tokenize::{ tokenize, Token };

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

    let content = get_content(args.clone(), extension);

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
    /*let tokens = tokenize(content);*/


    // test
    /*for token in tokens {
        match token {
            Token::StringLiteral(string) => println!("StringLiteral: {}", string),
            Token::IntLiteral(int) => println!("IntLiteral: {}", int),
            Token::BooleanLiteral(boolean) => println!("BooleanLiteral: {}", boolean),
            _ => println!("{:?}", token),
        }
    }
    */
}
