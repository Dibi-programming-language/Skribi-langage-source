use std::fmt::Debug;
use std::fmt::{Display, Formatter};

use logos::{Logos, SpannedIter};
use string_interner::DefaultStringInterner;
use string_interner::DefaultSymbol;
use string_interner::Symbol;

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
            write!(f, "{}", str.to_usize())
        } else if let Self::Error(err) = self {
            write!(f, "{}", err.to_usize())
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
pub fn tokenise<'a>(
    arg: &'a str,
    interner: &'a mut DefaultStringInterner,
) -> SpannedIter<'a, Tokens> {
    // Inspired from the logos example
    Tokens::lexer_with_extras(arg, interner).spanned()
}
