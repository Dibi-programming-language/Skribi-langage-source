use std::{collections::HashMap, sync::Arc};

use chumsky::error::Rich;
use log::{debug, info, trace, warn};
use miette::{Context, Diagnostic, LabeledSpan, NamedSource, Result, Severity, SourceSpan, miette};
use string_interner::DefaultSymbol;
use thiserror::Error;

use crate::{
    ast::nodes::FileTreeRoot,
    file::File,
    interner::INTERNER,
    lexer::{Tokens, tokenise},
    parse::parse,
};

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
    src: NamedSource<Arc<str>>,
    #[related]
    related: Vec<ParsingSingleError>,
}

fn convert_to_err(file: &File, errs: Vec<Rich<'_, Tokens>>) -> ParsingErrors {
    // Greatly inspired from
    // https://codeberg.org/zesterer/chumsky/src/branch/main/examples/nano_rust.rs
    ParsingErrors {
        src: file.create_source(),
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

pub struct Source {
    file: File,
    // TODO: add first user of the tree to remove this
    #[allow(dead_code)]
    root: FileTreeRoot,
}

fn error_symbol() -> Result<DefaultSymbol> {
    let mut interner = INTERNER
        .try_lock()
        .map_err(|e| miette!("Unable to access interner: {}", e))?;
    Ok(interner.get_or_intern_static("?"))
}

impl Source {
    pub fn new(file: File) -> Result<Source> {
        trace!("Entenring source creation for `{}`", file.name);
        let error_symbol = error_symbol()?;

        let tokens = tokenise(&file.content).context("Failed to tokenise the input")?;
        let size = tokens.len();
        info!(
            "File `{}` splitted into at least {} tokens",
            file.name, size,
        );

        // Not able to log tokens without consuming them (ownership)
        let result = parse(tokens, file.content.len(), &error_symbol);
        match result {
            Ok(root) => Ok(Source { file, root }),
            Err(errs) => Err(convert_to_err(&file, errs).into()),
        }
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
    files: HashMap<Arc<str>, Source>,
}

impl SourceManager {
    pub fn empty() -> Self {
        SourceManager {
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file: File) -> Result<()> {
        debug!("Adding file {} into source files", file.name);
        self.files.insert(
            file.name.clone(),
            Source::new(file).context("Failed parse the file")?,
        );
        Ok(())
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
