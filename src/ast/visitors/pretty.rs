use std::fmt::{Display};

use crate::{
    ast::{
        nodes::{
            AstRoot,
            calls::{FunctionCall, IdentifierChain, VariableModification},
            conditions::{Condition, Sula},
            declarations::{FunctionDeclaration, VariableDeclaration},
            expressions::Expression,
            loops::Ci,
            operations::BinaryOperation,
            statements::{Return, StatementList},
        },
        visitors::NodeVisitor,
    },
    execute::IntType,
};

pub struct Pretty<'a> {
    root: &'a AstRoot<'a>,
}

struct Printer<'a, 'b> {
    f: &'a mut std::fmt::Formatter<'b>,
}

impl Display for Pretty<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("AST:\n")?;
        let mut printer = Printer { f };
        printer.visit_root(self.root)
    }
}

impl Pretty<'_> {
    pub fn eprint(root: &AstRoot) {
        eprint!("{}", Pretty { root })
    }
}

impl NodeVisitor for Printer<'_, '_> {
    type Value = std::fmt::Result;

    fn visit_root(&mut self, v: &AstRoot) -> Self::Value {
        for sta in &v.content {
            sta.accept(self)?;
            self.f.write_str("\n")?;
        }
        self.f.write_str("\n")
    }

    fn visit_return(&mut self, _v: &Return) -> Self::Value {
        todo!()
    }

    fn visit_statements(&mut self, v: &StatementList) -> Self::Value {
        if v.bubble {
            self.f.write_str("biuli ")?;
        }
        if v.unused {
            self.f.write_str("spoki ")?;
        }
        if v.simple {
            self.f.write_str("kodi ")?;
        }
        self.f.write_str("{\n")?;
        for sta in &v.statements {
            sta.accept(self)?;
            self.f.write_str("\n")?;
        }
        self.f.write_str("}\n")
    }

    fn visit_binary(&mut self, v: &BinaryOperation) -> Self::Value {
        self.f.write_str("(")?;
        v.left.accept(self)?;
        write!(self.f, " {} ", v.binop)?;
        v.right.accept(self)?;
        self.f.write_str(")")
    }

    fn visit_not(&mut self, _v: &Expression) -> Self::Value {
        self.f.write_str("")
    }

    fn visit_plus(&mut self, _v: &Expression) -> Self::Value {
        self.f.write_str("")
    }

    fn visit_minus(&mut self, _v: &Expression) -> Self::Value {
        self.f.write_str("")
    }

    fn visit_ci(&mut self, _v: &Ci) -> Self::Value {
        self.f.write_str("")
    }

    fn visit_function_dec(&mut self, _v: &FunctionDeclaration) -> Self::Value {
        self.f.write_str("")
    }

    fn visit_variable_dec(&mut self, v: &VariableDeclaration) -> Self::Value {
        if v.private {
            self.f.write_str("pu ")?;
        }
        if v.constant {
            self.f.write_str("ju ")?;
        }
        if v.global {
            self.f.write_str("fu ")?;
        }
        write!(self.f, ".{} {} ", v.var_type, v.identifier)?;
        v.content.accept(self)
    }

    fn visit_condition(&mut self, v: &Condition) -> Self::Value {
        write!(self.f, "ij ")?;
        v.condition.accept(self)?;
        self.visit_statements(&v.positive)?;
        if let Some(content) = &v.negative {
            write!(self.f, "sula ")?;
            match &**content {
                Sula::Scope(sta) => self.visit_statements(sta)?,
                Sula::Condition(cond) => self.visit_condition(cond)?
            }
        }
        Ok(())
    }

    fn visit_variable_mod(&mut self, v: &VariableModification) -> Self::Value {
        self.visit_identifier_chain(&v.identifier)?;
        self.f.write_str(" ")?;
        v.value.accept(self)
    }

    fn visit_function_call(&mut self, _v: &FunctionCall) -> Self::Value {
        self.f.write_str("")
    }

    fn visit_identifier_chain(&mut self, _v: &IdentifierChain) -> Self::Value {
        self.f.write_str("")
    }

    fn visit_bool(&mut self, v: bool) -> Self::Value {
        write!(self.f, "{}", v)
    }

    fn visit_int(&mut self, v: IntType) -> Self::Value {
        write!(self.f, "{}", v)
    }

    fn visit_f32(&mut self, v: f32) -> Self::Value {
        write!(self.f, "{}", v)
    }

    fn visit_string(&mut self, v: &String) -> Self::Value {
        write!(self.f, "{}", v)
    }
}
