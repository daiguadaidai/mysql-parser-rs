use crate::ast::ci_str::CIStr;
use crate::ast::column_name::ColumnName;
use crate::ast::expr_node::ColumnNameExpr;
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn column_name(i: Input) -> IResult<ColumnName> {
    alt((
        map(rule!(Ident), |(col_name)| {
            let name = col_name.get_trim_start_end_text('`');

            let mut cn = ColumnName::default();
            cn.name = CIStr::new(name);
            cn
        }),
        map(rule!(Ident ~ "." ~ Ident), |(tbl_name, _, col_name)| {
            let table = tbl_name.get_trim_start_end_text('`');
            let name = col_name.get_trim_start_end_text('`');

            let mut cn = ColumnName::default();
            cn.table = CIStr::new(table);
            cn.name = CIStr::new(name);
            cn
        }),
        map(
            rule!(Ident ~ "." ~ Ident ~ "." ~ Ident),
            |(schema_name, _, tbl_name, _, col_name)| {
                let schema = schema_name.get_trim_start_end_text('`');
                let table = tbl_name.get_trim_start_end_text('`');
                let name = col_name.get_trim_start_end_text('`');

                let mut cn = ColumnName::default();
                cn.schema = CIStr::new(schema);
                cn.table = CIStr::new(table);
                cn.name = CIStr::new(name);
                cn
            },
        ),
    ))(i)
}

pub fn column_name_list(i: Input) -> IResult<Vec<ColumnName>> {
    separated_list1(map(rule!(","), |_| ()), column_name)(i)
}

pub fn simple_ident(i: Input) -> IResult<ColumnNameExpr> {
    map(rule!(#column_name), |name| ColumnNameExpr { name: name })(i)
}
