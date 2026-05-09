use std::{collections::VecDeque, fmt::Display, io::ErrorKind};

use ariadne::{Color, Label, Report, Source};
use chumsky::error::Rich;
use colored::Colorize;
use thiserror::Error;

use crate::{
    execute::ExecutionError,
    get_file_content::GotFile,
    tokens::{NewTokens, TokenContainer},
};

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
pub enum ParsingError {
    #[error("Invalid float: {0} at line {1}")]
    InvalidFloat(String, usize),
    #[error("Invalid string: {0} at line {1}")]
    InvalidString(String, usize),
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("Not yet implemented: {0}")]
    NotYetImplemented(#[from] NotYetImplementedType),
    // Add other kinds of errors as needed
}

impl ParsingError {
    pub fn element_expected(
        from: TokenContainer,
        at: &VecDeque<TokenContainer>,
        what: &str,
    ) -> Self {
        Self::UnexpectedToken(format!(
            "{}",
            if let Some(token) = at.front() {
                format!(
                    "Token at line {}:{} is expecting a(n) {} at line {}:{}",
                    from.line, from.column, what, token.line, token.column
                )
            } else {
                format!(
                    "Token at line {}:{} is expecting a(n) {} after the last line.",
                    from.line, from.column, what
                )
            }
            .bold()
            .red()
        ))
    }
}

pub type ShortResult<T> = Result<T, ParsingError>;

pub type ResultOption<T> = ShortResult<Option<T>>;

#[derive(Error, Debug)]
pub enum RootError {
    ParsingError(#[from] ParsingError),
    ExecutionError(#[from] ExecutionError),
    FileError(Vec<String>, ErrorKind),
    GlobalParsingError,
    EmptyFile,
    CompilationError,
}

impl RootError {
    pub fn num(&self) -> usize {
        match self {
            Self::GlobalParsingError => 1,
            _ => 0,
        }
    }

    pub fn show(&self) {
        eprintln!("{}", self);
    }
}

impl Display for RootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParsingError(pe) => {
                write!(f, "{}\n{}", "Parsing error: the code is wrong.".red(), pe)
            }
            Self::ExecutionError(ee) => write!(
                f,
                "{}\n{}\n{}",
                "Execution error: your program stopped in an unexpected way.".red(),
                ee,
                "End of error message.".red()
            ),
            Self::FileError(vec, kind) => write!(
                f,
                "{}\n{}\nValid file extensions: {vec:?}.\nError message: {kind}.",
                "Error while getting the content of the file.".red(),
                "Check the file extension and the file path.".green()
            ),
            Self::EmptyFile => write!(
                f,
                "{}",
                "This file does not have any executable content.".red()
            ),
            Self::GlobalParsingError => write!(f, "Parsing Error"),
            Self::CompilationError => write!(f, "Compilation Error"),
        }
    }
}

pub fn print_parsing_errors(errors: Vec<Rich<'_, NewTokens<'_>>>, content: &GotFile) {
    let gap = errors.len() < 5;
    for err in errors {
        Report::build(
            ariadne::ReportKind::Error,
            (content.to_string(), err.span().into_range()),
        )
        .with_config(
            ariadne::Config::new()
                .with_index_type(ariadne::IndexType::Byte)
                .with_compact(!gap),
        )
        .with_code(RootError::GlobalParsingError.num())
        .with_message(err.to_string())
        .with_label(
            Label::new((content.to_string(), err.span().into_range()))
                .with_message(err.reason().to_string())
                .with_color(Color::Red),
        )
        .finish()
        .eprint((content.to_string(), Source::from(&content.content)))
        .expect("Error message failed to show error");
    }
}
