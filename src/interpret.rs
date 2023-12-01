use skribi_language_source::{capsule_words, error};

pub fn main(code: Vec<String>, _args: Vec<String>) {
    // main loop of the interpreter
    let mut is_running = true;
    let mut line_number: u16 = 0;
    while is_running {
        // get the instructions on the current line
        let line = capsule_words(code[line_number as usize].clone());
        interpret(line, line_number);
        line_number += 1;
        if line_number >= code.len() as u16 {
            is_running = false;
        }
    }
}
fn interpret(line: Vec<String>, line_number: u16) {
    // TODO: interpret the code
    match line[0].as_str() {
        _ => error(("Unknown command on line ".to_string() + &line_number.to_string()).as_str()),
    }
}
