use std::collections::VecDeque;

use crate::parse::nodes::expressions::{parse_exp, Exp};
use crate::skr_errors::OptionResult;
use crate::tokens::Token;

/// Node representing a file. This is the root node of the AST.
pub struct FileNode {
    exps: Vec<Exp>,
}

pub fn parse_file(tokens: &mut VecDeque<Token>) -> OptionResult<FileNode> {
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
