use crate::parse::nodes::expressions::{Exp, ExpBase};
use crate::parse::nodes::id_nodes::IdGet;
use crate::tokens::Token;

mod parse_variables;
mod parse_values;
pub(crate) mod nodes;

pub enum ValueBase {
    Bool(bool),
    Int(i32),
    String(String),
    Float(f32),
}

pub enum Value {
    Base(ValueBase),
    ExpBase(ExpBase), // TODO: Définir la structure ou l'énumération ExpBase
}

pub enum TakePrio {
    LeftP(Box<Exp>), // TODO: Définir la structure ou l'énumération Exp
    RightP(Box<Exp>), // TODO: Définir la structure ou l'énumération Exp
    Value(Value),
}

pub enum Tp {
    Plus(Box<Tp>),
    Minus(Box<Tp>),
    Not(Box<Tp>),
    TakePrio(TakePrio),
}

pub enum Mult {
    Mult(Box<Tp1>), // TODO: Définir la structure ou l'énumération Tp1
}

pub enum Div {
    Div(Box<Tp1>), // TODO: Définir la structure ou l'énumération Tp1
}

pub enum Md {
    Mult(Mult),
    Div(Div),
}

pub enum Tp1 {
    Tp(Tp),
    Md(Option<Md>),
}

pub enum Add {
    Add(Box<Tp2>), // TODO: Définir la structure ou l'énumération Tp2
}

pub enum Sub {
    Sub(Box<Tp2>), // TODO: Définir la structure ou l'énumération Tp2
}

pub enum As {
    Add(Add),
    Sub(Sub),
}

pub enum Tp2 {
    Tp1(Tp1),
    As(Option<As>),
}

pub enum Eq {
    Equal(Box<Tp3>), // TODO: Définir la structure ou l'énumération Tp3
}

pub enum NotEq {
    NotEqual(Box<Tp3>), // TODO: Définir la structure ou l'énumération Tp3
}

pub enum EqNot {
    Eq(Eq),
    NotEq(NotEq),
}

pub enum Tp3 {
    Tp2(Tp2),
    EqNot(Option<EqNot>),
}

pub enum And {
    And(Box<Tp4>), // TODO: Définir la structure ou l'énumération Tp4
}

pub enum Tp4 {
    Tp3(Tp3),
    And(Option<And>),
}

pub enum Or {
    Or(Box<Tp5>), // TODO: Définir la structure ou l'énumération Tp5
}

pub enum Tp5 {
    Tp4(Tp4),
    Or(Option<Or>),
}

pub enum TpLast {
    Tp5(Tp5),
}

pub enum NoValue {
    Md(Option<Md>),
    As(Option<As>),
    EqNot(Option<EqNot>),
    And(Option<And>),
    Or(Option<Or>),
}

pub struct NatCallIn {
    identifier: String,
    next: Option<Box<NatCallIn>>,
}

pub struct NatCall {
    nat_call_in: NatCallIn,
}

pub enum IdUse {
    IdSet(String), // TODO: Définir la structure ou l'énumération IdSet
    IdGet(IdGet),
}

pub fn main(tokens: Vec<Token>) {
    // let mut line = 0;
    let mut i = 0;
    // let vec: Vec<Vec<String>> = Vec::new();
    let nodes = ();
    if i != tokens.len() {
        panic!("Scope closed with }} before the end");
    }
    nodes
}
