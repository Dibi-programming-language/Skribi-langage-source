mod variables;
mod native_call;

use crate::interpret::variables::{is_variable_type, Variable};
use skribi_language_source::{capsule_words, error};
use std::collections::HashMap;

/**
Main loop of the interpreter
 */
pub fn main(code: Vec<String>, _args: Vec<String>) {
    let mut line_number: u16 = 0;
    let mut is_running = line_number < code.len() as u16 - 1;
    let mut _variables: HashMap<String, Variable> = HashMap::new();
    while is_running {
        // get the instructions on the current line
        let line = capsule_words(code[line_number as usize].clone(), line_number);
        interpret(line, line_number, &mut _variables);
        line_number += 1;
        if line_number >= code.len() as u16 - 1 {
            is_running = false;
        }
    }
}

/**
Interpret a line of code
 */
fn interpret(line: Vec<String>, line_number: u16, variables: &mut HashMap<String, Variable>) {
    let scope_level: u8 = 1;

    let word = line[0].as_str();
    match word {
        "skr_app" => {
            // TEMPORARY - Call a native function
            native_call::native_call(line, line_number, variables);
        }
        "pu" | "fu" | "ju" => {
            create_variable(&line, line_number, variables, scope_level);
        }
        // In all other cases, a last check with a condition is made to see if the line is a variable assignment :
        _ => {
            if is_variable_type(word) {
                create_variable(&line, line_number, variables, scope_level);
            } else if variables.contains_key(word) {
                // TODO: assign a value to a variable
            } else {
                error(
                    ("Unknown command on line ".to_string() + &line_number.to_string()).as_str(),
                    line_number,
                );
            }
        }
    }
}

fn create_variable(line: &[String], line_number: u16, variables: &mut HashMap<String, Variable>, scope_level: u8) {
    // create a new variable
    let (temp, name) = Variable::new(line, scope_level, line_number);
    // check if the variable already exists
    if variables.contains_key(&name) {
        error(
            &format!("Variable {} already exists", name),
            line_number,
        );
    }
    (*variables).insert(name, temp);
}
