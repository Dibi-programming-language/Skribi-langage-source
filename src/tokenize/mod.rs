mod token_types;
pub use token_types::{ Token };

pub fn tokenize(content: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_char_index: usize = 0;

    while current_char_index < content.len() {
        let current_char = content.chars().nth(current_char_index).unwrap();
        match current_char {
            ' ' | '\n' | '\t' => {},

            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '%' => tokens.push(Token::Percent),
            '^' => tokens.push(Token::Caret),

            '(' => tokens.push(Token::OpenParenthesis),
            ')' => tokens.push(Token::CloseParenthesis),

            _ => {
                let (long_token, new_current_char_index) = get_long_tokens(content.clone(), current_char_index, current_char);
                current_char_index = new_current_char_index;

                match long_token {
                    Some(token) => tokens.push(token),
                    None => {},
                }
            }
        }
        current_char_index += 1;
    }

    tokens
}
fn get_long_tokens(content: String, mut current_char_index: usize, current_char: char) -> (Option<Token>, usize) {
    let mut long_token: Option<Token> = None;
    let mut word = String::new();

    word.push(current_char);

    if current_char.is_numeric() {

        while current_char_index < content.len()-1 && content.chars().nth(current_char_index + 1).unwrap().is_numeric() {
            current_char_index += 1;
            word.push(content.chars().nth(current_char_index).unwrap());
        }

        long_token = Some(Token::IntLiteral(word.parse::<i32>().unwrap()));
    }
    else if current_char.is_alphabetic() {

        while current_char_index < content.len()-1 && content.chars().nth(current_char_index + 1).unwrap().is_alphanumeric() {
            current_char_index += 1;
            word.push(content.chars().nth(current_char_index).unwrap());
        }

        match word.as_str() {
            "exit" => long_token = Some(Token::Exit),
            "true" => long_token = Some(Token::BooleanLiteral(true)),
            "false" => long_token = Some(Token::BooleanLiteral(false)),
            _ => {} // TODO:
        }
    }
    else if current_char == '"' {
        word = String::new();
        let mut is_escaped = false;
        let mut old_is_escaped = false;

        while current_char_index < content.len()-1 && !(content.chars().nth(current_char_index + 1).unwrap() == '"' && !is_escaped) {
            current_char_index += 1;
            old_is_escaped = is_escaped;
            is_escaped = content.chars().nth(current_char_index).unwrap() == '\\' && !old_is_escaped;
            if !is_escaped {
                word.push(content.chars().nth(current_char_index).unwrap());
            }
        }

        current_char_index += 1;
        long_token = Some(Token::StringLiteral(word));
    }

    return (long_token, current_char_index);
}
