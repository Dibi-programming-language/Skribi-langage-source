use log::trace;
use miette::{Context, Report, Result};

pub fn execute() -> Result<()> {
    trace!("Entenring execute");
    Err(Report::msg("Rework in progress."))
        .context("Skribi is not usable right now.")
}
