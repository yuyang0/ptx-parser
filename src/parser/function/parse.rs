use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{multispace0, space0, space1},
    combinator::{map, opt, value},
    sequence::preceded,
    IResult, Parser,
};

use crate::parser::{
    comment::parse::many1_comments_or_whitespace, parse_braced_balanced, parse_name,
    parse_parenthesized_naive,
};

use super::{body::FunctionBody, Function, FunctionSignature, Parameter, Parameters, ReturnValue};

pub(crate) fn parse_function(input: &str) -> IResult<&str, Function> {
    let (input, signature) = parse_function_signature(input)?;
    let (input, body) = preceded(
        opt(many1_comments_or_whitespace),
        alt((map(char(';'), |_| None), parse_function_body.map(Some))),
    )(input)?;
    Ok((input, Function { signature, body }))
}

pub(super) fn parse_function_body(input: &str) -> IResult<&str, FunctionBody> {
    parse_braced_balanced
        .map(|raw_string| FunctionBody {
            body: Some(raw_string),
        })
        .parse(input)
}

pub(super) fn parse_function_signature(input: &str) -> IResult<&str, FunctionSignature> {
    let (input, (visible, entry)) = alt((
        value((true, true), tag(".visible").and(space1).and(tag(".entry"))),
        value((false, false), tag(".func")),
    ))(input)?;

    let (input, return_value) = preceded(
        space1,
        opt(parse_parenthesized_naive.map(|raw_string| ReturnValue { raw_string })),
    )(input)?;

    let (input, name) = preceded(space0, parse_name)(input)?;

    let (input, parameters) = preceded(
        multispace0,
        opt(parse_parenthesized_naive.map(|raw_string| -> Parameters {
            let params: Vec<Parameter> = vec![];
            let mut parmas = Parameters { raw_string, params };
            parmas.parse();
            parmas
        })),
    )(input)?;

    Ok((
        input,
        FunctionSignature {
            visible,
            entry,
            return_value,
            name,
            parameters,
        },
    ))
}
