use std::fmt::{Display, Error, Formatter};

use crate::ast::{nodes::FileTreeRoot, visitors::AstMutVisitor};

struct PrettyPrinterVisitor<'fmt_ref, 'fmt_object> {
    f: &'fmt_ref mut Formatter<'fmt_object>,
    indent: usize,
}

impl Display for FileTreeRoot<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut printer = PrettyPrinterVisitor {
            f,
            indent: 0,
        };
        printer.visit_file_tree_root(self)
    }
}

macro_rules! write_self_indent {
    ($self: ident, $content: expr) => {
        write!($self.f, "{:-<1$}", $content, $self.indent)
    };
}

macro_rules! write_self {
    ($self: ident $(, $content: expr)*) => {
        write!($self.f $(, $content)*)
    };
}

impl AstMutVisitor<'_, (), Error> for PrettyPrinterVisitor<'_, '_> {
    fn default_t(_: super::DefaultCause) -> miette::Result<(),Error> {
        Ok(())
    }

    fn visit_statement(&mut self,statement: &crate::ast::nodes::statements::Statement<'_>,) -> miette::Result<(),Error> {
        self.default_statement(statement)?;
        write_self_indent!(self, "\n")
    }

    fn visit_expression(&mut self,expression: &crate::ast::nodes::expressions::Expression<'_>,) -> miette::Result<(),Error> {
        self.indent += 4;
        write_self_indent!(self, "(\n")?;
        self.default_expression(expression)?;
        self.indent -= 4;
        write_self_indent!(self, "\n")?;
        write_self!(self, ")")
    }

    fn visit_deprecated(&mut self,deprecated: &crate::ast::nodes::deprecated::Deprecated,) -> miette::Result<(),Error> {
        self.default_deprecated(deprecated)?;
        write_self!(self, "DEPRECATED [{}]", deprecated.message)
    }

    fn visit_function_call(&mut self,function_call: &crate::ast::nodes::calls::functions::FunctionCall<'_>,) -> miette::Result<(),Error> {
        self.default_function_call(function_call)?;
        write_self!(self, "{}()", function_call.name)
    }
}
