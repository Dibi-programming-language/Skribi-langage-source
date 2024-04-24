use std::fmt::Error;
use crate::parse::{Cond, FctDec, IdUse, IdUseV, ScopeBase, TpLast};
use crate::parse::nodes::vars::VarDec;
use crate::tokens::Token;

pub enum ExpBase {
    IdUse(Box<IdUse>),
    VarDec(Box<VarDec>),
    Cond(Box<Cond>), // TODO: Définir la structure ou l'énumération Cond
    ScopeBase(Box<ScopeBase>), // TODO: Définir la structure ou l'énumération ScopeBase
    FctDec(Box<FctDec>), // TODO: Définir la structure ou l'énumération FctDec
    LeftP(Box<Exp>),
    RightP(Box<Exp>),
}

pub enum ExpTp {
    ExpBase(ExpBase),
    IdUseV(IdUseV),
}

pub struct Exp {
    exp_tp: ExpTp,
    tp_last: TpLast,
}

pub fn parse_exp(tokens: &mut Vec<Token>) -> Option<Result<Exp, Error>> {
    None
}