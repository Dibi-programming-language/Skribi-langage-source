use std::collections::VecDeque;

use chumsky::error::Rich;
use chumsky::input::{Input, Stream, ValueInput};
use chumsky::span::SimpleSpan;
use chumsky::{IterParser, Parser, extra, select};
use logos::SpannedIter;

use crate::ast::nodes::ParsedFileRoot;
use crate::ast::nodes::expressions::Expression;
use crate::ast::nodes::primitive_values::PrimitiveValue;
use crate::ast::nodes::statements::Statement;
use crate::parse::nodes::files_node::FileNode;
use crate::skr_errors::ResultOption;
use crate::tokens::{NewTokens, TokenContainer};

pub(crate) mod nodes;

fn value_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, PrimitiveValue, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    select! {
        NewTokens::Bool(val) => PrimitiveValue::Bool(val),
        NewTokens::Float(val) => PrimitiveValue::Float(val),
        NewTokens::Int(val) => PrimitiveValue::Int(val),
        NewTokens::String(source) => PrimitiveValue::String(source.to_owned())
    }
}

fn expression_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    value_parser().map(Expression::ValueBase)
}

fn statement_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, Statement<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    expression_parser().map(Statement::Exp)
}

fn root_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, ParsedFileRoot<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    statement_parser()
        .repeated()
        .collect()
        .map(ParsedFileRoot::new)
}

#[allow(dead_code)]
pub fn new_parse<'a>(
    tokens: SpannedIter<'a, NewTokens<'a>>,
    src_len: usize,
) -> Result<ParsedFileRoot<'a>, Vec<Rich<'a, NewTokens<'a>>>> {
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
