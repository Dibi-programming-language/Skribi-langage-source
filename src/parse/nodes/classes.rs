use crate::impl_debug;
use crate::parse::nodes::blocs::Scope;
use crate::parse::nodes::GraphDisplay;

// Grammar of this file :
// <class_dec> ::= kat T_IDENTIFIER <scope>

// TODO - implement Debug for ClassDec

/// `ClassDec` represents a class declaration. It is not yet implemented. Will be implemented in a
/// future pull request.
#[derive(PartialEq)]
pub struct ClassDec {
    identifier: String,
    scope: Box<Scope>,
}

impl GraphDisplay for ClassDec {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph ClassDec_{}[ClassDec {}]\nend",
            "", id, self.identifier, indent=indent
        ));
        *id += 1;
    }
}

impl_debug!(ClassDec);

pub fn is_type_def(identifier: &str) -> bool {
    // TODO: implement this function with complex types
    matches!(identifier, "int" | "dar" | "ioi" | "skr")
}
