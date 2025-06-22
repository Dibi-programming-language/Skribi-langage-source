pub type IntType = u32;
pub type OperationIO = u32;
pub type OperationContext = ();

pub trait EvaluateFromInput {
    fn evaluate_from_input(
        &self,
        operation_context: &OperationContext,
        input: OperationIO,
    ) -> OperationIO;
}

pub trait Evaluate {
    fn evaluate(&self, operation_context: &OperationContext) -> OperationIO;
}

pub trait Execute {
    fn execute(&self, operation_context: &OperationContext);
}
