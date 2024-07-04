use std::collections::VecDeque;

use crate::impl_debug;
use crate::parse::nodes::expressions::Exp;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::OptionResult;
use crate::tokens::Token;

/// Node representing a file. This is the root node of the AST.
#[derive(PartialEq)]
pub struct FileNode {
    exps: Vec<Exp>,
}

impl GraphDisplay for FileNode {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph File_{}[File]", id));
        *id += 1;
        for exp in &self.exps {
            exp.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(FileNode);

impl FileNode {
    pub fn new(exps: Vec<Exp>) -> Self {
        Self { exps }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> OptionResult<FileNode> {
        let mut exps = Vec::new();
        loop {
            match Exp::parse(tokens) {
                Ok(Some(exp)) => {
                    exps.push(exp);
                }
                Err(e) => {
                    return Some(Err(e));
                }
                Ok(None) => {
                    break;
                }
            }
        }

        Some(Ok(FileNode { exps }))
    }
}
