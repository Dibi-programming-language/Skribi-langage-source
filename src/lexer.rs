use std::fmt::{Display, Formatter};

use logos::{Logos, SpannedIter};

#[derive(Logos, Clone, PartialEq)]
pub enum Tokens<'src> {
    /// Names: variables, functions, ...
    #[regex(r#"[a-zA-Z][a-zA-Z0-9_]*"#)]
    Identifier(&'src str),
    /// Deprecated keyword to detect native calls,
    /// still there to test compatibility
    #[token("skr_app")]
    NativeCall,

    /// Just a (
    #[token("(")]
    LeftParenthesis,
    /// Just a )
    #[token(")")]
    RightParenthesis,

    /// Anything that is skipped
    #[regex(r"[ \t\n]+", logos::skip)]
    Ignore,

    /// Any character not used by other tokens,
    /// mainly used when parsing bloc title
    #[regex(".", priority = 0)]
    Error(&'src str),
}

impl Display for Tokens<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Identifier(str) => str,
            Self::LeftParenthesis => "(",
            Self::RightParenthesis => ")",
            Self::Ignore => " ",
            Self::NativeCall => "skr_app",
            Self::Error(err) => err,
        })
    }
}

/// Split a file content into tokens
pub fn tokenise<'src>(arg: &'src str) -> SpannedIter<'src, Tokens<'src>> {
    // Inspired from the logos example
    Tokens::lexer(arg).spanned()
}
