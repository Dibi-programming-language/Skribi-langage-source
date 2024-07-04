use crate::impl_debug;
use crate::parse::nodes::blocs::Scope;
use crate::parse::nodes::id_nodes::{parse_tuple, TupleNode};
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, ResultOption};
use crate::tokens::Token;
use std::collections::VecDeque;

// Grammar of this file :
// <fct_dec> ::= ums T_IDENTIFIER <tuple> <scope>

// --------------
// --- FctDec ---
// --------------

/// `FctDec` represents a function declaration. It contains the identifier of the function, the
/// tuple of arguments and the scope of the function.
///
/// # Grammar
///
/// `<fct_dec> ::= ums T_IDENTIFIER <tuple> <scope>`
///
/// See also [TupleNode] and [Scope].
#[derive(PartialEq)]
pub struct FctDec {
    identifier: String,
    tuple: TupleNode,
    scope: Scope,
}

impl GraphDisplay for FctDec {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph FctDec_{}[FctDec {}]",
            id, self.identifier
        ));
        *id += 1;
        self.tuple.graph_display(graph, id);
        self.scope.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(FctDec);

impl FctDec {
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <fct_dec> ::= ums T_IDENTIFIER <tuple> <scope>
        if let Some(Token::KeywordFunction) = tokens.front() {
            tokens.pop_front();
            if let Some(Token::Identifier(identifier)) = tokens.pop_front() {
                match parse_tuple(tokens) {
                    Some(Ok(tuple)) => match Scope::parse(tokens)? {
                        Some(scope) => Ok(Some(FctDec {
                            identifier,
                            tuple,
                            scope,
                        })),
                        None => Err(CustomError::UnexpectedToken("Expected a scope".to_string())),
                    },
                    Some(Err(err)) => Err(err),
                    None => Err(CustomError::UnexpectedToken("Expected a tuple".to_string())),
                }
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected an identifier".to_string(),
                ))
            }
        } else {
            Err(CustomError::UnexpectedToken(
                "Expected keyword 'ums'".to_string(),
            ))
        }
    }
}
