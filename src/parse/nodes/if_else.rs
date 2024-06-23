use std::collections::VecDeque;
use crate::impl_debug;
use crate::parse::nodes::expressions::Exp;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::ResultOption;
use crate::tokens::Token;

// Grammar for this file:
// <sula> ::= sula (<ij> (<sula> |) | <scope>)
// <ij> ::= ij <exp> <scope>
// <cond> ::= <ij> (<sula> |)

// TODO : scope

#[derive(PartialEq)]
pub struct Scope {}

impl GraphDisplay for Scope {
    fn graph_display(&self, _graph: &mut String, _id: &mut usize) {
        // TODO
    }
}

impl_debug!(Scope);

impl Scope {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn parse(_tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // TODO
        Ok(Some(Scope::new()))
    }
}

// ------------
// --- Sula ---
// ------------

#[derive(PartialEq)]
pub enum Sula {
    Ij {
        ij: Ij,
        sula: Option<Box<Sula>>,
    },
    Scope(Scope)
}

// ----------
// --- Ij ---
// ----------

#[derive(PartialEq)]
pub struct Ij {
    exp: Exp,
    scope: Scope,
}

// ------------
// --- Cond ---
// ------------

#[derive(PartialEq)]
pub struct Cond {
    ij: Ij,
    sula: Option<Box<Sula>>,
}
