use std::collections::HashMap;

use crate::file::File;

pub struct Source<'file> {
    file: File<'file>,
}

impl Source<'_> {
    pub fn new<'file>(file: File<'file>) -> Source<'file> {
        Source {
            file,
        }
    }
}

pub struct SourceManager<'sources> {
    files: HashMap<&'sources str, Source<'sources>>,
}

impl<'manager> SourceManager<'manager> {
    pub fn empty() -> Self {
        SourceManager { files: HashMap::new() }
    }

    pub fn add_file<'file: 'manager>(&mut self, file: File<'file>) {
        self.files.insert(file.name, Source::new(file));
    }
}
