use miette::{Diagnostic, LabeledSpan, Result};
use thiserror::Error;

use crate::ast::{nodes::FileTreeRoot, visitors::AstMutVisitor};

#[derive(Default)]
pub struct DeprecatedNodesVisitor {
    spans: Vec<LabeledSpan>,
}

impl AstMutVisitor<'_, ()> for DeprecatedNodesVisitor {
    fn default_t(_: super::DefaultCause) -> miette::Result<(), miette::Error> {
        Ok(())
    }

    fn visit_deprecated(
        &mut self,
        deprecated: &crate::ast::nodes::deprecated::Deprecated,
    ) -> miette::Result<(), miette::Error> {
        self.spans.push(LabeledSpan::new_with_span(
            Some(deprecated.message.to_owned()),
            deprecated.span.into_range(),
        ));
        self.default_deprecated(deprecated)
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Found deprecated parsing features")]
#[diagnostic(severity(Warning))]
pub struct DeprecatedError {
    #[label(collection)]
    spans: Vec<LabeledSpan>,
}

impl DeprecatedNodesVisitor {
    pub fn find(file_tree_root: &FileTreeRoot) -> Result<Option<DeprecatedError>> {
        let mut visitor = DeprecatedNodesVisitor::default();
        visitor.visit_file_tree_root(file_tree_root)?;
        if visitor.spans.len() > 0 {
            Ok(Some(DeprecatedError {
                spans: visitor.spans,
            }))
        } else {
            Ok(None)
        }
    }
}
