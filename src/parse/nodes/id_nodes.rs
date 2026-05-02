use std::collections::VecDeque;

use crate::parse::nodes::classes::is_type_def;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{ParsingError, ResultOption};
use crate::tokens::{Token, TokenContainer};
use crate::{impl_debug, skr_errors, some_token};

// Grammar of this file :
// <cget> ::= T_TYPE_DEF
// <op_in> ::= (T_IN (<cget> | <id_get>) |)
// <id_get> ::= T_IDENTIFIER (<tuple> |) <op_in>

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

impl GraphDisplay for TupleNode {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph TupleNode_{}[TupleNode]\nend",
            "",
            id,
            indent = indent
        ));
        *id += 1;
    }
}

impl_debug!(TupleNode);

impl TupleNode {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn parse(_tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // TODO: implémenter cette fonction
        Ok(None)
    }
}

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
/// A, E and D can be accessed with a `CGet` node while B, B0 of D and C cannot.
/// See the [IdGet] for further information.
#[derive(PartialEq)]
pub struct CGet {
    pub(crate) name: String,
}

impl GraphDisplay for CGet {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph CGet_{}[CGet {}]\nend",
            "",
            id,
            self.name,
            indent = indent
        ));
        *id += 1;
    }
}

impl_debug!(CGet);

pub(crate) fn parse_cget(tokens: &mut VecDeque<TokenContainer>) -> Option<CGet> {
    if let some_token!(Token::Identifier(identifier)) = tokens.front() {
        if is_type_def(identifier) {
            if let some_token!(Token::Identifier(identifier)) = tokens.pop_front() {
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
///   reality, T will be represented as an IdSet if we can set it, (inside an OpIn node), inside a
///   CGet node. This will not be detailed in latter examples.
/// - `B0:D`, get the field B0 of the variable D
/// - `C0:D`, get the field C0 of the variable D
/// - `T1:F():D`, get the field T1 of the result of the function F with no arguments. Here, F() must
///   be an IdGet node, this is the only solution.
/// - `T1:C0:D`, get the field T1 of the field C0 of the variable D. Here, C0 must also be an IdGet.
#[derive(PartialEq)]
pub struct IdGet {
    pub identifier: String,
    pub tuple: Option<TupleNode>,
    pub op_in: Box<OpIn>,
}

impl GraphDisplay for IdGet {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph IdGet_{}[IdGet {}]",
            "",
            id,
            self.identifier,
            indent = indent
        ));
        *id += 1;
        if let Some(tuple) = &self.tuple {
            tuple.graph_display(graph, id, indent + 2);
        }
        self.op_in.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(IdGet);

impl IdGet {
    pub(crate) fn new(identifier: String, tuple: Option<TupleNode>, op_in: OpIn) -> Self {
        Self {
            identifier,
            tuple,
            op_in: Box::new(op_in),
        }
    }

    pub(crate) fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <id_get> ::= T_IDENTIFIER (<tuple> |) <op_in>
        if let some_token!(Token::Identifier(_)) = tokens.front() {
            if let some_token!(Token::Identifier(identifier)) = tokens.pop_front() {
                let tuple_parsed = TupleNode::parse(tokens)?;
                let tuple = tuple_parsed;
                let op_in = parse_op_in(tokens)?;
                Ok(Some(IdGet {
                    identifier,
                    tuple,
                    op_in: Box::new(op_in),
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph OpIn_{}[OpIn]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        match self {
            OpIn::IdGet(id_get) => id_get.graph_display(graph, id, indent + 2),
            OpIn::CGet(c_get) => c_get.graph_display(graph, id, indent + 2),
            OpIn::Empty => {}
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(OpIn);

pub(crate) fn parse_op_in(tokens: &mut VecDeque<TokenContainer>) -> skr_errors::ShortResult<OpIn> {
    // <op_in> ::= (T_IN (<id_get> | <cget>) |)
    if let some_token!(Token::Inside) = tokens.front() {
        tokens.pop_front();
        if let Some(c_get) = parse_cget(tokens) {
            Ok(OpIn::CGet(c_get))
        } else if let Some(id_get) = IdGet::parse(tokens)? {
            Ok(OpIn::IdGet(id_get))
        } else {
            Err(ParsingError::UnexpectedToken(
                "Expected id_get or cget after \"indide\" token".to_string(),
            ))
        }
    } else {
        Ok(OpIn::Empty)
    }
}
