mod parse_variables;
mod parse_values;

use std::collections::LinkedList;
use crate::tokens::{ModifierKeyword, Token, ValueToken};
use skribi_language_source::error;

pub enum ValueNode {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Unset,
}

pub enum Value {
    ValueNode(ValueToken),
    Operation(Operation)
}

pub enum Operation {
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Mul(Box<Value>, Box<Value>),
    Div(Box<Value>, Box<Value>),
    Mod(Box<Value>, Box<Value>),
    Pow(Box<Value>, Box<Value>),
}

pub enum Node {
    Scope(Vec<Node>),
    NewVariable(Vec<ModifierKeyword>, String, ValueNode),
    NewValue(String, ValueNode),
    NativeCall(String, Vec<ValueNode>),
    Operation(Operation)
}

struct ParseFunction {
    name: String,
    arguments: Vec<String>,
    return_type: String,
}

/// Only used to check if a variable exists in a scope
struct ParseScope {
    /// Variables that can be used in this scope
    variables: Vec<String>,
    /// Types that can be used in this scope
    types: Vec<String>,
    /// Functions that can be used in this scope. UNUSED FOR NOW
    functions: Vec<ParseFunction>,
    parent: Option<Box<ParseScope>>,
}

impl ParseScope {
    fn new(parent: Option<Box<ParseScope>>) -> Self {
        ParseScope {
            variables: Vec::new(),
            types: Vec::new(),
            functions: Vec::new(),
            parent,
        }
    }

    fn base() -> Self {
        ParseScope {
            variables: Vec::new(),
            types: vec![
                "skr".to_string(),
                "int".to_string(),
                "dar".to_string(),
                "ioi".to_string(),
            ],
            functions: Vec::new(),
            parent: None,
        }
    }

    /// Check if a name can be used in this scope for a variable
    fn is_valid_name_for_variable(&self, name: String) -> bool {
        !(
            self.variables.contains(&name)
            || self.types.contains(&name)
            || self.functions.iter().any(|f| f.name == name)
            || (
                if let Some(parent) = &self.parent {
                    parent.is_valid_name_for_variable(name)
                } else {
                    false
                }
            )
        )
    }

    /// Check if a type exists in this scope
    fn is_valid_type(&self, name: String) -> bool {
        self.types.contains(&name)
        || (
            if let Some(parent) = &self.parent {
                parent.is_valid_type(name)
            } else {
                false
            }
        )
    }

    /// Check if a variable exists in this scope
    fn is_valid_variable(&self, name: String) -> bool {
        self.variables.contains(&name)
        || (
            if let Some(parent) = &self.parent {
                parent.is_valid_variable(name)
            } else {
                false
            }
        )
    }
}

fn parse_scope(
    tokens: &Vec<Token>,
    i: &mut usize,
    line: &mut u16,
    variables: &Vec<Vec<String>>,
) -> Vec<Node> {
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
                let scope_nodes = parse_scope(tokens, i, line, variables);
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

enum TreeElement {
    Node(Node),
    Token(Token)
}

// macro

fn parse_scope2(
    mut tokens: LinkedList<TreeElement>,
    i: &mut usize,
    line: &mut u16,
    variables: &Vec<Vec<String>>,
) -> Vec<Node> {
    let mut not_finished = true;
    let mut before: LinkedList<TreeElement> = LinkedList::new();
    // Current node
    let mut element: TreeElement = match tokens.pop_front() {
        None => {
            return Vec::new();
        }
        Some(n) => {
            n
        }
    };

    // Start an iterator with a index
    while not_finished && *i < tokens.len() {
        match element {
            TreeElement::Token(Token::NewLine) => {
                *line += 1;
            }
            TreeElement::Token(Token::Value(_)) => {
                match tokens.pop_front() {
                    None => {
                        // SKIP
                    }
                    Some(token) => {
                        match token {
                            TreeElement::Token(Token::OperatorMul) => {
                                match tokens.pop_front() {
                                    None => {

                                    }
                                    Some(token) => {
                                        match token {
                                            TreeElement::Token(Token::Value(_)) => {
                                                // create
                                                let op = Operation::Mul(Box::new(Value::ValueNode(ValueToken::Integer(0))), Box::new(Value::ValueNode(ValueToken::Integer(0))));
                                                element = TreeElement::Node(Node::Operation(op));
                                            }
                                            _ => {
                                                // SKIP
                                                before.push_back(element);
                                                element = token;
                                            }
                                        }
                                    }
                                }
                            },
                            _ => {
                                // SKIP
                                before.push_back(element);
                                element = token;
                            }
                        }
                    }
                }
            }
            TreeElement::Node(_) => {

            }
            _ => {
                // SKIP
                before.push_back(element);
                match tokens.pop_front() {
                    None => {
                        tokens = before;
                        before = LinkedList::new();
                        match tokens.pop_front() {
                            None => {
                                element = TreeElement::Token(Token::NewLine);
                                not_finished = false;
                            }
                            Some(e) => {
                                element = e;
                            }
                        }
                    }
                    Some(n) => {
                        element = n;
                    }
                }
            }
        }
    }

    Vec::new()
}

pub fn main(tokens: Vec<Token>) -> Vec<Node> {
    let mut line = 0;
    let mut i = 0;
    let vec: Vec<Vec<String>> = Vec::new();
    let nodes = parse_scope(&tokens, &mut i, &mut line, &vec);
    if i != tokens.len() {
        error("Scope closed with } before the end", line);
    }
    nodes
}
