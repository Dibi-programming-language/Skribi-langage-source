use std::collections::VecDeque;

use chumsky::error::Rich;
use chumsky::input::{Input, Stream, ValueInput};
use chumsky::prelude::{choice, empty, just, recursive, via_parser};
use chumsky::span::SimpleSpan;
use chumsky::{IterParser, Parser, extra};
use logos::SpannedIter;

use crate::ast::nodes::AstRoot;
use crate::ast::nodes::expressions::Expression;
use crate::ast::nodes::statements::{Statement, StatementList};
use crate::parse::calls::native_call_parser;
use crate::parse::conditions::condition_parser;
use crate::parse::declarations::variable_declaration_parser;
use crate::parse::nodes::files_node::FileNode;
use crate::parse::nums::{binop_parser, value_parser};
use crate::skr_errors::ResultOption;
use crate::tokens::{NewTokens, TokenContainer};

mod calls;
mod conditions;
mod declarations;
pub(crate) mod nodes;
mod nums;

fn expression_parser<'tok, 'src: 'tok, I>(
    stal: impl Parser<'tok, I, StatementList<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
    + Clone
    + 'tok,
) -> impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone + 'tok
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    recursive(|exp| {
        let priority = choice((
            value_parser().map(Expression::ValueBase),
            exp.clone().delimited_by(
                just(NewTokens::LeftParenthesis),
                just(NewTokens::RightParenthesis)
                    .recover_with(via_parser(empty().to(NewTokens::RightParenthesis))),
            ),
            variable_declaration_parser(exp.clone()).map(|arg| Expression::VarDec(Box::new(arg))),
            condition_parser(exp, stal),
            native_call_parser().map(|x| Expression::FctCall(x)),
        ));

        choice((binop_parser(priority.clone()), priority.clone()))
    })
}

fn statement_list_parser<'tok, 'src: 'tok, I>(
    sta: impl Parser<'tok, I, Statement<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone + 'tok,
) -> impl Parser<'tok, I, StatementList<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone + 'tok
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    sta.repeated()
        .collect()
        .map(StatementList::new)
        .delimited_by(
            just(NewTokens::LeftBrace),
            just(NewTokens::RightBrace).recover_with(via_parser(empty().to(NewTokens::RightBrace))),
        )
}

fn statement_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, Statement<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    recursive(|sta| {
        let list = statement_list_parser(sta);
        expression_parser(list)
            .map(Statement::Exp)
            .then_ignore(just(NewTokens::Space).repeated())
    })
}

fn root_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, AstRoot<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    statement_parser().repeated().collect().map(AstRoot::new)
}

pub fn new_parse<'a>(
    tokens: SpannedIter<'a, NewTokens<'a>>,
    src_len: usize,
) -> Result<AstRoot<'a>, Vec<Rich<'a, NewTokens<'a>>>> {
    let iter = tokens.map(|(token, span)| match token {
        Ok(tok) => (tok, span.into()),
        Err(()) => (NewTokens::Error("?"), span.into()),
    });

    let token_stream = Stream::from_iter(iter).map((0..src_len).into(), |(t, s): (_, _)| (t, s));
    root_parser().parse(token_stream).into_result()
}

/// Parse the tokens into an AST.
pub fn parse(mut tokens: VecDeque<TokenContainer>) -> ResultOption<FileNode> {
    // This function will add more code when the other functions are implemented
    FileNode::parse(&mut tokens)
}
