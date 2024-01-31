use std::process::{exit, Command};
use std::fs;
use std::fmt::Display;
use std::io::{ErrorKind, stdin, stdout, Write};

/**
 * This function clear the shell
 */
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
/**
 * This function print an error message in red and stop the program
 */
pub fn error<T: Display>(message: T) {
    //print the error message in red
    println!("\x1b[31mError: {}\x1b[0m", message);
    exit(0);
}

/**
 * This function read all the content from a file and return a vector of String, each string being a line of the file
 */
pub fn read(file_name: &str) -> String {
    let content_option = fs::read_to_string(file_name);

    match content_option {
        Ok(file_content) => {
            file_content
        }
        Err(err) => {
            let error_title = String::from("Cannot read file ") + "\"file_name\": ";
            let error_message = match err.kind() {
                ErrorKind::InvalidData => "bad encoding",
                ErrorKind::TimedOut => "the file took too long to answer",
                ErrorKind::PermissionDenied => "permission denied",
                ErrorKind::NotFound => "file not found",
                _ => "unknown error"
            };
            error(error_title + error_message);
            String::new()
        }
    }
}

/**
 * This function ask the user for an input and return the user's answer
*/
pub fn input<T: Display>(message: T) -> String {
    print!("{}", message);
    stdout().flush().unwrap();

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    user_input
}
