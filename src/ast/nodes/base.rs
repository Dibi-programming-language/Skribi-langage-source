use crate::{ast::visitors::NodeVisitor, execute::IntType};

/// `ValueBase` represents the base of a value in the AST. This is the smallest
/// unit of a value. This node is not dependent on any other node. The value can
/// be a boolean, an integer, a float or a string.
#[derive(PartialEq, Clone)]
pub enum ValueBase {
    Bool(bool),
    Int(IntType),
    Float(f32),
    String(String),
}

impl ValueBase {
    pub fn accept<U, T: NodeVisitor<Value = U>>(&self, v: &mut T) -> U {
        match self {
            Self::Bool(b) => v.visit_bool(*b),
            Self::Int(i) => v.visit_int(*i),
            Self::Float(f) => v.visit_f32(*f),
            Self::String(s) => v.visit_string(s),
        }
    }
}
