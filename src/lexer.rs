use std::fmt::{Display, Formatter};

use logos::{Logos, SpannedIter};

// NOTE: logos is smart: like CSS, it calculates a priority score based on the
// specificity of the rule. "token" has the priority over anything else. Then,
// regex, with complicated rules. Sometimes, the priority argument can be used
// to avoid confusions.

#[derive(Logos, Clone, PartialEq, Debug)]
pub enum Tokens<'src> {
    /// Names: variables, functions, ...
    /// If they want a name that finish in - they can have it.
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_-]*"#)]
    Identifier(&'src str),
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
