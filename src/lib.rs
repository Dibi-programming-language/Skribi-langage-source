/// This module is used to store ast structs
pub(crate) mod ast;
/// This module handles reading from inputs
pub mod file;
/// Used to lex the files
pub(crate) mod lexer;
/// To parse the tokens into an AST
pub(crate) mod parse;
/// This module handles multi sources
pub mod source;
