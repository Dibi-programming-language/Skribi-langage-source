use std::collections::VecDeque;

use thiserror::Error;

use crate::tokens::TokenContainer;

#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum NotYetImplementedType {
    #[error("Missing symbol, the symbol for this does not exist yet: {0}")]
    MissingSymbol(String),
    #[error("Missing grammar, the grammar for this does not exist yet: {0}")]
    MissingGrammar(String),
    #[error("Not yet voted, the vote for this is not done yet: {0}")]
    NotYetVoted(String),
    #[error("In progress, the implementation for this is in progress: {0}")]
    InProgress(String),
    #[error("Planed, the implementation for this is planed: {0}")]
    Planed(String),
    #[error("Other, other reason: {0}")]
    Other(String),
}

#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum CustomError {
    #[error("Invalid float: {0} at line {1}")]
    InvalidFloat(String, usize),
    #[error("Invalid string: {0} at line {1}")]
    InvalidString(String, usize),
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("Not yet implemented: {0}")]
    NotYetImplemented(NotYetImplementedType),
    // Add other kinds of errors as needed
}

impl CustomError {
    pub fn element_expected(
        from: TokenContainer,
        at: &VecDeque<TokenContainer>,
        what: &str,
    ) -> Self {
        if let Some(token) = at.front() {
            Self::UnexpectedToken(format!(
                "Token at line {}:{} is expecting a {} at line {}:{}",
                from.line, from.column, what, token.line, token.column
            ))
        } else {
            Self::UnexpectedToken(format!(
                "Token at line {}:{} is expecting a(n) {} after the last line.",
                from.line, from.column, what
            ))
        }
    }
}

pub type ShortResult<T> = Result<T, CustomError>;

pub type ResultOption<T> = ShortResult<Option<T>>;
