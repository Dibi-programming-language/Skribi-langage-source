use skribi_language_source::error;

pub(crate) enum Token {
    KeywordGlobal,
    KeywordConstant,
    KeywordPrivate,
    KeywordIf,
    KeywordElse,
    KeywordNativeCall,
    Identifier(String),
    String(String),
    Integer(isize),
    // Always positive on tokenization step
    Float(f32),
    Boolean(bool),
    TwoDots,
    Semicolon,
    OpenParenthesis,
    CloseParenthesis,
    OperatorAdd,
    OperatorSub,
    OperatorMul,
    OperatorDiv,
    OperatorMod,
    OperatorPow,
    NewLine,
    Invalid,
}

enum State {
    Base,
    InString,
    InWord,
    // Identifier or Keyword
    InNumber,
    InComment,
}

pub(crate) fn tokenize(file: String) -> Vec<Token> {
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
                    current_token.push(
                        match c {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '0' => '\0',
                            _ => c
                        }
                    );
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
                        "fu" => Token::KeywordGlobal,
                        "ju" => Token::KeywordConstant,
                        "pu" => Token::KeywordPrivate,
                        "ij" => Token::KeywordIf,
                        "sula" => Token::KeywordElse,
                        "skr_app" => Token::KeywordNativeCall,
                        "ioial" => Token::Boolean(true),
                        "noial" => Token::Boolean(false),
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
                        error("Invalid number", line);
                    } else {
                        number_is_float = true;
                        current_token.push(c);
                    }
                } else {
                    tokens.push(if number_is_float {
                        Token::Float(current_token.parse().unwrap())
                    } else {
                        Token::Integer(current_token.parse().unwrap())
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
    tokens
}

fn base_tokenize(tokens: &mut Vec<Token>, current_token: &mut String, line: &mut u16, c: char) -> State {
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
            ':' => Token::TwoDots,
            ';' => Token::Semicolon,
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            '+' => Token::OperatorAdd,
            '-' => Token::OperatorSub,
            '*' => Token::OperatorMul,
            '/' => Token::OperatorDiv,
            '%' => Token::OperatorMod,
            '^' => Token::OperatorPow,
            '\n' => {
                *line += 1;
                Token::NewLine
            }
            _ => Token::Invalid
        };
        if let Token::Invalid = token {
            error(&format!("Invalid character {}", c), *line);
            State::Base
        } else if let Token::OperatorDiv = token {
            if let Some(var) = tokens.last() {
                if let Token::OperatorDiv = var {
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
