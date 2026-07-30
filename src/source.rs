use std::{collections::HashMap, path::Path, process::Command};

use chumsky::error::Rich;
use log::{info, trace};
use miette::{
    Context, Diagnostic, IntoDiagnostic, LabeledSpan, NamedSource, Report, Result, SourceSpan,
};
use thiserror::Error;

use crate::{
    ast::{
        nodes::FileTreeRoot,
        visitors::{code_generator::CodeGenerator, deprecated::DeprecatedNodesVisitor},
    },
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
        if let Some(error) = DeprecatedNodesVisitor::find(&self.root)? {
            let report: Report = error.into();
            return Err(report.with_source_code(self.file.into_named()));
        }
        todo!("Finish execution (not the point for now)")
    }
}

pub struct SourceManager<'sources> {
    files: HashMap<&'sources str, Source<'sources>>,
}

fn link_files(inputs: Vec<String>, output: &str) -> Result<()> {
    let output = Path::new(output).with_added_extension("out");
    let command = Command::new("clang")
        // For nix
        .arg("-Wno-unused-command-line-argument")
        .args(&["-o", output.to_str().expect("Cannot create output")])
        .args(inputs)
        .status();
    command.into_diagnostic().context("While linking files")?;
    Ok(())
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

    pub fn compile(&self, folder: &str, output: &str) -> Result<()> {
        let mut paths = vec![];
        for (name, file) in &self.files {
            info!("Compiling `{}`", name);
            CodeGenerator::compile(&file.root, name, folder)
                .context(format!("While compiling file `{}`", name))?;
            let name = Path::new(".skribi")
                .join(name)
                .with_added_extension("ll")
                .to_str()
                .context("Compiled file has an invalid name")?
                .to_owned();
            paths.push(name);
        }
        link_files(paths, output)
            .context(format!("After building all files needed for {}", output))?;
        info!("Result saved into {}", output);
        Ok(())
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

    pub fn pretty(&self) -> Result<()> {
        for (name, file) in &self.files {
            std::println!("File {} AST is:\n{}", name, file.root);
        }
        Ok(())
    }
}
