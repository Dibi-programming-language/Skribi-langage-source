use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CustomError {
    InvalidFloat(String, u16),
    InvalidString(String, u16),
    UnexpectedToken(String),
    // Add other kinds of errors as needed
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::InvalidFloat(message, line) => write!(f, "Invalid float: {} at line {}", message, line),
            CustomError::InvalidString(message, line) => write!(f, "Invalid string: {} at line {}", message, line),
            CustomError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            _ => write!(f, "Unknown error"),
            // Handle other kinds of errors as needed
        }
    }
}

impl std::error::Error for CustomError {}