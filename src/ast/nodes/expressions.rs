use crate::ast::nodes::{
    base::ValueBase, calls::{FunctionCall, IdentifierChain, VariableModification}, conditions::Condition, declarations::{FunctionDeclaration, VariableDeclaration}, loops::Ci, operations::{BinaryOperation, UnaryOperation}
};

pub enum Expression<'a> {
    ValueBase(ValueBase),
    BinOp(Box<BinaryOperation<'a>>),
    UnaryOp(Box<UnaryOperation<'a>>),
    FctCall(FunctionCall<'a>),
    Identifier(IdentifierChain<'a>),
    Cond(Box<Condition<'a>>),
    FctDec(Box<FunctionDeclaration<'a>>),
    VarDec(Box<VariableDeclaration<'a>>),
    VarMod(Box<VariableModification<'a>>),
    While(Box<Ci<'a>>),
}
