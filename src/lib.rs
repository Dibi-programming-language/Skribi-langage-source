use log::trace;
use miette::{Context, Report, Result};

/// This module handles reading from inputs
pub mod file;
/// This module handles multi sources
pub mod source;

pub fn execute() -> Result<()> {
    trace!("Entenring execute");
    Err(Report::msg("Rework in progress.")).context("Skribi is not usable right now.")
}
