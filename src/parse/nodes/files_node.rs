use std::collections::VecDeque;

use crate::impl_debug;
use crate::parse::nodes::expressions::Exp;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::ResultOption;
use crate::tokens::TokenContainer;

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

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        let mut exps = Vec::new();
        while let Some(exp) = Exp::parse(tokens)? {
            exps.push(exp);
        }
        Ok(Some(FileNode { exps }))
    }
}
