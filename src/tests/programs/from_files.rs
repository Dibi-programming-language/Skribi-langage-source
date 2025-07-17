use crate::execute;

macro_rules! file {
    ($str: expr) => {
        concat!("resources/test_programs/", $str, ".skrb")
    };
}

fn execute_from_name(name: &str) {
    execute(vec![
        "".to_owned(),
        name.to_owned(),
        "--compiler-debug".to_owned(),
        "--show-ast".to_owned(),
    ]);
}

#[test]
fn test_addition() {
    execute_from_name(file!("addition"));
}

#[test]
fn test_complex_substraction() {
    execute_from_name(file!("complex"));
}

#[test]
fn test_switch() {
    execute_from_name(file!("switch"));
}

#[test]
fn test_division() {
    execute_from_name(file!("division"));
}

#[test]
fn test_multiplication() {
    execute_from_name(file!("multiplication"));
}
