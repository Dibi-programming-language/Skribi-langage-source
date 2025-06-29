use std::collections::{HashMap};

use colored::Colorize;

pub type OperationI = u32;

struct Variable {
    pub content: u32,
}

impl Variable {
    fn new(value: OperationI) -> Self {
        Self {
            content: value
        }
    }
}

struct ExecutionScope {
    variables: HashMap<String, Variable>,
    outer_scope: Option<Box<ExecutionScope>>
}

impl ExecutionScope {
    fn new(outer_scope: Option<Box<ExecutionScope>>) -> Self {
        Self {
            variables: HashMap::new(),
            outer_scope
        }
    }

    fn associate_new(&mut self, name: String, value: OperationI) {
        self.variables.insert(name, Variable::new(value));
    }

    fn edit_variable(
        &mut self,
        name: &String,
        new_value: OperationI,
        line: usize
    ) -> Result<(), ExecutionError> {
        if let Some(variable) = self.variables.get_mut(name) {
            variable.content = new_value;
            Ok(())
        } else if let Some(ref mut outer) = self.outer_scope {
            outer.edit_variable(name, new_value, line)
        } else {
            Err(ExecutionError::variable_not_exists(name, line))
        }
    }

    fn get_variable(
        &mut self,
        name: &String,
        line: usize
    ) -> Result<OperationI, ExecutionError> {
        if let Some(variable) = self.variables.get_mut(name) {
            Ok(variable.content)
        } else if let Some(ref mut outer) = self.outer_scope {
            outer.get_variable(name, line)
        } else {
            Err(ExecutionError::variable_not_exists(name, line))
        }
    }
}

pub struct ExecutionContext {
    scope: Option<ExecutionScope>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            scope: None
        }
    }

    pub fn associate_new(&mut self, name: String, value: OperationI) {
        if let Some(ref mut scope) = self.scope {
            scope.associate_new(name, value);
        } else {
            self.scope = Some(ExecutionScope::new(None));
            self.associate_new(name, value);
        }
    }

    pub fn change_value(
        &mut self,
        name: &String,
        value: OperationI,
        line: usize,
    ) -> Result<(), ExecutionError> {
        if let Some(ref mut scope) = self.scope {
            scope.edit_variable(name, value, line)
        } else {
            Err(ExecutionError::variable_not_exists(name, line))
        }
    }

    pub fn get_variable(
        &mut self,
        name: &String,
        line: usize,
    ) -> Result<OperationI, ExecutionError> {
        if let Some(ref mut scope) = self.scope {
            scope.get_variable(name, line)
        } else {
            Err(ExecutionError::variable_not_exists(name, line))
        }
    }
}

pub type IntType = u32;
pub type OperationCleanOutput = u32;
pub type OperationO = Result<OperationCleanOutput, ExecutionError>;
pub type GeneralOutput = Result<(), ExecutionError>;
pub type OperationContext = ExecutionContext;

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
pub struct ExecutionHint {
    message: String,
}

impl ExecutionHint {
    fn new(message: &str) -> Self {
        ExecutionHint {
            message: message.to_string()
        }
    }

    pub fn move_to_function() -> Self {
        Self::new("Consider moving your code into a function.")
    }

    pub fn remove_line() -> Self {
        Self::new("Consider removing this line.")
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct ExecutionError {
    message: String,
    hints: Vec<ExecutionHint>
}

impl ExecutionError {
    fn new(message: &str) -> Self {
        ExecutionError {
            message: message.to_string(),
            hints: Vec::new()
        }
    }

    fn new_str(message: String) -> Self {
        ExecutionError {
            message: message,
            hints: Vec::new()
        }
    }

    fn add_hint(mut self, hint: ExecutionHint) -> Self {
        self.hints.push(hint);
        self
    }

    pub fn variable_not_exists(name: &String, line: usize) -> Self {
        Self::new_str(format!(
                "The variable {} does not exists at line {}",
                name, line
        ))
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
        println!("Error: {}", self.message.red().bold());

        if self.hints.len() == 1 {
            println!(
                "{} {}",
                "Hint:".bold().green(),
                self.hints[0].message.green()
            )
        } else if self.hints.len() > 1 {
            println!("{}", "Hints:".bold().green());
            for hint in &self.hints {
                println!("{} {}", "-".green(), hint.message.green());
            }
        }
    }
}

