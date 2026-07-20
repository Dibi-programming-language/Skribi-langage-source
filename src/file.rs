use log::{trace, warn};
use miette::{Context, IntoDiagnostic, NamedSource, Result};


pub struct File<'name> {
    pub(crate) name: &'name str,
    pub(crate) content: String,
}

impl File<'_> {
    pub fn from_file<'name>(path: &'name str) -> Result<File<'name>> {
        trace!("Reading file `{}`", path);
        if !path.ends_with(".skrb") {
            warn!("File `{}` does not end in .skrb", path);
        }
        let content = std::fs::read_to_string(path)
            .into_diagnostic()
            .context(format!("While reading file `{}`", path))?;
        Ok(File {
            name: path,
            content,
        })
    }
}

impl Into<NamedSource<String>> for File<'_> {
    fn into(self) -> NamedSource<String> {
        NamedSource::new(self.name, self.content)
    }
}
