use std::collections::VecDeque;
use std::fmt;

use crate::parse::nodes::classes::is_type_def;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, OptionResult};
use crate::tokens::Token;
use crate::{impl_debug, skr_errors};

/// `TupleNode` represents a tuple in the AST.
///
/// The grammar of a tuple is not yet defined, so this class is not implemented yet.
///
/// # Use cases
///
/// Tuples will be a datatype, and they will mainly be used to store fonction arguments.
#[derive(PartialEq)]
pub struct TupleNode {
    // TODO: définir les champs du tuple ici
}

fn parse_tuple(_tokens: &mut VecDeque<Token>) -> OptionResult<TupleNode> {
    // TODO: implémenter cette fonction
    None
}

impl GraphDisplay for TupleNode {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph TupleNode_{}[TupleNode]", id));
        *id += 1;
    }
}

impl_debug!(TupleNode);

// ------------
// --- CGet ---
// ------------

/// `CGet` represents the smallest piece of an identifier in the AST. It directly references a type
/// or a variable that can be used directly in this scope.
///
/// # Examples
///
/// ## What is a CGet node?
///
/// Let us consider the following pseudocode (this is not Skribi code)
///
/// > `Class of name A with (` <br/>
/// > `-  static field of type int and name B` <br/>
/// > `-  static field of type int and name C` <br/>
/// > `-  field of type int and name B0` <br/>
/// > `)`
/// >
/// > `Variable of type A and name D`
/// > `Variable of type int and name E`
///
/// A, E and D can be accessed with a `CGet` node while B, B0 of D and C cannot. See the [IdGet]
/// for further information.
#[derive(PartialEq)]
pub struct CGet {
    pub(crate) name: String,
}

impl GraphDisplay for CGet {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph CGet_{}[CGet {}]\nend", id, self.name));
        *id += 1;
    }
}

impl_debug!(CGet);

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

/// `IdGet` represents a piece of an identifier in the AST. It is specialized in getting values that
/// cannot be set. It can be a simple identifier followed by an [OpIn] node or a function call with
/// arguments using a tuple.
///
/// # Use cases
///
/// - Anywhere in a chain that get a value
/// - Anywhere, except at the start in a chain that set a value
/// - Any function call must be an IdGet node
/// - Any element of an identifier chain that is not the first one or the last one must be an IdGet
///
/// # Examples
///
/// ## What is an IdGet node?
///
/// Let us consider the following pseudocode (this is not Skribi code)
///
/// > `Class of name T with (` <br/>
/// > `-  static field of type int and name T0` <br/>
/// > `-  field of type int and name T1` <br/>
/// > `)`
/// >
/// > `Class of name A with (` <br/>
/// > `-  static field of type int and name B` <br/>
/// > `-  static field of type int and name C` <br/>
/// > `-  field of type int and name B0` <br/>
/// > `-  field of type T and name C0` <br/>
/// > `-  function of return type T and name F with no arguments` <br/>
/// > `)`
/// >
/// > `Variable of type A and name D`
/// > `Variable of type int and name E`
///
/// In a Skribi code we can do :
/// - `T0:T` to get the static field T0 of the class T, T0 can be an IdGet node and also T. But in
/// reality, T will be represented as an IdSet if we can set it, (inside an OpIn node), inside a
/// CGet node. This will not be detailed in latter examples.
/// - `B0:D`, get the field B0 of the variable D
/// - `C0:D`, get the field C0 of the variable D
/// - `T1:F():D`, get the field T1 of the result of the function F with no arguments. Here, F() must
/// be an IdGet node, this is the only solution.
/// - `T1:C0:D`, get the field T1 of the field C0 of the variable D. Here, C0 must also be an IdGet.
#[derive(PartialEq)]
pub struct IdGet {
    pub identifier: String,
    pub tuple: Option<TupleNode>,
    pub op_in: Box<OpIn>,
}

impl GraphDisplay for IdGet {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph IdGet_{}[IdGet {}]",
            id, self.identifier
        ));
        *id += 1;
        if let Some(tuple) = &self.tuple {
            tuple.graph_display(graph, id);
        }
        self.op_in.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(IdGet);

pub(crate) fn parse_id_get(tokens: &mut VecDeque<Token>) -> OptionResult<IdGet> {
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

/// `OpIn` is used by nodes that represent a part of an identifier. It contains the next part of the
/// chain of the identifier. It can be an [IdGet] node or a [CGet] node. The `OpIn` can also be
/// empty if this is the last part of the identifier.
///
/// It will first try to parse the [CGet] node, if it fails, it will try to parse the [IdGet] node.
/// If both fail, it will return an empty `OpIn`. Here, "fail" means that there is no parsing error,
/// but that the token is not the one expected for an identifier.
#[derive(PartialEq)]
pub enum OpIn {
    IdGet(IdGet),
    CGet(CGet),
    Empty,
}

impl GraphDisplay for OpIn {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph OpIn_{}[OpIn]", id));
        *id += 1;
        match self {
            OpIn::IdGet(id_get) => id_get.graph_display(graph, id),
            OpIn::CGet(c_get) => c_get.graph_display(graph, id),
            OpIn::Empty => {}
        }
        graph.push_str("\nend");
    }
}

impl_debug!(OpIn);

pub(crate) fn parse_op_in(tokens: &mut VecDeque<Token>) -> skr_errors::Result<OpIn> {
    // <op_in> ::= (T_IN (<id_get> | <cget>) |)
    return if let Some(Token::Inside) = tokens.front() {
        tokens.pop_front();
        if let Some(c_get) = parse_cget(tokens) {
            Ok(OpIn::CGet(c_get))
        } else if let Some(id_get) = parse_id_get(tokens) {
            Ok(OpIn::IdGet(id_get?))
        } else {
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

/// `IdSet` represents a piece of an identifier in the AST. It is specialized in setting and getting
/// values. It can only be the first part of an identifier chain. It works exactly like an [IdGet],
/// but excludes fonctions calls.
#[derive(PartialEq)]
pub struct IdSet {
    pub identifier: String,
    pub op_in: Box<OpIn>,
}

impl GraphDisplay for IdSet {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph IdSet_{}[IdSet {}]",
            id, self.identifier
        ));
        *id += 1;
        self.op_in.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(IdSet);

pub(crate) fn parse_id_set(tokens: &mut VecDeque<Token>) -> OptionResult<IdSet> {
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
