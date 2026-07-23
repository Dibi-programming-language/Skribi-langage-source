use std::collections::HashMap;

use chumsky::error::Rich;
use log::{info, trace};
use miette::{Context, Diagnostic, LabeledSpan, NamedSource, Result, Severity, SourceSpan, miette};
use thiserror::Error;

use crate::{
    ast::nodes::FileTreeRoot,
    file::File,
    lexer::{Tokens, tokenise},
    parse::parse,
};

pub struct Source<'file> {
    file: &'file File<'file>,
    // TODO: add first user of the tree to remove this
    #[allow(dead_code)]
    root: FileTreeRoot<'file>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("{message}")]
#[diagnostic()]
struct ParsingSingleError {
    message: String,
    #[label(primary, "{span_message}")]
    span: SourceSpan,
    span_message: String,
    #[label(collection)]
    spans: Vec<LabeledSpan>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Parsing error")]
#[diagnostic(help("Always try to fix the first parsing error as they might be cascades"))]
struct ParsingErrors {
    #[source_code]
    src: NamedSource<String>,
    #[related]
    related: Vec<ParsingSingleError>,
}

fn convert_to_err(file: &File<'_>, errs: Vec<Rich<'_, Tokens<'_>>>) -> ParsingErrors {
    // Greatly inspired from
    // https://codeberg.org/zesterer/chumsky/src/branch/main/examples/nano_rust.rs
    ParsingErrors {
        src: file.into_named(),
        related: errs
            .iter()
            .map(|err| ParsingSingleError {
                message: err.to_string(),
                span: err.span().into_range().into(),
                span_message: err.reason().to_string(),
                spans: err
                    .contexts()
                    .map(|(label, span)| {
                        LabeledSpan::new_with_span(
                            Some(format!("parsing {label}")),
                            span.into_range(),
                        )
                    })
                    .collect(),
            })
            .collect(),
    }
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
        let result = parse(tokens, file.content.len());
        match result {
            Ok(root) => Ok(Source { file, root }),
            Err(errs) => Err(convert_to_err(file, errs).into()),
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
