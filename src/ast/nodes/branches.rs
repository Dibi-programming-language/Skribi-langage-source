// Legacy grammar for this file:
// <sula> ::= sula (<ij> (<sula> |) | <scope>)
// <ij> ::= ij <exp> <scope>
// <cond> ::= <ij> (<sula> |)

use crate::ast::nodes::{expressions::Expression, statements::StatementList};

/// The "else" and "else if" parts of a condition
pub enum Sula<'a> {
    NestedBranch(Branch<'a>),
    AllFalseCase(StatementList<'a>),
}

/// `Branch` is the starting node for an "if" block in the AST.
pub struct Branch<'a> {
    condition: Expression<'a>,
    true_case: StatementList<'a>,
    false_case: Option<Box<Sula<'a>>>,
}
