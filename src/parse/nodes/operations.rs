use crate::impl_debug;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::CustomError;
use crate::tokens::Token;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Formatter};

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
                    "\nsubgraph_ValueBase_{}[ValueBase Bool {}]\nend",
                    id, value
                ));
            }
            ValueBase::Int(value) => {
                graph.push_str(&format!(
                    "\nsubgraph_ValueBase_{}[ValueBase Int {}]\nend",
                    id, value
                ));
            }
            ValueBase::Float(value) => {
                graph.push_str(&format!(
                    "\nsubgraph_ValueBase_{}[ValueBase Float {}]\nend",
                    id, value
                ));
            }
            ValueBase::String(value) => {
                graph.push_str(&format!(
                    "\nsubgraph_ValueBase_{}[ValueBase String {}]\nend",
                    id, value
                ));
            }
        }
        *id += 1;
    }
}

impl_debug!(ValueBase);

fn parse_value_base(tokens: &mut VecDeque<Token>) -> Option<ValueBase> {
    // <value_base> ::= T_BOOL | T_INT | T_STRING | T_FLOAT
    match tokens.pop_front() {
        Some(Token::Bool(value)) => Some(ValueBase::Bool(value)),
        Some(Token::Int(value)) => Some(ValueBase::Int(value)),
        Some(Token::String(value)) => Some(ValueBase::String(value)),
        Some(Token::Float(value)) => Some(ValueBase::Float(value)),
        va => {
            tokens.push_front(va.unwrap());
            None
        }
    }
}

// -------------
// --- Value ---
// -------------

#[derive(PartialEq)]
pub enum ValueNode {
    ValueBase(ValueBase),
    ExpBase, // TODO
}

