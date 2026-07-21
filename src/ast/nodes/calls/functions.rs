/// Represent a call to a function.
/// TODO: add arguments.
/// TOOD: replace name with a full path.
#[derive(PartialEq, Clone)]
pub struct FunctionCall<'tok> {
    pub name: &'tok str,
}

impl FunctionCall<'_> {
    pub fn new<'a>(name: &'a str) -> FunctionCall<'a> {
        FunctionCall {
            name,
        }
    }
}

