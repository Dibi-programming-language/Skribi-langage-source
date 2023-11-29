// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

// Import
use std::{env, io};

// Main
fn main() {
    let args: Vec<_> = env::args().collect(); // get the command line arguments
    let mut path: String = Default::default();
    let extension: Vec<&str> = vec!["sk", "skribi"];

    // Get the path of the file to run
    if args.len() > 1 {
        path = args[1].clone();
    } else {
        println!("Please enter a path: ");
        let _ = io::stdin().read_line(&mut path);
    }

    // Check if the file is a skribi file
    if !extension.contains(&path.split(".").last().unwrap()) {
        println!("Not a skribi file");
        println!("{}",path.split(".").last().unwrap());
        return;
    }

    // TODO read the file
    println!("Path: {}", path);
}
