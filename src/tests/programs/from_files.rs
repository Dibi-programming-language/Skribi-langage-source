use crate::{execute, skr_errors::RootError};

macro_rules! file {
    ($str: expr) => {
        concat!("resources/test_programs/", $str, ".skrb")
    };
}

fn execute_from_name(name: &str) -> Result<(), RootError> {
    execute(
        vec!["".to_owned(), name.to_owned(), "--show-ast".to_owned()],
        true,
    )
}

#[test]
fn test_addition() -> Result<(), RootError> {
    execute_from_name(file!("addition"))
}

#[test]
fn test_complex_substraction() -> Result<(), RootError> {
    execute_from_name(file!("complex"))
}

#[test]
fn test_switch() -> Result<(), RootError> {
    execute_from_name(file!("switch"))
}

#[test]
fn test_division() -> Result<(), RootError> {
    execute_from_name(file!("division"))
}

#[test]
fn test_multiplication() -> Result<(), RootError> {
    execute_from_name(file!("multiplication"))
}

#[test]
fn test_ij_sula() -> Result<(), RootError> {
    execute_from_name(file!("ij"))
}

#[test]
fn test_or_eq() -> Result<(), RootError> {
    execute_from_name(file!("cmp/or_eq"))
}

#[test]
fn test_simple_cmp() -> Result<(), RootError> {
    execute_from_name(file!("cmp/simple"))
}

#[test]
fn test_bool() -> Result<(), RootError> {
    execute_from_name(file!("bool"))
}

#[test]
fn test_fibo() -> Result<(), RootError> {
    execute_from_name(file!("algo/fibo"))
}
