// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use clap::Parser;

use env_logger::{Builder, Env};
use log::{LevelFilter, trace};
use miette::{Context, Result};

use skribi::{file::File, source::SourceManager};

#[derive(Parser, Debug)]
struct Build {
    /// The source file to use. Defaults to STDIN.
    /// STDIN is currently not supported.
    source: Option<String>,
}

#[derive(Parser, Debug)]
struct Run {
    #[command(flatten)]
    build: Build,
}

#[derive(Parser, Debug)]
enum Command {
    /// Build the source code into machine code
    Build(Build),
    /// Build the source code and run it directly after
    Run(Run),
}

impl Command {
	fn exec(self) -> Result<()> {
		match self {
			Command::Build(build) => compile(build.source),
			Command::Run(run) => {
				compile(run.build.source)?;
				todo!("Execute the compiled code")
			}
		}
	}
}

/// The Skribi compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
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
    verbose: Option<LevelFilter>,
    #[clap(subcommand)]
    pub cmd: Command,
}

fn compile(path: Option<String>) -> Result<()> {
    if let Some(path) = path {
        let file = File::from_file(&path).context("While reading file passed as argument")?;
        let mut manager = SourceManager::empty();
        manager.add_file(file);

        manager.compile()
    } else {
        todo!("STDIN is currently not supported")
    }
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

	args.cmd.exec()
}
