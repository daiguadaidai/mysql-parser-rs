use crate::ast::statement::Statement;
use crate::common::error::{ParseError, Result};
use crate::parser::common::{transform_span, IResult};
use crate::parser::error::{display_parser_error, Backtrace};
use crate::parser::input::{Dialect, Input, ParseMode};
use crate::parser::statements::statement::statement;
use crate::parser::token::{Token, Tokenizer};
use crate::parser::token_kind::TokenKind;

pub fn tokenize_sql(sql: &str) -> Result<Vec<Token>> {
    Tokenizer::new(sql).collect::<Result<Vec<_>>>()
}

/// Parse a SQL string into `Statement`s.
#[fastrace::trace]
pub fn parse_sql(tokens: &[Token], dialect: Dialect) -> Result<Statement> {
    run_parser(tokens, dialect, ParseMode::Default, false, statement)
}

pub fn run_parser<O>(
    tokens: &[Token],
    dialect: Dialect,
    mode: ParseMode,
    allow_partial: bool,
    mut parser: impl FnMut(Input) -> IResult<O>,
) -> Result<O> {
    let backtrace = Backtrace::new();
    let input = Input {
        tokens,
        dialect,
        mode,
        backtrace: &backtrace,
    };
    match parser(input) {
        Ok((rest, res)) => {
            let is_complete = rest[0].kind == TokenKind::EOI;
            if is_complete || allow_partial {
                Ok(res)
            } else {
                Err(ParseError(
                    transform_span(&rest[..1]),
                    "unable to parse rest of the sql".to_string(),
                ))
            }
        }
        Err(nom::Err::Error(err) | nom::Err::Failure(err)) => {
            let source = tokens[0].source;
            Err(ParseError(None, display_parser_error(err, source)))
        }
        Err(nom::Err::Incomplete(_)) => unreachable!(),
    }
}