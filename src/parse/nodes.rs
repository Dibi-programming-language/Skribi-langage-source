#![allow(dead_code)]

use crate::skr_errors::ResultOption;
use crate::tokens::TokenContainer;
use std::collections::VecDeque;

mod blocs;
mod classes;
pub(crate) mod expressions;
pub mod files_node;
mod functions;
pub(crate) mod id_nodes;
mod if_else;
mod loops;
pub(crate) mod operations;
mod vars;

/// Macro to implement the Debug trait for a GraphDisplay
#[macro_export]
macro_rules! impl_debug {
    ($t:ty) => {
        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
/// ```ignore
/// use std::fmt;
/// // Import GraphDisplay and impl_debug
///
/// struct MyNode {
///    name: String,
/// }
///
/// impl GraphDisplay for MyNode {
///    fn graph_display(&self, graph: &mut String, id: &mut usize) {
///       graph.push_str(&format!("\nsubgraph MyNode_{}[MyNode {}]\nend", id, self.name));
///       *id += 1;
///   }
/// }
///
/// impl_debug!(MyNode);
///
/// let node = MyNode { name: "test".to_string() };
/// println!("{:?}", node);
/// ```
trait GraphDisplay {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize);

    fn graph(&self) -> String {
        let mut graph = String::new();
        graph.push_str("flowchart TD");
        let mut id = 0;
        self.graph_display(&mut graph, &mut id, 0);
        graph
    }
}

#[macro_export]
macro_rules! some_token {
    ($token:pat) => {
        Some($crate::token_m!($token))
    };
}

#[macro_export]
macro_rules! token_m {
    ($token:pat) => {
        TokenContainer { token: $token, .. }
    };
}

pub trait Parsable {
    fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self>
    where
        Self: Sized;
}

pub trait ParsableWithLevel {
    fn parse(tokens: &mut VecDeque<TokenContainer>, level: u8) -> ResultOption<Self>
    where
        Self: Sized;
}
