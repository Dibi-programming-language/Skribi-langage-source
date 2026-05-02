use crate::execute::{Evaluate, ExecutionContext};
use crate::parse::nodes::operations::TakePriorityLast;
use crate::parse::nodes::Parsable;
use crate::tokens::{Token, TokenContainer};
use std::collections::VecDeque;

/// Test if 1 + 2 = 3
#[test]
fn add_test() {
    let mut vec: VecDeque<TokenContainer> = vec![Token::Int(1), Token::Add, Token::Int(2)]
        .into_iter()
        .map(|x| x.into())
        .collect();

    let res = TakePriorityLast::parse(&mut vec)
        .expect("Fail with error to parse TakePriorityLast.")
        .expect("TakePriorityLast should not return None.")
        .evaluate(&mut ExecutionContext::new())
        .expect("Evaluation of 1 + 2 should not fail.");
    assert_eq!(res, 3);
}
