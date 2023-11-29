// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

// Import
use skribi_language_source::{clear, read};
use std::{env, io};

// Main
fn main() {
    let args: Vec<_> = env::args().collect(); // get the command line arguments
    let mut path = String::new();
    let extension: Vec<String> = vec!["sk".to_string(), "skribi".to_string()];
    let flag_char = "/"; // if it was "-", it would sometimes interfere with cargo's flags

    // clear the shell for the user
    if !args.contains(&String::from(flag_char.to_string() + "interpret-debug")) {
        clear();
    }

    // Get the path of the file to run
    if args.len() > 1 && !args[1].starts_with(flag_char) {
        path = args[1].clone();
    } else {
        println!("Enter a file to run");
        let _ = io::stdin().read_line(&mut path);
        path = path.trim().to_string();
    }

    // Check if the file has the right extension
    if !extension.contains(&String::from(path.split('.').last().unwrap())) {
        println!("Not a valid file extension");
        return;
    }

    // TODO read the file
    let lines = read(&path);
    println!("{:?}", lines)
}
