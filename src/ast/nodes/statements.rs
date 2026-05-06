use crate::ast::{
    nodes::{calls::FunctionCall, expressions::Expression},
    visitors::NodeVisitor,
};

#[derive(PartialEq, Clone)]
pub struct Return<'a> {
    pub exp: Expression<'a>,
}

#[derive(PartialEq, Clone)]
pub enum Statement<'a> {
    Return(Return<'a>),
    Exp(Expression<'a>),
    NatCall(FunctionCall<'a>),
}

impl Statement<'_> {
    pub fn accept<U, T: NodeVisitor<Value = U>>(&self, v: &mut T) -> U {
        match self {
            Self::Return(node) => v.visit_return(node),
            Self::Exp(node) => node.accept(v),
            Self::NatCall(node) => v.visit_function_call(node),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct StatementList<'a> {
    pub statements: Vec<Statement<'a>>,
    pub unused: bool,
    pub simple: bool,
    pub bubble: bool,
}

impl StatementList<'_> {
    pub fn new<'a>(
        statements: Vec<Statement<'a>>,
    ) -> StatementList<'a> {
        StatementList {
            statements,
            unused: false,
            simple: false,
            bubble: false,
        }
    }

    pub fn unused(mut self) -> Self {
        self.unused = true;
        self
    }

    pub fn simple(mut self) -> Self {
        self.simple = true;
        self
    }

    pub fn bubble(mut self) -> Self {
        self.bubble = true;
        self
    }
}
