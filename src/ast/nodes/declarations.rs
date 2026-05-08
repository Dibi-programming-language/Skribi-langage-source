// Grammar of this file :
// <fct_dec> ::= ums T_IDENTIFIER <tuple> <scope>

// --------------
// --- FctDec ---
// --------------

use crate::ast::nodes::{expressions::Expression, statements::StatementList};

/// `FunctionDeclaration` represents a function declaration. It contains the
/// identifier of the function, the tuple of arguments and the scope of the
/// function.
///
/// Return types will be added later.
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
    variable_type: &'a str,
    variable_name: &'a str,
    content: Expression<'a>,
    private: bool,
    global: bool,
    constant: bool,
}
