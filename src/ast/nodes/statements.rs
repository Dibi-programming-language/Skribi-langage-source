use crate::ast::nodes::expressions::Expression;

#[derive(PartialEq, Clone)]
pub enum Statement<'tok> {
    Expression(Expression<'tok>),
}
