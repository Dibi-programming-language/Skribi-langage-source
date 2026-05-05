use crate::execute::IntType;

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
