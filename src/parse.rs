use std::collections::VecDeque;

use chumsky::error::Rich;
use chumsky::input::{Input, Stream, ValueInput};
use chumsky::span::SimpleSpan;
use chumsky::{Parser, extra, select};
use logos::SpannedIter;

use crate::ast::nodes::AstRoot;
use crate::parse::nodes::files_node::FileNode;
use crate::skr_errors::ResultOption;
use crate::tokens::{NewTokens, TokenContainer};

pub(crate) mod nodes;

fn root_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, AstRoot<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    select! {
        NewTokens::Bool(_) => AstRoot {content: vec!()}
    }
}

#[allow(dead_code)]
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
