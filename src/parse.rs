use std::collections::VecDeque;

use crate::parse::nodes::files_node::{parse_file, FileNode};
use crate::skr_errors::OptionResult;
use crate::tokens::Token;

pub(crate) mod nodes;

/// Parse the tokens into an AST.
pub fn parse(mut tokens: VecDeque<Token>) -> OptionResult<FileNode> {
    // TODO - this fonction is dependant of functions that are not yet implemented
    // This function will add more code when the other functions are implemented
    parse_file(&mut tokens)
}
