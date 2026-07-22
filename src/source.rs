use std::collections::HashMap;

use log::{info, trace};
use miette::{Context, LabeledSpan, Result, Severity, miette};
use miette::Report;

use crate::{ast::nodes::FileTreeRoot, file::File, lexer::tokenise, parse::parse};

pub struct Source<'file> {
    file: &'file File<'file>,
    root: FileTreeRoot<'file>,
}

impl Source<'_> {
    pub fn new<'file>(file: &'file File<'file>) -> Result<Source<'file>> {
        trace!("Entenring source creation for `{}`", file.name);
        let tokens = tokenise(&file.content);
        let size = tokens.size_hint();
        info!(
            "File `{}` splitted into at least {} tokens",
            file.name, size.0,
        );
        // TODO: finish
        let result = parse(tokens, file.content.len());
        match result {
            Ok(root) => Ok(Source {
                file,
                root,
            }),
            Err(err) => {
                todo!()
            }
        }
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
        todo!("Finish execution (not the point for now)")
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

    pub fn add_file<'file: 'manager>(&mut self, file: &'file File<'file>) -> Result<()> {
        info!("Adding file {} into source files", file.name);
        self.files.insert(file.name, Source::new(file)?);
        Ok(())
    }

    pub fn compile(&self) -> Result<()> {
        todo!("Cannot compile for now, planned later")
    }

    pub fn execute(&self) -> Result<()> {
        trace!("Start executing sources");
        // This is just a simple "Hello, World!" to see that the file
        // reading is working.
        for (name, file) in &self.files {
            file.execute()
                .context(format!("While executing `{}`", name))?;
        }
        todo!("Cannot exected for now, planned later")
    }
}
