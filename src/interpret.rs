mod variables;

use crate::interpret::variables::{new_variable, VariableStruct};
use skribi_language_source::{capsule_words, error};
use std::collections::HashMap;

pub fn main(code: Vec<String>, _args: Vec<String>) {
    // main loop of the interpreter
    let mut line_number: u16 = 0;
    let mut is_running = line_number < code.len() as u16 - 1;
    let mut _variables: HashMap<String, VariableStruct> = HashMap::new();
    while is_running {
        // get the instructions on the current line
        let line = capsule_words(code[line_number as usize].clone());
        interpret(line, line_number, &mut _variables);
        line_number += 1;
        if line_number >= code.len() as u16 - 1 {
            is_running = false;
        }
    }
}
fn interpret(line: Vec<String>, line_number: u16, variables: &mut HashMap<String, VariableStruct>) {
    let scope_level: u8 = 1;

    match line[0].as_str() {
        "pu" | "fu" | "ju" => {
            let mut temp = new_variable(line, scope_level);
            let name = temp.name.clone();
            (*variables).insert(name, temp);
        }
        _ => error(("Unknown command on line ".to_string() + &line_number.to_string()).as_str()),
    }
}
