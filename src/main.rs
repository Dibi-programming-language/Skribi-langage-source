// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

// Import
use std::{env, io};
use skribi_language_source::clear;

// Main
fn main() {
    let args: Vec<_> = env::args().collect(); // get the command line arguments
    let mut path = String::new();
    let extension: Vec<String> = vec!["sk".to_string(), "skribi".to_string()];

    // Get the path of the file to run
    if args.len() > 1 {
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
    println!("Path: {}", path);
}
