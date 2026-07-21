use crate::ast::nodes::statements::Statement;

pub mod statements;
pub mod expressions;
pub mod calls;

pub struct FileTreeRoot<'tok> {
    pub content: Vec<Statement<'tok>>,
}

impl FileTreeRoot<'_> {
    pub fn new<'tok>(content: Vec<Statement<'tok>>) -> FileTreeRoot<'tok> {
        return FileTreeRoot { content };
    }
}
