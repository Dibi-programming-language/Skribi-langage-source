use std::fs::create_dir_all;
use std::path::Path;

use colored::Colorize;
use get_file_content::get_content;
use skr_errors::RootError;

use crate::ast::visitors::compile::CodeGenerator;
use crate::ast::visitors::pretty::Pretty;
use crate::execute::{Execute, ExecutionContext};
use crate::parse::new_parse;
use crate::skr_errors::print_parsing_errors;
// Import
use crate::tokens::{new_tokenise, tokenize};
use crate::utils::{clear, link_files};

pub(crate) mod ast;
pub mod execute;
mod get_file_content;
mod parse;
mod skr_errors;
#[cfg(test)]
mod tests;
mod tokens;
mod utils;

const FLAG_CHAR: &str = "--";

pub fn new_execute(args: Vec<String>, verbose: bool) -> Result<(), RootError> {
    // parameters
    let extension: Vec<String> = vec!["skrb".to_string(), "skribi".to_string()];
    if create_dir_all(".skribi").is_err() {
        RootError::CompilationError.show();
    }

    // clear the shell for the user
    if args.contains(&format!("{FLAG_CHAR}clear")) && verbose {
        clear();
    }

    // Read the file
    match get_content(args, extension.clone()) {
        Ok(content) => {
            if verbose {
                eprintln!("{}", "Reading...".italic())
            };
            // Remove the comments and split the code into instructions
            let tokens = new_tokenise(&content.content);

            if verbose {
                eprintln!("{}", "Analysing...".italic())
            };

            match new_parse(tokens, content.content.len()) {
                Ok(ast) => {
                    Pretty::eprint(&ast);
                    let name = content.to_str();
                    if let Err(_) = CodeGenerator::compile(&ast, verbose, name) {
                        RootError::CompilationError.show();
                    }
                    let raw_path = Path::new(".skribi").join(name).with_added_extension("ll");
                    let path = raw_path.to_str().expect("Compiled file not found");
                    if link_files(vec![path], name).is_err() {
                        RootError::CompilationError.show();
                    }
                    println!("Saving result into {}", name);
                    Ok(())
                }
                Err(errs) => {
                    print_parsing_errors(errs, &content);
                    Err(RootError::GlobalParsingError)
                }
            }
        }
        Err(err) => Err(RootError::FileError(extension, err)),
    }
}

pub fn execute(args: Vec<String>, verbose: bool) -> Result<(), RootError> {
    // parameters
    let extension: Vec<String> = vec!["skrb".to_string(), "skribi".to_string()];

    // clear the shell for the user
    if args.contains(&format!("{FLAG_CHAR}clear")) && verbose {
        clear();
    }

    let show_ast = args.contains(&format!("{FLAG_CHAR}show-ast")) && verbose;

    // Read the file
    match get_content(args, extension.clone()) {
        Ok(content) => {
            if verbose {
                eprintln!("{}", "Reading...".italic())
            };
            // Remove the comments and split the code into instructions
            let tokens = tokenize(content.content)?;

            if verbose {
                eprintln!("{}", "Analysing...".italic())
            };
            let nodes = parse::parse(tokens)?;
            if let Some(ast) = nodes {
                if verbose {
                    eprintln!("{}", "Executing...".italic())
                };
                if show_ast {
                    println!("{:?}", ast);
                }
                ast.execute(&mut ExecutionContext::new())?;
                if verbose {
                    eprintln!("\n{}", "Program's end with no error".bold())
                };
                Ok(())
            } else {
                Err(RootError::EmptyFile)
            }
        }
        Err(err) => Err(RootError::FileError(extension, err)),
    }
}
