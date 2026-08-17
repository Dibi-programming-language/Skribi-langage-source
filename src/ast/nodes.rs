use std::sync::Arc;

use crate::{ast::nodes::statements::Statement, file::File};

pub mod calls;
pub mod deprecated;
pub mod expressions;
pub mod statements;

pub struct FileTreeRoot {
    pub content: Vec<Statement>,
    pub file: Option<Arc<File>>,
}

impl FileTreeRoot {
    pub fn new(content: Vec<Statement>) -> FileTreeRoot {
        FileTreeRoot {
            content,
            file: None,
        }
    }
}
