pub mod constants;
mod mappers;

use crate::types::{Expr, ParseError};
use constants::{
    ADD_OPERATOR, DIVIDE_OPERATOR, EXPONENT_OPERATOR, MULTIPLY_OPERATOR, SUBTRACT_OPERATOR,
    UNARY_MINUS_OPERATOR,
};
use mappers::{fold_binary_operator, map_binary_operator, map_number, map_unary_operator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{all_consuming, map, opt},
    error::ParseError as NomParseError,
    multi::many0,
    number::complete::double,
    sequence::{delimited, pair},
    IResult,
};

use self::constants::UNARY_PLUS_OPERATOR;

fn space0_delimited<'a, O, F, E>(parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
    E: NomParseError<&'a str>,
{
    delimited(space0, parser, space0)
}

fn parse_number(i: &str) -> IResult<&str, Expr> {
    map(space0_delimited(double), |number| {
        map_number(number).expect("placeholder result")
    })(i)
}

fn parse_parens(i: &str) -> IResult<&str, Expr> {
    delimited(
        space0_delimited(tag("(")),
        parse_expr,
        space0_delimited(tag(")")),
    )(i)
}

fn parse_item(i: &str) -> IResult<&str, Expr> {
    alt((parse_number, parse_parens))(i)
}

fn parse_factor(i: &str) -> IResult<&str, Expr> {
    let (i, _) = space0(i)?;
    let (i, unary_operator) = opt(alt((tag(UNARY_MINUS_OPERATOR), tag(UNARY_PLUS_OPERATOR))))(i)?;

    let (i, mut item) = parse_item(i)?;
    if let Some(unary_operator) = unary_operator {
        item = map_unary_operator(unary_operator, item).expect("verified unary operators");
    }

    let (i, exponent_operator) = opt(tag(EXPONENT_OPERATOR))(i)?;

    if let Some(exponent_operator) = exponent_operator {
        let (i, exponent) = parse_factor(i)?;
        Ok((
            i,
            map_binary_operator(exponent_operator, item, exponent)
                .expect("verified binary operator (exponent)"),
        ))
    } else {
        Ok((i, item))
    }
}

fn parse_term(i: &str) -> IResult<&str, Expr> {
    let (i, term) = parse_factor(i)?;
    let (i, _) = space0(i)?;
    let (i, expressions) = many0(pair(
        alt((tag(MULTIPLY_OPERATOR), tag(DIVIDE_OPERATOR))),
        parse_factor,
    ))(i)?;

    Ok((
        i,
        fold_binary_operator(term, expressions)
            .expect("verified binary operators (multiply and divide)"),
    ))
}

fn parse_expr(i: &str) -> IResult<&str, Expr> {
    let (i, term) = parse_term(i)?;
    let (i, _) = space0(i)?;
    let (i, expressions) = many0(pair(
        alt((tag(ADD_OPERATOR), tag(SUBTRACT_OPERATOR))),
        parse_term,
    ))(i)?;

    Ok((
        i,
        fold_binary_operator(term, expressions)
            .expect("verified binary operators (add and subtract)"),
    ))
}

pub fn parse(i: &str) -> Result<Expr, ParseError> {
    all_consuming(parse_expr)(i).map(|(_, expr)| expr)
}
