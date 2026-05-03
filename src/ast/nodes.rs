use crate::ast::nodes::statements::Statement;

mod base;
mod calls;
mod conditions;
mod expressions;
mod operations;
mod statements;
mod declarations;
mod loops;

pub struct AstRoot<'a> {
    content: Vec<Statement<'a>>
}
