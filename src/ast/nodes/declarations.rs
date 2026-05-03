// Grammar of this file :
// <fct_dec> ::= ums T_IDENTIFIER <tuple> <scope>

// --------------
// --- FctDec ---
// --------------

use crate::ast::nodes::{expressions::Expression, statements::StatementList};

/// `FctDec` represents a function declaration. It contains the identifier of
/// the function, the tuple of arguments and the scope of the function.
///
/// # Grammar
///
/// `<fct_dec> ::= ums T_IDENTIFIER <tuple> <scope>`
///
/// See also [TupleNode] and [Scope].
pub struct FunctionDeclaration<'a> {
    identifier: &'a str,
    args: Vec<Expression<'a>>,
    scope: StatementList<'a>,
}

pub struct VariableDeclaration<'a> {
    var_type: &'a str,
    identifier: &'a str,
    content: Expression<'a>,
    private: bool,
    global: bool,
    constant: bool,
}
