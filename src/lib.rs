use ariadne::{Color, Label, Report, Source};
use colored::Colorize;
use get_file_content::get_content;
use skr_errors::RootError;

use crate::ast::visitors::pretty::Pretty;
use crate::execute::{Execute, ExecutionContext};
use crate::parse::new_parse;
use crate::skr_errors::ErrorCodes;
// Import
use crate::tokens::{new_tokenise, tokenize};
use crate::utils::clear;

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
                    Ok(())
                },
                Err(errs) => {
                    let gap = errs.len() < 5;
                    for err in errs {
                        Report::build(
                            ariadne::ReportKind::Error,
                            (content.to_string(), err.span().into_range()),
                        )
                        .with_config(
                            ariadne::Config::new()
                                .with_index_type(ariadne::IndexType::Byte)
                                .with_compact(!gap),
                        )
                        .with_code(ErrorCodes::ParsingError.num())
                        .with_message(err.to_string())
                        .with_label(
                            Label::new((content.to_string(), err.span().into_range()))
                                .with_message(err.reason().to_string())
                                .with_color(Color::Red),
                        )
                        .finish()
                        .eprint((content.to_string(), Source::from(&content.content)))
                        .expect("Error message failed to show error");
                    }
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
