use std::fmt::Display;

use super::{BasicValue, ExecutionError, IntType, VariableValue, OperationContext};

pub struct InternalIoi {
    content: bool,
}

impl InternalIoi {
    pub fn new(content: bool) -> VariableValue {
        Box::new(InternalIoi {
            content,
        })
    }
}

impl BasicValue for InternalIoi {
    fn clone(&self) -> super::VariableValue {
        Box::new(InternalIoi {
            content: self.content,
        })
    }

    fn add(
        mut self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        if self.content {
            Ok(self)
        } else {
            let other = other.as_ioi(context)?;
            self.content |= other;
            Ok(self)
        }
    }

    fn sub(
        self: Box<Self>,
        _other: &VariableValue,
        _context: &OperationContext,
    ) -> Result<super::VariableValue, super::ExecutionError> {
        Err(ExecutionError::not_implemented_for("-", "ioi"))
    }

    fn div(
        self: Box<Self>,
        _other: &super::VariableValue,
        _context: &super::OperationContext,
    ) -> Result<super::VariableValue, super::ExecutionError> {
        Err(ExecutionError::not_implemented_for("/", "ioi"))
    }

    fn mul(
        mut self: Box<Self>,
        other: &super::VariableValue,
        context: &super::OperationContext,
    ) -> Result<super::VariableValue, super::ExecutionError> {
        if !self.content {
            Ok(self)
        } else {
            let other = other.as_ioi(context)?;
            self.content &= other;
            Ok(self)
        }
    }

    fn minus(
        self: Box<Self>,
        _context: &super::OperationContext,
    ) -> Result<super::VariableValue, super::ExecutionError> {
        Err(ExecutionError::not_implemented_for("-", "ioi"))
    }

    fn as_int(&self, _context: &super::OperationContext) -> Result<IntType, super::ExecutionError> {
        Err(ExecutionError::wrong_type("int", "ioi"))
    }

    fn as_ioi(&self, _context: &super::OperationContext) -> Result<bool, super::ExecutionError> {
        Ok(self.content)
    }

    fn equal(&self, other: &VariableValue, context: &super::OperationContext) -> Result<VariableValue, ExecutionError> {
        self.basic_equal(other, context).map(|x| Self::new(x))
    }

    fn basic_equal(&self, other: &VariableValue, context: &super::OperationContext) -> Result<bool, ExecutionError> {
        let other = other.as_ioi(context)?;
        Ok(self.content == other)
    }
}

impl Display for InternalIoi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}
