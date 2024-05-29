use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::impl_debug;
use crate::parse::nodes::GraphDisplay;

// Grammar of this file :
// <class_dec> ::= kat T_IDENTIFIER <scope>

// TODO - Derive PartialEq and implement Debug for ClassDec
#[derive(PartialEq)]
pub struct ClassDec {
    identifier: String,
    // scope: Box<Scope>, // TODO - Implement Scope
}

impl GraphDisplay for ClassDec {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph ClassDec_{}[ClassDec {}]\nend", id, self.identifier));
        *id += 1;
    }
}

impl_debug!(ClassDec);

pub fn is_type_def(identifier: &str) -> bool {
    // TODO: implémenter cette fonction avec des types complexes
    match identifier {
        "int" => true,
        "dar" => true,
        "ioi" => true,
        "skr" => true,
        _ => false,
    }
}
