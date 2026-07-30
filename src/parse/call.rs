use chumsky::{
    Parser,
    error::Rich,
    extra::{self},
    input::ValueInput,
    prelude::{empty, just},
    recovery::via_parser,
    select,
    span::SimpleSpan,
};

use crate::{
    ast::nodes::{calls::functions::FunctionCall, deprecated::Deprecated},
    lexer::Tokens,
};

pub fn function_call_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, FunctionCall<'src>, extra::Err<Rich<'tok, Tokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = Tokens<'src>, Span = SimpleSpan>,
{
    let identifier = select! {
        Tokens::Identifier(str) => str
    };
    // TODO: add a parser for chains
    let base = identifier;
    let call = empty()
        .delimited_by(
            just(Tokens::LeftParenthesis),
            just(Tokens::RightParenthesis)
                .recover_with(via_parser(empty().to(Tokens::RightParenthesis))),
        )
        .labelled("function call body");

    base.then_ignore(call)
        .map_with(|base, extra| FunctionCall::new(base, extra.span()))
        .labelled("function call")
        .as_context()
}

pub fn native_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, Deprecated, extra::Err<Rich<'tok, Tokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = Tokens<'src>, Span = SimpleSpan>,
{
    just(Tokens::NativeCall)
        .map_with(|_, extra| Deprecated::new("skr_app should not be used", extra.span()))
}
