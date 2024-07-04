use crate::impl_debug;
use crate::parse::nodes::blocs::Scope;
use crate::parse::nodes::expressions::Exp;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, ResultOption};
use crate::tokens::Token;
use std::collections::VecDeque;
// Grammar for this file:
// <sula> ::= sula (<ij> (<sula> |) | <scope>)
// <ij> ::= ij <exp> <scope>
// <cond> ::= <ij> (<sula> |)

// ------------
// --- Sula ---
// ------------

/// `Sula` represents an "else" block in the AST. It can contain an [Ij] if this is an "else if"
/// block, or a [Scope] if this is an "else" block. It there is an "else if" block, it can contain
/// another [Sula].
///
/// The principe is to have a [Cond] node that contains an [Ij] and an optional [Sula]. The [Sula]
/// chains the "else if" blocks and the "else" block, while [Ij] contains the condition and the
/// block to execute if the condition is true.
///
/// The [Sula] node is recursive, as it can contain another [Sula] node.
///
/// # Grammar
///
/// `<sula> ::= sula (<ij> (<sula> |) | <scope>)`
///
/// See also [Ij] and [Scope].
#[derive(PartialEq)]
pub enum Sula {
    Ij { ij: Ij, sula: Option<Box<Sula>> },
    Scope(Scope),
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
        graph.push_str("\nend");
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
                    Ok(Some(Sula::Ij {
                        ij,
                        sula: Some(Box::new(sula)),
                    }))
                } else {
                    Err(CustomError::UnexpectedToken("Expected a sula".to_string()))
                }
            } else if let Some(scope) = Scope::parse(tokens)? {
                Ok(Some(Sula::Scope(scope)))
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected an ij or a scope".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

// ----------
// --- Ij ---
// ----------

/// `Ij` represents an "if" block in the AST. It contains an [Exp] and a [Scope]. It can be followed
/// by a [Sula] IN ITS PARENT NODE, to represent an "else if" block or an "else" block. The [Ij]
/// node itself is not recursive (well ... unless there is another "if" block in the [Scope]).
///
/// # Grammar
///
/// `<ij> ::= ij <exp> <scope>`
///
/// See also [Exp] and [Scope].
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
        graph.push_str("\nend");
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
            match Exp::parse(tokens)? {
                Some(exp) => match Scope::parse(tokens)? {
                    Some(scope) => Ok(Some(Ij::new(exp, scope))),
                    None => Err(CustomError::UnexpectedToken("Expected a scope".to_string())),
                },
                None => Err(CustomError::UnexpectedToken(
                    "Expected an expression".to_string(),
                )),
            }
        } else {
            Ok(None)
        }
    }
}

// ------------
// --- Cond ---
// ------------

/// `Cond` is the starting node for an "if" block in the AST. It contains an [Ij] and an optional
/// [Sula]. The [Sula] is used to chain "else if" blocks and "else" blocks. [Cond] allows to use
/// any chain of "if", "else if" and "else" blocks in the same way.
///
/// # Grammar
///
/// `<cond> ::= <ij> (<sula> |)`
///
/// See also [Ij] and [Sula].
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
        graph.push_str("\nend");
    }
}

impl_debug!(Cond);

impl Cond {
    pub fn new(ij: Ij, sula: Option<Sula>) -> Self {
        Self {
            ij,
            sula: sula.map(Box::new),
        }
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
