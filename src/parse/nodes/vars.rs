use crate::impl_debug;
use crate::parse::nodes::classes::is_type_def;
use crate::parse::nodes::expressions::Exp;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, ResultOption};
use crate::tokens::{ModifierKeyword, Token};
use std::collections::VecDeque;
use std::fmt;

// Grammar of this file :
/*
<type> ::= T_TYPE_DEF
<vd> ::= <type> T_IDENTIFIER <exp>
<global_var> ::= fu <vd>
<private_var> ::= pu <vd>
<const_var> ::= ju (<private_var> | <global_var> | <vd>)
<var_dec> ::= <const_var> | <private_var> | <global_var> | <vd>

<var_mod> ::= <exp>
 */

// ------------
// --- Type ---
// ------------

/// `Type` represents a defined type in the AST. This node detect any identifier and ask the class
/// manager if this is a type or not.
#[derive(PartialEq)]
pub struct Type {
    pub(crate) name: String,
}

impl GraphDisplay for Type {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph CGet_{}[CGet {}]\nend", id, self.name));
        *id += 1;
    }
}

impl_debug!(Type);

pub(crate) fn parse_type(tokens: &mut VecDeque<Token>) -> Option<Type> {
    if let Some(Token::Identifier(identifier)) = tokens.front() {
        if is_type_def(identifier) {
            if let Some(Token::Identifier(identifier)) = tokens.pop_front() {
                return Some(Type { name: identifier });
            }
        }
    }

    None
}

// ----------
// --- Vd ---
// ----------

/// `Vd` represents a variable declaration in the AST. It contains a type, an identifier and an
/// expression. The expression is not yet implemented.
#[derive(PartialEq)]
pub struct Vd {
    type_: Type,
    identifier: String,
    // exp: Box<Exp>, // TODO - Implement Exp
}

impl GraphDisplay for Vd {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Vd_{}[Vd {}]", id, self.identifier));
        *id += 1;
        self.type_.graph_display(graph, id);
        graph.push_str("\nend")
    }
}

impl_debug!(Vd);

impl Vd {
    fn new(type_: Type, identifier: String) -> Self {
        Vd { type_, identifier }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <vd> ::= <type> T_IDENTIFIER <exp> // TODO - Implement <exp>
        let type_ = match parse_type(tokens) {
            Some(type_) => type_,
            None => return Ok(None),
        };

        if let Some(Token::Identifier(identifier)) = tokens.pop_front() {
            Ok(Some(Vd::new(type_, identifier)))
        } else {
            Err(CustomError::UnexpectedToken(
                "Expected an identifier".to_string(),
            ))
        }
    }
}

// --------------------------------
// --- GlobalVar and PrivateVar ---
// --------------------------------

/// `GlobalVar` represents a global variable declaration in the AST. It contains a variable
/// declaration.
#[derive(PartialEq)]
pub struct GlobalVar {
    vd: Vd,
}

/// `PrivateVar` represents a private variable declaration in the AST. It contains a variable
/// declaration.
#[derive(PartialEq)]
pub struct PrivateVar {
    vd: Vd,
}

impl GraphDisplay for GlobalVar {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph GlobalVar_{}[GlobalVar]", id));
        *id += 1;
        self.vd.graph_display(graph, id);
        graph.push_str("\nend")
    }
}

impl_debug!(GlobalVar);

impl GraphDisplay for PrivateVar {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph PrivateVar_{}[PrivateVar]", id));
        *id += 1;
        self.vd.graph_display(graph, id);
        graph.push_str("\nend")
    }
}

impl_debug!(PrivateVar);

impl GlobalVar {
    fn new(vd: Vd) -> Self {
        GlobalVar { vd }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <global_var> ::= fu <vd>
        if let Some(Token::KeywordModifier(ModifierKeyword::Global)) = tokens.pop_front() {
            match Vd::parse(tokens) {
                Ok(Some(vd)) => Ok(Some(GlobalVar::new(vd))),
                Ok(None) => Err(CustomError::UnexpectedToken(
                    "Expected a variable declaration".to_string(),
                )),
                Err(err) => Err(err),
            }
        } else {
            Ok(None)
        }
    }
}

impl PrivateVar {
    fn new(vd: Vd) -> Self {
        PrivateVar { vd }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <private_var> ::= pu <vd>
        if let Some(Token::KeywordModifier(ModifierKeyword::Private)) = tokens.pop_front() {
            match Vd::parse(tokens) {
                Ok(Some(vd)) => Ok(Some(PrivateVar::new(vd))),
                Ok(None) => Err(CustomError::UnexpectedToken(
                    "Expected a variable declaration".to_string(),
                )),
                Err(err) => Err(err),
            }
        } else {
            Ok(None)
        }
    }
}

// ----------------
// --- ConstVar ---
// ----------------

// TODO

/// NOT IMPLEMENTED YET
pub struct ConstVar {
    private_var: Option<PrivateVar>,
    global_var: Option<GlobalVar>,
    vd: Option<Vd>,
}

/// NOT IMPLEMENTED YET
pub enum VarDec {
    ConstVar(ConstVar),
    PrivateVar(PrivateVar),
    GlobalVar(GlobalVar),
    Vd(Vd),
}

/// NOT IMPLEMENTED YET
pub struct VarMod {
    exp: Exp,
}
