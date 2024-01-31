
#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    OpenParenthesis,
    CloseParenthesis,

    IntLiteral(i32),
    Exit
}
