use std::{collections::HashMap, fmt::Display};

use colored::Colorize;

pub mod int;
pub mod ioi;
pub mod unit;

pub type IntType = i32;

pub trait BasicValue: Display {
    /// A clone function that does not require Sized.
    /// Used to avoid useless allocations of VariableValue
    fn clone(&self) -> VariableValue;

    fn add(
        self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError>;

    fn sub(
        self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError>;

    fn div(
        self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError>;

    fn mul(
        self: Box<Self>,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError>;

    fn minus(self: Box<Self>, context: &OperationContext) -> Result<VariableValue, ExecutionError>;

    fn as_int(&self, context: &OperationContext) -> Result<IntType, ExecutionError>;
    fn as_ioi(&self, context: &OperationContext) -> Result<bool, ExecutionError>;

    fn basic_equal(
        &self,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<bool, ExecutionError>;
    fn equal(
        &self,
        other: &VariableValue,
        context: &OperationContext,
    ) -> Result<VariableValue, ExecutionError>;
}

pub type VariableValue = Box<dyn BasicValue>;
pub type OperationI = VariableValue;

struct Variable {
    pub content: VariableValue,
}

impl Variable {
    fn new(value: OperationI) -> Self {
        Self { content: value }
    }
}

struct ExecutionScope {
    variables: HashMap<String, Variable>,
    outer_scope: Option<Box<ExecutionScope>>,
}

impl ExecutionScope {
    fn new(outer_scope: Option<Box<ExecutionScope>>) -> Self {
        Self {
            variables: HashMap::new(),
            outer_scope,
        }
    }

    fn associate_new(&mut self, name: String, value: OperationI) {
        self.variables.insert(name, Variable::new(value));
    }

    fn edit_variable(
        &mut self,
        name: &str,
        new_value: OperationI,
        line: usize,
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
        line: usize,
    ) -> Result<VariableValue, ExecutionError> {
        if let Some(variable) = self.variables.get_mut(name) {
            Ok(variable.content.clone())
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
        Self { scope: None }
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
        name: &str,
        value: VariableValue,
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

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}

pub type OperationCleanOutput = VariableValue;
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
    fn execute(&self, operation_context: &mut OperationContext) -> GeneralOutput;
}

#[allow(dead_code)]
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

    pub fn try_again() -> Self {
        Self::new("Try again the same action, this could work this time.")
    }

    pub fn try_another_input() -> Self {
        Self::new("Try to enter another input.")
    }

    pub fn check_types() -> Self {
        Self::new("Check operations around this value to verify the type.")
    }

    pub fn return_scope() -> Self {
        Self::new("Scopes (ij / sula / ...) that return a value are not yet implmented.")
    }
}

#[allow(dead_code)]
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

    pub fn variable_not_exists(name: &str, line: usize) -> Self {
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

    pub fn cannot_read_input() -> Self {
        Self::new("Failed to read user input.")
            .add_hint(ExecutionHint::try_again())
            .add_hint(ExecutionHint::try_another_input())
    }

    pub fn wrong_input_type(expected: &str, received: &str) -> Self {
        Self::new_str(format!(
            "The input type is wrong. Fail to parse {} to type {}.",
            received, expected
        ))
        .add_hint(ExecutionHint::try_another_input())
    }

    pub fn wrong_number_of_inputs() -> Self {
        Self::new("More inputs were expected.")
    }

    pub fn assertion_error(expected: OperationI, received: OperationI) -> Self {
        Self::new_str(format!(
            "Assertion Error: expected {} got {}.",
            expected, received,
        ))
    }

    pub fn wrong_type(expected: &str, got: &str) -> Self {
        Self::new_str(format!("Type Error: expected {} got {}.", expected, got))
            .add_hint(ExecutionHint::check_types())
    }

    pub fn not_implemented_for(operation: &str, got: &str) -> Self {
        Self::new_str(format!(
            "Type Error: operation {} does not have any meaning for {}.",
            operation, got
        ))
    }

    pub fn unit_used() -> Self {
        Self::new(
            "A unit element was used in any operation. Unit values should not be used in any case.",
        )
        .add_hint(ExecutionHint::return_scope())
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
