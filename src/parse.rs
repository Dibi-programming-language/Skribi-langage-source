use std::collections::LinkedList;

use crate::tokens::{Space, Token};

mod parse_variables;
mod parse_values;
mod nodes;

pub struct Tuple {
    // TODO: définir les champs du tuple ici
}

pub struct CGet {
    name: String,
}

pub struct IdGet {
    identifier: String,
    tuple: Option<Tuple>,
    op_in: Box<OpIn>,
}

pub enum OpIn {
    In {
        id_get: Option<IdGet>,
        cget: Option<CGet>,
    },
    Empty,
}

pub enum BaseNode {
    CGet(CGet),
    OpIn(OpIn),
}

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

pub struct ClassDec {
    identifier: String,
    scope: Box<Scope>,
}

pub struct Type {
    type_def: String, // TODO: Définir le type de type_def
}

pub struct Vd {
    type_: Type,
    identifier: String,
    exp: Box<Exp>,
}

pub struct GlobalVar {
    vd: Vd,
}

pub struct PrivateVar {
    vd: Vd,
}

pub struct ConstVar {
    private_var: Option<PrivateVar>,
    global_var: Option<GlobalVar>,
    vd: Option<Vd>,
}

pub enum VarDec {
    ConstVar(ConstVar),
    PrivateVar(PrivateVar),
    GlobalVar(GlobalVar),
    Vd(Vd),
}

pub struct VarMod {
    exp: Exp,
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

pub struct IdUseV {
    id_use: IdUse,
    no_value: Option<NoValue>,
}

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

pub struct Return {
    exp: Exp,
}

pub enum Sta {
    Return(Return),
    Exp(Exp),
}

pub struct StaL {
    sta: Vec<Sta>,
}

pub struct KName {
    identifier: Option<String>,
    left_e: Option<Vec<String>>, // TODO: Définir le type de left_e
}

pub struct KStart {
    sta_l: StaL,
    k_name: Option<KName>,
}

pub struct Kodi {
    k_start: KStart,
}

pub struct Biuli {
    k_start: KStart,
}

pub struct Spoki {
    k_start: KStart,
}

pub enum ScopeBase {
    StaL(StaL),
    Kodi(Kodi),
    Spoki(Spoki),
    Biuli(Biuli),
}

pub enum Scope {
    ScopeBase(ScopeBase),
    Sta(Sta),
}

pub struct Sula {
    ij: Option<Ij>, // TODO: Définir la structure ou l'énumération Ij
    sula: Option<Box<Sula>>,
    scope: Scope,
}

pub struct Ij {
    exp: Exp,
    scope: Scope,
}

pub struct Cond {
    ij: Ij,
    sula: Option<Sula>,
}

pub struct FctDec {
    identifier: String,
    tuple: Tuple,
    scope: Scope,
}

fn parse_tuple(tokens: &mut Vec<Token>) -> Option<Tuple> {
    // TODO: implémenter cette fonction
    None
}

fn is_type_def(identifier: &str) -> bool {
    // TODO: implémenter cette fonction
    false
}

fn parse_cget(tokens: &mut Vec<Token>) -> Option<CGet> {
    for token in tokens.iter() {
        if let Token::Identifier(identifier) = token {
            if is_type_def(identifier) {
                return Some(CGet { name: identifier.clone() });
            }
        }
    }
    None
}

fn parse_op_in(tokens: &mut Vec<Token>) -> Option<OpIn> {
    for token in tokens.iter() {
        if let Token::Inside = token {
            return Some(OpIn::In {
                id_get: parse_id_get(tokens),
                cget: parse_cget(tokens),
            });
        }
    }
    Some(OpIn::Empty)
}

fn parse_id_get(tokens: &mut Vec<Token>) -> Option<IdGet> {
    for token in tokens.iter() {
        if let Token::Identifier(identifier) = token {
            return Some(IdGet {
                identifier: identifier.clone(),
                tuple: parse_tuple(tokens),
                op_in: Box::new(parse_op_in(tokens)?),
            });
        }
    }
    None
}

fn parse_base_node(tokens: &mut Vec<Token>) -> Option<BaseNode> {
    if let Some(cget) = parse_cget(tokens) {
        Some(BaseNode::CGet(cget))
    } else if let Some(op_in) = parse_op_in(tokens) {
        Some(BaseNode::OpIn(op_in))
    } else {
        None
    }
}

struct ParseFunction {
    name: String,
    arguments: Vec<String>,
    return_type: String,
}

/// Only used to check if a variable exists in a scope
struct ParseScope {
    /// Variables that can be used in this scope
    variables: Vec<String>,
    /// Types that can be used in this scope
    types: Vec<String>,
    /// Functions that can be used in this scope. UNUSED FOR NOW
    functions: Vec<ParseFunction>,
    parent: Option<Box<ParseScope>>,
}

impl ParseScope {
    fn new(parent: Option<Box<ParseScope>>) -> Self {
        ParseScope {
            variables: Vec::new(),
            types: Vec::new(),
            functions: Vec::new(),
            parent,
        }
    }

    fn base() -> Self {
        ParseScope {
            variables: Vec::new(),
            types: vec![
                "skr".to_string(),
                "int".to_string(),
                "dar".to_string(),
                "ioi".to_string(),
            ],
            functions: Vec::new(),
            parent: None,
        }
    }

    /// Check if a name can be used in this scope for a variable
    fn is_valid_name_for_variable(&self, name: String) -> bool {
        !(
            self.variables.contains(&name)
            || self.types.contains(&name)
            || self.functions.iter().any(|f| f.name == name)
            || (
                if let Some(parent) = &self.parent {
                    parent.is_valid_name_for_variable(name)
                } else {
                    false
                }
            )
        )
    }

    /// Check if a type exists in this scope
    fn is_valid_type(&self, name: String) -> bool {
        self.types.contains(&name)
        || (
            if let Some(parent) = &self.parent {
                parent.is_valid_type(name)
            } else {
                false
            }
        )
    }

    /// Check if a variable exists in this scope
    fn is_valid_variable(&self, name: String) -> bool {
        self.variables.contains(&name)
        || (
            if let Some(parent) = &self.parent {
                parent.is_valid_variable(name)
            } else {
                false
            }
        )
    }
}

pub fn main(tokens: Vec<Token>) {
    let mut line = 0;
    let mut i = 0;
    let vec: Vec<Vec<String>> = Vec::new();
    let nodes = ();
    if i != tokens.len() {
        panic!("Scope closed with }} before the end");
    }
    nodes
}
