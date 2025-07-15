use crate::execute;

macro_rules! file {
    ($str: expr) => {
        concat!("resources/test_programs/", $str, ".skrb")
    };
}

fn execute_from_name(name: &str) {
    execute(vec!["".to_owned(), name.to_owned(), "--compiler-debug".to_owned()]);
}

#[test]
fn test_addition() {
    execute_from_name(file!("addition"));
}

