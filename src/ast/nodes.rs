use crate::ast::nodes::statements::Statement;

pub mod branches;
pub mod calls;
pub mod declarations;
pub mod expressions;
pub mod loops;
pub mod operations;
pub mod primitive_values;
pub mod statements;

pub struct ParsedFileRoot<'a> {
    pub content: Vec<Statement<'a>>,
}

impl ParsedFileRoot<'_> {
    pub fn new<'a>(content: Vec<Statement<'a>>) -> ParsedFileRoot<'a> {
        return ParsedFileRoot { content };
    }
}
