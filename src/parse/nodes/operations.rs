use crate::parse::nodes::expressions::{Exp, ExpBase};
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, ResultOption};
use crate::tokens::{Token, TokenContainer};
use crate::{impl_debug, some_token};
use std::collections::VecDeque;

// Grammar for this file :
/*
<value_base> ::= T_BOOL | T_INT | T_STRING | T_FLOAT
<value> ::=
  <value_base>
  | <exp_base>
<take_prio> ::=
  T_LEFT_P <exp> T_RIGHT_P
  | <value>

 */

// -----------------
// --- ValueBase ---
// -----------------

/// `ValueBase` represents the base of a value in the AST. This is the smallest unit of a value.
/// This node is not dependent on any other node. The value can be a boolean, an integer, a float or
/// a string.
#[derive(PartialEq)]
pub enum ValueBase {
    Bool(bool),
    Int(u32),
    Float(f32),
    String(String),
}

impl GraphDisplay for ValueBase {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        match self {
            ValueBase::Bool(value) => {
                graph.push_str(&format!(
                    "\nsubgraph ValueBase_{}[ValueBase Bool {}]\nend",
                    id, value
                ));
            }
            ValueBase::Int(value) => {
                graph.push_str(&format!(
                    "\nsubgraph ValueBase_{}[ValueBase Int {}]\nend",
                    id, value
                ));
            }
            ValueBase::Float(value) => {
                graph.push_str(&format!(
                    "\nsubgraph ValueBase_{}[ValueBase Float {}]\nend",
                    id, value
                ));
            }
            ValueBase::String(value) => {
                graph.push_str(&format!(
                    "\nsubgraph ValueBase_{}[ValueBase String {}]\nend",
                    id, value
                ));
            }
        }
        *id += 1;
    }
}

impl_debug!(ValueBase);

