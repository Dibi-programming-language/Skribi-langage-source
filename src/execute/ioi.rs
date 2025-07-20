use std::fmt::Display;

use crate::parse::nodes::operations::Operations;

use super::{BasicValue, ExecutionError, IntType, OperationContext, VariableValue};

pub struct InternalIoi {
    content: bool,
}

impl InternalIoi {
    pub fn new_boxed(content: bool) -> VariableValue {
        Box::new(InternalIoi { content })
    }
}

impl BasicValue for InternalIoi {
    fn clone(&self) -> VariableValue {
        Box::new(InternalIoi {
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
                self.content |= other.as_ioi(context)?;
                Ok(self)
            }
            Operations::Mul => {
                self.content &= other.as_ioi(context)?;
                Ok(self)
            }
            Operations::And => {
                self.content &= other.as_ioi(context)?;
                Ok(self)
            }
            Operations::Or => {
                self.content |= other.as_ioi(context)?;
                Ok(self)
            }
            Operations::Equal => self.basic_equal(other, context).map(|x| Self::new_boxed(x)),
            Operations::NotEqual => self
                .basic_equal(other, context)
                .map(|x| Self::new_boxed(!x)),
            _ => Err(ExecutionError::not_implemented_for(operation, "ioi")),
        }
    }

    fn minus(
        self: Box<Self>,
        _context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        Err(ExecutionError::not_implemented_for("-", "ioi"))
    }

    fn not(
        mut self: Box<Self>,
        _context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        self.content = !self.content;
        Ok(self)
    }

    fn as_int(&self, _context: &OperationContext) -> Result<IntType, ExecutionError> {
        Err(ExecutionError::wrong_type("int", "ioi"))
    }

    fn as_ioi(&self, _context: &OperationContext) -> Result<bool, ExecutionError> {
        Ok(self.content)
    }

    fn basic_equal(
        &self,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<bool, ExecutionError> {
        let other = other.as_ioi(context)?;
        Ok(self.content == other)
    }
}

impl Display for InternalIoi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}
