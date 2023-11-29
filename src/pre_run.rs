use skribi_language_source::error;
use std::io;

pub fn get_path(args: Vec<String>, flag_char: &str) -> String {
    let mut path = String::new();
    // Get the path of the file to run
    if args.len() > 1 && !args[1].starts_with(flag_char) {
        path = args[1].clone();
    } else {
        println!("Enter a file to run");
        let _ = io::stdin().read_line(&mut path);
        path = path.trim().to_string();
    }
    path
}
pub fn get_instructions(lines: Vec<String>) -> Vec<String> {
    let mut in_string = false;
    let mut in_comment = false;
    let mut code: Vec<String> = vec![String::new()];
    for line in lines.iter() {
        let code_len = code.len() - 1;
        code[code_len] += " ";
        let current_line = line.trim();
        for (i, c) in current_line.chars().enumerate() {
            let is_max = i == current_line.trim().len() - 1;
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
                if i != 0 && in_comment && current_line.chars().nth(i - 1).unwrap() == '*' {
                    in_comment = false;
                    continue;
                }
            }
            if !in_comment {
                if c == '"' && (i == 0 || current_line.chars().nth(i - 1).unwrap() != '\\') {
                    in_string = !in_string;
                    code[code_len] += "\"";
                } else if in_string
                    || !(code[code_len].len() != 0
                        && c == ' '
                        && code[code_len].chars().last().unwrap() == ' ')
                {
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
    code
}
