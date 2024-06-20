use std::collections::VecDeque;

use crate::parse::nodes::vars::VarDec;
use crate::skr_errors::OptionResult;
use crate::tokens::Token;

/// Not yet implemented
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
pub enum ExpTp {
    ExpBase(ExpBase),
    // IdUseV(IdUseV), // TODO - define id_use_v
}

/// Not yet implemented, but already used by some nodes that are dependent on it
pub struct Exp {
    exp_tp: ExpTp,
    // tp_last: TpLast, // TODO - define tp_last
}

/// Not yet implemented, but already used by some nodes that are dependent on it
pub fn parse_exp(_tokens: &mut VecDeque<Token>) -> OptionResult<Exp> {
    // TODO
    None
}
