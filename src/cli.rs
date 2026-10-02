use std::{fs::create_dir_all, sync::Arc};

use log::{LevelFilter, info, trace};

use crate::file::File;
use crate::source::SourceManager;

use clap::Parser;
use miette::{Context, IntoDiagnostic, Result};

#[derive(Parser, Debug)]
pub(crate) struct Build {
    /// The source file to use. Defaults to STDIN.
    /// STDIN is currently not supported.
    pub(crate) source: Option<Arc<str>>,
    /// Sets the path of the compilation folder.
    /// Defaults to `.skribi`.
    #[arg(short, long, default_value = ".skribi")]
    compile_path: String,
}

/// Creates a folder to store everything
fn create_skribi_directory(path: &str) -> Result<()> {
    trace!("About to create directory `{}`", path);
    create_dir_all(path).into_diagnostic().context(format!(
        "While creating `{}` directory to store compiled files",
        path
    ))?;
    info!("Directory `{}` created for compiled files", path);
    Ok(())
}

impl Build {
    /// Compile the source code
    pub(crate) fn execute(self) -> Result<()> {
        create_skribi_directory(&self.compile_path)?;

        if let Some(path) = self.source {
            let file = File::from_file(path).context("While reading file passed as argument")?;
            let mut manager = SourceManager::empty();
            manager.add_file(file);

            manager.compile()
        } else {
            todo!("STDIN is currently not supported")
        }
    }
}

#[derive(Parser, Debug)]
pub(crate) struct Run {
    #[command(flatten)]
    pub(crate) build: Build,
}

impl Run {
    /// Compile the source code, then execute the compiled code
    pub(crate) fn execute(self) -> Result<()> {
        self.build.execute()?;
        todo!("Execute the compiled code")
    }
}

#[derive(Parser, Debug)]
pub(crate) enum Command {
    /// Build the source code into machine code
    Build(Build),
    /// Build the source code and run it directly after
    Run(Run),
}

impl Command {
    /// Run the subcommand's specific code
    pub(crate) fn execute(self) -> Result<()> {
        match self {
            Command::Build(build) => build.execute(),
            Command::Run(run) => run.execute(),
        }
    }
}

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    /// Log more information. Fine-grained control.
    ///
    /// The SKRIBI_C_LOG variable can also be used.
    /// To specify a style, use SKRIBI_C_LOG_STYLE.
    /// The variable is overriden by the argument.
    ///
    /// With nothing set, defaults to warn.
    ///
    /// Possible values: off, error, warn, info, debug, trace
    #[arg(short, long, global = true)]
    pub(crate) verbose: Option<LevelFilter>,
    #[clap(subcommand)]
    pub cmd: Command,
}
