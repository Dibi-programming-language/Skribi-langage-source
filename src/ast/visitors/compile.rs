use crate::{
    ast::{
        nodes::{
            AstRoot,
            calls::{FunctionCall, IdentifierChain, VariableModification},
            conditions::Condition,
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

#[allow(dead_code)]
struct CodeGenerator {}

impl NodeVisitor for CodeGenerator {
    type Value = Result<(), ()>;

    fn visit_root(&mut self, _v: &AstRoot) -> Self::Value {
        todo!()
    }

    fn visit_return(&mut self, _v: &Return) -> Self::Value {
        todo!()
    }

    fn visit_statements(&mut self, _v: &StatementList) -> Self::Value {
        todo!()
    }

    fn visit_binary(&mut self, _v: &BinaryOperation) -> Self::Value {
        todo!()
    }

    fn visit_not(&mut self, _v: &Expression) -> Self::Value {
        todo!()
    }

    fn visit_plus(&mut self, _v: &Expression) -> Self::Value {
        todo!()
    }

    fn visit_minus(&mut self, _v: &Expression) -> Self::Value {
        todo!()
    }

    fn visit_ci(&mut self, _v: &Ci) -> Self::Value {
        todo!()
    }

    fn visit_function_dec(&mut self, _v: &FunctionDeclaration) -> Self::Value {
        todo!()
    }

    fn visit_variable_dec(&mut self, _v: &VariableDeclaration) -> Self::Value {
        todo!()
    }

    fn visit_condition(&mut self, _v: &Condition) -> Self::Value {
        todo!()
    }

    fn visit_variable_mod(&mut self, _v: &VariableModification) -> Self::Value {
        todo!()
    }

    fn visit_function_call(&mut self, _v: &FunctionCall) -> Self::Value {
        todo!()
    }

    fn visit_identifier_chain(&mut self, _v: &IdentifierChain) -> Self::Value {
        todo!()
    }

    fn visit_bool(&mut self, _v: bool) -> Self::Value {
        todo!()
    }

    fn visit_int(&mut self, _v: IntType) -> Self::Value {
        todo!()
    }

    fn visit_f32(&mut self, _v: f32) -> Self::Value {
        todo!()
    }

    fn visit_string(&mut self, _v: &String) -> Self::Value {
        todo!()
    }
}
