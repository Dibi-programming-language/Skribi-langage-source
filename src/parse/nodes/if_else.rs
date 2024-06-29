use std::collections::VecDeque;
use crate::impl_debug;
use crate::parse::nodes::expressions::Exp;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, ResultOption};
use crate::tokens::Token;

// Grammar for this file:
// <sula> ::= sula (<ij> (<sula> |) | <scope>)
// <ij> ::= ij <exp> <scope>
// <cond> ::= <ij> (<sula> |)

// TODO : scope

#[derive(PartialEq)]
pub struct Scope {}

impl GraphDisplay for Scope {
    fn graph_display(&self, _graph: &mut String, _id: &mut usize) {
        // TODO
    }
}

impl_debug!(Scope);

impl Scope {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn parse(_tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // TODO
        Ok(Some(Scope::new()))
    }
}

// ------------
// --- Sula ---
// ------------

#[derive(PartialEq)]
pub enum Sula {
    Ij {
        ij: Ij,
        sula: Option<Box<Sula>>,
    },
    Scope(Scope)
}

impl GraphDisplay for Sula {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Sula_{}[Sula]", id));
        *id += 1;
        match self {
            Sula::Ij { ij, sula } => {
                ij.graph_display(graph, id);
                if let Some(sula) = sula {
                    sula.graph_display(graph, id);
                }
            }
            Sula::Scope(scope) => {
                scope.graph_display(graph, id);
            }
        }
    }
}

impl_debug!(Sula);

impl Sula {
    pub fn new(ij: Ij, sula: Option<Sula>) -> Self {
        Self::Ij {
            ij,
            sula: sula.map(Box::new),
        }
    }
    
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <sula> ::= sula (<ij> (<sula> |) | <scope>)
        if let Some(Token::KeywordElse) = tokens.front() {
            tokens.pop_front();
            if let Some(ij) = Ij::parse(tokens)? {
                if let Some(sula) = Sula::parse(tokens)? {
                    Ok(Some(Sula::Ij { ij, sula: Some(Box::new(sula)) }))
                } else {
                    Err(CustomError::UnexpectedToken("Expected a sula".to_string()))
                }
            } else if let Some(scope) = Scope::parse(tokens)? {
                Ok(Some(Sula::Scope(scope)))
            } else {
                Err(CustomError::UnexpectedToken("Expected an ij or a scope".to_string()))
            }
        } else {
            Ok(None)
        }
    }
}

// ----------
// --- Ij ---
// ----------

#[derive(PartialEq)]
pub struct Ij {
    exp: Exp,
    scope: Scope,
}

impl GraphDisplay for Ij {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Ij_{}[Ij]", id));
        *id += 1;
        self.exp.graph_display(graph, id);
        self.scope.graph_display(graph, id);
    }
}

impl_debug!(Ij);

impl Ij {
    pub fn new(exp: Exp, scope: Scope) -> Self {
        Self { exp, scope }
    }
    
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <ij> ::= ij <exp> <scope>
        if let Some(Token::KeywordIf) = tokens.front() {
            tokens.pop_front();
            match Exp::parse(tokens) {
                Some(Ok(exp)) => {
                    match Scope::parse(tokens)? {
                        Some(scope) => Ok(Some(Ij::new(exp, scope))),
                        None => Err(CustomError::UnexpectedToken("Expected a scope".to_string()))
                    }
                }
                Some(Err(err)) => Err(err),
                None => Err(CustomError::UnexpectedToken("Expected an expression".to_string()))
            }
        } else {
            Ok(None)
        }
    }
}

// ------------
// --- Cond ---
// ------------

#[derive(PartialEq)]
pub struct Cond {
    ij: Ij,
    sula: Option<Box<Sula>>,
}

impl GraphDisplay for Cond {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Cond_{}[Cond]", id));
        *id += 1;
        self.ij.graph_display(graph, id);
        if let Some(sula) = &self.sula {
            sula.graph_display(graph, id);
        }
    }
}

impl_debug!(Cond);

impl Cond {
    pub fn new(ij: Ij, sula: Option<Sula>) -> Self {
        Self { ij, sula: sula.map(Box::new) }
    }
    
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <cond> ::= <ij> (<sula> |)
        if let Some(ij) = Ij::parse(tokens)? {
            if let Some(sula) = Sula::parse(tokens)? {
                Ok(Some(Cond::new(ij, Some(sula))))
            } else {
                Ok(Some(Cond::new(ij, None)))
            }
        } else {
            Ok(None)
        }
    }
}
