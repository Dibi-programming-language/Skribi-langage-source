use std::collections::VecDeque;

use crate::execute::Evaluate;
use crate::execute::{OperationContext, OperationO};
use crate::parse::nodes::classes::is_type_def;
use crate::parse::nodes::expressions::Exp;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, ResultOption};
use crate::tokens::{ModifierKeyword, Token, TokenContainer};
use crate::{impl_debug, some_token};

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

impl Type {
    pub(crate) fn parse(tokens: &mut VecDeque<TokenContainer>) -> Option<Type> {
        if let some_token!(Token::Identifier(identifier)) = tokens.front() {
            if is_type_def(identifier) {
                if let some_token!(Token::Identifier(identifier)) = tokens.pop_front() {
                    return Some(Type { name: identifier });
                }
            }
        }

        None
    }
}

impl GraphDisplay for Type {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph CGet_{}[CGet {}]\n{:indent$}end",
            "",
            id,
            self.name,
            "",
            indent = indent
        ));
        *id += 1;
    }
}

impl_debug!(Type);

// ----------
// --- Vd ---
// ----------

/// `Vd` represents a variable declaration in the AST.
/// It contains a type, an identifier and an expression.
/// The expression is not yet implemented.
#[derive(PartialEq)]
pub struct Vd {
    type_: Type,
    identifier: String,
    exp: Box<Exp>,
}

impl GraphDisplay for Vd {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph Vd_{}[Vd {}]",
            "",
            id,
            self.identifier,
            indent = indent
        ));
        *id += 1;
        self.type_.graph_display(graph, id, indent + 2);
        self.exp.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent))
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

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <vd> ::= <type> T_IDENTIFIER <exp>
        let type_ = match Type::parse(tokens) {
            Some(type_) => type_,
            None => return Ok(None),
        };

        if let some_token!(Token::Identifier(identifier)) = tokens.pop_front() {
            if let Some(exp0) = Exp::parse(tokens)? {
                Ok(Some(Vd::new(type_, identifier, exp0)))
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

impl Evaluate for Vd {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        let content = self.exp.evaluate(operation_context)?;
        operation_context.associate_new(self.identifier.clone(), content);
        operation_context.get_variable(&self.identifier, 0)
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph GlobalVar_{}[GlobalVar]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        self.vd.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent))
    }
}

impl_debug!(GlobalVar);

impl GraphDisplay for PrivateVar {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph PrivateVar_{}[PrivateVar]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        self.vd.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent))
    }
}

impl_debug!(PrivateVar);

impl GlobalVar {
    fn new(vd: Vd) -> Self {
        Self { vd }
    }

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <global_var> ::= fu <vd>
        if let some_token!(Token::KeywordModifier(ModifierKeyword::Global)) = tokens.front() {
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

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <private_var> ::= pu <vd>
        if let some_token!(Token::KeywordModifier(ModifierKeyword::Private)) = tokens.front() {
            tokens.pop_front();
            match Vd::parse(tokens)? {
                Some(vd) => Ok(Some(PrivateVar::new(vd))),
                None => Err(CustomError::UnexpectedToken(
                    "Expected a variable declaration".to_string(),
                )),
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph ConstVar_{}[ConstVar]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        match self {
            ConstVar::PrivateVar(private_var) => private_var.graph_display(graph, id, indent + 2),
            ConstVar::GlobalVar(global_var) => global_var.graph_display(graph, id, indent + 2),
            ConstVar::Vd(vd) => vd.graph_display(graph, id, indent + 2),
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent))
    }
}

impl_debug!(ConstVar);

impl ConstVar {
    fn new(vd: Vd) -> Self {
        ConstVar::Vd(vd)
    }

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <const_var> ::= ju (<private_var> | <global_var> | <vd>)
        if let some_token!(Token::KeywordModifier(ModifierKeyword::Constant)) = tokens.front() {
            tokens.pop_front();
            if let Some(private_var) = PrivateVar::parse(tokens)? {
                Ok(Some(ConstVar::PrivateVar(private_var)))
            } else if let Some(global_var) = GlobalVar::parse(tokens)? {
                Ok(Some(ConstVar::GlobalVar(global_var)))
            } else if let Some(vd) = Vd::parse(tokens)? {
                Ok(Some(ConstVar::Vd(vd)))
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected a variable declaration".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

// --------------
// --- VarDec ---
// --------------

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

impl GraphDisplay for VarDec {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph VarDec_{}[VarDec]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        match self {
            VarDec::ConstVar(const_var) => const_var.graph_display(graph, id, indent + 2),
            VarDec::PrivateVar(private_var) => private_var.graph_display(graph, id, indent + 2),
            VarDec::GlobalVar(global_var) => global_var.graph_display(graph, id, indent + 2),
            VarDec::Vd(vd) => vd.graph_display(graph, id, indent + 2),
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent))
    }
}

impl_debug!(VarDec);

impl VarDec {
    pub(crate) fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <var_dec> ::= <const_var> | <private_var> | <global_var> | <vd>
        if let Some(const_var) = ConstVar::parse(tokens)? {
            Ok(Some(VarDec::ConstVar(const_var)))
        } else if let Some(private_var) = PrivateVar::parse(tokens)? {
            Ok(Some(VarDec::PrivateVar(private_var)))
        } else if let Some(global_var) = GlobalVar::parse(tokens)? {
            Ok(Some(VarDec::GlobalVar(global_var)))
        } else if let Some(vd) = Vd::parse(tokens)? {
            Ok(Some(VarDec::Vd(vd)))
        } else {
            Ok(None)
        }
    }
}

impl Evaluate for VarDec {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            Self::Vd(vd) => vd.evaluate(operation_context),
            _ => todo!(),
        }
    }
}

// ---------------
// --- VarMod ----
// ---------------

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
pub enum VarMod {
    Exp(Exp),
}

impl GraphDisplay for VarMod {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph VarMod_{}[VarMod]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        match &self {
            Self::Exp(exp) => exp.graph_display(graph, id, indent + 2),
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent))
    }
}

impl_debug!(VarMod);

impl VarMod {
    pub(crate) fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        if let Some(exp) = Exp::parse(tokens)? {
            Ok(Some(Self::Exp(exp)))
        } else {
            Ok(None)
        }
    }
}

impl Evaluate for VarMod {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            Self::Exp(exp) => exp.evaluate(operation_context),
        }
    }
}
