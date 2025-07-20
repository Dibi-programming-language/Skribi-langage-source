use std::fmt::Display;

use super::{BasicValue, ExecutionError, IntType, OperationContext, VariableValue};

pub struct InternalUnit;

impl InternalUnit {
    pub fn new_boxed() -> VariableValue {
        Box::new(InternalUnit {})
    }
}

impl BasicValue for InternalUnit {
    fn clone(&self) -> VariableValue {
        Self::new_boxed()
    }

    fn apply_operation(
        self: Box<Self>,
        _operation: &crate::parse::nodes::operations::Operations,
        _other: &VariableValue,
        _context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        Err(ExecutionError::unit_used())
    }

    fn minus(
        self: Box<Self>,
        _context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError> {
        Err(ExecutionError::unit_used())
    }

    fn not(self: Box<Self>, _context: &OperationContext) -> Result<VariableValue, ExecutionError> {
        Err(ExecutionError::unit_used())
    }

    fn basic_equal(
        &self,
        _other: &VariableValue,
        _context: &OperationContext,
    ) -> Result<bool, ExecutionError> {
        Err(ExecutionError::unit_used())
    }

    fn as_int(&self, _context: &OperationContext) -> Result<IntType, ExecutionError> {
        Err(ExecutionError::unit_used())
    }

    fn as_ioi(&self, _context: &OperationContext) -> Result<bool, ExecutionError> {
        Err(ExecutionError::unit_used())
    }
}

impl Display for InternalUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unit")
    }
}
