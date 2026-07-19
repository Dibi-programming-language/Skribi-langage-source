// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::process::ExitCode;

use skribi::execute;

/// Launch the interpreter
fn main() -> ExitCode {
    match execute() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
