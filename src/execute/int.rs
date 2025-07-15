use std::fmt::Display;

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

    fn add(
        mut self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        let other = other.as_int(context)?;
        self.content += other;
        Ok(self)
    }

    fn sub(
        mut self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        let other = other.as_int(context)?;
        self.content -= other;
        Ok(self)
    }

    fn div(
        mut self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        let other = other.as_int(context)?;
        self.content /= other;
        Ok(self)
    }

    fn mul(
        mut self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        let other = other.as_int(context)?;
        self.content *= other;
        Ok(self)
    }

    fn minus(
        mut self: Box<Self>,
        _context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        self.content = -self.content;
        Ok(self)
    }

    fn as_int(&self, _context: &OperationContext) -> Result<IntType, ExecutionError> {
        Ok(self.content)
    }

    fn as_ioi(&self, _context: &OperationContext) -> Result<bool, ExecutionError> {
        Err(ExecutionError::wrong_type("ioi", "int"))
    }

    fn equal(
        &self,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        self.basic_equal(other, context)
            .map(|x| InternalIoi::new_boxed(x))
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
