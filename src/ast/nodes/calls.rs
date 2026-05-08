use crate::ast::nodes::expressions::Expression;

pub struct IdentifierChain<'a> {
    identifier: &'a str,
    previous: Option<Box<Expression<'a>>>,
}

/// Represent a call to a function.
/// Identifier is the name of the function.
/// Native indicates if this call refers to a function implemented
/// by the compiler itself, or even in the runtime.
pub struct FunctionCall<'a> {
    native: bool,
    function_path: IdentifierChain<'a>,
    arguments: Vec<Expression<'a>>,
}

pub struct VariableReassignment<'a> {
    identifier: IdentifierChain<'a>,
    value: Expression<'a>,
}
