use std::collections::HashMap;

use log::{info, trace};
use miette::{Context, LabeledSpan, Result, Severity, miette};

use crate::file::File;

pub struct Source<'file> {
    file: File<'file>,
}

impl Source<'_> {
    pub fn new<'file>(file: File<'file>) -> Source<'file> {
        Source { file }
    }

    pub fn execute(&self) -> Result<()> {
        // Placeholder for later checks
        // May be moved later to the new function
        // Only do not do too much on a pull request
        if let Some(index) = self.file.content.find("skr_app") {
            let error = miette!(
                severity = Severity::Warning,
                labels = vec![LabeledSpan::at(index..(index + 7), "There"),],
                "Found deprecated skr_app"
            )
            .with_source_code(self.file.into_named());
            return Err(error);
        }
        Ok(())
    }
}

pub struct SourceManager<'sources> {
    files: HashMap<&'sources str, Source<'sources>>,
}

impl<'manager> SourceManager<'manager> {
    pub fn empty() -> Self {
        SourceManager {
            files: HashMap::new(),
        }
    }

    pub fn add_file<'file: 'manager>(&mut self, file: File<'file>) {
        info!("Adding file {} into source files", file.name);
        self.files.insert(file.name, Source::new(file));
    }

    pub fn compile(&self) -> Result<()> {
        Err(miette!("Cannot compile for now"))
    }

    pub fn execute(&self) -> Result<()> {
        trace!("Start executing sources");
        // This is just a simple "Hello, World!" to see that the file
        // reading is working.
        for (name, file) in &self.files {
            file.execute()
                .context(format!("While executing `{}`", name))?;
        }
        Err(miette!("Cannot execute for now"))
    }
}
