use crate::execute::unit::InternalUnit;
use crate::execute::{Evaluate, ExecutionError, OperationContext, OperationO};
use crate::parse::nodes::blocs::Scope;
use crate::parse::nodes::expressions::Exp;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{ParsingError, ResultOption};
use crate::tokens::{Token, TokenContainer};
use crate::{impl_debug, some_token};
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!("\nsubgraph Sula_{}[Sula]", id));
        *id += 1;
        match self {
            Sula::Ij { ij, sula } => {
                ij.graph_display(graph, id, indent + 2);
                if let Some(sula) = sula {
                    sula.graph_display(graph, id, indent + 2);
                }
            }
            Sula::Scope(scope) => {
                scope.graph_display(graph, id, indent + 2);
            }
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
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

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <sula> ::= sula (<ij> (<sula> |) | <scope>)
        if let some_token!(Token::KeywordElse) = tokens.front() {
            tokens.pop_front();
            if let Some(ij) = Ij::parse(tokens)? {
                if let Some(sula) = Sula::parse(tokens)? {
                    Ok(Some(Sula::Ij {
                        ij,
                        sula: Some(Box::new(sula)),
                    }))
                } else {
                    Err(ParsingError::UnexpectedToken("Expected a sula".to_string()))
                }
            } else if let Some(scope) = Scope::parse(tokens)? {
                Ok(Some(Sula::Scope(scope)))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Expected an ij or a scope".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

impl Evaluate for Sula {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            Self::Ij { ij, sula } => {
                if ij.can_execute(operation_context)? {
                    ij.evaluate(operation_context)
                } else if let Some(sula) = sula {
                    sula.evaluate(operation_context)
                } else {
                    Ok(InternalUnit::new_boxed())
                }
            }
            Self::Scope(scope) => scope.evaluate(operation_context),
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!("\nsubgraph Ij_{}[Ij]", id));
        *id += 1;
        self.exp.graph_display(graph, id, indent + 2);
        self.scope.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(Ij);

impl Ij {
    pub fn new(exp: Exp, scope: Scope) -> Self {
        Self { exp, scope }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <ij> ::= ij <exp> <scope>
        if let some_token!(Token::KeywordIf) = tokens.front() {
            let container = tokens.pop_front().expect("Container Some");
            match Exp::parse(tokens)? {
                Some(exp) => match Scope::parse(tokens)? {
                    Some(scope) => Ok(Some(Ij::new(exp, scope))),
                    None => Err(ParsingError::element_expected(container, tokens, "scope")),
                },
                None => Err(ParsingError::element_expected(
                    container,
                    tokens,
                    "expression",
                )),
            }
        } else {
            Ok(None)
        }
    }

    pub fn can_execute(
        &self,
        operation_context: &mut OperationContext,
    ) -> Result<bool, ExecutionError> {
        self.exp
            .evaluate(operation_context)?
            .as_ioi(operation_context)
    }
}

impl Evaluate for Ij {
    /// For optimisation purposes, this function is not testing the condition.
    /// The function [Ij::can_execute] should be called before.
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        self.scope.evaluate(operation_context)
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "{:indent$}\nsubgraph Cond_{}[Cond]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        self.ij.graph_display(graph, id, indent + 2);
        if let Some(sula) = &self.sula {
            sula.graph_display(graph, id, indent + 2);
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
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

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
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

impl Evaluate for Cond {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        if self.ij.can_execute(operation_context)? {
            self.ij.evaluate(operation_context)
        } else if let Some(sula) = &self.sula {
            sula.evaluate(operation_context)
        } else {
            Ok(InternalUnit::new_boxed())
        }
    }
}