impl GraphDisplay for ValueNode {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_ValueNode_{}[ValueNode]", id));
        *id += 1;
        match self {
            ValueNode::ValueBase(value) => {
                value.graph_display(graph, id);
            }
            ValueNode::ExpBase => {
                // TODO
            }
        }
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(ValueNode);

fn parse_value(tokens: &mut VecDeque<Token>) -> Option<Result<ValueNode, CustomError>> {
    // <value> ::=
    //   | <value_base>
    //   | <exp_base>
    if let Some(value_base) = parse_value_base(tokens) {
        Some(Ok(ValueNode::ValueBase(value_base)))
    } else {
        // TODO
        None
    }
}

// ----------------
// --- TakePrio ---
// ----------------

#[derive(PartialEq)]
pub enum TakePriority {
    Exp, // TODO
    Value(ValueNode),
}

impl GraphDisplay for TakePriority {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_TakePriority_{}[TakePriority]", id));
        *id += 1;
        match self {
            TakePriority::Exp => {
                // TODO
            }
            TakePriority::Value(value) => {
                value.graph_display(graph, id);
            }
        }
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(TakePriority);

fn parse_take_prio(tokens: &mut VecDeque<Token>) -> Option<Result<TakePriority, CustomError>> {
    // <take_prio> ::=
    //   T_LEFT_P <exp> T_RIGHT_P
    //   | <value>
    let front = tokens.front();
    if let Some(Token::LeftParenthesis) = front {
        tokens.pop_front();
        // TODO
        None
    } else {
        if let Some(value) = parse_value(tokens) {
            match value {
                Ok(value) => Some(Ok(TakePriority::Value(value))),
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    }
}

// ----------------
// --- Unary TP ---
// ----------------

#[derive(PartialEq)]
pub enum UnaryTP {
    Plus(Box<UnaryTP>),
    Minus(Box<UnaryTP>),
    Not(Box<UnaryTP>),
    TakePriority(TakePriority),
}

impl GraphDisplay for UnaryTP {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_UnaryTP_{}[unary_tp]", id));
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
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(UnaryTP);

fn parse_unary_tp(tokens: &mut VecDeque<Token>) -> Option<Result<UnaryTP, CustomError>> {
    // <tp> ::=
    //   (T_PLUS | T_MINUS | T_NOT) <tp>
    //   | <take_prio>
    let front = tokens.front();
    match front {
        Some(Token::Add) => {
            tokens.pop_front();
            let unary_tp = parse_unary_tp(tokens);
            match unary_tp {
                Some(Ok(unary_tp)) => Some(Ok(UnaryTP::Plus(Box::new(unary_tp)))),
                Some(Err(err)) => Some(Err(err)),
                None => None,
            }
        }
        Some(Token::Sub) => {
            tokens.pop_front();
            let unary_tp = parse_unary_tp(tokens);
            match unary_tp {
                Some(Ok(unary_tp)) => Some(Ok(UnaryTP::Minus(Box::new(unary_tp)))),
                Some(Err(err)) => Some(Err(err)),
                None => None,
            }
        }
        // TODO not
        _ => {
            let take_priority = parse_take_prio(tokens);
            match take_priority {
                Some(Ok(take_priority)) => Some(Ok(UnaryTP::TakePriority(take_priority))),
                Some(Err(err)) => Some(Err(err)),
                None => None,
            }
        }
    }
}

// -----------------------------
// --- Mult, div, md and TP1 ---
// -----------------------------

// Enums and structs for Mult, Div, Md and TP1

#[derive(PartialEq)]
pub struct Mult {
    tp1: TP1,
}

#[derive(PartialEq)]
pub struct Div {
    tp1: TP1,
}

#[derive(PartialEq)]
pub enum Md {
    Mult(Mult),
    Div(Div),
}

#[derive(PartialEq)]
pub struct TP1 {
    unary_tp: UnaryTP,
    md: Option<Box<Md>>,
}

// Implementations for Mult, Div, Md and TP1 of GraphDisplay

impl GraphDisplay for Mult {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_Mult_{}[Mult]", id));
        *id += 1;
        self.tp1.graph_display(graph, id);
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(Mult);

impl GraphDisplay for Div {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_Div_{}[Div]", id));
        *id += 1;
        self.tp1.graph_display(graph, id);
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(Div);

impl GraphDisplay for Md {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_Md_{}[Md]", id));
        *id += 1;
        match self {
            Md::Mult(mult) => {
                mult.graph_display(graph, id);
            }
            Md::Div(div) => {
                div.graph_display(graph, id);
            }
        }
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(Md);

impl GraphDisplay for TP1 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_TP1_{}[TP1]", id));
        *id += 1;
        self.unary_tp.graph_display(graph, id);
        if let Some(md) = &self.md {
            md.graph_display(graph, id);
        }
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(TP1);

// Functions for parsing Mult, Div, Md and TP1

fn parse_mult(tokens: &mut VecDeque<Token>) -> Option<Result<Mult, CustomError>> {
    // <mult> ::= T_MULT <tp1>
    let front = tokens.front();
    if let Some(Token::Mult) = front {
        tokens.pop_front();
        let tp1 = parse_tp1(tokens);
        match tp1 {
            Some(Ok(tp1)) => Some(Ok(Mult { tp1 })),
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    } else {
        None
    }
}

fn parse_div(tokens: &mut VecDeque<Token>) -> Option<Result<Div, CustomError>> {
    // <div> ::= T_DIV <tp1>
    let front = tokens.front();
    if let Some(Token::Div) = front {
        tokens.pop_front();
        let tp1 = parse_tp1(tokens);
        match tp1 {
            Some(Ok(tp1)) => Some(Ok(Div { tp1 })),
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    } else {
        None
    }
}

fn parse_md(tokens: &mut VecDeque<Token>) -> Option<Result<Md, CustomError>> {
    // <md> ::= <mult> | <div>
    if let Some(mult) = parse_mult(tokens) {
        match mult {
            Ok(mult) => Some(Ok(Md::Mult(mult))),
            Err(err) => Some(Err(err)),
        }
    } else if let Some(div) = parse_div(tokens) {
        match div {
            Ok(div) => Some(Ok(Md::Div(div))),
            Err(err) => Some(Err(err)),
        }
    } else {
        None
    }
}

fn parse_tp1(tokens: &mut VecDeque<Token>) -> Option<Result<TP1, CustomError>> {
    // <tp1> ::= <tp> (<md> |)
    let unary_tp = parse_unary_tp(tokens);
    match unary_tp {
        Some(Ok(unary_tp)) => {
            let md = parse_md(tokens);
            match md {
                Some(Ok(md)) => Some(Ok(TP1 {
                    unary_tp,
                    md: Some(Box::new(md)),
                })),
                Some(Err(err)) => Some(Err(err)),
                None => Some(Ok(TP1 { unary_tp, md: None })),
            }
        }
        Some(Err(err)) => Some(Err(err)),
        None => None,
    }
}

// ----------------------------
// --- Add, sub, as and TP2 ---
// ----------------------------

// Enums and structs for Add, Sub, As and TP2

#[derive(PartialEq)]
pub struct Add {
    tp2: TP2,
}

#[derive(PartialEq)]
pub struct Sub {
    tp2: TP2,
}

#[derive(PartialEq)]
pub enum As {
    Add(Add),
    Sub(Sub),
}

#[derive(PartialEq)]
pub struct TP2 {
    tp1: TP1,
    as_: Option<Box<As>>,
}

// Implementations for Add, Sub, As and TP2 of GraphDisplay

impl GraphDisplay for Add {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_Add_{}[Add]", id));
        *id += 1;
        self.tp2.graph_display(graph, id);
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(Add);

impl GraphDisplay for Sub {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_Sub_{}[Sub]", id));
        *id += 1;
        self.tp2.graph_display(graph, id);
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(Sub);

impl GraphDisplay for As {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_As_{}[As]", id));
        *id += 1;
        match self {
            As::Add(add) => {
                add.graph_display(graph, id);
            }
            As::Sub(sub) => {
                sub.graph_display(graph, id);
            }
        }
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(As);

impl GraphDisplay for TP2 {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_TP2_{}[TP2]", id));
        *id += 1;
        self.tp1.graph_display(graph, id);
        if let Some(as_) = &self.as_ {
            as_.graph_display(graph, id);
        }
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(TP2);

// Functions for parsing Add, Sub, As and TP2

impl Add {
    fn new(tp2: TP2) -> Add {
        Add { tp2 }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> Option<Result<Add, CustomError>> {
        // <add> ::= T_ADD <tp2>
        let front = tokens.front();
        if let Some(Token::Add) = front {
            tokens.pop_front();
            let tp2 = TP2::parse(tokens);
            match tp2 {
                Some(Ok(tp2)) => Some(Ok(Add::new(tp2))),
                Some(Err(err)) => Some(Err(err)),
                None => None,
            }
        } else {
            None
        }
    }
}

impl Sub {
    fn new(tp2: TP2) -> Sub {
        Sub { tp2 }
    }

    fn parse(tokens: &mut VecDeque<Token>) -> Option<Result<Sub, CustomError>> {
        // <sub> ::= T_SUB <tp2>
        let front = tokens.front();
        if let Some(Token::Sub) = front {
            tokens.pop_front();
            let tp2 = TP2::parse(tokens);
            match tp2 {
                Some(Ok(tp2)) => Some(Ok(Sub::new(tp2))),
                Some(Err(err)) => Some(Err(err)),
                None => None,
            }
        } else {
            None
        }
    }
}

impl As {
    fn parse(tokens: &mut VecDeque<Token>) -> Option<Result<As, CustomError>> {
        // <as> ::= <add> | <sub>
        if let Some(add) = Add::parse(tokens) {
            match add {
                Ok(add) => Some(Ok(As::Add(add))),
                Err(err) => Some(Err(err)),
            }
        } else if let Some(sub) = Sub::parse(tokens) {
            match sub {
                Ok(sub) => Some(Ok(As::Sub(sub))),
                Err(err) => Some(Err(err)),
            }
        } else {
            None
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

    fn parse(tokens: &mut VecDeque<Token>) -> Option<Result<TP2, CustomError>> {
        // <tp2> ::= <tp1> (<as> |)
        let tp1 = parse_tp1(tokens);
        match tp1 {
            Some(Ok(tp1)) => {
                let as_ = As::parse(tokens);
                match as_ {
                    Some(Ok(as_)) => Some(Ok(TP2::new(tp1, Some(as_)))),
                    Some(Err(err)) => Some(Err(err)),
                    None => Some(Ok(TP2::new(tp1, None))),
                }
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}
