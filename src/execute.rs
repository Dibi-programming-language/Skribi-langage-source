
pub type OperationIO = usize;
pub type OperationContext = ();

pub trait ExecuteFromInput {
    fn executeFromInput(operation_context: OperationContext, input: OperationIO) -> OperationIO;
}
