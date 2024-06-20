use crate::tokens::{tokenize, SpaceTypes, Token};

#[test]
fn test_simple() {
    let content = String::from("1 + 2");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(tokens.len(), 3);

            assert!(matches!(tokens[0], Token::Int(1)));
            assert!(matches!(tokens[1], Token::Add));
            assert!(matches!(tokens[2], Token::Int(2)));
        }
        Err(_) => {
            panic!("Error while tokenizing the content");
        }
    }
}

#[test]
fn test_simple_chain() {
    let content = String::from("1 + 2 + 3 + 4 + 5 * 2 - 15 / 8 + 125 // abcd");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(
                vec![
                    Token::Int(1),
                    Token::Add,
                    Token::Int(2),
                    Token::Add,
                    Token::Int(3),
                    Token::Add,
                    Token::Int(4),
                    Token::Add,
                    Token::Int(5),
                    Token::Mult,
                    Token::Int(2),
                    Token::Sub,
                    Token::Int(15),
                    Token::Div,
                    Token::Int(8),
                    Token::Add,
                    Token::Int(125),
                    Token::Space(SpaceTypes::NewLine)
                ],
                tokens
            );
        }
        Err(_) => {
            panic!("Error while tokenizing the content");
        }
    }
}

#[test]
fn test_simple_word() {
    let content = String::from("hello");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(vec![Token::Identifier(String::from("hello"))], tokens);
        }
        Err(_) => {
            panic!("Error while tokenizing the content");
        }
    }
}

#[test]
fn test_simple_string() {
    let content = String::from("\"hello\"");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(vec![Token::String(String::from("hello"))], tokens);
        }
        Err(error) => {
            panic!("Error while tokenizing the content {:?}", error);
        }
    }
}

#[test]
fn test_simple_string_with_space() {
    let content = String::from("\"hello world\"");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(vec![Token::String(String::from("hello world"))], tokens);
        }
        Err(error) => {
            panic!("Error while tokenizing the content {:?}", error);
        }
    }
}

#[test]
fn test_simple_string_with_space_and_escape() {
    let content = String::from("\"hello \\\"world\\\"\"");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(vec![Token::String(String::from("hello \"world\""))], tokens);
        }
        Err(error) => {
            panic!("Error while tokenizing the content {:?}", error);
        }
    }
}

#[test]
fn test_strings() {
    let content = String::from("\"hello\" \"world\"");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(
                vec![
                    Token::String(String::from("hello")),
                    Token::String(String::from("world"))
                ],
                tokens
            );
        }
        Err(error) => {
            panic!("Error while tokenizing the content {:?}", error);
        }
    }
}

#[test]
fn test_strings_hard_1() {
    // escape, special characters, alphanumerics, etc.
    let content = String::from("\"start with simple\" \"harder 11234 5489 \\\"\" \"escape characters \\n \\t \\r \\0\" \"special ... ^éà@¨ï$\"");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(
                vec![
                    Token::String(String::from("start with simple")),
                    Token::String(String::from("harder 11234 5489 \"")),
                    Token::String(String::from("escape characters \n \t \r \0")),
                    Token::String(String::from("special ... ^éà@¨ï$"))
                ],
                tokens
            );
        }
        Err(error) => {
            panic!("Error while tokenizing the content {:?}", error);
        }
    }
}

#[test]
fn test_code_1() {
    let content = String::from("ums my_function() {\nei 1 + 2\n}\n\nmy_function()");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(
                vec![
                    Token::KeywordFunction,
                    Token::Identifier(String::from("my_function")),
                    Token::LeftParenthesis,
                    Token::RightParenthesis,
                    Token::LeftBrace,
                    Token::Space(SpaceTypes::NewLine),
                    Token::KeywordReturn,
                    Token::Int(1),
                    Token::Add,
                    Token::Int(2),
                    Token::Space(SpaceTypes::NewLine),
                    Token::RightBrace,
                    Token::Space(SpaceTypes::NewLine),
                    Token::Space(SpaceTypes::NewLine),
                    Token::Identifier(String::from("my_function")),
                    Token::LeftParenthesis,
                    Token::RightParenthesis,
                ],
                tokens
            );
        }
        Err(error) => {
            panic!("Error while tokenizing the content {:?}", error);
        }
    }
}

#[test]
fn test_float() {
    let content = String::from("1.0 + 2.0");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(
                vec![Token::Float(1.0), Token::Add, Token::Float(2.0),],
                tokens
            );
        }
        Err(error) => {
            panic!("Error while tokenizing the content {:?}", error);
        }
    }
}
