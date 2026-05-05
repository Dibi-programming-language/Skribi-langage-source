use std::fmt::Display;

use crate::{
    ast::{nodes::expressions::Expression, visitors::NodeVisitor},
    tokens::NewTokens,
};

#[derive(PartialEq, Clone)]
pub enum BinOps {
    Mul,
    Div,
    Add,
    Sub,
    Equal,
    NotEqual,
    And,
    Or,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
}

impl Display for BinOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Div => "/",
            Self::Mul => "*",
            Self::Equal => "=",
            Self::NotEqual => "!=",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::LessOrEqual => "<=",
            Self::GreaterOrEqual => ">=",
            Self::And => "&&",
            Self::Or => "||",
        })
    }
}

#[derive(PartialEq, Clone)]
pub struct BinaryOperation<'a> {
    pub binop: BinOps,
    pub left: Expression<'a>,
    pub right: Expression<'a>,
}

impl BinaryOperation<'_> {
    pub fn new<'a>(
        binop: BinOps,
        left: Expression<'a>,
        right: Expression<'a>,
    ) -> BinaryOperation<'a> {
        BinaryOperation { binop, left, right }
    }

    pub fn from<'a>(
        token: NewTokens,
        left: Expression<'a>,
        right: Expression<'a>,
    ) -> Expression<'a> {
        Expression::BinOp(Box::new(BinaryOperation {
            binop: match token {
                NewTokens::Mul => BinOps::Mul,
                NewTokens::Div => BinOps::Div,
                NewTokens::Add => BinOps::Add,
                NewTokens::Sub => BinOps::Sub,
                NewTokens::Equal => BinOps::Equal,
                NewTokens::NotEqual => BinOps::NotEqual,
                NewTokens::And => BinOps::And,
                NewTokens::Or => BinOps::Or,
                NewTokens::LessThan => BinOps::LessThan,
                NewTokens::GreaterThan => BinOps::GreaterThan,
                NewTokens::LessOrEqual => BinOps::LessOrEqual,
                NewTokens::GreaterOrEqual => BinOps::GreaterOrEqual,
                _ => unreachable!(),
            },
            left,
            right,
        }))
    }
}

/// None is there only if the case of a parsing that would not need it.
#[derive(PartialEq, Clone)]
pub enum UnaryOperation<'a> {
    Plus(Expression<'a>),
    Minus(Expression<'a>),
    Not(Expression<'a>),
    None(Expression<'a>),
}

impl UnaryOperation<'_> {
    pub fn accept<U, T: NodeVisitor<Value = U>>(&self, v: &mut T) -> U {
        match self {
            Self::Plus(b) => v.visit_plus(b),
            Self::Minus(i) => v.visit_minus(i),
            Self::Not(f) => v.visit_not(f),
            Self::None(s) => s.accept(v),
        }
    }
}
