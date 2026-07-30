use chumsky::span::SimpleSpan;

/// Represents a deprecated parsing feature
#[derive(PartialEq, Clone)]
pub struct Deprecated {
    pub message: &'static str,
    pub span: SimpleSpan,
}

impl Deprecated {
    pub fn new(message: &'static str, span: SimpleSpan) -> Self {
        Deprecated { message, span }
    }
}
