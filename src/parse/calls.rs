use chumsky::{
    Parser,
    error::Rich,
    extra::{self},
    input::ValueInput,
    prelude::{empty, just},
    select,
    span::SimpleSpan,
};

use crate::{ast::nodes::calls::FunctionCall, tokens::NewTokens};

pub fn function_call_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, FunctionCall<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    let identifier = select! {
        NewTokens::Identifier(str) => str
    };
    // TODO: add a parser for chains
    let base = identifier;
    let call = empty().delimited_by(
        just(NewTokens::LeftParenthesis),
        just(NewTokens::RightParenthesis),
    );

    base.then_ignore(call).map(|base| FunctionCall::new(base))
}

pub fn native_call_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, FunctionCall<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    just(NewTokens::NatCall)
        .ignore_then(function_call_parser())
        .map(|mut x| {
            x.native = true;
            x
        })
}
