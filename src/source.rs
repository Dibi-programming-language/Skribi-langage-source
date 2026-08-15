use std::{collections::HashMap, sync::Arc};

use log::{debug, info, trace, warn};
use miette::{Context, LabeledSpan, Result, Severity, miette};
use string_interner::DefaultStringInterner;

use crate::{file::File, lexer::tokenise};

pub struct Source {
    file: File,
}

impl Source {
    pub fn new(file: File, interner: &mut DefaultStringInterner) -> Source {
        trace!("Entenring source creation for `{}`", file.name);
        let tokens = tokenise(&file.content, interner);
        let size = tokens.size_hint();
        // Not used for anything else right now
        // Will be directly used in parser in next PR
        info!(
            // In general, 0 is detected as we have an indefinite size
            // The tokens are parsed on demand I suppose
            "File `{}` splitted into at least {} tokens",
            file.name, size.0,
        );
        // Added to see something
        trace!("Tokens: {:?}", tokens.map(|(r, _)| r).collect::<Vec<_>>());
        Source { file }
    }

    pub fn compile(&self) -> Result<()> {
        // Placeholder for later checks
        // May be moved later to the new function
        // Only do not do too much on a pull request
        if let Some(index) = self.file.content.find("skr_app") {
            let error = miette!(
                severity = Severity::Warning,
                labels = vec![LabeledSpan::at(index..(index + 7), "There"),],
                "Found deprecated skr_app"
            )
            .with_source_code(self.file.create_source());

            warn!("Warning: {:?}", error);
        }
        todo!("Finish execution (not the point for now)")
    }
}

pub struct SourceManager {
    interner: DefaultStringInterner,
    files: HashMap<Arc<str>, Source>,
}

impl SourceManager {
    pub fn empty() -> Self {
        SourceManager {
            interner: DefaultStringInterner::default(),
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        debug!("Adding file {} into source files", file.name);
        self.files
            .insert(file.name.clone(), Source::new(file, &mut self.interner));
    }

    pub fn compile(&self) -> Result<()> {
        trace!("Start compiling sources");
        // This is just a simple "Hello, World!" to see that the file
        // reading is working.
        for (name, file) in &self.files {
            file.compile()
                .context(format!("While executing `{}`", name))?;
        }
        todo!("Cannot compile for now, planned later")
    }
}
