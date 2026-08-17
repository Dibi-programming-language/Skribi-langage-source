use crate::ast::nodes::calls::functions::FunctionCall;

#[derive(PartialEq, Clone)]
pub enum Expression {
    FunctionCall(FunctionCall),
}
