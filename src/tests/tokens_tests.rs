use crate::skr_errors::ParsingError;
use crate::token_m;
use crate::tokens::TokenContainer;
use crate::tokens::{tokenize, SpaceTypes, Token};
use std::collections::VecDeque;

#[test]
fn test_simple() {
    let content = String::from("1 + 2");
    let tokens_res = tokenize(content);

    match tokens_res {
        Ok(tokens) => {
            assert_eq!(tokens.len(), 3);

            assert!(matches!(tokens[0], token_m!(Token::Int(1))));
            assert!(matches!(tokens[1], token_m!(Token::Add)));
            assert!(matches!(tokens[2], token_m!(Token::Int(2))));
        }
        Err(_) => {
            panic!("Error while tokenizing the content");
        }
    }
}

fn assert_valid_tokens(
    expected: Vec<Token>,
    actual: Result<VecDeque<TokenContainer>, ParsingError>,
) {
    match actual {
        Ok(tokens) => {
            assert_eq!(tokens.len(), expected.len());
            for (token, expected) in tokens.iter().zip(expected.iter()) {
                assert_eq!(token.token, *expected)
            }
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
    let expected = vec![
        Token::Int(1),
        Token::Add,
        Token::Int(2),
        Token::Add,
        Token::Int(3),
        Token::Add,
        Token::Int(4),
        Token::Add,
        Token::Int(5),
        Token::Mul,
        Token::Int(2),
        Token::Sub,
        Token::Int(15),
        Token::Div,
        Token::Int(8),
        Token::Add,
        Token::Int(125),
        Token::Space(SpaceTypes::NewLine),
    ];

    assert_valid_tokens(expected, tokens_res);
}

#[test]
fn test_simple_word() {
    let content = String::from("hello");
    let tokens_res = tokenize(content);
    let expected = vec![Token::Identifier(String::from("hello"))];

    assert_valid_tokens(expected, tokens_res);
}

#[test]
fn test_simple_string() {
    let content = String::from("\"hello\"");
    let tokens_res = tokenize(content);
    let expected = vec![Token::String(String::from("hello"))];

    assert_valid_tokens(expected, tokens_res);
}

#[test]
fn test_simple_string_with_space() {
    let content = String::from("\"hello world\"");
    let tokens_res = tokenize(content);
    let expected = vec![Token::String(String::from("hello world"))];

    assert_valid_tokens(expected, tokens_res);
}

#[test]
fn test_simple_string_with_space_and_escape() {
    let content = String::from("\"hello \\\"world\\\"\"");
    let tokens_res = tokenize(content);
    let expected = vec![Token::String(String::from("hello \"world\""))];

    assert_valid_tokens(expected, tokens_res);
}

#[test]
fn test_strings() {
    let content = String::from("\"hello\" \"world\"");
    let tokens_res = tokenize(content);
    let expected = vec![
        Token::String(String::from("hello")),
        Token::String(String::from("world")),
    ];

    assert_valid_tokens(expected, tokens_res);
}

#[test]
fn test_strings_hard_1() {
    // escape, special characters, alphanumerics, etc.
    let content = String::from("\"start with simple\" \"harder 11234 5489 \\\"\" \"escape characters \\n \\t \\r \\0\" \"special ... ^éà@¨ï$\"");
    let tokens_res = tokenize(content);
    let expected = vec![
        Token::String(String::from("start with simple")),
        Token::String(String::from("harder 11234 5489 \"")),
        Token::String(String::from("escape characters \n \t \r \0")),
        Token::String(String::from("special ... ^éà@¨ï$")),
    ];

    assert_valid_tokens(expected, tokens_res);
}

#[test]
fn test_code_1() {
    let content = String::from("ums my_function() {\nei 1 + 2\n}\n\nmy_function()");
    let tokens_res = tokenize(content);
    let expected = vec![
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
    ];

    assert_valid_tokens(expected, tokens_res);
}

#[test]
fn test_float() {
    let content = String::from("1.0 + 2.0");
    let tokens_res = tokenize(content);
    let expected = vec![Token::Float(1.0), Token::Add, Token::Float(2.0)];

    assert_valid_tokens(expected, tokens_res);
}
