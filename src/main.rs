// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

// Import
use std::{env};

// Main
fn main() {
    let args: Vec<_> = env::args().collect(); // get the command line arguments
    let path: String = args[1].clone();
    let extension: Vec<&str> = vec!["sk", "skribi"];

    // Get the path of the file to run
    if args.len() == 1 {
        println!("Please specify the file to run");
        return;
    }

    // Check if the file has the right extension
    if !extension.contains(&path.split('.').last().unwrap()) {
        println!("Not a valid file extension");
        return;
    }

    // TODO read the file
    println!("Path: {}", path);
}
