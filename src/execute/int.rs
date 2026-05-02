use std::fmt::Display;

use crate::parse::nodes::operations::Operations;

use super::ioi::InternalIoi;
use super::{BasicValue, ExecutionError, IntType, OperationContext, VariableValue};

pub struct InternalInt {
    content: IntType,
}

impl InternalInt {
    pub fn new_boxed(content: IntType) -> VariableValue {
        Box::new(InternalInt { content })
    }
}

impl BasicValue for InternalInt {
    fn clone(&self) -> VariableValue {
        Box::new(InternalInt {
            content: self.content,
        })
    }

    fn apply_operation(
        mut self: Box<Self>,
        operation: &crate::parse::nodes::operations::Operations,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        match operation {
            Operations::Add => {
                let other = other.as_int(context)?;
                self.content += other;
                Ok(self)
            }
            Operations::Sub => {
                let other = other.as_int(context)?;
                self.content -= other;
                Ok(self)
            }
            Operations::Div => {
                let other = other.as_int(context)?;
                self.content /= other;
                Ok(self)
            }
            Operations::Mul => {
                let other = other.as_int(context)?;
                self.content *= other;
                Ok(self)
            }
            Operations::LessThan => {
                let other = other.as_int(context)?;
                Ok(InternalIoi::new_boxed(self.content < other))
            }
            Operations::GreaterThan => {
                let other = other.as_int(context)?;
                Ok(InternalIoi::new_boxed(self.content > other))
            }
            Operations::LessOrEqual => {
                let other = other.as_int(context)?;
                Ok(InternalIoi::new_boxed(self.content <= other))
            }
            Operations::GreaterOrEqual => {
                let other = other.as_int(context)?;
                Ok(InternalIoi::new_boxed(self.content >= other))
            }
            Operations::Equal => self
                .basic_equal(other, context)
                .map(|x| InternalIoi::new_boxed(x)),
            Operations::NotEqual => self
                .basic_equal(other, context)
                .map(|x| InternalIoi::new_boxed(!x)),
            _ => Err(ExecutionError::not_implemented_for(operation, "int")),
        }
    }

    fn minus(
        mut self: Box<Self>,
        _context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        self.content = -self.content;
        Ok(self)
    }

    fn not(self: Box<Self>, _context: &OperationContext) -> Result<VariableValue, ExecutionError> {
        Err(ExecutionError::not_implemented_for("!", "int"))
    }

    fn as_int(&self, _context: &OperationContext) -> Result<IntType, ExecutionError> {
        Ok(self.content)
    }

    fn as_ioi(&self, _context: &OperationContext) -> Result<bool, ExecutionError> {
        Err(ExecutionError::wrong_type("ioi", "int"))
    }

    fn basic_equal(
        &self,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<bool, ExecutionError> {
        let other = other.as_int(context)?;
        Ok(self.content == other)
    }
}

impl Display for InternalInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}
