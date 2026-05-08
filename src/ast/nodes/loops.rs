use crate::ast::nodes::{expressions::Expression, statements::StatementList};

/// Represents an "while" block in the AST.
/// It contains an [Exp] and a [Scope].
pub struct While<'a> {
    condition: Expression<'a>,
    content: StatementList<'a>,
}
