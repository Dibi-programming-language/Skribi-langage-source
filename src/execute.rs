use colored::Colorize;

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
    fn execute(&self, operation_context: &mut OperationContext) -> GeneralOutput;
}

#[derive(PartialEq, Debug)]
pub struct ExecutionHint {
    message: String,
}

impl ExecutionHint {
    fn new(message: &str) -> Self {
        ExecutionHint {
            message: message.to_string(),
        }
    }

    pub fn move_to_function() -> Self {
        Self::new("Consider moving your code into a function.")
    }

    pub fn remove_line() -> Self {
        Self::new("Consider removing this line.")
    }
}

#[derive(PartialEq, Debug)]
pub struct ExecutionError {
    message: String,
    hints: Vec<ExecutionHint>,
}

impl ExecutionError {
    fn new(message: &str) -> Self {
        ExecutionError {
            message: message.to_string(),
            hints: Vec::new(),
        }
    }

    fn new_str(message: String) -> Self {
        ExecutionError {
            message,
            hints: Vec::new(),
        }
    }

    fn add_hint(mut self, hint: ExecutionHint) -> Self {
        self.hints.push(hint);
        self
    }

    pub fn variable_not_exists() -> Self {
        Self::new("This variable does not exists.")
    }

    pub fn native_call_invalid(name: &str) -> Self {
        Self::new_str(format!(
            "Cannot process the native call named {}.",
            name.italic()
        ))
    }

    pub fn no_return_at_root() -> Self {
        Self::new("Cannot return a value outside of a function.")
            .add_hint(ExecutionHint::remove_line())
            .add_hint(ExecutionHint::move_to_function())
    }

    pub fn show(&self) {
        eprintln!("Error: {}", self.message.red().bold());

        if self.hints.len() == 1 {
            eprintln!(
                "{} {}",
                "Hint:".bold().green(),
                self.hints[0].message.green()
            )
        } else if self.hints.len() > 1 {
            eprintln!("{}", "Hints:".bold().green());
            for hint in &self.hints {
                eprintln!("{} {}", "-".green(), hint.message.green());
            }
        }
    }
}
