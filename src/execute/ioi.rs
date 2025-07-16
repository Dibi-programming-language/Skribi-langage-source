use std::fmt::Display;

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

    fn add(
        mut self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        self.content |= other.as_ioi(context)?;
        Ok(self)
    }

    fn sub(
        self: Box<Self>,
        _other: &VariableValue,
        _context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        Err(ExecutionError::not_implemented_for("-", "ioi"))
    }

    fn div(
        self: Box<Self>,
        _other: &VariableValue,
        _context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        Err(ExecutionError::not_implemented_for("/", "ioi"))
    }

    fn mul(
        mut self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        self.content &= other.as_ioi(context)?;
        Ok(self)
    }

    fn minus(
        self: Box<Self>,
        _context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        Err(ExecutionError::not_implemented_for("-", "ioi"))
    }

    fn as_int(&self, _context: &OperationContext) -> Result<IntType, ExecutionError> {
        Err(ExecutionError::wrong_type("int", "ioi"))
    }

    fn as_ioi(&self, _context: &OperationContext) -> Result<bool, ExecutionError> {
        Ok(self.content)
    }

    fn equal(
        &self,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        self.basic_equal(other, context).map(|x| Self::new_boxed(x))
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
