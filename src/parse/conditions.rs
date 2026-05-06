use chumsky::{
    Parser,
    error::Rich,
    extra::{self},
    input::ValueInput,
    prelude::{choice, empty, just, recursive},
    span::SimpleSpan,
};

use crate::{
    ast::nodes::{
        conditions::{Condition, Sula},
        expressions::Expression,
        statements::StatementList,
    },
    tokens::NewTokens,
};

pub fn condition_parser<'tok, 'src: 'tok, I>(
    exp: impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone + 'tok,
    stal: impl Parser<'tok, I, StatementList<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>>
    + Clone
    + 'tok,
) -> impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    let ij = just(NewTokens::KeywordIf);
    let sula = just(NewTokens::KeywordElse);

    let full = recursive(|cond| {
        let else_if = sula
            .clone()
            .ignore_then(cond.clone())
            .map(|condition| Some(Box::new(Sula::Condition(condition))));
        let default_case = sula
            .clone()
            .ignore_then(stal.clone())
            .map(|statements| Some(Box::new(Sula::Scope(statements))));
        let negative = choice((else_if, default_case, empty().map(|_| None)));

        ij.ignore_then(exp)
            .then(stal)
            .then(negative)
            .map(|((condition, positive), negative)| Condition {
                condition,
                positive,
                negative,
            })
    });

    full.map(|conditon| Expression::Cond(Box::new(conditon)))
}
