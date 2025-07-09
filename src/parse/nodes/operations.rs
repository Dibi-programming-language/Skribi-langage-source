use crate::execute::{
    Evaluate, EvaluateFromInput, IntType, OperationContext, OperationI, OperationO,
};
use crate::parse::nodes::expressions::{Exp, ExpBase};
use crate::parse::nodes::operations::Operations::{Add, Div, Equal, Mul, NotEqual, Sub};
use crate::parse::nodes::{GraphDisplay, Parsable, ParsableWithLevel};
use crate::skr_errors::CustomError::UnexpectedToken;
use crate::skr_errors::{CustomError, ResultOption};
use crate::tokens::{Token, TokenContainer};
use crate::{impl_debug, some_token};
use std::collections::VecDeque;
// This file is pretty long
// Start of grammar for this file :
// ```
// <value_base> ::= T_BOOL | T_INT | T_STRING | T_FLOAT
// <value> ::=
//   <value_base>
//   | <exp_base>
// <take_prio> ::=
//   T_LEFT_P <exp> T_RIGHT_P
//   | <value>
// ```

// -----------------
// --- ValueBase ---
// -----------------

/// `ValueBase` represents the base of a value in the AST. This is the smallest unit of a value.
/// This node is not dependent on any other node. The value can be a boolean, an integer, a float or
/// a string.
#[derive(PartialEq)]
pub enum ValueBase {
    Bool(bool),
    Int(IntType),
    Float(f32),
    String(String),
}

