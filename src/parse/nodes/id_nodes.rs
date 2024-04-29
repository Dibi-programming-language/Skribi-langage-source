use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::impl_debug;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::CustomError;
use crate::tokens::Token;

#[derive(PartialEq)]
pub struct TupleNode {
    // TODO: définir les champs du tuple ici
}

fn parse_tuple(tokens: &mut VecDeque<Token>) -> Option<Result<TupleNode, CustomError>> {
    // TODO: implémenter cette fonction
    None
}

impl GraphDisplay for TupleNode {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_TupleNode_{}[TupleNode]", id));
        *id += 1;
    }
}

impl_debug!(TupleNode);

// ------------
// --- CGet ---
// ------------

#[derive(PartialEq)]
pub struct CGet {
    pub(crate) name: String,
}

impl GraphDisplay for CGet {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_CGet_{}[CGet {}]\nend", id, self.name));
        *id += 1;
    }
}

impl_debug!(CGet);

fn is_type_def(identifier: &str) -> bool {
    // TODO: implémenter cette fonction avec des types complexes
    match identifier {
        "int" => true,
        "dar" => true,
        "ioi" => true,
        "skr" => true,
        _ => false,
    }
}

pub(crate) fn parse_cget(tokens: &mut VecDeque<Token>) -> Option<CGet> {
    if let Some(Token::Identifier(identifier)) = tokens.front() {
        if is_type_def(identifier) {
            if let Some(Token::Identifier(identifier)) = tokens.pop_front() {
                return Some(CGet { name: identifier });
            }
        }
    }

    None
}

// -------------
// --- IdGet ---
// -------------

#[derive(PartialEq)]
pub struct IdGet {
    pub identifier: String,
    pub tuple: Option<TupleNode>,
    pub op_in: Box<OpIn>,
}

impl GraphDisplay for IdGet {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph_IdGet_{}[IdGet {}]",
            id, self.identifier
        ));
        *id += 1;
        if let Some(tuple) = &self.tuple {
            tuple.graph_display(graph, id);
        }
        self.op_in.graph_display(graph, id);
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(IdGet);

pub(crate) fn parse_id_get(tokens: &mut VecDeque<Token>) -> Option<Result<IdGet, CustomError>> {
    // <id_get> ::= T_IDENTIFIER (<tuple> |) <op_in>
    if let Some(Token::Identifier(_)) = tokens.front() {
        if let Some(Token::Identifier(identifier)) = tokens.pop_front() {
            let tuple_parsed = parse_tuple(tokens);
            let tuple = match tuple_parsed {
                Some(Ok(tuple)) => Some(tuple),
                Some(Err(err)) => return Some(Err(err)),
                None => None,
            };
            let op_in = parse_op_in(tokens);
            match op_in {
                Ok(op_in) => Some(Ok(IdGet {
                    identifier,
                    tuple,
                    op_in: Box::new(op_in),
                })),
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    } else {
        None
    }
}

// ------------
// --- OpIn ---
// ------------

#[derive(PartialEq)]
pub enum OpIn {
    IdGet(IdGet),
    CGet(CGet),
    Empty,
}

impl GraphDisplay for OpIn {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_OpIn_{}[OpIn]", id));
        *id += 1;
        match self {
            OpIn::IdGet(id_get) => id_get.graph_display(graph, id),
            OpIn::CGet(c_get) => c_get.graph_display(graph, id),
            OpIn::Empty => {}
        }
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(OpIn);

pub(crate) fn parse_op_in(tokens: &mut VecDeque<Token>) -> Result<OpIn, CustomError> {
    // <op_in> ::= (T_IN (<id_get> | <cget>) |)
    return if let Some(Token::Inside) = tokens.front() {
        tokens.pop_front();
        if let Some(c_get) = parse_cget(tokens) {
            Ok(OpIn::CGet(c_get))
        } else if let Some(id_get) = parse_id_get(tokens) {
            Ok(OpIn::IdGet(id_get?))
        } else  {
            Err(CustomError::UnexpectedToken(
                "Expected id_get or cget after \"indide\" token".to_string(),
            ))
        }
    } else {
        Ok(OpIn::Empty)
    };
}

// -------------
// --- IdSet ---
// -------------

#[derive(PartialEq)]
pub struct IdSet {
    pub identifier: String,
    pub op_in: Box<OpIn>,
}

impl GraphDisplay for IdSet {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph_IdSet_{}[IdSet {}]",
            id, self.identifier
        ));
        *id += 1;
        self.op_in.graph_display(graph, id);
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(IdSet);

pub(crate) fn parse_id_set(tokens: &mut VecDeque<Token>) -> Option<Result<IdSet, CustomError>> {
    // <id_set> ::= T_IDENTIFIER <op_in>
    if let Some(Token::Identifier(_)) = tokens.front() {
        if let Some(Token::Identifier(identifier)) = tokens.pop_front() {
            let op_in = parse_op_in(tokens);
            match op_in {
                Ok(op_in) => Some(Ok(IdSet {
                    identifier,
                    op_in: Box::new(op_in),
                })),
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    } else {
        None
    }
}
