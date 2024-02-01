// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

mod get_file_content;
mod tokenize;

// Import
use get_file_content::{get_content};
use skribi_language_source::{clear};
use std::env;
use tokenize::{ tokenize, Token };

const FLAG_CHAR: &str = "--";

/**
 * Launch the interpreter
 */
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

    let tokens = tokenize(content);


    // test
    for token in tokens {
        match token {
            Token::StringLiteral(string) => println!("StringLiteral: {}", string),
            Token::IntLiteral(int) => println!("IntLiteral: {}", int),
            Token::BooleanLiteral(boolean) => println!("BooleanLiteral: {}", boolean),
            _ => println!("{:?}", token),
        }
    }
}
