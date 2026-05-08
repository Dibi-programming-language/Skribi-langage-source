use crate::ast::nodes::{calls::FunctionCall, expressions::Expression};

pub struct Return<'a> {
    exp: Expression<'a>,
}

pub enum Statement<'a> {
    Return(Return<'a>),
    Exp(Expression<'a>),
    NatCall(FunctionCall<'a>),
}

pub struct StatementList<'a> {
    statements: Vec<Statement<'a>>,
    unused: bool,
    simple: bool,
    bubble: bool,
}
