use crate::ast::ci_str::CIStr;
use crate::ast::table_name::TableName;
use crate::common::misc::is_in_correct_identifier_name;
use crate::parser::common::*;
use crate::parser::error::ErrorKind;
use crate::parser::input::Input;
use crate::parser::statements::common::identifier;
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

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
