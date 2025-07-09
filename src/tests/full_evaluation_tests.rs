use crate::execute::{Evaluate, ExecutionContext, OperationCleanOutput};
use crate::parse::nodes::operations::TakePriorityLast;
use crate::parse::nodes::Parsable;
use crate::tokens::tokenize;

fn assert_evaluation(file: String, expected: OperationCleanOutput) {
    let mut tokens = tokenize(file).unwrap();
    let ast = TakePriorityLast::parse(&mut tokens).unwrap().unwrap();
    println!("{:?}", ast);
    let result = ast.evaluate(&mut ExecutionContext::new()).unwrap();
    assert_eq!(result, expected, "{:?}", ast);
}

#[test]
fn test_addition() {
    assert_evaluation(String::from("1"), 1);
    assert_evaluation(String::from("1+1"), 2);
    assert_evaluation(String::from("1+2"), 3);
    assert_evaluation(String::from("1+2+3"), 6);
    assert_evaluation(String::from(" 1 + 2 + 3 "), 6);
}

#[test]
fn test_subtraction() {
    assert_evaluation(String::from("1-1"), 0);
    assert_evaluation(String::from("1 - 1"), 0);
    assert_evaluation(String::from("2 - 1"), 1);
}

#[test]
fn test_multiplication() {
    assert_evaluation(String::from("1*1"), 1);
    assert_evaluation(String::from("1*1*1"), 1);
    assert_evaluation(String::from("2*2*2"), 8);
}

#[test]
fn test_division() {
    assert_evaluation(String::from("1/1"), 1);
    assert_evaluation(String::from("8/2"), 4);
    assert_evaluation(String::from("7/2"), 3);
}

#[test]
fn test_combination() {
    assert_evaluation(String::from("1+2*3"), 7);
    assert_evaluation(String::from("2*3+1"), 7);
    assert_evaluation(String::from("2*3+1"), 7);

    assert_evaluation(String::from("1+2*3+1"), 8);
    assert_evaluation(String::from("1*2+3*4"), 14);

    assert_evaluation(String::from("3*4-1*2"), 10);
}

// TODO - fix in a later pull request
// #[test]
// fn test_combination_difficult() {
//     assert_evaluation(String::from("3+2-5"), 0);
//     assert_evaluation(String::from("2*5/10"), 1);
// }
