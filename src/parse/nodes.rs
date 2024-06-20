#![allow(dead_code)]

mod blocs;
mod classes;
pub(crate) mod expressions;
pub mod files_node;
mod functions;
pub(crate) mod id_nodes;
mod if_else;
mod operations;
mod vars;

/// Macro to implement the Debug trait for a GraphDisplay
#[macro_export]
macro_rules! impl_debug {
    ($t:ty) => {
        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.graph())
            }
        }
    };
}

/// Trait to display a graph with mermaid syntax. Used to display trees of nodes easily when
/// debugging the AST.
///
/// # Warning
///
/// You must use the macro [impl_debug] to implement the Debug trait for a struct that implements
/// `GraphDisplay`.
///
/// # Example
///
/// ```
/// use std::fmt;
/// use skr::parse::nodes::GraphDisplay;
///
/// struct MyNode {
///    name: String,
/// }
///
/// impl GraphDisplay for MyNode {
///    fn graph_display(&self, graph: &mut String, id: &mut usize) {
///       graph.push_str(&format!("\nsubgraph MyNode_{}[MyNode {}]\nend", id, self.name));
///      *id += 1;
///   }
/// }
///
/// impl_debug!(MyNode);
///
/// let node = MyNode { name: "test".to_string() };
/// println!("{:?}", node);
/// ```
trait GraphDisplay {
    fn graph_display(&self, graph: &mut String, id: &mut usize);

    fn graph(&self) -> String {
        let mut graph = String::new();
        graph.push_str("flowchart TD");
        let mut id = 0;
        self.graph_display(&mut graph, &mut id);
        graph
    }
}
