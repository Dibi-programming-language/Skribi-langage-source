use crate::ast::nodes::expressions::Expression;

/// With:
/// 1. * and /
/// 2. + and -
/// 3. <=, >=, <, >, = and !=
/// 4. &&
/// 5. ||
///
/// 0 is for unary

#[derive(PartialEq)]
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

pub struct BinaryOperation<'a> {
    binop: BinOps,
    left: Expression<'a>,
    right: Expression<'a>,
}

/// None is there only if the case of a parsing that would not need it.
pub enum UnaryOperation<'a> {
    Plus(Expression<'a>),
    Minus(Expression<'a>),
    Not(Expression<'a>),
    None(Expression<'a>),
}
