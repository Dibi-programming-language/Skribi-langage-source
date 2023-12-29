use crate::tokens::{ModifierKeyword, Token};
use skribi_language_source::error;

pub enum ValueNode {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Unset,
}

pub enum Node {
    Scope(Vec<Node>),
    NewVariable(Vec<ModifierKeyword>, String, ValueNode),
    NewValue(String, ValueNode),
    NativeCall(String, Vec<ValueNode>),
}

fn parse_scope(tokens: &Vec<Token>, i: &mut usize, line: &mut u16) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut not_finished = true;

    // Start an iterator with a index
    while not_finished && *i < tokens.len() {
        match tokens[*i] {
            Token::KeywordModifier(_) => {
                // Start a new variable
            }
            Token::KeywordNativeCall => {
                // Start a new native call
            }
            Token::Identifier(_) => {
                // Check if the identifier exists in this scope, and his type
            }
            Token::OpenBrace => {
                // Start a new scope
                *i += 1;
                let scope_nodes = parse_scope(tokens, i, line);
                nodes.push(Node::Scope(scope_nodes));
            }
            Token::CloseBrace => {
                // Close the current scope
                not_finished = false;
            }
            Token::NewLine => {
                *line += 1;
            }
            // Ignored tokens
            Token::Semicolon => {}
            _ => {
                error("[PARSE] Invalid token or not implemented yet!", *line);
            }
        }
        *i += 1;
    }

    nodes
}

pub fn main(tokens: Vec<Token>) -> Vec<Node> {
    let mut line = 0;
    let mut i = 0;
    let nodes = parse_scope(&tokens, &mut i, &mut line);
    if i != tokens.len() {
        error("Scope closed with } before the end", line);
    }
    nodes
}
