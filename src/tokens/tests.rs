use crate::tokens::NewTokens;
use crate::tokens::new_tokenise;

macro_rules! assert_valid {
    ($tokens: expr, $expected: expr) => {
        assert_eq!($tokens.nth(0).unwrap().unwrap(), $expected);
    };
}

#[test]
fn test_simple() {
    let content = "1 + 2";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);

    assert_valid!(tokens, NewTokens::Int(1));
    assert_valid!(tokens, NewTokens::Add);
    assert_valid!(tokens, NewTokens::Int(2));
    assert!(tokens.nth(0).is_none())
}

#[test]
fn test_simple_chain() {
    let content = "1 + 2 + 3 + 4 + 5 * 2 - 15 / 8 + 125 // abcd";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);
    assert_valid!(tokens, NewTokens::Int(1));
    assert_valid!(tokens, NewTokens::Add);
    assert_valid!(tokens, NewTokens::Int(2));
    assert_valid!(tokens, NewTokens::Add);
    assert_valid!(tokens, NewTokens::Int(3));
    assert_valid!(tokens, NewTokens::Add);
    assert_valid!(tokens, NewTokens::Int(4));
    assert_valid!(tokens, NewTokens::Add);
    assert_valid!(tokens, NewTokens::Int(5));
    assert_valid!(tokens, NewTokens::Mul);
    assert_valid!(tokens, NewTokens::Int(2));
    assert_valid!(tokens, NewTokens::Sub);
    assert_valid!(tokens, NewTokens::Int(15));
    assert_valid!(tokens, NewTokens::Div);
    assert_valid!(tokens, NewTokens::Int(8));
    assert_valid!(tokens, NewTokens::Add);
    assert_valid!(tokens, NewTokens::Int(125));
    assert_eq!(tokens.nth(0), None)
}

#[test]
fn test_simple_word() {
    let content = "hello";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);
    assert_valid!(tokens, NewTokens::Identifier("hello"));
    assert!(tokens.nth(0).is_none())
}

#[test]
fn test_simple_string() {
    let content = "\"hello\"";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);
    assert_valid!(tokens, NewTokens::String("\"hello\""));
    assert!(tokens.nth(0).is_none())
}

#[test]
fn test_simple_string_with_space() {
    let content = "\"hello world\"";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);
    assert_valid!(tokens, NewTokens::String("\"hello world\""));
    assert!(tokens.nth(0).is_none())
}

#[test]
fn test_simple_string_with_space_and_escape() {
    let content = "\"hello \\\"world\\\"\"";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);
    assert_valid!(tokens, NewTokens::String("\"hello \\\"world\\\"\""));
    assert!(tokens.nth(0).is_none())
}

#[test]
fn test_strings() {
    let content = "\"hello\" \"world\"";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);
    assert_valid!(tokens, NewTokens::String("\"hello\""));
    assert_valid!(tokens, NewTokens::String("\"world\""));
    assert!(tokens.nth(0).is_none())
}

#[test]
fn test_strings_hard_1() {
    // escape, special characters, alphanumerics, etc.
    let content = "\"start with simple\" \"harder 11234 5489 \\\"\" \"escape characters \\n \\t \\r \\0\" \"special ... ^éà@¨ï$\"";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);
    assert_valid!(tokens, NewTokens::String("\"start with simple\""));
    assert_valid!(tokens, NewTokens::String("\"harder 11234 5489 \\\"\""));
    assert_valid!(tokens, NewTokens::String("\"escape characters \\n \\t \\r \\0\""));
    assert_valid!(tokens, NewTokens::String("\"special ... ^éà@¨ï$\""));
    assert!(tokens.nth(0).is_none())
}

#[test]
fn test_code_1() {
    let content = "ums my_function() {\nei 1 + 2\n}\n\nmy_function()";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);
    assert_valid!(tokens, NewTokens::KeywordFunction);
    assert_valid!(tokens, NewTokens::Identifier("my_function"));
    assert_valid!(tokens, NewTokens::LeftParenthesis);
    assert_valid!(tokens, NewTokens::RightParenthesis);
    assert_valid!(tokens, NewTokens::LeftBrace);
    assert_valid!(tokens, NewTokens::KeywordReturn);
    assert_valid!(tokens, NewTokens::Int(1));
    assert_valid!(tokens, NewTokens::Add);
    assert_valid!(tokens, NewTokens::Int(2));
    assert_valid!(tokens, NewTokens::RightBrace);
    assert_valid!(tokens, NewTokens::Identifier("my_function"));
    assert_valid!(tokens, NewTokens::LeftParenthesis);
    assert_valid!(tokens, NewTokens::RightParenthesis);
    assert!(tokens.nth(0).is_none())
}

#[test]
fn test_float() {
    let content = "1.0 + 2.0";
    let mut tokens = new_tokenise(content).map(|(a, _)| a);
    assert_valid!(tokens, NewTokens::Float(1.0));
    assert_valid!(tokens, NewTokens::Add);
    assert_valid!(tokens, NewTokens::Float(2.0));
    assert!(tokens.nth(0).is_none())
}
