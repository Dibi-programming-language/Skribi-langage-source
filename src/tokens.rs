use std::str::Chars;
use crate::skr_errors::CustomError;

pub enum ModifierKeyword {
    Global,
    Constant,
    Private,
}

pub enum ValueToken {
    String(String),
    /// Always positive on tokenization step
    Integer(isize),
    Float(f32),
    Boolean(bool),
}

pub enum Space {
    Space,
    NewLine,
    Tab,
}

pub enum Token {
    Bool(bool),
    Int(u32),
    Float(f32),
    String(String),
    NatCall,
    Add,
    Sub,
    Div,
    Mult,
    // TODO : Pow
    // TODO : and, or, xor, not
    // TODO : comparison operators
    Plus,
    Minus,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Inside,
    Identifier(String),
    Space(Space),
    KeywordModifier(ModifierKeyword),
    KeywordIf,
    KeywordElse,
    KeywordClass,
    KeywordFunction,
    Invalid(String), // Any character not used by other tokens, only used when parsing bloc title
}

enum State {
    Base,
    InString,
    /// Identifier or keyword
    InWord,
    InNumber,
    InComment,
}

fn tokenize_string(mut file: Chars<'_>) -> Result<Token, CustomError> {
    let mut current_ch = file.next();
    let mut res = String::new();
    
    while let Some(ch) = current_ch {
        if ch == '"' {
            return Ok(Token::String(res));
        }
        res.push(ch);
        current_ch = file.next();
    }
    
    Err(CustomError::InvalidString("String not closed".to_string(), 0))
}

pub(crate) fn tokenize(file: String) -> Result<Vec<Token>, CustomError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token = String::new();
    let mut state = State::Base;
    let mut string_escape = false;
    let mut number_is_float = false;
    let mut line = 1;

    for c in file.chars() {
        match state {
            State::Base => {
                state = base_tokenize(&mut tokens, &mut current_token, &mut line, c);
            }
            State::InString => {
                if string_escape {
                    current_token.push(match c {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '0' => '\0',
                        _ => c,
                    });
                    string_escape = false;
                } else if c == '\\' {
                    string_escape = true;
                } else if c == '"' {
                    tokens.push(Token::String(current_token.clone()));
                    current_token.clear();
                    state = State::Base;
                } else {
                    current_token.push(c);
                }
            }
            State::InWord => {
                if c.is_alphanumeric() || c == '_' {
                    current_token.push(c);
                } else {
                    tokens.push(match current_token.as_str() {
                        "fu" => Token::KeywordModifier(ModifierKeyword::Global),
                        "ju" => Token::KeywordModifier(ModifierKeyword::Constant),
                        "pu" => Token::KeywordModifier(ModifierKeyword::Private),
                        "ij" => Token::KeywordIf,
                        "sula" => Token::KeywordElse,
                        "skr_app" => Token::NatCall,
                        "io" => Token::Bool(true),
                        "no" => Token::Bool(false),
                        _ => Token::Identifier(current_token.clone()),
                    });
                    current_token.clear();
                    state = base_tokenize(&mut tokens, &mut current_token, &mut line, c);
                }
            }
            State::InNumber => {
                if c.is_numeric() {
                    current_token.push(c);
                } else if c == '.' {
                    if number_is_float {
                        return Err(CustomError::InvalidFloat("A float can have only one . !".to_string(), line));
                    } else {
                        number_is_float = true;
                        current_token.push(c);
                    }
                } else {
                    tokens.push(if number_is_float {
                        Token::Float(current_token.parse().unwrap())
                    } else {
                        Token::Int(current_token.parse().unwrap())
                    });
                    current_token.clear();
                    state = base_tokenize(&mut tokens, &mut current_token, &mut line, c);
                }
            }
            State::InComment => {
                if c == '\n' {
                    line += 1;
                    state = State::Base;
                }
            }
        }
    }
    Ok(tokens)
}

fn base_tokenize(
    tokens: &mut Vec<Token>,
    current_token: &mut String,
    line: &mut u16,
    c: char,
) -> State {
    if c == '"' {
        State::InString
    } else if c.is_alphabetic() || c == '_' {
        current_token.push(c);
        State::InWord
    } else if c.is_numeric() {
        current_token.push(c);
        State::InNumber
    } else if c == ' ' {
        State::Base
    } else {
        let token = match c {
            ':' => Token::Inside,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mult,
            '/' => Token::Div,
            '\n' => {
                *line += 1;
                Token::Space(Space::NewLine)
            }
            _ => Token::Invalid(c.to_string()),
        };
        if let Token::Div = token {
            if let Some(var) = tokens.last() {
                if let Token::Div = var {
                    tokens.remove(tokens.len() - 1);
                    State::InComment
                } else {
                    tokens.push(token);
                    State::Base
                }
            } else {
                tokens.push(token);
                State::Base
            }
        } else {
            tokens.push(token);
            State::Base
        }
    }
}
