// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::common::span::{Range, Span};
use crate::parser::error::{Error, ErrorKind};
use crate::parser::input::Input;
use crate::parser::input::WithSpan;
use crate::parser::token::*;
use crate::parser::token_kind::TokenKind;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::Offset;
use nom::Slice;
use pratt::{PrattError, PrattParser, Precedence};

pub type IResult<'a, Output> = nom::IResult<Input<'a>, Output, Error<'a>>;

pub fn match_text(text: &'static str) -> impl FnMut(Input) -> IResult<&Token> {
    move |i| match i.tokens.first().filter(|token| token.text() == text) {
        Some(token) => Ok((i.slice(1..), token)),
        _ => Err(nom::Err::Error(Error::from_error_kind(
            i,
            ErrorKind::ExpectText(text),
        ))),
    }
}

pub fn match_token(kind: TokenKind) -> impl FnMut(Input) -> IResult<&Token> {
    move |i| match i.tokens.first().filter(|token| token.kind == kind) {
        Some(token) => Ok((i.slice(1..), token)),
        _ => Err(nom::Err::Error(Error::from_error_kind(
            i,
            ErrorKind::ExpectToken(kind),
        ))),
    }
}

pub fn any_token(i: Input) -> IResult<&Token> {
    match i.tokens.first().filter(|token| token.kind != TokenKind::EOI) {
        Some(token) => Ok((i.slice(1..), token)),
        _ => Err(nom::Err::Error(Error::from_error_kind(
            i,
            ErrorKind::Other("expected any token but reached the end"),
        ))),
    }
}

