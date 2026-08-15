//! Heavily inspired from the concepts of
//! https://doc.rust-lang.org/nightly/nightly-rustc/src/rustc_middle/mir/visit.rs.html#68-1011
//!
//! A visitor is a pattern often used when we have a set of nodes that we want
//! to apply actions on. It avoids having struct impl that have 2000 lines.
//! https://refactoring.guru/design-patterns/visitor
//!
//! The overall result might be simpler as the needs of Skribi are much lower.
//! Concepts:
//! - for each node type N, a default_N function and a visit_N function are
//!   provided (default is super in rustc)
//! - these functions returns a generic type Result<T> using the miette
//!   error framework
//! - we call visit_N, that fallbacks to default_N, to simulate a kind of
//!   inheritance
//! - like in rustc, we provide both mutable and unmutable traits, but also
//!   for self
//! - inline is not used for default_N as rustc / LLVM might do it automatically
//!   better then us
//!
//! Visitors are the heart of a compiler.

use crate::ast::nodes::FileTreeRoot;
use crate::ast::nodes::calls::functions::FunctionCall;
use crate::ast::nodes::deprecated::Deprecated;
use crate::ast::nodes::expressions::Expression;
use crate::ast::nodes::statements::Statement;
use miette::Result;

pub mod deprecated;
pub mod pretty;

/// A enum that indicated why we need the default value of T.
pub enum DefaultCause {
    ZeroElements,
    Deprecated,
    FunctionCall,
}

// I think I have chosen an evil syntax, but the result is nice
macro_rules! make_ast_visitor {
    ($trait_name: ident self=&$($self_mutable:ident)?, ast=&$($mutable:ident)?) => {
        // To unallow dead_code, we will need more visitors
        /// A visitor for an AST.
        /// Please select the right visitor trait:
        /// - AstVisitor for umutable visitor and ast
        /// - MutAstVisitor for mutable ast
        /// - AstMutVisitor for mutable visitor
        /// - MutAstMutVisitor for both mutable
        #[allow(dead_code)]
        pub trait $trait_name<'life, T, R = miette::Error> {
            /// Called if returning a default value is needed.
            /// Considere it lazy: you can throw an exception
            /// if it should never be reached.
            fn default_t(cause: DefaultCause) -> Result<T, R>;

            fn aggregate_t(mut current: Option<T>, new: T) -> Option<T> {
                // To avoid unused warnings
                current.replace(new);
                current
            }

            fn visit_file_tree_root(
                &$($self_mutable)? self,
                file_tree_root: &$($mutable)? FileTreeRoot,
            ) -> Result<T, R> {
                self.default_file_tree_root(file_tree_root)
            }

            fn default_file_tree_root(
                &$($self_mutable)? self,
                file_tree_root: &$($mutable)? FileTreeRoot,
            ) -> Result<T, R> {
                let mut res = None;
                for statement in &$($mutable)? file_tree_root.content {
                    res = Self::aggregate_t(res, self.visit_statement(statement)?);
                }
                if let Some(t) = res {
                    Ok(t)
                } else {
                    Self::default_t(DefaultCause::ZeroElements)
                }
            }

            fn visit_statement(
                &$($self_mutable)? self,
                statement: &$($mutable)? Statement,
            ) -> Result<T, R> {
                self.default_statement(statement)
            }

            fn default_statement(
                &$($self_mutable)? self,
                statement: &$($mutable)? Statement,
            ) -> Result<T, R> {
                match statement {
                    Statement::Expression(expression) => self.visit_expression(expression),
                    Statement::Deprecated(deprecated) => self.visit_deprecated(deprecated),
                }
            }

            fn visit_deprecated(
                &$($self_mutable)? self,
                deprecated: &$($mutable)? Deprecated,
            ) -> Result<T, R> {
                self.default_deprecated(deprecated)
            }

            fn default_deprecated(
                &$($self_mutable)? self,
                #[allow(unused)]
                deprecated: &$($mutable)? Deprecated,
            ) -> Result<T, R> {
                Self::default_t(DefaultCause::Deprecated)
            }

            fn visit_expression(
                &$($self_mutable)? self,
                expression: &$($mutable)? Expression,
            ) -> Result<T, R> {
                self.default_expression(expression)
            }

            fn default_expression(
                &$($self_mutable)? self,
                expression: &$($mutable)? Expression,
            ) -> Result<T, R> {
                match expression {
                    Expression::FunctionCall(function_call) => self.visit_function_call(function_call),
                }
            }

            fn visit_function_call(
                &$($self_mutable)? self,
                function_call: &$($mutable)? FunctionCall,
            ) -> Result<T, R> {
                self.default_function_call(function_call)
            }

            fn default_function_call(
                &$($self_mutable)? self,
                // Remove this when adding arguments and full path
                #[allow(unused)]
                function_call: &$($mutable)? FunctionCall,
            ) -> Result<T, R> {
                Self::default_t(DefaultCause::FunctionCall)
            }
        }
    };
}

make_ast_visitor!(AstVisitor       self=&,    ast=&);
make_ast_visitor!(MutAstVisitor    self=&,    ast=&mut);
make_ast_visitor!(AstMutVisitor    self=&mut, ast=&);
make_ast_visitor!(MutAstMutVisitor self=&mut, ast=&mut);
