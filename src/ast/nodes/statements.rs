use crate::ast::nodes::{calls::FunctionCall, expressions::Expression};

#[derive(PartialEq, Clone)]
pub struct Return<'a> {
    exp: Expression<'a>,
}

#[derive(PartialEq, Clone)]
pub enum Statement<'a> {
    Return(Return<'a>),
    Exp(Expression<'a>),
    NatCall(FunctionCall<'a>),
}

#[derive(PartialEq, Clone)]
pub struct StatementList<'a> {
    statements: Vec<Statement<'a>>,
    unused: bool,
    simple: bool,
    bubble: bool,
}
