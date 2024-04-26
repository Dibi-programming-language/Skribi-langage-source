use std::fmt;
use std::fmt::{Formatter};

mod blocs;
mod main_nodes;
pub(crate) mod expressions;
mod if_else;
mod fct;
mod vars;
mod classes;
pub(crate) mod id_nodes;

trait GraphDisplay {
    fn graph_display(&self, graph: &mut String, id: &mut usize);

    fn graph(&self) -> String {
        let mut graph = String::new();
        graph.push_str("flowchart TD");
        let mut id = 0;
        self.graph_display(&mut graph, &mut id);
        graph
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.graph())
    }
}

// macro to implement the Debug trait for a GraphDisplay

#[macro_export] macro_rules! impl_debug {
    ($t:ty) => {
        impl Debug for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.graph())
            }
        }
    };
}
