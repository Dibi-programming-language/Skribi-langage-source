use crate::{
    ast::nodes::{
        AstRoot,
        calls::{FunctionCall, IdentifierChain, VariableModification},
        conditions::Condition,
        declarations::{FunctionDeclaration, VariableDeclaration},
        expressions::Expression,
        loops::Ci,
        operations::BinaryOperation,
        statements::{Return, StatementList},
    },
    execute::IntType,
};

pub mod pretty;
pub mod compile;

/// Adapted visitor: we "accept" in enums, matching the element type, then
/// "visiting" the resulting struct.
/// Visitor is useless when there is no enum.
/// We do not visit enums: Sula, Expression, Statement.
/// UnaryOperation and ValueBase are exceptions: elements are visited as they
/// are not structs.
pub trait NodeVisitor {
    type Value;

    fn visit_root(&mut self, v: &AstRoot) -> Self::Value;
    fn visit_return(&mut self, v: &Return) -> Self::Value;
    fn visit_statements(&mut self, v: &StatementList) -> Self::Value;
    fn visit_binary(&mut self, v: &BinaryOperation) -> Self::Value;

    fn visit_not(&mut self, v: &Expression) -> Self::Value;
    fn visit_plus(&mut self, v: &Expression) -> Self::Value;
    fn visit_minus(&mut self, v: &Expression) -> Self::Value;

    fn visit_ci(&mut self, v: &Ci) -> Self::Value;
    fn visit_function_dec(&mut self, v: &FunctionDeclaration) -> Self::Value;
    fn visit_variable_dec(&mut self, v: &VariableDeclaration) -> Self::Value;
    fn visit_condition(&mut self, v: &Condition) -> Self::Value;
    fn visit_variable_mod(&mut self, v: &VariableModification) -> Self::Value;
    fn visit_function_call(&mut self, v: &FunctionCall) -> Self::Value;
    fn visit_identifier_chain(&mut self, v: &IdentifierChain) -> Self::Value;

    fn visit_bool(&mut self, v: bool) -> Self::Value;
    fn visit_int(&mut self, v: IntType) -> Self::Value;
    fn visit_f32(&mut self, v: f32) -> Self::Value;
    fn visit_string(&mut self, v: &String) -> Self::Value;
}
