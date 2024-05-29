use std::fmt::Error;

use crate::parse::{IdUse, TpLast};
use crate::parse::nodes::vars::VarDec;
use crate::tokens::Token;

pub enum ExpBase {
    IdUse(Box<IdUse>),
    VarDec(Box<VarDec>),
    // Cond(Box<Cond>), // TODO - define cond
    // ScopeBase(Box<ScopeBase>), // TODO - define scope_base
    // FctDec(Box<FctDec>), // TODO - define fct_dec
    LeftP(Box<Exp>),
    RightP(Box<Exp>),
}

pub enum ExpTp {
    ExpBase(ExpBase),
    // IdUseV(IdUseV), // TODO - define id_use_v
}

pub struct Exp {
    exp_tp: ExpTp,
    tp_last: TpLast,
}

pub fn parse_exp(tokens: &mut Vec<Token>) -> Option<Result<Exp, Error>> {
    None
}