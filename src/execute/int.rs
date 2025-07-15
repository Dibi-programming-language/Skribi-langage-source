use std::fmt::Display;

use super::ioi::InternalIoi;
use super::{BasicValue, ExecutionError, IntType, VariableValue};

struct InternalInt {
    content: IntType,
}

impl BasicValue for InternalInt {
    fn clone(&self) -> super::VariableValue {
        Box::new(InternalInt {
            content: self.content,
        })
    }

    fn add(
        mut self: Box<Self>,
        other: &VariableValue,
        context: &super::OperationContext,
    ) -> Result<super::VariableValue, super::ExecutionError> {
        let other = other.as_int(context)?;
        self.content += other;
        Ok(self)
    }

    fn sub(
        mut self: Box<Self>,
        other: &super::VariableValue,
        context: &super::OperationContext,
    ) -> Result<super::VariableValue, super::ExecutionError> {
        let other = other.as_int(context)?;
        self.content -= other;
        Ok(self)
    }

    fn div(
        mut self: Box<Self>,
        other: &super::VariableValue,
        context: &super::OperationContext,
    ) -> Result<super::VariableValue, super::ExecutionError> {
        let other = other.as_int(context)?;
        self.content /= other;
        Ok(self)
    }

    fn mul(
        mut self: Box<Self>,
        other: &super::VariableValue,
        context: &super::OperationContext,
    ) -> Result<super::VariableValue, super::ExecutionError> {
        let other = other.as_int(context)?;
        self.content *= other;
        Ok(self)
    }

    fn minus(
        mut self: Box<Self>,
        _context: &super::OperationContext,
    ) -> Result<super::VariableValue, super::ExecutionError> {
        self.content = -self.content;
        Ok(self)
    }

    fn as_int(&self, _context: &super::OperationContext) -> Result<IntType, super::ExecutionError> {
        Ok(self.content)
    }

    fn as_ioi(&self, _context: &super::OperationContext) -> Result<bool, super::ExecutionError> {
        Err(ExecutionError::wrong_type("ioi", "int"))
    }

    fn equal(&self, other: &VariableValue, context: &super::OperationContext) -> Result<VariableValue, ExecutionError> {
        self.basic_equal(other, context).map(|x| InternalIoi::new(x))
    }

    fn basic_equal(&self, other: &VariableValue, context: &super::OperationContext) -> Result<bool, ExecutionError> {
        let other = other.as_int(context)?;
        Ok(self.content == other)
    }
}

impl Display for InternalInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}
