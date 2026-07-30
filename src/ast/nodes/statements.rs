use crate::ast::nodes::{deprecated::Deprecated, expressions::Expression};

#[derive(PartialEq, Clone)]
pub enum Statement<'tok> {
    Expression(Expression<'tok>),
    Deprecated(Deprecated),
}
