use chumsky::{
    Parser,
    error::Rich,
    extra::{self},
    input::ValueInput,
    prelude::{just, one_of, recursive},
    select,
    span::SimpleSpan,
};

use crate::{
    ast::nodes::{declarations::VariableDeclaration, expressions::Expression},
    tokens::NewTokens,
};

pub fn variable_declaration_parser<'tok, 'src: 'tok, I>(
    exp: impl Parser<'tok, I, Expression<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone + 'tok,
) -> impl Parser<'tok, I, VariableDeclaration<'src>, extra::Err<Rich<'tok, NewTokens<'src>>>> + Clone
where
    I: ValueInput<'tok, Token = NewTokens<'src>, Span = SimpleSpan>,
{
    let identifier = select! {
        NewTokens::Identifier(str) => str
    };
    let commun = identifier
        .then(identifier)
        .then(exp)
        .map(|((a, c), b)| VariableDeclaration::new(a, c, b));

    let res = just(NewTokens::UseType).ignore_then(commun.clone());

    let rec = recursive(|modifiers| {
        let base = one_of(vec![
            NewTokens::KeyPrivate,
            NewTokens::KeyGlobal,
            NewTokens::KeyConstant,
        ])
        .then(modifiers.or(commun));

        res.clone().or(base.map(
            |(token, var): (NewTokens<'src>, VariableDeclaration<'src>)| match token {
                NewTokens::KeyGlobal => var.global(),
                NewTokens::KeyConstant => var.constant(),
                NewTokens::KeyPrivate => var.private(),
                _ => unreachable!(),
            },
        ))
    });

    rec
}
