use chumsky::span::SimpleSpan;
use string_interner::DefaultSymbol;

/// Represent a call to a function.
/// TODO: add arguments.
/// TODO: replace name with a full path.
#[derive(PartialEq, Clone)]
pub struct FunctionCall {
    pub name: DefaultSymbol,
    pub span: SimpleSpan,
}

impl FunctionCall {
    pub fn new(name: DefaultSymbol, span: SimpleSpan) -> FunctionCall {
        FunctionCall { name, span }
    }
}
