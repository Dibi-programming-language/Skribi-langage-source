use std::fmt::Error;

use crate::parse::nodes::expressions::{Exp, parse_exp};
use crate::tokens::Token;

pub struct FileNode {
    exps: Vec<Exp>,
}

fn parse_file(tokens: &mut Vec<Token>) -> Option<Result<FileNode, Error>> {
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
