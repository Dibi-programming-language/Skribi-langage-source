use crate::ast::nodes::statements::Statement;

mod base;
mod calls;
mod conditions;
mod declarations;
mod expressions;
mod loops;
mod operations;
mod statements;

pub struct AstRoot<'a> {
    content: Vec<Statement<'a>>,
}
