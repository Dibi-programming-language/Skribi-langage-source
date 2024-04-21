use std::io::ErrorKind;

use crate::FLAG_CHAR;
use crate::utils::{input, read};

/// This function is used to get the path of the file to run
///
/// The path can either be passed as an argument or entered the terminal
pub fn get_content(args: Vec<String>, extensions: Vec<String>) -> Result<String, ErrorKind> {
    if args.len() > 1 && !args[1].starts_with(FLAG_CHAR) {
        let path = args[1].clone();

        // Check if the file has the right extension
        let extension = String::from(path.split('.').last().unwrap());
        if !extensions.contains(&extension) {
            return Err(ErrorKind::InvalidInput);
        }

        // Read the file
        return Ok(read(&path));
    }

    let mut content = String::new();

    loop {
        let user_input = input("");
        if user_input.trim_end() == "" {break};
        content += &*user_input;
    }

    Ok(content)
}

/*
/// This function formats the code to be interpreted
///
/// It typically removes comments and splits the code into instructions while keeping strings intact
fn get_instructions(lines: Vec<String>) -> Vec<String> {
    let mut in_string = false;
    let mut in_comment = false;
    let mut code: Vec<String> = vec![String::new()];
    let mut code_len = 0;

    // iterate over the lines of code
    for line in lines.iter() {
        code[code_len] += " ";
        let current_line = line.trim();

        // iterate over the characters of the current line
        for (i, c) in current_line.chars().enumerate() {
            if !in_string && c == '/' {
                if i != current_line.trim().len() - 1 {
                    // check if the current character is the start of a comment that ends at the end of the line
                    if current_line.chars().nth(i + 1).unwrap() == '/' {
                        code[code_len] = code[code_len].trim().to_string();
                        break;
                    }
                    // check if the current character is the start of a comment that ends somewhere else
                    else if current_line.chars().nth(i + 1).unwrap() == '*' {
                        in_comment = true;
                    }
                }
                // check if the current character is the end of a comment
                if i != 0 && in_comment && current_line.chars().nth(i - 1).unwrap() == '*' {
                    in_comment = false;
                    continue;
                }
            }
            if !in_comment {
                // check if the current character is a string delimiter
                if c == '"' && (i == 0 || current_line.chars().nth(i - 1).unwrap() != '\\') {
                    in_string = !in_string;
                    code[code_len] += "\"";
                }
                // if the current character is a space and the previous character is a space, don't add it to the code
                else if in_string
                    || !(code[code_len].len() != 0
                    && c == ' '
                    && code[code_len].chars().last().unwrap() == ' ')
                {
                    // split the code into instructions when a semicolon is encountered
                    if c == ';' && !in_string {
                        code[code_len] = code[code_len].trim().to_string();
                        code.push(String::new());
                        code_len += 1;
                    } else {
                        code[code_len] += &c.to_string();
                    }
                }
            }
        }
        if in_string {
            error("Unclosed string", code_len as u16);
        }
    }
    code[code_len] = code[code_len].trim().to_string();
    code
}
*/