// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

// Import
use skribi_language_source::{clear, error, read};
use std::{env, io};

// Main
fn main() {
    let args: Vec<_> = env::args().collect(); // get the command line arguments
    let mut path = String::new();
    let extension: Vec<String> = vec!["sk".to_string(), "skribi".to_string()];
    let flag_char = "/"; // if it was "-", it would sometimes interfere with cargo's flags

    // clear the shell for the user
    if !args.contains(&String::from(flag_char.to_string() + "interpret-debug")) {
        clear();
    }

    // Get the path of the file to run
    if args.len() > 1 && !args[1].starts_with(flag_char) {
        path = args[1].clone();
    } else {
        println!("Enter a file to run");
        let _ = io::stdin().read_line(&mut path);
        path = path.trim().to_string();
    }

    // Check if the file has the right extension
    if !extension.contains(&String::from(path.split('.').last().unwrap())) {
        println!("Not a valid file extension");
        return;
    }

    // Read the file
    let lines = read(&path);

    // Remove the comments, be careful to not remove the comments in strings
    let mut in_string = false;
    let mut in_comment = false;
    let mut code: Vec<String> = vec![String::new()];
    for line in lines.iter() {
        let code_len = code.len() - 1;
        code[code_len] += " ";
        let current_line = line.trim();
        for (i, c) in current_line.chars().enumerate() {
            let is_max = i == current_line.trim().len() - 1;
            println!("{} {}", c, i);
            if !in_string && c == '/' {
                if !is_max {
                    if current_line.chars().nth(i + 1).unwrap() == '/' {
                        code[code_len] = code[code_len].trim().to_string();
                        break;
                    }
                    if current_line.chars().nth(i + 1).unwrap() == '*' {
                        in_comment = true;
                    }
                }
                if i != 0 && in_comment
                    && current_line.chars().nth(i - 1).unwrap() == '*'
                {
                    in_comment = false;
                    continue;
                }
            }
            if !in_comment {
                if c == '"' && (i == 0 || current_line.chars().nth(i - 1).unwrap() != '\\') {
                    in_string = !in_string;
                    code[code_len] += "\"";
                }
                else if in_string || !(code[code_len].len() != 0 && c == ' ' && code[code_len].chars().last().unwrap() == ' ') {
                    if c == ';' && !in_string {
                        code[code_len] = code[code_len].trim().to_string();
                        code.push(String::new());
                    } else {
                        code[code_len] += &c.to_string();
                    }
                }
            }
        }
        if in_string {
            error("Unclosed string on line {}")
        }
    }
    let code_len = code.len() - 1;
    code[code_len] = code[code_len].trim().to_string();

    // TODO interpret the code
    println!("{:?}", code);
}
