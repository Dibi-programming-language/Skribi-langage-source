use std::fmt::Display;
use std::fs::read_to_string;
use std::io::ErrorKind;

use miette::{Context, IntoDiagnostic, Result, miette};

use crate::FLAG_CHAR;
use crate::utils::{input, read};

pub enum FileKind {
    Classic(String),
    Stdin,
}

pub struct GotFile {
    pub content: String,
    pub file: FileKind,
}

impl GotFile {
    pub fn to_str<'a>(&'a self) -> &'a str {
        match &self.file {
            FileKind::Classic(string) => string,
            FileKind::Stdin => "stdin",
        }
    }
}

impl Display for GotFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.file {
            FileKind::Classic(string) => f.write_str(string),
            FileKind::Stdin => f.write_str("stdin"),
        }
    }
}

/// This function is used to get the path of the file to run
///
/// The path can either be passed as an argument or entered the terminal
pub fn get_content(args: Vec<String>, extensions: Vec<String>) -> Result<GotFile, ErrorKind> {
    if args.len() > 1 && !args[1].starts_with(FLAG_CHAR) {
        let path = args[1].clone();

        // Check if the file has the right extension
        let extension = String::from(path.split('.').next_back().unwrap());
        if !extensions.contains(&extension) {
            return Err(ErrorKind::InvalidInput);
        }

        // Read the file
        return Ok(GotFile {
            content: read(&path)?,
            file: FileKind::Classic(path),
        });
    }

    eprintln!("No path or invalid path, reading input:");
    let mut content = String::new();

    loop {
        let user_input = input("");
        if user_input.trim_end().is_empty() {
            break;
        };
        content += &*user_input;
    }

    Ok(GotFile {
        content,
        file: FileKind::Stdin,
    })
}

const EXTENSIONS: [&str; 2] = ["skrb", "skribi"];

/// This function is used to get the path of the file to run
///
/// The path can either be passed as an argument or entered the terminal
pub fn new_get_content(file: &Option<String>) -> Result<GotFile> {
    if let Some(path) = file {
        // Check if the file has the right extension
        let extension = path.split('.').next_back().unwrap();
        if !EXTENSIONS.contains(&extension) {
            return Err(miette!("Invalid extension. Expected: {:?}. Got: {}", EXTENSIONS, extension));
        }

        let content = read_to_string(&path)
            .into_diagnostic()
            .context("While reading file content")?;
        // Read the file
        return Ok(GotFile {
            content,
            file: FileKind::Classic(path.clone()),
        });
    }

    eprintln!("No given path, reading input:");
    let mut content = String::new();

    loop {
        let user_input = input("> ");
        if user_input.trim_end().is_empty() {
            break;
        };
        content += &*user_input;
    }

    Ok(GotFile {
        content,
        file: FileKind::Stdin,
    })
}
