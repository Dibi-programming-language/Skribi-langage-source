use std::collections::VecDeque;

use crate::parse::nodes::files_node::FileNode;
use crate::skr_errors::ResultOption;
use crate::tokens::Token;

pub(crate) mod nodes;

/// Parse the tokens into an AST.
pub fn parse(mut tokens: VecDeque<Token>) -> ResultOption<FileNode> {
    // This function will add more code when the other functions are implemented
    FileNode::parse(&mut tokens)
}
