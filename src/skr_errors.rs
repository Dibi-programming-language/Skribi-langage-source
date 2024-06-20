use thiserror::Error;

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
pub enum CustomError {
    #[error("Invalid float: {0} at line {1}")]
    InvalidFloat(String, u16),
    #[error("Invalid string: {0} at line {1}")]
    InvalidString(String, u16),
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("Not yet implemented: {0}")]
    NotYetImplemented(NotYetImplementedType),
    // Add other kinds of errors as needed
}

pub type Result<T> = std::result::Result<T, CustomError>;

pub type OptionResult<T> = Option<Result<T>>;

pub type ResultOption<T> = Result<Option<T>>;
