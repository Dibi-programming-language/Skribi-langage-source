use std::sync::Arc;

use log::{trace, warn};
use miette::{Context, IntoDiagnostic, NamedSource, Result};

/// Usage of copies of strings has a big footprint.
/// Arcs avoid this footprint.
/// Used in many cases, even in this file.
/// Avoids lifetime and allows acceptable file cloning.
pub struct File {
    pub(crate) name: Arc<str>,
    pub(crate) content: Arc<str>,
}

impl File {
    pub fn from_file(path: Arc<str>) -> Result<File> {
        trace!("Reading file `{}`", path);
        if !path.ends_with(".skrb") {
            warn!("File `{}` does not end in .skrb", path);
        }
        let content = std::fs::read_to_string(path.as_ref())
            .into_diagnostic()
            .context(format!("While reading file `{}`", path))?;
        Ok(File {
            name: path,
            content: content.into(),
        })
    }

    pub fn create_source(&self) -> NamedSource<Arc<str>> {
        NamedSource::new(self.name.as_ref(), self.content.clone())
    }
}
