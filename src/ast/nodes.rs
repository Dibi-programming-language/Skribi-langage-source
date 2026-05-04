use crate::ast::nodes::statements::Statement;

pub mod base;
pub mod calls;
pub mod conditions;
pub mod declarations;
pub mod expressions;
pub mod loops;
pub mod operations;
pub mod statements;

pub struct AstRoot<'a> {
    pub content: Vec<Statement<'a>>,
}

impl AstRoot<'_> {
    pub fn new<'a>(content: Vec<Statement<'a>>) -> AstRoot<'a> {
        return AstRoot { content };
    }
}
