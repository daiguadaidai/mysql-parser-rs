use crate::ast::ci_str::CIStr;
use crate::ast::common::{
    FULLTEXT_SEARCH_MODIFIER_BOOLEAN_MODE, FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE,
    FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION,
};
use crate::ast::expr_node::{ExprNode, ValueExpr, ValueExprKind};
use crate::ast::functions::TimeUnitType;
use crate::ast::table_name::TableName;
use crate::charset::charset;
use crate::common::misc::is_in_correct_identifier_name;
use crate::mysql::consts::PriorityEnum;
use crate::parser::common::*;
use crate::parser::error::ErrorKind;
use crate::parser::input::Input;
use crate::parser::statements::keywords::{
    not_keyword_token, tidb_keyword, time_unit_1, timestamp_unit, timestamp_unit_sql_tsi,
    un_reserved_keyword,
};
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn priority(i: Input) -> IResult<PriorityEnum> {
    alt((
        map(rule!("LOW_PRIORITY"), |_| PriorityEnum::LowPriority),
        map(rule!("HIGH_PRIORITY"), |_| PriorityEnum::HighPriority),
        map(rule!("DELAYED"), |_| PriorityEnum::DelayedPriority),
    ))(i)
}

pub fn log_or(i: Input) -> IResult<()> {
    map(rule!(OR | PipesAsOr), |_| {})(i)
}

pub fn log_and(i: Input) -> IResult<()> {
    map(rule!(AND | "&&"), |_| {})(i)
}

pub fn time_unit(i: Input) -> IResult<TimeUnitType> {
    alt((
        map(rule!(#timestamp_unit), |(t)| t),
        map(rule!(#timestamp_unit_sql_tsi), |(t)| t),
        map(rule!(#time_unit_1), |(t)| t),
    ))(i)
}

pub fn fulltext_search_modifier_opt(i: Input) -> IResult<isize> {
    alt((
        map(rule!(IN ~ NATURAL ~ LANGUAGE ~ MODE), |(_, _, _, _)| {
            FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE
        }),
        map(
            rule!(IN ~ NATURAL ~ LANGUAGE ~ MODE ~ WITH ~ QUERY ~ EXPANSION),
            |(_, _, _, _, _, _, _)| {
                FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE
                    | FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION
            },
        ),
        map(rule!(IN ~ BOOLEAN ~ MODE), |(_, _, _)| {
            FULLTEXT_SEARCH_MODIFIER_BOOLEAN_MODE
        }),
        map(rule!(IN ~ QUERY ~ EXPANSION), |(_, _, _)| {
            FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION
        }),
    ))(i)
}

pub fn ident(i: Input) -> IResult<String> {
    map(rule!(Ident), |t| t.text().to_string())(i)
}

pub fn string_lit(i: Input) -> IResult<String> {
    map(rule!(LiteralString), |t| t.text().to_string())(i)
}

pub fn identifier(i: Input) -> IResult<String> {
    map(
        rule!(#ident | #un_reserved_keyword | #not_keyword_token | #tidb_keyword),
        |(s)| s,
    )(i)
}

pub fn string_name(i: Input) -> IResult<String> {
    map(rule!(#string_lit | #identifier), |(s)| s)(i)
}

pub fn charset_name(i: Input) -> IResult<String> {
    alt((
        map(rule!(#string_name), |(s)| {
            /*
            // Validate input charset name to keep the same behavior as parser of MySQL.
            cs, err := charset.GetCharsetInfo($1)
            if err != nil {
                yylex.AppendError(ErrUnknownCharacterSet.GenWithStackByArgs($1))
                return 1
            }
            // Use charset name returned from charset.GetCharsetInfo(),
            // to keep lower case of input for generated column restore.
            $$ = cs.Name

             */
            s
        }),
        map(rule!(BINARY), |(t)| t.text().to_string()),
    ))(i)
}

pub fn optional_braces(i: Input) -> IResult<()> {
    map(rule!("(" ~ ")"), |_| ())(i)
}

pub fn func_datetime_prec(i: Input) -> IResult<Option<ValueExpr>> {
    alt((
        map(rule!(#optional_braces?), |_| None),
        map(rule!("(" ~ LiteralInteger ~ ")"), |(_, t, _)| {
            let val = get_isize_form_num(t.text());
            let value_expr =
                ValueExpr::new(t.text(), ValueExprKind::Isize(val), i.charset, i.collation);
            Some(value_expr)
        }),
    ))(i)
}

pub fn func_datetime_prec_list_opt(i: Input) -> IResult<Vec<ExprNode>> {
    map(rule!(LiteralInteger?), |t| match t {
        None => vec![],
        Some(token) => {
            let val = get_isize_form_num(token.text());
            let value_expr = ValueExpr::new(
                token.text(),
                ValueExprKind::Isize(val),
                i.charset,
                i.collation,
            );
            vec![ExprNode::ValueExpr(value_expr)]
        }
    })(i)
}

pub fn field_len(i: Input) -> IResult<isize> {
    map(rule!("(" ~ #length_num ~ ")"), |(_, val, _)| val as isize)(i)
}

pub fn length_num(i: Input) -> IResult<u64> {
    map(rule!(LiteralInteger), |(val)| get_u64_form_num(val.text()))(i)
}

pub fn table_name(i: Input) -> IResult<TableName> {
    alt((
        map(rule!(#identifier), |(table_name)| {
            let mut tbl_name = TableName::default();
            tbl_name.name = CIStr::new(&table_name);
            tbl_name
        }),
        map_res(
            rule!(#identifier ~ "." ~ #identifier),
            |(schema_name, _, table_name)| {
                if is_in_correct_identifier_name(&schema_name) {
                    return Err(nom::Err::Error(ErrorKind::ExpectText(
                        "parse table_name in is_in_correct_identifier_name",
                    )));
                }

                let mut tbl_name = TableName::default();

                tbl_name.schema = CIStr::new(&schema_name);
                tbl_name.name = CIStr::new(&table_name);

                Ok(tbl_name)
            },
        ),
        map(rule!("*" ~ "." ~ #identifier), |(_, _, table_name)| {
            let mut tbl_name = TableName::default();
            tbl_name.schema = CIStr::new("*");
            tbl_name.name = CIStr::new(&table_name);
            tbl_name
        }),
    ))(i)
}

pub fn collation_name(i: Input) -> IResult<String> {
    alt((
        map_res(
            rule!(#string_name),
            |(name)| match charset::get_collation_by_name(&name) {
                Ok(v) => Ok(v.name),
                Err(e) => Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                    "{}",
                    e.to_string()
                )))),
            },
        ),
        map(rule!(BINARY), |(_)| Ok(charset::COLLATION_BIN.to_string())),
    ))(i)
}

pub fn opt_null_treatment(i: Input) -> IResult<bool> {
    map(rule!(#opt_null_treatment_sub?), |(b)| {
        b.unwrap_or_else(|| false)
    })(i)
}

pub fn opt_null_treatment_sub(i: Input) -> IResult<bool> {
    alt((
        map(rule!(RESPECT ~ NULLS), |(_, _)| false),
        map(rule!(IGNORE ~ NULLS), |(_, _)| true),
    ))(i)
}

pub fn opt_from_first_last(i: Input) -> IResult<bool> {
    map(rule!(#opt_from_first_last_sub?), |(b)| {
        b.unwrap_or_else(|| false)
    })(i)
}

pub fn opt_from_first_last_sub(i: Input) -> IResult<bool> {
    alt((
        map(rule!(FROM ~ FIRST), |(_, _)| false),
        map(rule!(FROM ~ LAST), |(_, _)| true),
    ))(i)
}
