pub type IntType = u32;
pub type OperationI = u32;
pub type OperationO = u32;
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
    fn execute(&self, operation_context: &mut OperationContext);
}
