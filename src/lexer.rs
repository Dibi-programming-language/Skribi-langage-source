use std::fmt::{Debug, Error};
use std::fmt::{Display, Formatter};

use log::error;
use logos::{Logos, Span};
use miette::{Result, miette};
use string_interner::DefaultStringInterner;
use string_interner::DefaultSymbol;

use crate::interner::INTERNER;

// NOTE: logos is smart: like CSS, it calculates a priority score based on the
// specificity of the rule. "token" has the priority over anything else. Then,
// regex, with complicated rules. Sometimes, the priority argument can be used
// to avoid confusions.

#[derive(Logos, Clone, PartialEq)]
#[logos(extras = &'s mut DefaultStringInterner)]
pub enum Tokens {
    /// Names: variables, functions, ...
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_]*"#, |lex| lex.extras.get_or_intern(lex.slice()))]
    Identifier(DefaultSymbol),
    /// Deprecated keyword to detect native calls,
    /// still there to test compatibility
    #[token("skr_app")]
    NativeCall,

    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,

    /// Note: no need of them in parsing
    #[regex(r"[ \t\n]+", logos::skip)]
    Ignore,

    /// Any character not used by other tokens,
    /// mainly used when parsing bloc title
    #[regex(".", |lex| lex.extras.get_or_intern(lex.slice()), priority = 0)]
    Error(DefaultSymbol),
}

impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Self::Identifier(str) = self {
            let interner = INTERNER.try_lock().map_err(|_| {
                error!("Failed to get the lock");
                Error::default()
            })?;
            let name = interner.resolve(str.clone()).unwrap_or_else(|| "ERROR");
            write!(f, "{}", name)
        } else if let Self::Error(err) = self {
            let interner = INTERNER.try_lock().map_err(|_| {
                error!("Failed to get the lock");
                Error::default()
            })?;
            let name = interner.resolve(err.clone()).unwrap_or_else(|| "ERROR");
            write!(f, "{}", name)
        } else {
            write!(
                f,
                "{}",
                match self {
                    Self::LeftParenthesis => "(",
                    Self::RightParenthesis => ")",
                    Self::Ignore => " ",
                    Self::NativeCall => "skr_app",
                    // WARNING: when adding tokens, always check the above list
                    _ => unreachable!(),
                }
            )
        }
    }
}

impl Debug for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self)
    }
}

/// Split a file content into tokens
pub fn tokenise(arg: &str) -> Result<Vec<(Result<Tokens, ()>, Span)>> {
    let mut interner = INTERNER
        .try_lock()
        .map_err(|e| miette!("Unable to access interner: {}", e))?;

    // Inspired from the logos example
    Ok(Tokens::lexer_with_extras(arg, &mut interner)
        .spanned()
        // Used to remove the lifetime
        // Implies that everything is tokenised, however we cannot do anything
        // else as we have a mutable borrow of the interner
        .collect())
}
