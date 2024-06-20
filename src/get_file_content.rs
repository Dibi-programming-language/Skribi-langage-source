use std::io::ErrorKind;

use crate::utils::{input, read};
use crate::FLAG_CHAR;

/// This function is used to get the path of the file to run
///
/// The path can either be passed as an argument or entered the terminal
pub fn get_content(args: Vec<String>, extensions: Vec<String>) -> Result<String, ErrorKind> {
    if args.len() > 1 && !args[1].starts_with(FLAG_CHAR) {
        let path = args[1].clone();

        // Check if the file has the right extension
        let extension = String::from(path.split('.').last().unwrap());
        if !extensions.contains(&extension) {
            return Err(ErrorKind::InvalidInput);
        }

        // Read the file
        return read(&path);
    }

    let mut content = String::new();

    loop {
        let user_input = input("");
        if user_input.trim_end().is_empty() {
            break;
        };
        content += &*user_input;
    }

    Ok(content)
}
