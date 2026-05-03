use crate::ast::nodes::{expressions::Expression, statements::StatementList};


/// `Ci` represents an "while" block in the AST.
/// It contains an [Exp] and a [Scope].
pub struct Ci<'a> {
    condition: Expression<'a>,
    content: StatementList<'a>,
}

