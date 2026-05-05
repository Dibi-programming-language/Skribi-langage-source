use crate::ast::{
    nodes::{
        base::ValueBase,
        calls::{FunctionCall, IdentifierChain, VariableModification},
        conditions::Condition,
        declarations::{FunctionDeclaration, VariableDeclaration},
        loops::Ci,
        operations::{BinaryOperation, UnaryOperation},
    },
    visitors::NodeVisitor,
};

#[derive(PartialEq, Clone)]
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

impl Expression<'_> {
    pub fn accept<U, T: NodeVisitor<Value = U>>(&self, v: &mut T) -> U {
        match self {
            Self::While(ci) => v.visit_ci(ci),
            Self::VarMod(varmod) => v.visit_variable_mod(varmod),
            Self::VarDec(vardec) => v.visit_variable_dec(vardec),
            Self::FctDec(fctdec) => v.visit_function_dec(fctdec),
            Self::Cond(cond) => v.visit_condition(cond),
            Self::Identifier(identifier) => v.visit_identifier_chain(identifier),
            Self::FctCall(fctcall) => v.visit_function_call(fctcall),
            Self::UnaryOp(unary) => unary.accept(v),
            Self::BinOp(binop) => v.visit_binary(binop),
            Self::ValueBase(base) => base.accept(v),
        }
    }
}
