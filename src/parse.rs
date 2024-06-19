use std::collections::VecDeque;
use crate::parse::nodes::files_node::{FileNode, parse_file};
use crate::skr_errors::CustomError;
use crate::tokens::Token;

pub(crate) mod nodes;

pub fn main(mut tokens: VecDeque<Token>) -> Option<Result<FileNode, CustomError>> {
    // TODO - this fonction is dependant of functions that are not yet implemented
    // This function will add more code when the other functions are implemented
    parse_file(&mut tokens)
}
