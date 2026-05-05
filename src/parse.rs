use std::collections::VecDeque;

use chumsky::error::Rich;
use chumsky::extra::Full;
use chumsky::input::{Input, Stream, ValueInput};
use chumsky::prelude::{Recursive, choice, just, one_of, recursive};
use chumsky::span::SimpleSpan;
use chumsky::{IterParser, Parser, extra, select};
use logos::SpannedIter;

use crate::ast::nodes::AstRoot;
use crate::ast::nodes::base::ValueBase;
use crate::ast::nodes::expressions::Expression;
use crate::ast::nodes::operations::BinaryOperation;
use crate::ast::nodes::statements::Statement;
use crate::parse::nodes::files_node::FileNode;
use crate::skr_errors::ResultOption;
use crate::tokens::{NewTokens, TokenContainer};

pub(crate) mod nodes;

fn value_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, ValueBase, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    select! {
        NewTokens::Bool(val) => ValueBase::Bool(val),
        NewTokens::Float(val) => ValueBase::Float(val),
        NewTokens::Int(val) => ValueBase::Int(val),
        NewTokens::String(source) => ValueBase::String(source.to_owned())
    }.labelled("base value")
}

fn list_binop_parser<'tok, 'src: 'tok, I>(
    elements: Vec<NewTokens<'src>>,
    then: impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone,
) -> impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    then.clone()
        .foldl(one_of(elements).then(then).repeated(), |lhs, (op, rhs)| {
            BinaryOperation::from(op, lhs, rhs)
        })
}

/// With:
/// 1. * and /
/// 2. + and -
/// 3. <=, >=, <, >, = and !=
/// 4. &&
/// 5. ||
///
/// 0 is for unary
fn binop_parser<'tok, 'src: 'tok, I>(
    exp: Recursive<
        dyn Parser<'tok, I, Expression<'src>, Full<Rich<'tok, NewTokens<'src>>, (), ()>> + 'tok,
    >,
) -> impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    let binary_md = list_binop_parser(vec![NewTokens::Mul, NewTokens::Div], exp);
    let binary_as = list_binop_parser(vec![NewTokens::Add, NewTokens::Sub], binary_md);
    let binary_cmp = list_binop_parser(
        vec![
            NewTokens::LessOrEqual,
            NewTokens::GreaterOrEqual,
            NewTokens::LessThan,
            NewTokens::GreaterThan,
            NewTokens::Equal,
            NewTokens::NotEqual,
        ],
        binary_as,
    );
    let binary_and = list_binop_parser(vec![NewTokens::And], binary_cmp);
    let binary_or = list_binop_parser(vec![NewTokens::Or], binary_and);

    binary_or
}

fn expression_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    recursive(|exp| {
        let priority = recursive(|_| {
            choice((
                value_parser().map(Expression::ValueBase),
                exp.delimited_by(
                    just(NewTokens::LeftParenthesis),
                    just(NewTokens::RightParenthesis),
                ),
            ))
        });

        choice((binop_parser(priority.clone()), priority.clone()))
    })
}

fn statement_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, Statement<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    expression_parser()
        .map(Statement::Exp)
        .then_ignore(just(NewTokens::Space).repeated())
}

fn root_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, AstRoot<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    statement_parser().repeated().collect().map(AstRoot::new)
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
