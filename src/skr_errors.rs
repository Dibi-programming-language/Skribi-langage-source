use std::fmt;

#[derive(Debug, PartialEq)]
pub enum NotYetImplementedType {
    MissingSymbol(String),
    MissingGrammar(String),
    NotYetVoted(String),
    InProgress(String),
    Planed(String),
    Other(String),
}

impl fmt::Display for NotYetImplementedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotYetImplementedType::MissingSymbol(message) => write!(f, "Missing symbol: {}", message),
            NotYetImplementedType::MissingGrammar(message) => write!(f, "Missing grammar: {}", message),
            NotYetImplementedType::NotYetVoted(message) => write!(f, "Not yet voted: {}", message),
            NotYetImplementedType::InProgress(message) => write!(f, "In progress: {}", message),
            NotYetImplementedType::Planed(message) => write!(f, "Planed: {}", message),
            NotYetImplementedType::Other(message) => write!(f, "Other: {}", message),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CustomError {
    InvalidFloat(String, u16),
    InvalidString(String, u16),
    UnexpectedToken(String),
    NotYetImplemented(NotYetImplementedType),
    // Add other kinds of errors as needed
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::InvalidFloat(message, line) => write!(f, "Invalid float: {} at line {}", message, line),
            CustomError::InvalidString(message, line) => write!(f, "Invalid string: {} at line {}", message, line),
            CustomError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            CustomError::NotYetImplemented(error) => write!(f, "Not yet implemented: {}", error),
            _ => write!(f, "Unknown error"),
            // Handle other kinds of errors as needed
        }
    }
}

impl std::error::Error for CustomError {}