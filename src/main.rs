// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::fs::create_dir_all;

use clap::Parser;

use log::{LevelFilter, info, trace};
use miette::{Context, IntoDiagnostic, Result, set_hook};
use env_logger::{Builder, Env};

use skribi::{file::File, source::SourceManager};

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// The source file to use. Defaults to STDIN.
    /// STDIN is currently not supported.
    source: Option<String>,
    /// Sets the path of the compilation folder.
    /// Defaults to `.skribi`.
    #[arg(short, long, default_value = ".skribi")]
    compile_path: String,
    /// Log more information. Fine-grained control.
    ///
    /// The SKRIBI_C_LOG variable can also be used.
    /// To specify a style, use SKRIBI_C_LOG_STYLE.
    /// The variable is overriden by the argument.
    ///
    /// With nothing set, defaults to warn.
    ///
    /// Possible values: off, error, warn, info, debug, trace
    #[arg(short, long)]
    verbose: Option<LevelFilter>,
    /// Run the code instead of compiling it.
    #[arg(short, long)]
    run: bool,
    /// Pretty printe the code instead of compiling it.
    #[arg(short, long)]
    pretty: bool,
}

/// Creates a folder to store everything
fn create_skribi_directory(path: &str) -> Result<()> {
    trace!("About to create hidden directory `{}`", path);
    create_dir_all(path).into_diagnostic().context(format!(
        "While creating hidden `{}` directory to store compiled files",
        path
    ))?;
    info!("Hidden directory `{}` created for compiled files", path);
    Ok(())
}

/// Launch the interpreter
fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut logger = Builder::from_env(
        Env::default()
            .filter_or("SKRIBI_C_LOG", "warn")
            .write_style("SKRIBI_C_LOG_STYLE"),
    );

    // To ignore the env variable in production:
    // #[cfg(not (debug_assertions))]
    // logger.filter_level(LevelFilter::Warn);
    // Or detect special values?
    // Did not add the env to clap as this would be a double with from_env
    // in debug mode.

    if let Some(level) = args.verbose {
        logger.filter_level(level);
    }

    logger.init();
    trace!("Logger initialised, entenring main");

    set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .show_related_errors_as_nested()
                .build(),
        )
    }))?;

    create_skribi_directory(&args.compile_path)?;

    if let Some(path) = args.source {
        let file = File::from_file(&path).context("While reading file passed as argument")?;
        let mut manager = SourceManager::empty();
        manager.add_file(&file)?;

        if args.run {
            manager.execute()
        } else if args.pretty {
            manager.pretty()
        } else {
            manager.compile()
        }
    } else {
        todo!("STDIN is currently not supported")
    }
}
