use crate::ast::statement::Statement;
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::statements::select_statement::select_statement;
use crate::parser::token_kind::TokenKind::EOI;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn statement(i: Input) -> IResult<Statement> {
    map(
        rule! (
            #statement_body ~ ";"? ~ &EOI
        ),
        |(stmt, _, _)| stmt,
    )(i)
}

pub fn statement_body(i: Input) -> IResult<Statement> {
    alt((rule!(
        #select_statement : "`SELECT <statement>`"
    ),))(i)
}
