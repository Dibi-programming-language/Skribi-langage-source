use crate::parse::nodes::id_nodes::{parse_cget, parse_id_set, CGet, IdGet, IdSet, OpIn};
use crate::skr_errors::CustomError;
use crate::tokens::Token;
use std::collections::VecDeque;

#[test]
fn test_id_simple() {
    let c_get_1 = CGet {
        name: String::from("hello"),
    };
    let c_get_2 = CGet {
        name: String::from("world"),
    };

    assert_ne!(c_get_1, c_get_2);

    let c_get_3 = CGet {
        name: String::from("hello"),
    };

    assert_eq!(c_get_1, c_get_3);
}

#[test]
fn test_parse_id_simple() {
    // Hello is not a "type def"

    let mut tokens = vec![Token::Identifier(String::from("hello"))]
        .into_iter()
        .collect();
    let c_get = parse_cget(&mut tokens);

    assert_eq!(None, c_get);

    // dar is a "type def"

    let mut tokens = vec![Token::Identifier(String::from("dar"))]
        .into_iter()
        .collect();
    let c_get = parse_cget(&mut tokens).unwrap();

    assert_eq!(
        CGet {
            name: String::from("dar")
        },
        c_get
    );
}

#[test]
fn test_parse_set_maxi() {
    // test with "maxi:mini:hello:dar"

    let mut tokens: VecDeque<Token> = vec![
        Token::Identifier(String::from("maxi")),
        Token::Inside,
        Token::Identifier(String::from("mini")),
        Token::Inside,
        Token::Identifier(String::from("hello")),
        Token::Inside,
        Token::Identifier(String::from("dar")),
    ]
    .into_iter()
    .collect();

    let res = parse_id_set(&mut tokens);
    let expected: Option<Result<IdSet, CustomError>> = Some(Ok(IdSet {
        identifier: String::from("maxi"),
        op_in: Box::new(OpIn::IdGet(IdGet {
            identifier: String::from("mini"),
            tuple: None,
            op_in: Box::new(OpIn::IdGet(IdGet {
                identifier: String::from("hello"),
                tuple: None,
                op_in: Box::new(OpIn::CGet(CGet {
                    name: String::from("dar"),
                })),
            })),
        })),
    }));

    assert_eq!(expected, res);
}
