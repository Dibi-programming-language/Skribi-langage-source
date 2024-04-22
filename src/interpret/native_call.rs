use crate::interpret::variables::{Variable, VariableType};
use std::collections::HashMap;

/// This function is used to print the value of a variable
pub(crate) fn print_variable(variable_type: &VariableType, line: u16) {
    match variable_type {
        VariableType::String(value) => {
            print!("{}", value);
        }
        VariableType::Integer(value) => {
            print!("{}", value);
        }
        VariableType::Float(value) => {
            print!("{}", value);
        }
        VariableType::Boolean(value) => {
            print!("{}", value);
        }
        // Other types will be added later
        VariableType::Unset => {
            panic!("Cannot print an unset variable");
        }
    }
}

/// This function is used to interpret a line of code that call a native function
pub(crate) fn native_call(
    line: Vec<String>,
    line_number: u16,
    variables: &mut HashMap<String, Variable>,
) {
    // get the name of the function
    let function_name = line[1].as_str();
    // get the arguments of the function
    let mut arguments: Vec<Variable> = Vec::new();
    for i in 2..line.len() {
        if let Some(var) = variables.get_mut(&line[i]) {
            arguments.push(var.clone());
        } else {
            panic!("Unknown variable {:?}", line[i]);
        }
    }
    // call the function
    match function_name {
        "print" => {
            // print the arguments
            for mut arg in arguments {
                print_variable(arg.get_value(line_number), line_number);
            }
        }
        "println" => {
            // println the arguments
            for mut arg in arguments {
                print_variable(arg.get_value(line_number), line_number);
            }
            println!();
        }
        _ => panic!("Unknown native call "),
    }
}
