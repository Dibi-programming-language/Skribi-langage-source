use chumsky::span::SimpleSpan;

/// Represents a deprecated parsing feature
#[derive(PartialEq, Clone)]
pub struct Deprecated {
    message: &'static str,
    span: SimpleSpan,
}

impl Deprecated {
    pub fn new(message: &'static str, span: SimpleSpan) -> Self {
        Deprecated { message, span }
    }
}
