use crate::execute::unit::InternalUnit;
use crate::execute::{Evaluate, ExecutionError, OperationContext, OperationO};
use crate::parse::nodes::blocs::Scope;
use crate::parse::nodes::expressions::Exp;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, ResultOption};
use crate::tokens::{Token, TokenContainer};
use crate::{impl_debug, some_token};
use std::collections::VecDeque;

// ------------
// --- Ci ---
// ------------

/// `Ci` represents an "while" block in the AST.
/// It contains an [Exp] and a [Scope].
///
/// # Grammar
///
/// `<ci> ::= ci <exp> <scope>`
///
/// See also [Exp] and [Scope].
#[derive(PartialEq)]
pub struct Ci {
    exp: Exp,
    scope: Scope,
}

impl GraphDisplay for Ci {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!("\nsubgraph Ci_{}[Ij]", id));
        *id += 1;
        self.exp.graph_display(graph, id, indent + 2);
        self.scope.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(Ci);

impl Ci {
    pub fn new(exp: Exp, scope: Scope) -> Self {
        Self { exp, scope }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        if let some_token!(Token::KeywordWhile) = tokens.front() {
            let container = tokens.pop_front().expect("Container Some");
            match Exp::parse(tokens)? {
                Some(exp) => match Scope::parse(tokens)? {
                    Some(scope) => Ok(Some(Ci::new(exp, scope))),
                    None => {
                        println!("{}", exp.graph());
                        Err(CustomError::element_expected(container, tokens, "scope"))
                    }
                },
                None => Err(CustomError::element_expected(
                    container,
                    tokens,
                    "expression",
                )),
            }
        } else {
            Ok(None)
        }
    }

    fn can_execute(
        &self,
        operation_context: &mut OperationContext,
    ) -> Result<bool, ExecutionError> {
        self.exp
            .evaluate(operation_context)?
            .as_ioi(operation_context)
    }
}

impl Evaluate for Ci {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        while self.can_execute(operation_context)? {
            self.scope.evaluate(operation_context)?;
        }
        Ok(InternalUnit::new_boxed())
    }
}
