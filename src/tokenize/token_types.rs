#[derive(Debug)]
pub enum Token {
    SemiColon,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,

    LowerThan,
    GreaterThan,

    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,

    IntLiteral(i32),
    StringLiteral(String),
    BooleanLiteral(bool),
    Exit
}
