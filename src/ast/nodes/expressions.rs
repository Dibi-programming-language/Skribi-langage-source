use crate::ast::nodes::{
    branches::Branch,
    calls::{FunctionCall, IdentifierChain, VariableReassignment},
    declarations::{FunctionDeclaration, VariableDeclaration},
    loops::While,
    operations::{BinaryOperation, UnaryOperation},
    primitive_values::PrimitiveValue,
};

pub enum Expression<'a> {
    ValueBase(PrimitiveValue),
    BinOp(Box<BinaryOperation<'a>>),
    UnaryOp(Box<UnaryOperation<'a>>),
    FunctionCall(FunctionCall<'a>),
    IdentifierChain(IdentifierChain<'a>),
    Branch(Box<Branch<'a>>),
    FunctionDeclaration(Box<FunctionDeclaration<'a>>),
    VariableDeclaration(Box<VariableDeclaration<'a>>),
    VariableReassignment(Box<VariableReassignment<'a>>),
    While(Box<While<'a>>),
}
