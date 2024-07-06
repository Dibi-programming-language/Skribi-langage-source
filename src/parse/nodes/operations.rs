use std::collections::VecDeque;

use crate::impl_debug;
use crate::parse::nodes::expressions::{Exp, ExpBase};
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, ResultOption, ShortResult};
use crate::tokens::Token;

// Grammar for this file :
/*
<value_base> ::= T_BOOL | T_INT | T_STRING | T_FLOAT
<value> ::=
  | <value_base>
  | <exp_base>
<take_prio> ::=
  T_LEFT_P <exp> T_RIGHT_P
  | <value>
<tp> ::=
  (T_PLUS | T_MINUS | T_NOT) <tp>
  | <take_prio>
<mult> ::= T_MULT <tp1>
<div> ::= T_DIV <tp1>
<md> ::= <mult> | <div>
<tp1> ::= <tp> (<md> |)
<add> ::= T_ADD <tp2>
<sub> ::= T_SUB <tp2>
<as> ::= <add> | <sub>
<tp2> ::= <tp1> (<as> |)
<eq> ::= T_EQUAL <tp3>
<not_eq> ::= T_NOT_EQUAL <tp3>
<eq_not> ::= <eq> | <not_eq>
<tp3> ::= <tp2> (<eq_not> |)
<and> ::= T_AND <tp4>
<tp4> ::= <tp3> (<and> |)
<or> ::= T_OR <tp5>
<tp5> ::= <tp4> (<or> |)
<tp_last> ::= <tp5>
<no_value> ::= (<md> |) (<as> |) (<eq_not> |) (<and> |) (<or> |)
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
    pub fn parse(tokens: &mut VecDeque<Token>) -> Option<Self> {
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
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <value> ::=
        //   | <value_base>
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
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <take_prio> ::=
        //   T_LEFT_P <exp> T_RIGHT_P
        //   | <value>
        let front = tokens.front();
        if let Some(Token::LeftParenthesis) = front {
            tokens.pop_front();
            match Exp::parse(tokens)? {
                Some(exp) => {
                    if let Some(Token::RightParenthesis) = tokens.pop_front() {
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
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <tp> ::=
        //   (T_PLUS | T_MINUS | T_NOT) <tp>
        //   | <take_prio>
        let front = tokens.front();
        match front {
            Some(Token::Add) => {
                tokens.pop_front();
                let unary_tp = UnaryTP::parse(tokens)?;
                match unary_tp {
                    Some(unary_tp) => Ok(Some(UnaryTP::Plus(Box::new(unary_tp)))),
                    None => Err(CustomError::UnexpectedToken(
                        "Expected an unary_tp".to_string(),
                    )),
                }
            }
            Some(Token::Sub) => {
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

// -----------------------------
// --- Mult, div, md and TP1 ---
// -----------------------------

// Enums and structs for Mult, Div, Md and TP1

/// `Mult` represents the right part of a multiplication in the AST. This node is composed of a
/// [TP1] node that can chain operations of same priority.
#[derive(PartialEq)]
pub struct Mult {
    tp1: TP1,
}

/// `Div` represents the right part of a division in the AST. This node is composed of a [TP1] node
/// that can chain operations of same priority.
#[derive(PartialEq)]
pub struct Div {
    tp1: TP1,
}

/// `Md` represents either a [Mult] or a [Div] in the AST. This node is only used for grammar
/// commodity and to simplify the structure of [TP1].
#[derive(PartialEq)]
pub enum Md {
    Mult(Mult),
    Div(Div),
}

/// `TP1` is used to chain operations of same priority. This node is composed of a [UnaryTP] and an
/// optional [Md]. The [UnaryTP] is the first operand of the chain and the [Md] is the rest of the
/// chain.
///
/// Example : `5 * 5 / 2` is represented by `TP1 {5, Mult {TP1 {5, Div {TP1 {2, Empty}}}}}`. In this
/// example, details of operand values are not shown.
///
/// Like all `TP` nodes, the first operand is the operation node with a priority just over this node
/// type. Here, unary operators have the priority over multiplications and divisions.
#[derive(PartialEq)]
pub struct TP1 {
    unary_tp: UnaryTP,
    md: Option<Box<Md>>,
}

// Implementations for Mult, Div, Md and TP1 of GraphDisplay

impl GraphDisplay for Mult {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Mult_{}[Mult]", id));
        *id += 1;
        self.tp1.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Mult);

impl GraphDisplay for Div {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Div_{}[Div]", id));
        *id += 1;
        self.tp1.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Div);

impl GraphDisplay for Md {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Md_{}[Md]", id));
        *id += 1;
        match self {
            Md::Mult(mult) => {
                mult.graph_display(graph, id);
            }
            Md::Div(div) => {
                div.graph_display(graph, id);
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(Md);

impl GraphDisplay for TP1 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph TP1_{}[TP1]", id));
        *id += 1;
        self.unary_tp.graph_display(graph, id);
        if let Some(md) = &self.md {
            md.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(TP1);

// Functions for parsing Mult, Div, Md and TP1

impl Mult {
    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <mult> ::= T_MULT <tp1>
        let front = tokens.front();
        if let Some(Token::Mult) = front {
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
    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <div> ::= T_DIV <tp1>
        let front = tokens.front();
        if let Some(Token::Div) = front {
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
    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
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
    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
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

// Enums and structs for Add, Sub, As and TP2

/// `Add` represents the right part of an addition in the AST. This node is composed of a [TP2] node
/// that can chain operations of same priority.
#[derive(PartialEq)]
pub struct Add {
    tp2: TP2,
}

/// `Sub` represents the right part of a subtraction in the AST. This node is composed of a [TP2]
/// node that can chain operations of same priority.
#[derive(PartialEq)]
pub struct Sub {
    tp2: TP2,
}

/// `As` represents either an [Add] or a [Sub] in the AST. This node is only used for grammar
/// commodity and to simplify the structure of [TP2].
#[derive(PartialEq)]
pub enum As {
    Add(Add),
    Sub(Sub),
}

/// `TP2` is used to chain operations of same priority. This node works exactly like [TP1] but with
/// a lower priority. This node is composed of a [TP1] and an optional [As]. The [TP1] is the first
/// operand of the chain and the [As] is the rest of the chain.
///
/// Example : `5 + 5 - 2` is represented by `TP2 {5, Add {TP2 {5, TP2 {Sub {2, Empty}}}}}`. In this
/// example, details of operand values are not shown.
///
/// Like all `TP` nodes, the first operand is the operation node with a priority just over this node
/// type. Here, multiplications and divisions have the priority over additions and subtractions.
#[derive(PartialEq)]
pub struct TP2 {
    tp1: TP1,
    as_: Option<Box<As>>,
}

// Implementations for Add, Sub, As and TP2 of GraphDisplay

impl GraphDisplay for Add {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Add_{}[Add]", id));
        *id += 1;
        self.tp2.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Add);

impl GraphDisplay for Sub {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Sub_{}[Sub]", id));
        *id += 1;
        self.tp2.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Sub);

impl GraphDisplay for As {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph As_{}[As]", id));
        *id += 1;
        match self {
            As::Add(add) => {
                add.graph_display(graph, id);
            }
            As::Sub(sub) => {
                sub.graph_display(graph, id);
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(As);

impl GraphDisplay for TP2 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph TP2_{}[TP2]", id));
        *id += 1;
        self.tp1.graph_display(graph, id);
        if let Some(as_) = &self.as_ {
            as_.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(TP2);

// Functions for parsing Add, Sub, As and TP2

impl Add {
    fn new(tp2: TP2) -> Add {
        Add { tp2 }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Add> {
        // <add> ::= T_ADD <tp2>
        let front = tokens.front();
        if let Some(Token::Add) = front {
            tokens.pop_front();
            let tp2 = TP2::parse(tokens)?;
            match tp2 {
                Some(tp2) => Ok(Some(Add::new(tp2))),
                None => Err(CustomError::UnexpectedToken("Expected a tp2".to_string())),
            }
        } else {
            Ok(None)
        }
    }
}

impl Sub {
    fn new(tp2: TP2) -> Sub {
        Sub { tp2 }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Sub> {
        // <sub> ::= T_SUB <tp2>
        let front = tokens.front();
        if let Some(Token::Sub) = front {
            tokens.pop_front();
            let tp2 = TP2::parse(tokens)?;
            match tp2 {
                Some(tp2) => Ok(Some(Sub::new(tp2))),
                None => Err(CustomError::UnexpectedToken("Expected a tp2".to_string())),
            }
        } else {
            Ok(None)
        }
    }
}

impl As {
    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<As> {
        // <as> ::= <add> | <sub>
        if let Some(add) = Add::parse(tokens)? {
            Ok(Some(As::Add(add)))
        } else if let Some(sub) = Sub::parse(tokens)? {
            Ok(Some(As::Sub(sub)))
        } else {
            Ok(None)
        }
    }
}

impl TP2 {
    fn new(tp1: TP1, as_: Option<As>) -> TP2 {
        TP2 {
            tp1,
            as_: as_.map(Box::new),
        }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<TP2> {
        // <tp2> ::= <tp1> (<as> |)
        let tp1 = TP1::parse(tokens)?;
        match tp1 {
            Some(tp1) => {
                let as_ = As::parse(tokens)?;
                Ok(Some(TP2::new(tp1, as_)))
            }
            None => Ok(None),
        }
    }
}

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

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Eq> {
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

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<NotEq> {
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
    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<EqNot> {
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

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<TP3> {
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

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<And> {
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

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
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

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Or> {
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

    fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
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

// ----------------------------
// --- TP Last and No Value ---
// ----------------------------

// Enums and structs for TP Last and No Value

/// `TPLast` is the last `TP` node. It is composed of a [TP5] node. It is only used to avoid
/// recoding some parts if we want to add more operations in the future.
#[derive(PartialEq)]
pub struct TPLast {
    tp5: TP5,
}

/// `NoValue` is a node used to parse the right part of an operation chain, without any value at the
/// left. This is used when the left value is already parsed, and we see the operator after. This
/// node is composed of 5 optional nodes : [Md], [As], [EqNot], [And] and [Or], in the order of
/// their priority.
#[derive(PartialEq)]
pub struct NoValue {
    md: Option<Box<Md>>,
    as_: Option<Box<As>>,
    eq_not: Option<Box<EqNot>>,
    and: Option<Box<And>>,
    or: Option<Box<Or>>,
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
        if let Some(md) = &self.md {
            md.graph_display(graph, id);
        }
        if let Some(as_) = &self.as_ {
            as_.graph_display(graph, id);
        }
        if let Some(eq_not) = &self.eq_not {
            eq_not.graph_display(graph, id);
        }
        if let Some(and) = &self.and {
            and.graph_display(graph, id);
        }
        if let Some(or) = &self.or {
            or.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(NoValue);

// Functions for parsing TPLast and NoValue

impl TPLast {
    pub(crate) fn new(tp5: TP5) -> TPLast {
        TPLast { tp5 }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<TPLast> {
        // <tp_last> ::= <tp5>
        let tp5 = TP5::parse(tokens)?;
        match tp5 {
            Some(tp5) => Ok(Some(TPLast::new(tp5))),
            None => Ok(None),
        }
    }
}

impl NoValue {
    pub(crate) fn new(
        md: Option<Md>,
        as_: Option<As>,
        eq_not: Option<EqNot>,
        and: Option<And>,
        or: Option<Or>,
    ) -> NoValue {
        NoValue {
            md: md.map(Box::new),
            as_: as_.map(Box::new),
            eq_not: eq_not.map(Box::new),
            and: and.map(Box::new),
            or: or.map(Box::new),
        }
    }

    pub(crate) fn parse(tokens: &mut VecDeque<Token>) -> ShortResult<Self> {
        // <no_value> ::= (<md> |) (<as> |) (<eq_not> |) (<and> |) (<or> |)
        let md = Md::parse(tokens)?;
        let as_ = As::parse(tokens)?;
        let eq_not = EqNot::parse(tokens)?;
        let and = And::parse(tokens)?;
        let or = Or::parse(tokens)?;

        Ok(NoValue::new(md, as_, eq_not, and, or))
    }
}

// -------------------
// --- END OF FILE ---
// -------------------
