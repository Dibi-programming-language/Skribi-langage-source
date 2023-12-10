use std::process::{exit, Command};
use std::{
    fs::File,
    io::{self, BufRead},
};
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
pub fn error(message: &str, line: u16) {
    //print the error message in red
    println!("\x1b[31mError: {} in instruction {}\x1b[0m", message, line + 1);
    exit(0);
}
/**
 * This function read all the content from a file and return a vector of String, each string being a line of the file
 */
pub fn read(file_name: &str) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    match File::open(file_name) {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            // read the file line by line
            for line in reader.lines() {
                match line {
                    Ok(text) => {
                        lines.push(text);
                    }
                    Err(err) => {
                        if err.kind() == io::ErrorKind::InvalidData {
                            error("Cannot read file: Bad encoding", 0);
                        }
                        error("Cannot read file: Unknown error", 0);
                    }
                }
            }
        }
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                error("Cannot open file: File not found", 0);
            } else if err.kind() == io::ErrorKind::PermissionDenied {
                error("Cannot open file: Permission denied", 0);
            }
            error("Cannot open file: Unknown error", 0)
        }
    }
    lines
}
/**
 * This function split a String on every space, except if the space is in a string or in parenthesis
 *
 * # Example
 * "Hello, I'm coding in "skribi language" (a programming language)"
 *
 * ->
 *
 * ["Hello,", "I'm", "coding", "in", "skribi language", "(a programming language)"]
 */
pub fn capsule_words(line: String, line_number: u16) -> Vec<String> {
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
                    error("Unexpected ')'", line_number);
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
