use std::collections::VecDeque;

use crate::parse::nodes::expressions::{Exp, parse_exp};
use crate::skr_errors::CustomError;
use crate::tokens::Token;

/// Node representing a file. This is the root node of the AST.
pub struct FileNode {
    exps: Vec<Exp>,
}

pub fn parse_file(tokens: &mut VecDeque<Token>) -> Option<Result<FileNode, CustomError>> {
    let mut exps = Vec::new();
    loop {
        match parse_exp(tokens) {
            Some(Ok(exp)) => {
                exps.push(exp);
            }
            Some(Err(e)) => {
                return Some(Err(e));
            }
            None => {
                break;
            }
        }
    }

    Some(Ok(FileNode { exps }))
}
