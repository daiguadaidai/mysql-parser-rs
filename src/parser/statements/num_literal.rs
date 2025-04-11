use crate::ast::expr_node::{ValueExpr, ValueExprKind};
use crate::parser::common::*;
use crate::parser::error::ErrorKind;
use crate::parser::input::Input;
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn i64_num(i: Input) -> IResult<i64> {
    map_res(rule!(LiteralInteger), |(val)| {
        let v = get_i64_form_num(val.text());
        match v {
            Ok(val) => Ok(val),
            Err(e) => Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                "{} is out of range [–9223372036854775808,9223372036854775807]. {}",
                val.text(),
                e.to_string()
            )))),
        }
    })(i)
}

pub fn f64_num(i: Input) -> IResult<f64> {
    map_res(rule!(LiteralFloat), |(val)| {
        let v = get_f64_form_num(val.text());
        match v {
            Ok(val) => Ok(val),
            Err(e) => Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                "{} parse to f64 error. {}",
                val.text(),
                e.to_string()
            )))),
        }
    })(i)
}

pub fn signed_num(i: Input) -> IResult<i64> {
    alt((
        map(rule!(#i64_num), |(val)| val),
        map(rule!("+" ~ #i64_num), |(_, val)| val),
        map_res(rule!("-" ~ LiteralInteger), |(_, val)| {
            let unsigned_num = get_u64_form_num(val.text());
            if unsigned_num > 9223372036854775808 {
                return Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                    "current value: {}. the Signed Value should be at the range of [-9223372036854775808, 9223372036854775807].",
                    val.text(),
                ))));
            } else if unsigned_num == 9223372036854775808 {
                let d = 1_i64 << 63;
                return Ok(d);
            } else {
                let d = -(unsigned_num as i64);
                Ok(d)
            }
        }),
    ))(i)
}
pub fn num_literal(i: Input) -> IResult<ValueExpr> {
    alt((
        map(rule!(#i64_num), |(v)| ValueExpr {
            s: v.to_string(),
            kind: ValueExprKind::I64(v),
            charset: i.charset.to_string(),
            collation: i.collation.to_string(),
        }),
        map(rule!(#f64_num), |(v)| ValueExpr {
            s: v.to_string(),
            kind: ValueExprKind::F64(v),
            charset: i.charset.to_string(),
            collation: i.collation.to_string(),
        }),
    ))(i)
}
