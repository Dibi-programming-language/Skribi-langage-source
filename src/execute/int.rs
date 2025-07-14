use std::fmt::Display;

use super::{BasicValue, IntType};

struct InternalInt {
    content: IntType,
}

impl BasicValue for InternalInt {
    fn clone(&self) -> super::VariableValue {
        Box::new(InternalInt {
            content: self.content
        })
    }

    fn add(self, other: super::VariableValue, context: &super::OperationContext) -> Result<super::VariableValue, super::ExecutionError> {
        todo!()
    }

    fn sub(self, other: super::VariableValue, context: &super::OperationContext) -> Result<super::VariableValue, super::ExecutionError> {
        todo!()
    }

    fn div(self, other: super::VariableValue, context: &super::OperationContext) -> Result<super::VariableValue, super::ExecutionError> {
        todo!()
    }

    fn mul(self, other: super::VariableValue, context: &super::OperationContext) -> Result<super::VariableValue, super::ExecutionError> {
        todo!()
    }

    fn minus(self, other: super::VariableValue, context: &super::OperationContext) -> Result<super::VariableValue, super::ExecutionError> {
        todo!()
    }

    fn as_int(self, context: &super::OperationContext) -> Result<IntType, super::ExecutionError> {
        todo!()
    }

    fn as_ioi(self, context: &super::OperationContext) -> Result<bool, super::ExecutionError> {
        todo!()
    }
}

impl Display for InternalInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}

