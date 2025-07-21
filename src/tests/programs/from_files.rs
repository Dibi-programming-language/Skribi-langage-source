use crate::execute;

macro_rules! file {
    ($str: expr) => {
        concat!("resources/test_programs/", $str, ".skrb")
    };
}

fn execute_from_name(name: &str) -> Result<(), ()> {
    execute(vec![
        "".to_owned(),
        name.to_owned(),
        "--compiler-debug".to_owned(),
        "--show-ast".to_owned(),
    ])
}

#[test]
fn test_addition() -> Result<(), ()> {
    execute_from_name(file!("addition"))
}

#[test]
fn test_complex_substraction() -> Result<(), ()> {
    execute_from_name(file!("complex"))
}

#[test]
fn test_switch() -> Result<(), ()> {
    execute_from_name(file!("switch"))
}

#[test]
fn test_division() -> Result<(), ()> {
    execute_from_name(file!("division"))
}

#[test]
fn test_multiplication() -> Result<(), ()> {
    execute_from_name(file!("multiplication"))
}

#[test]
fn test_ij_sula() -> Result<(), ()> {
    execute_from_name(file!("ij"))
}

#[test]
fn test_or_eq() -> Result<(), ()> {
    execute_from_name(file!("cmp/or_eq"))
}

#[test]
fn test_simple_cmp() -> Result<(), ()> {
    execute_from_name(file!("cmp/simple"))
}

#[test]
fn test_bool() -> Result<(), ()> {
    execute_from_name(file!("bool"))
}

#[test]
fn test_fibo() -> Result<(), ()> {
    execute_from_name(file!("algo/fibo"))
}
