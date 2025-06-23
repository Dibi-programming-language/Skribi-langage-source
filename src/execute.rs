pub type IntType = u32;
pub type OperationI = u32;
pub type OperationCleanOutput = u32;
pub type OperationO = Result<OperationCleanOutput, ExecutionError>;
pub type GeneralOutput = Result<(), ExecutionError>;
pub type OperationContext = ();

pub trait EvaluateFromInput {
    fn evaluate_from_input(
        &self,
        operation_context: &mut OperationContext,
        input: OperationI,
    ) -> OperationO;
}

pub trait Evaluate {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO;
}

pub trait Execute {
    fn execute(
        &self,
        operation_context: &mut OperationContext
    ) -> GeneralOutput;
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct ExecutionError {
    message: String,
}

impl ExecutionError {
    pub fn variable_not_exists() -> Self {
        ExecutionError { message: "This variable does not exists.".to_string() }
    }

    pub fn native_call_invalid() -> Self {
        ExecutionError { message: "Cannot process this native call.".to_string() }
    }
}

