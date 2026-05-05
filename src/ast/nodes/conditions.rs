// Grammar for this file:
// <sula> ::= sula (<ij> (<sula> |) | <scope>)
// <ij> ::= ij <exp> <scope>
// <cond> ::= <ij> (<sula> |)

use crate::ast::nodes::{expressions::Expression, statements::StatementList};

/// The "else" and "else if" parts of a condition
#[derive(PartialEq, Clone)]
pub enum Sula<'a> {
    Condition(Condition<'a>),
    Scope(StatementList<'a>),
}

/// `Condition` is the starting node for an "if" block in the AST.
#[derive(PartialEq, Clone)]
pub struct Condition<'a> {
    condition: Expression<'a>,
    positive: StatementList<'a>,
    negative: Option<Box<Sula<'a>>>,
}
