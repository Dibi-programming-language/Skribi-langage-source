use std::collections::VecDeque;

use crate::execute::{Execute, ExecutionError, GeneralOutput};
use crate::impl_debug;
use crate::parse::nodes::expressions::{Sta};
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::ResultOption;
use crate::tokens::TokenContainer;
use crate::execute::OperationContext;

type BaseContent = Sta;

/// Node representing a file.
/// This is the root node of the AST.
#[derive(PartialEq)]
pub struct FileNode {
    exps: Vec<BaseContent>,
}

impl GraphDisplay for FileNode {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!("\nsubgraph File_{}[File]", id));
        *id += 1;
        for exp in &self.exps {
            exp.graph_display(graph, id, indent);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(FileNode);

impl FileNode {
    pub fn new(exps: Vec<BaseContent>) -> Self {
        Self { exps }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        let mut exps = Vec::new();
        while !tokens.is_empty() {
            if let Some(exp) = BaseContent::parse(tokens)? {
                exps.push(exp);
            } else {
                tokens.pop_front();
            }
        }
        Ok(Some(FileNode { exps }))
    }
}

impl Execute for FileNode {
    fn execute(&self, operation_context: &mut OperationContext) -> GeneralOutput {
        println!("Executing {} lines\n", self.exps.len());
        for exp in &self.exps {
            if let Sta::Return(_) = exp {
                return Err(ExecutionError::no_return_at_root());
            } else {
                exp.execute(operation_context)?;
            }
        }
        Ok(())
    }
}

