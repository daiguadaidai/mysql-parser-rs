use crate::ast::expr_node::{ExprNode, ValueExpr, ValueExprKind};
use crate::charset::charset;
use crate::parser::common::*;
use crate::parser::error::ErrorKind;
use crate::parser::input::Input;
use crate::parser::statements::common::string_lit;
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn string_literal(i: Input) -> IResult<ValueExpr> {
    map(rule!(#string_literal_sub), |ss| ValueExpr {
        s: ss.join(""),
        kind: ValueExprKind::String,
        charset: i.charset.to_string(),
        collation: i.collation.to_string(),
    })(i)
}
pub fn string_literal_sub(i: Input) -> IResult<Vec<String>> {
    separated_list1(map(rule!(""), |_| ()), string_lit)(i)
}

pub fn literal(i: Input) -> IResult<ValueExpr> {
    alt((
        map(rule!(FALSE), |(t)| ValueExpr {
            s: t.text().to_string(),
            kind: ValueExprKind::Bool(false),
            charset: i.charset.to_string(),
            collation: i.collation.to_string(),
        }),
        map(rule!(NULL), |(t)| ValueExpr {
            s: t.text().to_string(),
            kind: ValueExprKind::None,
            charset: i.charset.to_string(),
            collation: i.collation.to_string(),
        }),
        map(rule!(TRUE), |(t)| ValueExpr {
            s: t.text().to_string(),
            kind: ValueExprKind::Bool(true),
            charset: i.charset.to_string(),
            collation: i.collation.to_string(),
        }),
        map_res(rule!(LiteralFloat), |(t)| {
            let s = t.text().to_string();
            let v = get_f64_form_num(&s);
            match v {
                Ok(val) => Ok(ValueExpr {
                    s,
                    kind: ValueExprKind::F64(val),
                    charset: i.charset.to_string(),
                    collation: i.collation.to_string(),
                }),
                Err(e) => Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                    "{} parse to f64 error. {}",
                    &s,
                    e.to_string()
                )))),
            }
        }),
        map_res(rule!(LiteralInteger), |(t)| {
            let s = t.text().to_string();
            let v = get_i64_form_num(&s);
            match v {
                Ok(val) => Ok(ValueExpr {
                    s,
                    kind: ValueExprKind::I64(val),
                    charset: i.charset.to_string(),
                    collation: i.collation.to_string(),
                }),
                Err(e) => Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                    "{} is out of range [–9223372036854775808,9223372036854775807]. {}",
                    &s,
                    e.to_string()
                )))),
            }
        }),
        map(rule!(#string_literal), |(e)| ExprNode::ValueExpr(e)),
        map_res(rule!("UNDERSCORE_CHARSET" ~ LiteralString), |(t, st)| {
            let s = t.text();
            let co = match charset::get_default_collation_legacy(s) {
                Ok(v) => v,
                Err(e) => {
                    return Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                        "{}, Unsupported character introducer: '{}'",
                        e.to_string(),
                        s
                    ))))
                }
            };

            Ok(ExprNode::ValueExpr(ValueExpr {
                s: st.text().to_string(),
                kind: ValueExprKind::String,
                charset: s.to_string(),
                collation: co,
            }))
        }),
        map(rule!(LiteralHex), |(t)| {
            ExprNode::ValueExpr(ValueExpr {
                s: t.text().to_string(),
                kind: ValueExprKind::HexLiteral,
                charset: i.charset.to_string(),
                collation: i.collation.to_string(),
            })
        }),
        map(rule!(LiteralBit), |(t)| {
            ExprNode::ValueExpr(ValueExpr {
                s: t.text().to_string(),
                kind: ValueExprKind::BitLiteral,
                charset: i.charset.to_string(),
                collation: i.collation.to_string(),
            })
        }),
        map_res(rule!("UNDERSCORE_CHARSET" ~ LiteralHex), |(t, st)| {
            let s = t.text();
            let co = match charset::get_default_collation_legacy(s) {
                Ok(v) => v,
                Err(e) => {
                    return Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                        "{}, Unsupported character introducer: '{}'",
                        e.to_string(),
                        s
                    ))))
                }
            };

            Ok(ExprNode::ValueExpr(ValueExpr {
                s: st.text().to_string(),
                kind: ValueExprKind::HexLiteral,
                charset: s.to_string(),
                collation: co,
            }))
        }),
        map_res(rule!("UNDERSCORE_CHARSET" ~ LiteralBit), |(t, st)| {
            let s = t.text();
            let co = match charset::get_default_collation_legacy(s) {
                Ok(v) => v,
                Err(e) => {
                    return Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                        "{}, Unsupported character introducer: '{}'",
                        e.to_string(),
                        s
                    ))))
                }
            };

            Ok(ExprNode::ValueExpr(ValueExpr {
                s: st.text().to_string(),
                kind: ValueExprKind::BitLiteral,
                charset: s.to_string(),
                collation: co,
            }))
        }),
    ))(i)
}
