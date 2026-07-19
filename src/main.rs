// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use miette::{Context, Result};

use skribi::execute;


/// Launch the interpreter
fn main() -> Result<()> {
    execute()
        .context("Failed to execute your file.")
}
