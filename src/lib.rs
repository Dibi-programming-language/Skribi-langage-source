use std::process::{Command, exit};
use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn clear() {
    match Command::new("clear").status() {
        Ok(_) => {}
        Err(_) => match Command::new("cls").status() {
            Ok(_) => {}
            Err(_) => {
                for _ in 0..100 {
                    println!()
                }
            }
        },
    }
}
pub fn error(message: &str) {
    //print the error message in red
    println!("\x1b[31mError: {}\x1b[0m", message);
    exit(0);
}
pub fn read(file_name: &str) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    match File::open(file_name) {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            // read the file line by line
            for line in reader.lines() {
                match line {
                    Ok(text) => {
                        lines.push(text);
                    }
                    Err(err) => {
                        if (err.kind() == io::ErrorKind::InvalidData) {
                            error("Impossible to read file: Bad encoding");
                        }
                        error("Impossible to read file: Unknown error");
                    }
                }
            }
        }
        Err(err) => {
            if (err.kind() == io::ErrorKind::NotFound) {
                error("Impossible to open file: File not found");
            } else if (err.kind() == io::ErrorKind::PermissionDenied) {
                error("Impossible to open file: Permission denied");
            }
            error("Impossible to open file: Unknown error")
        }
    }
    lines
}
