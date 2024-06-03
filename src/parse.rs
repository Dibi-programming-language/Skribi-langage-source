use crate::tokens::Token;

pub(crate) mod nodes;
mod parse_values;
mod parse_variables;

pub fn main(tokens: Vec<Token>) {
    // TODO - this fonction is dependant of functions that are not yet implemented

    // let mut line = 0;
    let mut i = 0;
    // let vec: Vec<Vec<String>> = Vec::new();
    let nodes = ();
    if i != tokens.len() {
        panic!("Scope closed with }} before the end");
    }
    nodes
}
