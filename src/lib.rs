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

/*
 * This function split a String on every space, except if the space is in a string or in parenthesis
 *
 * # Example
 * "Hello, I'm coding in "skribi language" (a programming language)"
 *
 * ->
 *
 * ["Hello,", "I'm", "coding", "in", "skribi language", "(a programming language)"]
 */
/* pub fn capsule_words(line: String, line_number: u16) -> Vec<String> {
    let mut capsule: Vec<String> = vec![String::from("")];
    let mut capsule_len = 0;
    let mut is_string = false;
    let mut in_par: u8 = 0;
    // iterate over the characters of the string
    for (i, c) in line.chars().enumerate() {
        // test if the current character is a string delimiter
        if c == '"' && (i == 0 || line.chars().nth(i - 1).unwrap() != '\\') {
            capsule[capsule_len] += "\"";
            is_string = !is_string;
        } else if !is_string {
            // test if the string is entering parenthesis
            if c == '(' {
                in_par += 1;
                capsule[capsule_len] += "(";
            // test if the string is exiting parenthesis
            } else if c == ')' {
                if in_par == 0 {
                    error("Unexpected ')'");
                }
                capsule[capsule_len] += ")";
                in_par -= 1;
            // test if the current character is between 2 words
            } else if c == ' ' && in_par == 0 {
                capsule.push(String::new());
                capsule_len += 1;
            // add the character to the current word
            } else {
                capsule[capsule_len] += &c.to_string();
            }
        } else {
            capsule[capsule_len] += &c.to_string();
        }
    }
    capsule
}
*/
