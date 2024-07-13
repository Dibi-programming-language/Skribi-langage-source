use crate::parse::nodes::expressions::{IdUse, IdUseV};
use crate::parse::nodes::operations::NoValue;
use crate::tokens::Token;

#[test]
fn test_simple_exp_id_use_v() {
    let tokens = vec![
        Token::Identifier("a".to_string()),
        Token::Add,
        Token::Int(1),
    ];

    let tokens2 = vec![Token::Add, Token::Int(1)];

    let mut tokens = tokens.into_iter().collect();
    let mut tokens2 = tokens2.into_iter().collect();
    let id_use_v = IdUseV::parse(&mut tokens);

    match id_use_v {
        Ok(Some(id_use_v)) => {
            assert_eq!(
                IdUseV::new(
                    IdUse::new_set(
                        
                        None
                    ),
                    Some(NoValue::parse(&mut tokens2).unwrap())
                ),
                id_use_v
            );
        }
        Ok(None) => panic!("Error parsing IdUseV: None"),
        Err(err) => panic!("Error parsing IdUseV: {:?}", err),
    }
}
