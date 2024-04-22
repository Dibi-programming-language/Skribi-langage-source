use crate::tokens::{Token, tokenize};

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