impl GraphDisplay for ValueBase {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        match self {
            ValueBase::Bool(value) => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph ValueBase_{}[ValueBase Bool {}]\n{:indent$}end",
                    "",
                    id,
                    value,
                    "",
                    indent = indent
                ));
            }
            ValueBase::Int(value) => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph ValueBase_{}[ValueBase Int {}]\n{:indent$}end",
                    "",
                    id,
                    value,
                    "",
                    indent = indent
                ));
            }
            ValueBase::Float(value) => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph ValueBase_{}[ValueBase Float {}]\n{:indent$}end",
                    "",
                    id,
                    value,
                    "",
                    indent = indent
                ));
            }
            ValueBase::String(value) => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph ValueBase_{}[ValueBase String {}]\n{:indent$}end",
                    "",
                    id,
                    value,
                    "",
                    indent = indent
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
            some_token!(Token::Bool(_)) => {
                if let some_token!(Token::Bool(value)) = tokens.pop_front() {
                    Some(Self::Bool(value))
                } else {
                    None
                }
            }
            some_token!(Token::Int(_)) => {
                if let some_token!(Token::Int(value)) = tokens.pop_front() {
                    Some(Self::Int(value))
                } else {
                    None
                }
            }
            some_token!(Token::Float(_)) => {
                if let some_token!(Token::Float(value)) = tokens.pop_front() {
                    Some(Self::Float(value))
                } else {
                    None
                }
            }
            some_token!(Token::String(_)) => {
                if let some_token!(Token::String(value)) = tokens.pop_front() {
                    Some(Self::String(value))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Evaluate for ValueBase {
    fn evaluate(&self, _operation_context: &mut OperationContext) -> OperationO {
        match self {
            ValueBase::Int(value) => Ok(*value),
            _ => todo!(),
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph ValueNode_{}[ValueNode]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        match self {
            ValueNode::ValueBase(value) => {
                value.graph_display(graph, id, indent + 2);
            }
            ValueNode::ExpBase(value) => {
                value.graph_display(graph, id, indent + 2);
            }
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
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

impl Evaluate for ValueNode {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            ValueNode::ValueBase(base) => base.evaluate(operation_context),
            ValueNode::ExpBase(exp_base) => exp_base.evaluate(operation_context),
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph TakePriority_{}[TakePriority]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        match self {
            TakePriority::Exp(value) => {
                value.graph_display(graph, id, indent + 2);
            }
            TakePriority::Value(value) => {
                value.graph_display(graph, id, indent + 2);
            }
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
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
                        Err(UnexpectedToken("Expected a right parenthesis".to_string()))
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

impl Evaluate for TakePriority {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            TakePriority::Exp(exp) => exp.evaluate(operation_context),
            TakePriority::Value(value) => value.evaluate(operation_context),
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph UnaryTP_{}[unary_tp]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        match self {
            UnaryTP::Plus(unary_tp) | UnaryTP::Minus(unary_tp) | UnaryTP::Not(unary_tp) => {
                unary_tp.graph_display(graph, id, indent + 2);
            }
            UnaryTP::TakePriority(take_priority) => {
                take_priority.graph_display(graph, id, indent + 2);
            }
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(UnaryTP);

macro_rules! extract_unary {
    ($ret:path, $tokens: ident) => {{
        $tokens.pop_front();
        let unary_tp = UnaryTP::parse($tokens)?;
        match unary_tp {
            Some(unary_tp) => Ok(Some($ret(Box::new(unary_tp)))),
            None => Err(CustomError::UnexpectedToken(
                "Expected an unary_tp".to_string(),
            )),
        }
    }};
}

impl Parsable for UnaryTP {
    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <tp> ::=
        //   (T_PLUS | T_MINUS | T_NOT) <tp>
        //   | <take_prio>
        let front = tokens.front();
        match front {
            some_token!(Token::Add) => extract_unary!(UnaryTP::Plus, tokens),
            some_token!(Token::Sub) => extract_unary!(UnaryTP::Minus, tokens),
            some_token!(Token::Not) => extract_unary!(UnaryTP::Not, tokens),
            _ => {
                if let Some(take_priority) = TakePriority::parse(tokens)? {
                    Ok(Some(UnaryTP::TakePriority(take_priority)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl Evaluate for UnaryTP {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            UnaryTP::Plus(unary_tp) => unary_tp.evaluate(operation_context),
            UnaryTP::TakePriority(take_priority) => take_priority.evaluate(operation_context),
            UnaryTP::Minus(minus) => minus.evaluate(operation_context).map(|x| -x),
            _ => todo!(),
        }
    }
}

#[derive(PartialEq)]
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

const HIGHEST_LEVEL: u8 = 5;
const LOWEST_LEVEL: u8 = 1;

/// With:
/// 1. * and /
/// 2. + and -
/// 3. = and !=
/// 4. &&
/// 5. ||
///
/// 0 is for unary
impl Token {
    pub fn get_level(&self) -> Option<u8> {
        match self {
            Token::Mul => Some(1),
            Token::Div => Some(1),
            Token::Add => Some(2),
            Token::Sub => Some(2),
            Token::Equal => Some(3),
            Token::NotEqual => Some(3),
            Token::And => Some(4),
            Token::Or => Some(5),
            _ => None,
        }
    }

    /// [Token::get_level] should be called before
    pub fn get_operation(&self) -> Operations {
        match self {
            Token::Mul => Mul,
            Token::Div => Div,
            Token::Add => Add,
            Token::Sub => Sub,
            Token::Equal => Equal,
            Token::NotEqual => NotEqual,
            Token::And => Operations::And,
            Token::Or => Operations::Or,
            _ => panic!("Unexpected token found"),
        }
    }
}

/// Grammar for [OperationN]
/// ```grammar
/// <op n> ::= T_OPERATIONS_N <tp n>
/// ```
/// See also [TakePriorityN] and [Operations]
#[derive(PartialEq)]
pub struct OperationN {
    level: u8,
    operation: Operations,
    tp_nm1: Box<TakePriorityN>,
}

impl ParsableWithLevel for OperationN {
    fn parse(tokens: &mut VecDeque<TokenContainer>, level: u8) -> ResultOption<Self> {
        if let Some(container) = tokens.front() {
            if let Some(level_token) = container.token.get_level() {
                if level_token != level {
                    return Ok(None);
                }
                let operation = tokens.pop_front().unwrap().token.get_operation();
                if let Some(tp_nm1) = TakePriorityN::parse(tokens, level)? {
                    Ok(Some(Self {
                        level,
                        operation,
                        tp_nm1: Box::new(tp_nm1),
                    }))
                } else {
                    Err(UnexpectedToken(String::from(
                        "Missing TakePriorityN with level",
                    )))
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl EvaluateFromInput for OperationN {
    fn evaluate_from_input(
        &self,
        operation_context: &mut OperationContext,
        input: OperationI,
    ) -> OperationO {
        Ok(match self.operation {
            Add => input + self.tp_nm1.evaluate(operation_context)?,
            Sub => input - self.tp_nm1.evaluate(operation_context)?,
            Div => input / self.tp_nm1.evaluate(operation_context)?,
            Mul => input * self.tp_nm1.evaluate(operation_context)?,
            _ => todo!(),
        })
    }
}

/// Grammar for [TakePriorityN]
/// ```grammar
/// <tp0> ::= <unary_tp> | <take_prio>
/// <tp n> ::= <tp n-1> (<op n> |)
/// ```
#[derive(PartialEq)]
pub enum TakePriorityN {
    ElementUnary0(Box<UnaryTP>),
    ElementSimple0(Box<TakePriority>),
    ElementN {
        level: u8,
        tp_nm1: Box<TakePriorityN>,
        op_n: Option<Box<OperationN>>,
    },
}

impl ParsableWithLevel for TakePriorityN {
    fn parse(tokens: &mut VecDeque<TokenContainer>, level: u8) -> ResultOption<Self> {
        if level + 1 == LOWEST_LEVEL {
            if let Some(unary) = UnaryTP::parse(tokens)? {
                Ok(Some(Self::ElementUnary0(Box::new(unary))))
            } else if let Some(take_priority) = TakePriority::parse(tokens)? {
                Ok(Some(Self::ElementSimple0(Box::new(take_priority))))
            } else {
                Ok(None)
            }
        } else if let Some(take_priority_nm1) = TakePriorityN::parse(tokens, level - 1)? {
            let optional = OperationN::parse(tokens, level)?;
            match optional {
                Some(op_n) => Ok(Some(Self::ElementN {
                    level,
                    tp_nm1: Box::new(take_priority_nm1),
                    op_n: Some(Box::new(op_n)),
                })),
                None => Ok(Some(take_priority_nm1)),
            }
        } else {
            Ok(None)
        }
    }
}

impl Evaluate for TakePriorityN {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            TakePriorityN::ElementUnary0(unary) => unary.evaluate(operation_context),
            TakePriorityN::ElementSimple0(take_priority) => {
                take_priority.evaluate(operation_context)
            }
            TakePriorityN::ElementN {
                level: _,
                tp_nm1,
                op_n: Some(op),
            } => {
                let res = tp_nm1.evaluate(operation_context)?;
                op.evaluate_from_input(operation_context, res)
            }
            TakePriorityN::ElementN {
                level: _,
                tp_nm1,
                op_n: _,
            } => tp_nm1.evaluate(operation_context),
        }
    }
}

/// Level is always the higher level.
/// ```grammar
/// <tp_last> ::= <tp max>
/// ```
/// See [HIGHEST_LEVEL] and [TakePriorityN]
#[derive(PartialEq)]
pub struct TakePriorityLast {
    child: TakePriorityN,
}

impl Parsable for TakePriorityLast {
    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        if let Some(child) = TakePriorityN::parse(tokens, HIGHEST_LEVEL)? {
            Ok(Some(Self { child }))
        } else {
            Ok(None)
        }
    }
}

impl Evaluate for TakePriorityLast {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        self.child.evaluate(operation_context)
    }
}

/// Grammar for [NoValueN]
/// ```grammar
/// <nv0> ::= <op max>
/// <nv n> ::= <op max-n> (<nv n-1> |) | <nv n-1>
/// <no_value> ::= <nv max>
/// ```
///
/// `NoValue` is a node used to parse the right part of an operation chain,
/// without any value at the left.
/// This is used when the left value is already parsed,
/// and we see the operator after.
/// At least one operator is expected in this node.
///
#[derive(PartialEq)]
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

impl ParsableWithLevel for NoValueN {
    fn parse(tokens: &mut VecDeque<TokenContainer>, level: u8) -> ResultOption<Self> {
        if level == LOWEST_LEVEL {
            if let Some(operation) = OperationN::parse(tokens, HIGHEST_LEVEL)? {
                Ok(Some(Self::Element0(Box::new(operation))))
            } else {
                Ok(None)
            }
        } else if let Some(operation) = OperationN::parse(tokens, HIGHEST_LEVEL - level)? {
            let nv_mn1 = <NoValueN as ParsableWithLevel>::parse(tokens, level - 1)?.map(Box::new);
            Ok(Some(Self::ElementOperationN {
                level,
                operation: Box::new(operation),
                no_value_before: nv_mn1,
            }))
        } else if let Some(no_value_before) =
            <NoValueN as ParsableWithLevel>::parse(tokens, level - 1)?
        {
            Ok(Some(Self::ElementSimpleN {
                level,
                no_value_before: Box::new(no_value_before),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Parsable for NoValueN {
    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self>
    where
        Self: Sized,
    {
        <NoValueN as ParsableWithLevel>::parse(tokens, HIGHEST_LEVEL)
    }
}

impl EvaluateFromInput for NoValueN {
    fn evaluate_from_input(
        &self,
        operation_context: &mut OperationContext,
        input: OperationI,
    ) -> OperationO {
        match self {
            NoValueN::Element0(op) => op.evaluate_from_input(operation_context, input),
            NoValueN::ElementSimpleN {
                level: _,
                no_value_before,
            } => no_value_before.evaluate_from_input(operation_context, input),
            NoValueN::ElementOperationN {
                level: _,
                operation,
                no_value_before: Some(value_before),
            } => {
                let res = operation.evaluate_from_input(operation_context, input)?;
                value_before.evaluate_from_input(operation_context, res)
            }
            NoValueN::ElementOperationN {
                level: _,
                operation,
                no_value_before: None,
            } => operation.evaluate_from_input(operation_context, input),
        }
    }
}

impl_debug!(Operations);
impl_debug!(OperationN);
impl_debug!(TakePriorityLast);
impl_debug!(NoValueN);
impl_debug!(TakePriorityN);

impl GraphDisplay for Operations {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph Operation_{}[Op {}]",
            "",
            id,
            match self {
                Mul => "OP *",
                Div => "OP /",
                Add => "OP +",
                Sub => "OP -",
                Equal => "CO =",
                NotEqual => "CO !=",
                Operations::And => "LG &&",
                Operations::Or => "LG ||",
            },
            indent = indent
        ));
        *id += 1;
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl GraphDisplay for OperationN {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph OperationN_{}[OP N={}]",
            "",
            id,
            self.level,
            indent = indent
        ));
        *id += 1;
        self.operation.graph_display(graph, id, indent + 2);
        self.tp_nm1.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl GraphDisplay for TakePriorityN {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        match self {
            TakePriorityN::ElementUnary0(unary) => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph TakePriorityN_{}[TP0 UNARY]",
                    "",
                    id,
                    indent = indent
                ));
                *id += 1;
                unary.graph_display(graph, id, indent + 2);
                graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
            }
            TakePriorityN::ElementSimple0(simple) => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph TakePriorityN_{}[TP0 SIMPLE]",
                    "",
                    id,
                    indent = indent
                ));
                *id += 1;
                simple.graph_display(graph, id, indent + 2);
                graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
            }
            TakePriorityN::ElementN {
                level,
                tp_nm1: tp_n1,
                op_n,
            } => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph TakePriorityN_{}[TP N={}]",
                    "",
                    id,
                    level,
                    indent = indent
                ));
                *id += 1;
                tp_n1.graph_display(graph, id, indent + 2);
                if let Some(op_n) = op_n {
                    op_n.graph_display(graph, id, indent + 2);
                }
                graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
            }
        }
    }
}

impl GraphDisplay for TakePriorityLast {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph TakePriorityLast_{}[TP_LAST]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        self.child.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl GraphDisplay for NoValueN {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        match self {
            NoValueN::Element0(op) => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph NoValueN_{}[NoValue0]",
                    "",
                    id,
                    indent = indent
                ));
                *id += 1;
                op.graph_display(graph, id, indent + 2);
                graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
            }
            NoValueN::ElementOperationN {
                level,
                operation,
                no_value_before,
            } => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph NoValueN_{}[NoValueOp N={}]",
                    "",
                    id,
                    level,
                    indent = indent
                ));
                *id += 1;
                operation.graph_display(graph, id, indent + 2);
                if let Some(no_value_before) = no_value_before {
                    no_value_before.graph_display(graph, id, indent + 2);
                }
                graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
            }
            NoValueN::ElementSimpleN {
                level,
                no_value_before,
            } => {
                graph.push_str(&format!(
                    "\n{:indent$}subgraph NoValueN_{}[NoValueSimple N={}]",
                    "",
                    id,
                    level,
                    indent = indent
                ));
                *id += 1;
                no_value_before.graph_display(graph, id, indent + 2);
                graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
            }
        }
    }
}

// ------------------------------------
// --- Some old comments - archives ---
// ------------------------------------

// Functions for parsing Mult, Div, Md and TP1

// <mult> ::= T_MULT <tp1>
// <div> ::= T_DIV <tp1>
// <md> ::= <mult> | <div>
// <tp1> ::= <tp> (<md> |)
// <eq> ::= T_EQUAL <tp3>
// <not_eq> ::= T_NOT_EQUAL <tp3>
// <eq_not> ::= <eq> | <not_eq>
// <tp3> ::= <tp2> (<eq_not> |)
// <tp4> ::= <tp3> (<and> |)
// <or> ::= T_OR <tp5>
// <nv0> ::= <and> (<or> |) | <or>
// <nv1> ::= <eq_not> (<nv0> |) | <nv0>
// <nv3> ::= <md> (<nv2> |) | <nv2>
// <tp_last> ::= <tp5>

// `NoValue` is a node used to parse the right part of an operation chain, without any value at the
// left. This is used when the left value is already parsed, and we see the operator after. At
// least one operator is expected in this node.

// -------------------
// --- END OF FILE ---
// -------------------
