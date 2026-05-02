use std::fmt::Display;
use std::fs;
use std::io::{stdin, stdout, ErrorKind, Write};
use std::process::Command;

/// This function clears the shell
pub fn clear() {
    // Tries the classic command
    if Command::new("clear").status().is_err() {
        // In case of error, tries another possible command (ex on Windows)
        if Command::new("cls").status().is_err() {
            // Just print to simulate a clear
            // Use ANSI escape codes to clear the screen and reset the cursor position
            print!("\x1B[2J\x1B[H");
            stdout().flush().unwrap();
        }
    }
}

/// This fonction will read the file with the given name and return the content of the file. This is
/// a wrapper around the [fs::read_to_string] function to return the right type.
pub fn read(file_name: &str) -> Result<String, ErrorKind> {
    let content_option = fs::read_to_string(file_name);
    match content_option {
        Ok(content) => Ok(content),
        Err(err) => Err(err.kind()),
    }
}

/// This function ask the user for an input and return the user's answer
pub fn input<T: Display>(message: T) -> String {
    print!("{}", message);
    stdout().flush().unwrap();

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    user_input
}