pub fn comma_separated_list0<'a, T>(
    item: impl FnMut(Input<'a>) -> IResult<'a, T>,
) -> impl FnMut(Input<'a>) -> IResult<'a, Vec<T>> {
    separated_list0(match_text(","), item)
}

pub fn comma_separated_list0_ignore_trailing<'a, T>(
    item: impl FnMut(Input<'a>) -> IResult<'a, T>,
) -> impl FnMut(Input<'a>) -> IResult<'a, Vec<T>> {
    nom::multi::separated_list0(match_text(","), item)
}

pub fn comma_separated_list1_ignore_trailing<'a, T>(
    item: impl FnMut(Input<'a>) -> IResult<'a, T>,
) -> impl FnMut(Input<'a>) -> IResult<'a, Vec<T>> {
    nom::multi::separated_list1(match_text(","), item)
}

pub fn semicolon_terminated_list1<'a, T>(
    item: impl FnMut(Input<'a>) -> IResult<'a, T>,
) -> impl FnMut(Input<'a>) -> IResult<'a, Vec<T>> {
    many1(terminated(item, match_text(";")))
}

pub fn comma_separated_list1<'a, T>(
    item: impl FnMut(Input<'a>) -> IResult<'a, T>,
) -> impl FnMut(Input<'a>) -> IResult<'a, Vec<T>> {
    separated_list1(match_text(","), item)
}

/// A fork of `separated_list0` from nom, but never forgive parser error
/// after a separator is encountered, and always forgive the first element
/// failure.
pub fn separated_list0<I, O, O2, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> nom::IResult<I, Vec<O>, E>
where
    I: Clone + nom::InputLength,
    F: nom::Parser<I, O, E>,
    G: nom::Parser<I, O2, E>,
    E: nom::error::ParseError<I>,
{
    move |mut i: I| {
        let mut res = Vec::new();

        match f.parse(i.clone()) {
            Err(_) => return Ok((i, res)),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(nom::Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(nom::Err::Error(E::from_error_kind(
                            i1,
                            nom::error::ErrorKind::SeparatedList,
                        )));
                    }

                    match f.parse(i1.clone()) {
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}

/// A fork of `separated_list1` from nom, but never forgive parser error
/// after a separator is encountered.
pub fn separated_list1<I, O, O2, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> nom::IResult<I, Vec<O>, E>
where
    I: Clone + nom::InputLength,
    F: nom::Parser<I, O, E>,
    G: nom::Parser<I, O2, E>,
    E: nom::error::ParseError<I>,
{
    move |mut i: I| {
        let mut res = Vec::new();

        // Parse the first element
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(nom::Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(nom::Err::Error(E::from_error_kind(
                            i1,
                            nom::error::ErrorKind::SeparatedList,
                        )));
                    }

                    match f.parse(i1.clone()) {
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}

/// A fork of `map_res` from nom, but doesn't require `FromExternalError`.
pub fn map_res<'a, O1, O2, F, G>(
    mut parser: F,
    mut f: G,
) -> impl FnMut(Input<'a>) -> IResult<'a, O2>
where
    F: nom::Parser<Input<'a>, O1, Error<'a>>,
    G: FnMut(O1) -> Result<O2, nom::Err<ErrorKind>>,
{
    move |input: Input| {
        let i = input;
        let bt = i.backtrace.clone();
        let (rest, o1) = parser.parse(input)?;
        match f(o1) {
            Ok(o2) => Ok((rest, o2)),
            Err(nom::Err::Error(e)) => {
                i.backtrace.restore(bt);
                Err(nom::Err::Error(Error::from_error_kind(i, e)))
            }
            Err(nom::Err::Failure(e)) => {
                i.backtrace.restore(bt);
                Err(nom::Err::Failure(Error::from_error_kind(i, e)))
            }
            Err(nom::Err::Incomplete(_)) => unreachable!(),
        }
    }
}

/// Try to find an error pattern that user may have made, and hint them with suggestion.
pub fn error_hint<'a, O, F>(
    mut match_error: F,
    message: &'static str,
) -> impl FnMut(Input<'a>) -> IResult<'a, ()>
where
    F: nom::Parser<Input<'a>, O, Error<'a>>,
{
    move |input: Input| match match_error.parse(input) {
        Ok(_) => Err(nom::Err::Error(Error::from_error_kind(
            input,
            ErrorKind::Other(message),
        ))),
        Err(_) => Ok((input, ())),
    }
}

pub fn transform_span(tokens: &[Token]) -> Span {
    Some(Range {
        start: tokens.first().unwrap().span.start,
        end: tokens.last().unwrap().span.end,
    })
}

pub fn run_pratt_parser<'a, I, P, E>(
    mut parser: P,
    iter: &I,
    rest: Input<'a>,
    input: Input<'a>,
) -> IResult<'a, P::Output>
where
    E: std::fmt::Debug,
    P: PrattParser<I, Input=WithSpan<'a, E>, Error=&'static str>,
    I: Iterator<Item=P::Input> + ExactSizeIterator + Clone,
{
    let mut iter_cloned = iter.clone();
    let mut iter = iter.clone().peekable();
    let len = iter.len();
    let expr = parser
        .parse_input(&mut iter, Precedence(0))
        .map_err(|err| {
            // Rollback parsing footprint on unused expr elements.
            input.backtrace.clear();

            let err_kind = match err {
                PrattError::EmptyInput => ErrorKind::Other("expecting an oprand"),
                PrattError::UnexpectedNilfix(_) => ErrorKind::Other("unable to parse the element"),
                PrattError::UnexpectedPrefix(_) => {
                    ErrorKind::Other("unable to parse the prefix operator")
                }
                PrattError::UnexpectedInfix(_) => {
                    ErrorKind::Other("missing lhs or rhs for the binary operator")
                }
                PrattError::UnexpectedPostfix(_) => {
                    ErrorKind::Other("unable to parse the postfix operator")
                }
                PrattError::UserError(err) => ErrorKind::Other(err),
            };

            let span = iter_cloned
                .nth(len - iter.len() - 1)
                .map(|elem| elem.span)
                // It's safe to slice one more token because input must contain EOI.
                .unwrap_or_else(|| rest.slice(..1));

            nom::Err::Error(Error::from_error_kind(span, err_kind))
        })?;
    if let Some(elem) = iter.peek() {
        // Rollback parsing footprint on unused expr elements.
        input.backtrace.clear();
        Ok((input.slice(input.offset(&elem.span)..), expr))
    } else {
        Ok((rest, expr))
    }
}

pub fn check_template_mode<'a, O, F>(mut parser: F) -> impl FnMut(Input<'a>) -> IResult<'a, O>
where
    F: nom::Parser<Input<'a>, O, Error<'a>>,
{
    move |input: Input| {
        parser.parse(input).and_then(|(i, res)| {
            if input.mode.is_template() {
                Ok((i, res))
            } else {
                i.backtrace.clear();
                let error = Error::from_error_kind(
                    input,
                    ErrorKind::Other("variable is only available in SQL template"),
                );
                Err(nom::Err::Failure(error))
            }
        })
    }
}

macro_rules! declare_experimental_feature {
    ($check_fn_name: ident, $feature_name: literal) => {
        pub fn $check_fn_name<'a, O, F>(
            is_exclusive: bool,
            mut parser: F,
        ) -> impl FnMut(Input<'a>) -> IResult<'a, O>
        where
            F: nom::Parser<Input<'a>, O, Error<'a>>,
        {
            move |input: Input| {
                parser.parse(input).and_then(|(i, res)| {
                    if input.dialect.is_experimental() {
                        Ok((i, res))
                    } else {
                        i.backtrace.clear();
                        let error = Error::from_error_kind(
                            input,
                            ErrorKind::Other(
                                concat!(
                                    $feature_name,
                                    " only works in experimental dialect, try `set sql_dialect = 'experimental'`"
                                )
                            ),
                        );
                        if is_exclusive {
                            Err(nom::Err::Failure(error))
                        } else {
                            Err(nom::Err::Error(error))
                        }
                    }
                })
            }
        }
    };
}

declare_experimental_feature!(check_experimental_chain_function, "chain function");
declare_experimental_feature!(check_experimental_list_comprehension, "list comprehension");
