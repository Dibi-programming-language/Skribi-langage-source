use miette::{Context, Report, Result};

pub fn execute() -> Result<()> {
    Err(Report::msg("Rework in progress.")).context("Skribi is not usable right now.")
}
