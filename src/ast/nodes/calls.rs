use crate::ast::nodes::expressions::Expression;

#[derive(PartialEq, Clone)]
pub struct IdentifierChain<'a> {
    identifier: &'a str,
    previous: Option<Box<Expression<'a>>>,
}

/// Represent a call to a function.
/// Identifier is the name of the function.
/// Native indicates if this call refers to a function implemented
/// by the compiler itself, or even in the runtime.
#[derive(PartialEq, Clone)]
pub struct FunctionCall<'a> {
    pub native: bool,
    pub identifier: IdentifierChain<'a>,
    pub arguments: Vec<Expression<'a>>,
}

impl FunctionCall<'_> {
    pub fn new<'a>(identifier: &'a str) -> FunctionCall<'a> {
        FunctionCall {
            native: false,
            identifier: IdentifierChain {
                identifier,
                previous: None,
            },
            arguments: vec![],
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct VariableModification<'a> {
    pub identifier: IdentifierChain<'a>,
    pub value: Expression<'a>,
}
