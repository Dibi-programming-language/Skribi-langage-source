use std::collections::VecDeque;

use crate::parse::nodes::blocs::Scope;
use crate::parse::nodes::id_nodes::TupleNode;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{ParsingError, ResultOption};
use crate::tokens::{Token, TokenContainer};
use crate::{impl_debug, some_token};

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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "{:indent$}\nsubgraph FctDec_{}[FctDec {}]",
            "",
            id,
            self.identifier,
            indent = indent
        ));
        *id += 1;
        self.tuple.graph_display(graph, id, indent + 2);
        self.scope.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(FctDec);

impl FctDec {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <fct_dec> ::= ums T_IDENTIFIER <tuple> <scope>
        if let some_token!(Token::KeywordFunction) = tokens.front() {
            tokens.pop_front();
            if let some_token!(Token::Identifier(identifier)) = tokens.pop_front() {
                match TupleNode::parse(tokens)? {
                    Some(tuple) => match Scope::parse(tokens)? {
                        Some(scope) => Ok(Some(FctDec {
                            identifier,
                            tuple,
                            scope,
                        })),
                        None => Err(ParsingError::UnexpectedToken("Expected a scope".to_string())),
                    },
                    None => Err(ParsingError::UnexpectedToken("Expected a tuple".to_string())),
                }
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Expected an identifier".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}
