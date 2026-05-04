// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::env;
use std::process::ExitCode;

use skribi::new_execute;

/// Launch the interpreter
fn main() -> ExitCode {
    // generic parameters
    let args = env::args().collect::<Vec<_>>(); // get the command line arguments

    match new_execute(args, true) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("\n{err}");
            ExitCode::FAILURE
        }
    }
}
