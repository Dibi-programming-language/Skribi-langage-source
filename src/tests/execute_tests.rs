use crate::execute::Evaluate;
use crate::parse::nodes::operations::TakePriorityLast;
use crate::parse::nodes::Parsable;
use crate::tokens::{Token, TokenContainer};
use std::collections::VecDeque;

#[test]
fn add_test() {
    let mut vec: VecDeque<TokenContainer> = vec![Token::Int(1), Token::Add, Token::Int(2)]
        .into_iter()
        .map(|x| x.into())
        .collect();

    let res = TakePriorityLast::parse(&mut vec)
        .unwrap()
        .unwrap()
        .evaluate(&mut ())
        .unwrap();
    assert_eq!(res, 3);
}