impl ValueBase {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> Option<Self> {
        // <value_base> ::= T_BOOL | T_INT | T_STRING | T_FLOAT
        match tokens.front() {
            Some(Token::Bool(_)) => {
                if let Some(Token::Bool(value)) = tokens.pop_front() {
                    Some(ValueBase::Bool(value))
                } else {
                    None
                }
            }
            Some(Token::Int(_)) => {
                if let Some(Token::Int(value)) = tokens.pop_front() {
                    Some(ValueBase::Int(value))
                } else {
                    None
                }
            }
            Some(Token::Float(_)) => {
                if let Some(Token::Float(value)) = tokens.pop_front() {
                    Some(ValueBase::Float(value))
                } else {
                    None
                }
            }
            Some(Token::String(_)) => {
                if let Some(Token::String(value)) = tokens.pop_front() {
                    Some(ValueBase::String(value))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// -------------
// --- Value ---
// -------------

/// `ValueNode` represents any value that has a priority over many nodes. This node cannot be
/// mistaken with a wrong node because the syntax is clear. This node is either a [ValueBase] or an
/// [ExpBase].
///
/// [ValueNode] and [ExpBase] have in common that all their possibles values start with a token that
/// can only mean one thing. Example : `T_BOOL` can only be a boolean, `biuli` can only mean that
/// this is a special scope.
#[derive(PartialEq)]
pub enum ValueNode {
    ValueBase(ValueBase),
    ExpBase(ExpBase),
}

impl GraphDisplay for ValueNode {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph ValueNode_{}[ValueNode]", id));
        *id += 1;
        match self {
            ValueNode::ValueBase(value) => {
                value.graph_display(graph, id);
            }
            ValueNode::ExpBase(value) => {
                value.graph_display(graph, id);
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(ValueNode);

impl ValueNode {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <value> ::=
        //   <value_base>
        //   | <exp_base>
        if let Some(value_base) = ValueBase::parse(tokens) {
            Ok(Some(ValueNode::ValueBase(value_base)))
        } else {
            match ExpBase::parse(tokens)? {
                Some(exp_base) => Ok(Some(ValueNode::ExpBase(exp_base))),
                None => Ok(None),
            }
        }
    }
}

// ----------------
// --- TakePrio ---
// ----------------

/// `TakePriority` represents either a [ValueNode] or an [Exp]. This node is used to give a priority
/// to a value. It can detect [Exp] only between parenthesis : this takes priority over everything.
#[derive(PartialEq)]
pub enum TakePriority {
    Exp(Box<Exp>),
    Value(ValueNode),
}

impl GraphDisplay for TakePriority {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph TakePriority_{}[TakePriority]", id));
        *id += 1;
        match self {
            TakePriority::Exp(value) => {
                value.graph_display(graph, id);
            }
            TakePriority::Value(value) => {
                value.graph_display(graph, id);
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(TakePriority);

impl TakePriority {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <take_prio> ::=
        //   T_LEFT_P <exp> T_RIGHT_P
        //   | <value>
        let front = tokens.front();
        if let some_token!(Token::LeftParenthesis) = front {
            tokens.pop_front();
            match Exp::parse(tokens)? {
                Some(exp) => {
                    if let some_token!(Token::RightParenthesis) = tokens.pop_front() {
                        Ok(Some(TakePriority::Exp(Box::new(exp))))
                    } else {
                        Err(CustomError::UnexpectedToken(
                            "Expected a right parenthesis".to_string(),
                        ))
                    }
                }
                None => Err(CustomError::UnexpectedToken(
                    "Expected an expression".to_string(),
                )),
            }
        } else if let Some(value) = ValueNode::parse(tokens)? {
            Ok(Some(TakePriority::Value(value)))
        } else {
            Ok(None)
        }
    }
}

// ----------------
// --- Unary TP ---
// ----------------

/// `UnaryTP` represents a chain (0 or more elements) of unary operators before a [TakePriority].
///
/// The unary operators are : `+`, `-` and `!`. Example : `+ -+ ![TakePriority]` is an [UnaryTP].
#[derive(PartialEq)]
pub enum UnaryTP {
    Plus(Box<UnaryTP>),
    Minus(Box<UnaryTP>),
    Not(Box<UnaryTP>),
    TakePriority(TakePriority),
}

impl GraphDisplay for UnaryTP {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph UnaryTP_{}[unary_tp]", id));
        *id += 1;
        match self {
            UnaryTP::Plus(unary_tp) => {
                unary_tp.graph_display(graph, id);
            }
            UnaryTP::Minus(unary_tp) => {
                unary_tp.graph_display(graph, id);
            }
            UnaryTP::Not(unary_tp) => {
                unary_tp.graph_display(graph, id);
            }
            UnaryTP::TakePriority(take_priority) => {
                take_priority.graph_display(graph, id);
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(UnaryTP);

impl UnaryTP {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <tp> ::=
        //   (T_PLUS | T_MINUS | T_NOT) <tp>
        //   | <take_prio>
        let front = tokens.front();
        match front {
            some_token!(Token::Add) => {
                tokens.pop_front();
                let unary_tp = UnaryTP::parse(tokens)?;
                match unary_tp {
                    Some(unary_tp) => Ok(Some(UnaryTP::Plus(Box::new(unary_tp)))),
                    None => Err(CustomError::UnexpectedToken(
                        "Expected an unary_tp".to_string(),
                    )),
                }
            }
            some_token!(Token::Sub) => {
                tokens.pop_front();
                let unary_tp = UnaryTP::parse(tokens)?;
                match unary_tp {
                    Some(unary_tp) => Ok(Some(UnaryTP::Minus(Box::new(unary_tp)))),
                    None => Err(CustomError::UnexpectedToken(
                        "Expected an unary_tp".to_string(),
                    )),
                }
            }
            // TODO not
            _ => {
                let take_priority = TakePriority::parse(tokens)?;
                match take_priority {
                    Some(take_priority) => Ok(Some(UnaryTP::TakePriority(take_priority))),
                    None => Ok(None),
                }
            }
        }
    }
}

/*
<op n> ::= T_OPERATIONS_N <tp n-1>
With:
1. * and /
2. + and -
3. = and !=
4. &&
5. ||

<tp0> ::=
  (T_PLUS | T_MINUS | T_NOT) <tp>
  | <take_prio>
<tp n> ::= <tp n-1> (<op n> |)

<tp_last> ::= <tp max>

<nv0> ::= <op max>
<nv n> ::= <op max-n> (<nv n-1> |) | <nv n-1>
<no_value> ::= <nv max>
 */

pub enum Operations {
    Mul,
    Div,
    Add,
    Sub,
    Equal,
    NotEqual,
    And,
    Or,
}

impl Token {
    pub fn get_level(&self) -> u8 {
        match self {
            Token::Mul => 1,
            Token::Div => 1,
            Token::Add => 2,
            Token::Sub => 2,
            Token::Equal => 3,
            Token::NotEqual => 3,
            Token::And => 4,
            Token::Or => 5,
            _ => panic!("Invalid token level"),
        }
    }
}

macro_rules! match_level {
    ($op: expr, $l: expr) => {
        (op.get_level() == l)
    };
}

pub struct OperationN {
    level: u8,
    operation: Operations,
    tp_n1: Box<TakePriorityN>,
}

pub enum TakePriorityN {
    ElementUnary0(Box<UnaryTP>),
    ElementSimple0(Box<TakePriority>),
    ElementN {
        level: u8,
        tp_n1: Box<TakePriorityN>,
        op_n: Option<Box<OperationN>>,
    },
}

pub struct TakePriorityLast {
    child: TakePriorityN,
}

pub enum NoValueN {
    Element0(Box<OperationN>),
    ElementOperationN {
        level: u8,
        operation: Box<OperationN>,
        no_value_before: Option<Box<NoValueN>>,
    },
    ElementSimpleN {
        level: u8,
        no_value_before: Box<NoValueN>,
    },
}



impl_debug!(Operations);
impl_debug!(OperationN);
impl_debug!(TakePriorityLast);
impl_debug!(NoValueN);
impl_debug!(TakePriorityN);

impl GraphDisplay for Operations {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph Operation_{}[Op {}]",
            id,
            match self {
                Operations::Mul => "OP *",
                Operations::Div => "OP /",
                Operations::Add => "OP +",
                Operations::Sub => "OP -",
                Operations::Equal => "CO =",
                Operations::NotEqual => "CO !=",
                Operations::And => "LG &&",
                Operations::Or => "LG ||",
            }
        ));
        *id += 1;
        graph.push_str("\nend");
    }
}

impl GraphDisplay for OperationN {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph OperationN_{}[OP N={}]",
            id, self.level
        ));
        *id += 1;
        self.operation.graph_display(graph, id);
        self.tp_n1.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl GraphDisplay for TakePriorityN {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        match self {
            TakePriorityN::ElementUnary0(unary) => {
                graph.push_str(&format!("\nsubgraph TakePriorityN_{}[TP0 UNARY]", id));
                *id += 1;
                unary.graph_display(graph, id);
                graph.push_str("\nend");
            }
            TakePriorityN::ElementSimple0(simple) => {
                graph.push_str(&format!("\nsubgraph TakePriorityN_{}[TP0 SIMPLE]", id));
                *id += 1;
                simple.graph_display(graph, id);
                graph.push_str("\nend");
            }
            TakePriorityN::ElementN { level, tp_n1, op_n } => {
                graph.push_str(&format!("\nsubgraph TakePriorityN_{}[TP N={}]", id, level));
                *id += 1;
                tp_n1.graph_display(graph, id);
                if let Some(op_n) = op_n {
                    op_n.graph_display(graph, id);
                }
                graph.push_str("\nend");
            }
        }
    }
}

impl GraphDisplay for TakePriorityLast {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph TakePriorityLst_{}[TP_LAST]", id));
        *id += 1;
        self.child.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl GraphDisplay for NoValueN {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        match self {
            NoValueN::Element0(op) => {
                graph.push_str(&format!("\nsubgraph NoValueN_{}[NoValue0]", id));
                *id += 1;
                op.graph_display(graph, id);
                graph.push_str("\nend");
            }
            NoValueN::ElementOperationN {
                level,
                operation,
                no_value_before,
            } => {
                graph.push_str(&format!(
                    "\nsubgraph NoValueN_{}[NoValueOp N={}]",
                    id, level
                ));
                *id += 1;
                operation.graph_display(graph, id);
                if let Some(no_value_before) = no_value_before {
                    no_value_before.graph_display(graph, id);
                }
                graph.push_str("\nend");
            }
            NoValueN::ElementSimpleN {
                level,
                no_value_before,
            } => {
                graph.push_str(&format!(
                    "\nsubgraph NoValueN_{}[NoValueSimple N={}]",
                    id, level
                ));
                *id += 1;
                no_value_before.graph_display(graph, id);
                graph.push_str("\nend");
            }
        }
    }
}

// -----------------------------
// --- Mult, div, md and TP1 ---
// -----------------------------

// Functions for parsing Mult, Div, Md and TP1

impl Mult {
    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <mult> ::= T_MULT <tp1>
        let front = tokens.front();
        if let some_token!(Token::Mul) = front {
            tokens.pop_front();
            let tp1 = TP1::parse(tokens)?;
            match tp1 {
                Some(tp1) => Ok(Some(Mult { tp1 })),
                None => Err(CustomError::UnexpectedToken("Expected a tp1".to_string())),
            }
        } else {
            Ok(None)
        }
    }
}

impl Div {
    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <div> ::= T_DIV <tp1>
        let front = tokens.front();
        if let some_token!(Token::Div) = front {
            tokens.pop_front();
            let tp1 = TP1::parse(tokens)?;
            match tp1 {
                Some(tp1) => Ok(Some(Div { tp1 })),
                None => Err(CustomError::UnexpectedToken("Expected a tp1".to_string())),
            }
        } else {
            Ok(None)
        }
    }
}

impl Md {
    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <md> ::= <mult> | <div>
        if let Some(mult) = Mult::parse(tokens)? {
            Ok(Some(Md::Mult(mult)))
        } else if let Some(div) = Div::parse(tokens)? {
            Ok(Some(Md::Div(div)))
        } else {
            Ok(None)
        }
    }
}

impl TP1 {
    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <tp1> ::= <tp> (<md> |)
        let unary_tp = UnaryTP::parse(tokens)?;
        match unary_tp {
            Some(unary_tp) => {
                let md = Md::parse(tokens)?;
                match md {
                    Some(md) => Ok(Some(TP1 {
                        unary_tp,
                        md: Some(Box::new(md)),
                    })),
                    None => Ok(Some(TP1 { unary_tp, md: None })),
                }
            }
            None => Ok(None),
        }
    }
}

// ----------------------------
// --- Add, sub, as and TP2 ---
// ----------------------------

// ----------------------------------
// --- Eq, not_eq, eq_not and TP3 ---
// ----------------------------------

// Enums and structs for Eq, NotEq, EqNot and TP3

/// `Eq` represents the right part of an equality in the AST. This node is composed of a [TP3] node
/// that can chain operations of same priority.
#[derive(PartialEq)]
pub struct Eq {
    tp3: TP3,
}

/// `NotEq` represents the right part of an inequality in the AST. This node is composed of a [TP3]
/// node that can chain operations of same priority.
#[derive(PartialEq)]
pub struct NotEq {
    tp3: TP3,
}

/// `EqNot` represents either an [struct@Eq] or a [NotEq] in the AST. This node is only used for grammar
/// commodity and to simplify the structure of [TP3].
#[derive(PartialEq)]
pub enum EqNot {
    Eq(Eq),
    NotEq(NotEq),
}

/// `TP3` is used to chain operations of same priority. This node works exactly like [TP2] but with
/// a lower priority. This node is composed of a [TP2] and an optional [EqNot]. The [TP2] is the
/// first operand of the chain and the [EqNot] is the rest of the chain.
///
/// Like all `TP` nodes, the first operand is the operation node with a priority just over this node
/// type. Here, additions and subtractions have the priority over equalities and inequalities.
#[derive(PartialEq)]
pub struct TP3 {
    tp2: TP2,
    eq_not: Option<Box<EqNot>>,
}

// Implementations for Eq, NotEq, EqNot and TP3 of GraphDisplay

impl GraphDisplay for Eq {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Eq_{}[Eq]", id));
        *id += 1;
        self.tp3.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Eq);

impl GraphDisplay for NotEq {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph NotEq_{}[NotEq]", id));
        *id += 1;
        self.tp3.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(NotEq);

impl GraphDisplay for EqNot {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph EqNot_{}[EqNot]", id));
        *id += 1;
        match self {
            EqNot::Eq(eq) => {
                eq.graph_display(graph, id);
            }
            EqNot::NotEq(not_eq) => {
                not_eq.graph_display(graph, id);
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(EqNot);

impl GraphDisplay for TP3 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph TP3_{}[TP3]", id));
        *id += 1;
        self.tp2.graph_display(graph, id);
        if let Some(eq_not) = &self.eq_not {
            eq_not.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(TP3);

// Functions for parsing Eq, NotEq, EqNot and TP3

impl Eq {
    fn new(tp3: TP3) -> Eq {
        Eq { tp3 }
    }

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Eq> {
        // <eq> ::= T_EQUAL <tp3>
        let front = tokens.front();
        if let Some(Token::Equal) = front {
            tokens.pop_front();
            let tp3 = TP3::parse(tokens)?;
            match tp3 {
                Some(tp3) => Ok(Some(Eq::new(tp3))),
                None => Err(CustomError::UnexpectedToken("Expected a tp3".to_string())),
            }
        } else {
            Ok(None)
        }
    }
}

impl NotEq {
    fn new(tp3: TP3) -> NotEq {
        NotEq { tp3 }
    }

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<NotEq> {
        // <not_eq> ::= T_NOT_EQUAL <tp3>
        let front = tokens.front();
        if let Some(Token::NotEqual) = front {
            tokens.pop_front();
            let tp3 = TP3::parse(tokens)?;
            match tp3 {
                Some(tp3) => Ok(Some(NotEq::new(tp3))),
                None => Err(CustomError::UnexpectedToken("Expected a tp3".to_string())),
            }
        } else {
            Ok(None)
        }
    }
}

impl EqNot {
    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<EqNot> {
        // <eq_not> ::= <eq> | <not_eq>
        if let Some(eq) = Eq::parse(tokens)? {
            Ok(Some(EqNot::Eq(eq)))
        } else if let Some(not_eq) = NotEq::parse(tokens)? {
            Ok(Some(EqNot::NotEq(not_eq)))
        } else {
            Ok(None)
        }
    }
}

impl TP3 {
    fn new(tp2: TP2, eq_not: Option<EqNot>) -> TP3 {
        TP3 {
            tp2,
            eq_not: eq_not.map(Box::new),
        }
    }

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<TP3> {
        // <tp3> ::= <tp2> (<eq_not> |)
        let tp2 = TP2::parse(tokens)?;
        match tp2 {
            Some(tp2) => {
                let eq_not = EqNot::parse(tokens)?;
                match eq_not {
                    Some(eq_not) => Ok(Some(TP3::new(tp2, Some(eq_not)))),
                    None => Ok(Some(TP3::new(tp2, None))),
                }
            }
            None => Ok(None),
        }
    }
}

// -------------------
// --- And and TP4 ---
// -------------------

// Enums and structs for And and TP4

/// `And` represents the right part of an AND operation in the AST. This node is composed of a [TP4]
/// node that can chain operations of same priority.
#[derive(PartialEq)]
pub struct And {
    tp4: TP4,
}

/// [TP4] is used to chain operations of same priority. This node works exactly like [TP3] but with
/// a lower priority. This node is composed of a [TP3] and an optional [And]. The [TP3] is the first
/// operand of the chain and the [And] is the rest of the chain.
///
/// Like all `TP` nodes, the first operand is the operation node with a priority just over this node
/// type. Here, equalities and inequalities have the priority over AND operations.
#[derive(PartialEq)]
pub struct TP4 {
    tp3: TP3,
    and: Option<Box<And>>,
}

// Implementations for And and TP4 of GraphDisplay

impl GraphDisplay for And {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph And_{}[And]", id));
        *id += 1;
        self.tp4.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(And);

impl GraphDisplay for TP4 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph TP4_{}[TP4]", id));
        *id += 1;
        self.tp3.graph_display(graph, id);
        if let Some(and) = &self.and {
            and.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(TP4);

// Functions for parsing And and TP4

impl And {
    fn new(tp4: TP4) -> And {
        And { tp4 }
    }

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<And> {
        // <and> ::= T_AND <tp4>
        let front = tokens.front();
        if let Some(Token::And) = front {
            tokens.pop_front();
            let tp4 = TP4::parse(tokens)?;
            match tp4 {
                Some(tp4) => Ok(Some(And::new(tp4))),
                None => Err(CustomError::UnexpectedToken("Expected a tp4".to_string())),
            }
        } else {
            Ok(None)
        }
    }
}

impl TP4 {
    fn new(tp3: TP3, and: Option<And>) -> TP4 {
        TP4 {
            tp3,
            and: and.map(Box::new),
        }
    }

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <tp4> ::= <tp3> (<and> |)
        let tp3 = TP3::parse(tokens)?;
        match tp3 {
            Some(tp3) => {
                let and = And::parse(tokens)?;
                Ok(Some(TP4::new(tp3, and)))
            }
            None => Ok(None),
        }
    }
}

// ------------------
// --- Or and TP5 ---
// ------------------

// Enums and structs for Or and TP5

/// `Or` represents the right part of an OR operation in the AST. This node is composed of a [TP5]
/// node that can chain operations of same priority.
#[derive(PartialEq)]
pub struct Or {
    tp5: TP5,
}

/// `TP5` is used to chain operations of same priority. This node works exactly like [TP4] but with
/// a lower priority. This node is composed of a [TP4] and an optional [Or]. The [TP4] is the first
/// operand of the chain and the [Or] is the rest of the chain.
///
/// Like all `TP` nodes, the first operand is the operation node with a priority just over this node
/// type. Here, AND operations have the priority over OR operations.
#[derive(PartialEq)]
pub struct TP5 {
    tp4: TP4,
    or: Option<Box<Or>>,
}

// Implementations for Or and TP5 of GraphDisplay

impl GraphDisplay for Or {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Or_{}[Or]", id));
        *id += 1;
        self.tp5.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Or);

impl GraphDisplay for TP5 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph TP5_{}[TP5]", id));
        *id += 1;
        self.tp4.graph_display(graph, id);
        if let Some(or) = &self.or {
            or.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(TP5);

// Functions for parsing Or and TP5

impl Or {
    fn new(tp5: TP5) -> Or {
        Or { tp5 }
    }

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Or> {
        // <or> ::= T_OR <tp5>
        let front = tokens.front();
        if let Some(Token::Or) = front {
            tokens.pop_front();
            let tp5 = TP5::parse(tokens)?;
            match tp5 {
                Some(tp5) => Ok(Some(Or::new(tp5))),
                None => Err(CustomError::UnexpectedToken("Expected a tp5".to_string())),
            }
        } else {
            Ok(None)
        }
    }
}

impl TP5 {
    fn new(tp4: TP4, or: Option<Or>) -> TP5 {
        TP5 {
            tp4,
            or: or.map(Box::new),
        }
    }

    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <tp5> ::= <tp4> (<or> |)
        let tp4 = TP4::parse(tokens)?;
        match tp4 {
            Some(tp4) => {
                let or = Or::parse(tokens)?;
                Ok(Some(TP5::new(tp4, or)))
            }
            None => Ok(None),
        }
    }
}

// -----------
// --- nv0 ---
// -----------

/// `NV0` is a node used to parse the right part of an operation chain, without any value at the
/// left. The chain inside this node can only contain [And] nodes or [Or] nodes.
///
/// # Grammar
///
/// `<nv0> ::= <and> (<or> |) | <or>`
///
/// See also [And] and [Or].
#[derive(PartialEq)]
pub enum NV0 {
    And { and: And, or: Option<Or> },
    Or(Or),
}

impl GraphDisplay for NV0 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph NV0_{}[NV0]", id));
        *id += 1;
        match self {
            NV0::And { and, or } => {
                and.graph_display(graph, id);
                if let Some(or) = or {
                    or.graph_display(graph, id);
                }
            }
            NV0::Or(or) => {
                or.graph_display(graph, id);
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(NV0);

impl NV0 {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <nv0> ::= <and> (<or> |) | <or>
        if let Some(and) = And::parse(tokens)? {
            Ok(Some(NV0::And {
                and,
                or: Or::parse(tokens)?,
            }))
        } else if let Some(or) = Or::parse(tokens)? {
            Ok(Some(NV0::Or(or)))
        } else {
            Ok(None)
        }
    }
}

// -----------
// --- NV1 ---
// -----------

/// `NV1` is a node used to parse the right part of an operation chain, without any value at the
/// left. The chain inside this node can only contain [EqNot] nodes, [And] nodes or [Or] nodes.
///
/// # Grammar
///
/// `<nv1> ::= <eq_not> (<nv0> |) | <nv0>`
///
/// See also [EqNot], and [NV0].
#[derive(PartialEq)]
pub enum NV1 {
    EqNot { eq_not: EqNot, nv0: Option<NV0> },
    NV0(NV0),
}

impl GraphDisplay for NV1 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph NV1_{}[NV1]", id));
        *id += 1;
        match self {
            NV1::EqNot { eq_not, nv0 } => {
                eq_not.graph_display(graph, id);
                if let Some(nv0) = nv0 {
                    nv0.graph_display(graph, id);
                }
            }
            NV1::NV0(nv0) => {
                nv0.graph_display(graph, id);
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(NV1);

impl NV1 {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <nv1> ::= <eq_not> (<nv0> |) | <nv0>
        if let Some(eq_not) = EqNot::parse(tokens)? {
            Ok(Some(NV1::EqNot {
                eq_not,
                nv0: NV0::parse(tokens)?,
            }))
        } else if let Some(nv0) = NV0::parse(tokens)? {
            Ok(Some(NV1::NV0(nv0)))
        } else {
            Ok(None)
        }
    }
}

// -----------
// --- NV2 ---
// -----------

// -----------
// --- NV3 ---
// -----------

/// `NV3` is a node used to parse the right part of an operation chain, without any value at the
/// left. The chain inside this node can only contain [TP5] nodes, [EqNot] nodes, [And] nodes, [Or]
/// nodes or [As] nodes.
///
/// # Grammar
///
/// `<nv3> ::= <md> (<nv2> |) | <nv2>`
///
/// See also [Md] and [NV2].
#[derive(PartialEq)]
pub enum NV3 {
    Md { md: Md, nv2: Option<NV2> },
    NV2(NV2),
}

impl GraphDisplay for NV3 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph NV3_{}[NV3]", id));
        *id += 1;
        match self {
            NV3::Md { md, nv2 } => {
                md.graph_display(graph, id);
                if let Some(nv2) = nv2 {
                    nv2.graph_display(graph, id);
                }
            }
            NV3::NV2(nv2) => {
                nv2.graph_display(graph, id);
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(NV3);

impl NV3 {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <nv3> ::= <md> (<nv2> |) | <nv2>
        if let Some(md) = Md::parse(tokens)? {
            Ok(Some(NV3::Md {
                md,
                nv2: NV2::parse(tokens)?,
            }))
        } else if let Some(nv2) = NV2::parse(tokens)? {
            Ok(Some(NV3::NV2(nv2)))
        } else {
            Ok(None)
        }
    }
}

// ----------------------------
// --- TP Last and No Value ---
// ----------------------------

/// `TPLast` is the last `TP` node. It is composed of a [TP5] node. It is only used to avoid
/// recoding some parts if we want to add more operations in the future.
#[derive(PartialEq)]
pub struct TPLast {
    tp5: TP5,
}

/// `NoValue` is a node used to parse the right part of an operation chain, without any value at the
/// left. This is used when the left value is already parsed, and we see the operator after. At
/// least one operator is expected in this node.
///
/// # Grammar
///
/// `<no_value> ::= <nv3>`
///
/// See also [NV3].
#[derive(PartialEq)]
pub struct NoValue {
    nv3: NV3,
}

// Implementations for TPLast and NoValue of GraphDisplay

impl GraphDisplay for TPLast {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph TPLast_{}[TPLast]", id));
        *id += 1;
        self.tp5.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(TPLast);

impl GraphDisplay for NoValue {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph NoValue_{}[NoValue]", id));
        *id += 1;
        self.nv3.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(NoValue);

// Functions for parsing TPLast and NoValue

impl TPLast {
    pub(crate) fn new(tp5: TP5) -> TPLast {
        TPLast { tp5 }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<TPLast> {
        // <tp_last> ::= <tp5>
        let tp5 = TP5::parse(tokens)?;
        match tp5 {
            Some(tp5) => Ok(Some(TPLast::new(tp5))),
            None => Ok(None),
        }
    }
}

impl NoValue {
    pub(crate) fn new(nv3: NV3) -> NoValue {
        NoValue { nv3 }
    }

    pub(crate) fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <no_value> ::= <nv3>
        let nv3 = NV3::parse(tokens)?;
        match nv3 {
            Some(nv3) => Ok(Some(NoValue::new(nv3))),
            None => Ok(None),
        }
    }
}

// -------------------
// --- END OF FILE ---
// -------------------
