use chumsky::{
    Parser,
    error::Rich,
    extra::{self, Full},
    input::ValueInput,
    prelude::one_of,
    select,
    span::SimpleSpan,
};

use crate::{
    ast::nodes::{base::ValueBase, expressions::Expression, operations::BinaryOperation},
    tokens::NewTokens,
};

pub fn value_parser<'tok, 'src: 'tok, I>()
-> impl Parser<'tok, I, ValueBase, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    select! {
        NewTokens::Bool(val) => ValueBase::Bool(val),
        NewTokens::Float(val) => ValueBase::Float(val),
        NewTokens::Int(val) => ValueBase::Int(val),
        NewTokens::String(source) => ValueBase::String(source.to_owned())
    }
    .labelled("base value")
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
pub fn binop_parser<'tok, 'src: 'tok, I>(
    exp: impl Parser<'tok, I, Expression<'src>, Full<Rich<'tok, NewTokens<'src>>, (), ()>>
    + Clone
    + 'tok,
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

    binary_or.labelled("binary operation")
}
