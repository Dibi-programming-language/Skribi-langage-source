use std::collections::HashMap;
use skribi_language_source::error;
use crate::interpret::variables::{VariableStruct, VariableType};

/**
 * This function is used to print the value of a variable
 */
pub fn print_variable(variable_type: &VariableType, line: u16) {
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
            error("Cannot print an unset variable", line);
        }
    }
}

/**
 * This function is used to interpret a line of code that call a native function
 */
pub fn native_call(line: Vec<String>, line_number: u16, variables: &mut HashMap<String, VariableStruct>) {
    // get the number of the function
    let function_name = line[1].parse::<u8>().unwrap();
    // get the arguments of the function
    let mut arguments: Vec<&VariableType> = Vec::new();
    for i in 2..line.len() {
        if variables.contains_key(&line[i]) {
            arguments.push(variables.get(&line[i]).unwrap().get_value(line_number));
        } else {
            error(
                ("Unknown variable ".to_string() + &line[i]).as_str(),
                line_number,
            );
        }
    }
    // call the function
    match function_name {
        1 => {
            // print the arguments
            for arg in arguments {
                print_variable(arg, line_number);
            }
        }
        2 => {
            // println the arguments
            for arg in arguments {
                print_variable(arg, line_number);
            }
            println!();
        }
        _ => error(
            ("Unknown native call ".to_string() + &function_name.to_string()).as_str(),
            line_number,
        ),
    }
}
