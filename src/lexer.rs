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

/// Split a file content into tokens
pub fn tokenise<'src>(arg: &'src str) -> SpannedIter<'src, Tokens<'src>> {
    // Inspired from the logos example
    Tokens::lexer(arg).spanned()
}
