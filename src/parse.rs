mod parse_variables;
mod parse_values;
mod nodes;

use std::collections::LinkedList;
use crate::tokens::{ModifierKeyword, Token, ValueToken};
use skribi_language_source::error;

pub enum Value {
    ValueNode(ValueToken),
    Operation(Operation)
}

pub enum Operation {
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Mul(Box<Value>, Box<Value>),
    Div(Box<Value>, Box<Value>),
    Mod(Box<Value>, Box<Value>),
    Pow(Box<Value>, Box<Value>),
}

pub enum Node {
    Scope(Vec<Node>),
    NewVariable(Vec<ModifierKeyword>, String, Value),
    NewValue(String, Value),
    NativeCall(String, Vec<Value>),
    Operation(Operation)
}

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
    exp: Exp,
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
    IdSet(IdSet), // TODO: Définir la structure ou l'énumération IdSet
    IdGet(IdGet),
}

pub struct IdUseV {
    id_use: IdUse,
    no_value: Option<NoValue>,
}

pub enum ExpBase {
    IdUse(IdUse),
    VarDec(VarDec),
    Cond(Cond), // TODO: Définir la structure ou l'énumération Cond
    ScopeBase(ScopeBase), // TODO: Définir la structure ou l'énumération ScopeBase
    FctDec(FctDec), // TODO: Définir la structure ou l'énumération FctDec
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
        if let Token::TIn = token {
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

fn parse_scope(
    tokens: &Vec<Token>,
    i: &mut usize,
    line: &mut u16,
    variables: &Vec<Vec<String>>,
) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut not_finished = true;

    // Start an iterator with a index
    while not_finished && *i < tokens.len() {
        match tokens[*i] {
            Token::KeywordModifier(_) => {
                // Start a new variable
            }
            Token::KeywordNativeCall => {
                // Start a new native call
            }
            Token::Identifier(_) => {
                // Check if the identifier exists in this scope, and his type
            }
            Token::OpenBrace => {
                // Start a new scope
                *i += 1;
                let scope_nodes = parse_scope(tokens, i, line, variables);
                nodes.push(Node::Scope(scope_nodes));
            }
            Token::CloseBrace => {
                // Close the current scope
                not_finished = false;
            }
            Token::NewLine => {
                *line += 1;
            }
            // Ignored tokens
            Token::Semicolon => {}
            _ => {
                error("[PARSE] Invalid token or not implemented yet!", *line);
            }
        }
        *i += 1;
    }

    nodes
}

enum TreeElement {
    Node(Node),
    Token(Token),
    Value(Value),
}

// macro

fn parse_scope2(
    mut tokens: LinkedList<TreeElement>,
    i: &mut usize,
    line: &mut u16,
    variables: &Vec<Vec<String>>,
) -> Vec<Node> {
    let mut not_finished = true;
    let mut before: LinkedList<TreeElement> = LinkedList::new();
    // Current node
    let mut element: TreeElement = match tokens.pop_front() {
        None => {
            return Vec::new();
        }
        Some(n) => {
            n
        }
    };

    /* Je vais essayer de match ce patterne :
    = value
    > = _ value
    > > = * -> node *
    > > = / -> node /
    > > = +
    > > > != * ou / -> node +
    > > = -
    > > > != * ou / -> node -
    */

    // Start an iterator with a index
    while not_finished && *i < tokens.len() {
        match element {
            TreeElement::Token(Token::NewLine) => {
                *line += 1;
            }
            TreeElement::Value(ref v0) => {
                if let Some(token_p1) = tokens.pop_front() {
                    if let Some(token_p2) = tokens.pop_front() {
                        if let TreeElement::Value(v1) = token_p2 {
                            match token_p1 {
                                TreeElement::Token(Token::OperatorMul) => {
                                    // create
                                    let op = Operation::Mul(Box::new(v0), Box::new(v1));
                                    element = TreeElement::Node(Node::Operation(op));
                                    // décalage vers la gauche de 2 éléments en empilant à chaque fois ...
                                }
                                TreeElement::Token(Token::OperatorDiv) => {
                                    // create
                                    let op = Operation::Div(Box::new(v0), Box::new(v1));
                                    element = TreeElement::Node(Node::Operation(op));
                                    // décalage vers la gauche ...
                                }
                                TreeElement::Token(Token::OperatorAdd) => {
                                    // ...
                                }
                                TreeElement::Token(Token::OperatorSub) => {

                                }
                                _ => {
                                    // SKIP
                                    before.push_back(element);
                                    element = token_p2;
                                }
                            }
                        }
                    }
                }
            }
            TreeElement::Node(_) => {

            }
            _ => {
                // SKIP
                before.push_back(element);
                match tokens.pop_front() {
                    None => {
                        tokens = before;
                        before = LinkedList::new();
                        match tokens.pop_front() {
                            None => {
                                element = TreeElement::Token(Token::NewLine);
                                not_finished = false;
                            }
                            Some(e) => {
                                element = e;
                            }
                        }
                    }
                    Some(n) => {
                        element = n;
                    }
                }
            }
        }
    }

    Vec::new()
}

pub fn main(tokens: Vec<Token>) -> Vec<Node> {
    let mut line = 0;
    let mut i = 0;
    let vec: Vec<Vec<String>> = Vec::new();
    let nodes = parse_scope(&tokens, &mut i, &mut line, &vec);
    if i != tokens.len() {
        error("Scope closed with } before the end", line);
    }
    nodes
}
