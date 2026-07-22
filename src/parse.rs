use chumsky::error::Rich;
use chumsky::input::{Input, Stream, ValueInput};
use chumsky::prelude::{choice, empty, just, recursive, via_parser};
use chumsky::span::SimpleSpan;
use chumsky::{IterParser, Parser, extra};
use logos::SpannedIter;

use crate::ast::nodes::FileTreeRoot;
use crate::ast::nodes::expressions::Expression;
use crate::ast::nodes::statements::Statement;
use crate::lexer::Tokens;
use crate::parse::call::{function_call_parser, native_parser};

pub mod call;

// Global warning on the parser: please use .boxed() sometimes, so that the
// compilation time decreases. This is like INTERCAL, you need to say please
// sometimes.

// This functions define abstract parsers that will be instancieted by chumsky.
// This does not actually parse anything.

fn expression_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, Tokens<'src>>>> + Clone + 'tok
where
    I: ValueInput<'tok, Token = Tokens<'src>, Span = SimpleSpan>,
{
    // exp := (exp) | native_call
    // This is over complicated as more rules will be added

    recursive(|exp| {
        // Anything that starts with a special unique token
        // --> has maximal priority and can be in anything
        let priority = choice((exp.clone().delimited_by(
            just(Tokens::LeftParenthesis),
            just(Tokens::RightParenthesis)
                .recover_with(via_parser(empty().to(Tokens::RightParenthesis))),
        ),))
        .boxed();

        choice((
            priority.clone(),
            function_call_parser().map(|x| Expression::FunctionCall(x)),
        ))
    })
}

fn statement_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, Statement<'src>, extra::Err<Rich<'tok, Tokens<'src>>>>
where
    I: ValueInput<'tok, Token = Tokens<'src>, Span = SimpleSpan>,
{
    choice((
        native_parser().map(|x| Statement::Deprecated(x)),
        expression_parser().boxed().map(Statement::Expression),
    ))
    .boxed()
}

fn root_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, FileTreeRoot<'src>, extra::Err<Rich<'tok, Tokens<'src>>>>
where
    I: ValueInput<'tok, Token = Tokens<'src>, Span = SimpleSpan>,
{
    statement_parser()
        .repeated()
        .collect()
        .boxed()
        .map(FileTreeRoot::new)
}

pub fn parse<'tok>(
    tokens: SpannedIter<'tok, Tokens<'tok>>,
    src_len: usize,
) -> Result<FileTreeRoot<'tok>, Vec<Rich<'tok, Tokens<'tok>>>> {
    // Greatly inspired from
    // https://codeberg.org/zesterer/chumsky/src/branch/main/examples/logos.rs
    // Converts from a logos format to a chumsky format
    // See the example for full explanations

    let iter = tokens.map(|(token, span)| match token {
        Ok(tok) => (tok, span.into()),
        Err(()) => (Tokens::Error("?"), span.into()),
    });

    let token_stream = Stream::from_iter(iter).map((0..src_len).into(), |(t, s): (_, _)| (t, s));

    root_parser().parse(token_stream).into_result()
}
