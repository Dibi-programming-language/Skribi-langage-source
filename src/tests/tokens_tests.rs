use crate::tokens::{Space, Token, tokenize};

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
    let content = String::from("1 + 2 + 3 + 4 + 5 * 2 - 15 / 8 //");
    let tokens_res = tokenize(content);
    
    match tokens_res {
        Ok(tokens) => {
            assert_eq!(vec![
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
                Token::Space(Space::NewLine)
            ], tokens);
        }
        Err(_) => {
            panic!("Error while tokenizing the content");
        }
    }
}