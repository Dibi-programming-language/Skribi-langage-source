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
    exp: Box<Exp>,
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
    fn new(type_: Type, identifier: String, exp: Exp) -> Self {
        Self {
            type_,
            identifier,
            exp: Box::new(exp),
        }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <vd> ::= <type> T_IDENTIFIER <exp>
        let type_ = match parse_type(tokens) {
            Some(type_) => type_,
            None => return Ok(None),
        };

        if let Some(Token::Identifier(identifier)) = tokens.pop_front() {
            if let Some(exp0) = Exp::parse(tokens) {
                Ok(Some(Vd::new(type_, identifier, exp0?)))
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected an expression".to_string(),
                ))
            }
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
        Self { vd }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <global_var> ::= fu <vd>
        if let Some(Token::KeywordModifier(ModifierKeyword::Global)) = tokens.front() {
            tokens.pop_front();
            match Vd::parse(tokens)? {
                Some(vd) => Ok(Some(GlobalVar::new(vd))),
                None => Err(CustomError::UnexpectedToken(
                    "Expected a variable declaration".to_string(),
                )),
            }
        } else {
            Ok(None)
        }
    }
}

impl PrivateVar {
    fn new(vd: Vd) -> Self {
        Self { vd }
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

/// `ConstVar` represents a constant variable declaration in the AST. It adds the property of being
/// constant to a variable declaration that can already be a global variable, a private variable or
/// a simple variable declaration. This is a kind of wrapper around a variable declaration.
///
/// # Grammar
///
/// `<const_var> ::= ju (<private_var> | <global_var> | <vd>)`
///
/// See [PrivateVar], [GlobalVar], [Vd]
#[derive(PartialEq)]
pub enum ConstVar {
    PrivateVar(PrivateVar),
    GlobalVar(GlobalVar),
    Vd(Vd),
}

impl GraphDisplay for ConstVar {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        match self {
            ConstVar::PrivateVar(private_var) => private_var.graph_display(graph, id),
            ConstVar::GlobalVar(global_var) => global_var.graph_display(graph, id),
            ConstVar::Vd(vd) => vd.graph_display(graph, id),
        }
    }
}

impl_debug!(ConstVar);

impl ConstVar {
    fn new(vd: Vd) -> Self {
        ConstVar::Vd(vd)
    }

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <const_var> ::= ju (<private_var> | <global_var> | <vd>)
        if let Some(Token::KeywordModifier(ModifierKeyword::Constant)) = tokens.front() {}
    }
}

/// `VarDec` represents any kind of variable declaration in the AST. It can be a constant variable,
/// a private variable, a global variable or a simple variable declaration. This is the root node of
/// a variable declaration.
///
/// # Grammar
///
/// `<var_dec> ::= <const_var> | <private_var> | <global_var> | <vd>`
///
/// See [ConstVar], [PrivateVar], [GlobalVar], [Vd]
#[derive(PartialEq)]
pub enum VarDec {
    ConstVar(ConstVar),
    PrivateVar(PrivateVar),
    GlobalVar(GlobalVar),
    Vd(Vd),
}

/// `VarMod` represents the left part of a variable modification in the AST. It only contains an
/// expression, so this is a simple node.
///
/// Keep in mind that a variable modification follows the syntax `<name> <exp>`. The `<name>` part
/// is not represented in this node : the LL1 grammar will take care of it -> the `<name>` part is
/// and identifier and is already detected by the parser when the modification is read.
///
/// # Grammar
///
/// `<var_mod> ::= <exp>`
///
/// See [Exp]
#[derive(PartialEq)]
pub struct VarMod {
    exp: Exp,
}
