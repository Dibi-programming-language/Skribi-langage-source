use crate::ast::nodes::statements::Statement;

pub mod calls;
pub mod deprecated;
pub mod expressions;
pub mod statements;

pub struct FileTreeRoot<'tok> {
    pub content: Vec<Statement<'tok>>,
}

impl FileTreeRoot<'_> {
    pub fn new<'tok>(content: Vec<Statement<'tok>>) -> FileTreeRoot<'tok> {
        FileTreeRoot { content }
    }
}
