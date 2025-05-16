use crate::ast::set_opr_stmt::SetOprStmt;
use crate::parser::common::IResult;
use crate::parser::input::Input;
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn sub_select(i: Input) -> IResult<SetOprStmt> {
    alt((
        map(rule!(#set_opr_stmt_wout_limit_order_by), |(stmt)| stmt),
        map(rule!(#set_opr_stmt_with_limit_order_by), |(stmt)| stmt),
    ))(i)
}

pub fn set_opr_stmt_wout_limit_order_by(i: Input) -> IResult<SetOprStmt> {}

pub fn set_opr_stmt_with_limit_order_by(i: Input) -> IResult<SetOprStmt> {}
