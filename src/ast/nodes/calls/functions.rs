use chumsky::span::SimpleSpan;

/// Represent a call to a function.
/// TODO: add arguments.
/// TODO: replace name with a full path.
#[derive(PartialEq, Clone)]
pub struct FunctionCall<'tok> {
    pub name: &'tok str,
    pub span: SimpleSpan,
}

impl FunctionCall<'_> {
    pub fn new<'a>(name: &'a str, span: SimpleSpan) -> FunctionCall<'a> {
        FunctionCall { name, span }
    }
}
