use crate::impl_debug;
use crate::parse::nodes::GraphDisplay;
use std::collections::VecDeque;

use crate::parse::nodes::vars::VarDec;
use crate::skr_errors::OptionResult;
use crate::tokens::Token;

/// Not yet implemented
#[derive(PartialEq)]
pub enum ExpBase {
    // IdUse(Box<IdUse>), // TODO - define id_use
    VarDec(Box<VarDec>),
    // Cond(Box<Cond>), // TODO - define cond
    // ScopeBase(Box<ScopeBase>), // TODO - define scope_base
    // FctDec(Box<FctDec>), // TODO - define fct_dec
    LeftP(Box<Exp>),
    RightP(Box<Exp>),
}

/// Not yet implemented
#[derive(PartialEq)]
pub enum ExpTp {
    ExpBase(ExpBase),
    // IdUseV(IdUseV), // TODO - define id_use_v
}

/// Not yet implemented, but already used by some nodes that are dependent on it
#[derive(PartialEq)]
pub struct Exp {
    exp_tp: ExpTp,
    // tp_last: TpLast, // TODO - define tp_last
}

impl GraphDisplay for Exp {
    fn graph_display(&self, _graph: &mut String, _id: &mut usize) {
        // TODO
    }
}

impl_debug!(Exp);

impl Exp {
    /// Not yet implemented
    fn new(exp_tp: ExpTp) -> Self {
        Self { exp_tp }
    }

    pub fn parse(_tokens: &mut VecDeque<Token>) -> OptionResult<Exp> {
        // TODO
        None
    }
}
