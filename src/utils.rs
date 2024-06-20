use std::fmt::Display;
use std::fs;
use std::io::{stdin, stdout, ErrorKind, Write};
use std::process::Command;

/// This function clear the shell
pub fn clear() {
    match Command::new("clear").status() {
        Ok(_) => {}
        Err(_) => match Command::new("cls").status() {
            Ok(_) => {}
            Err(_) => {
                for _ in 0..100 {
                    println!()
                }
            }
        },
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
