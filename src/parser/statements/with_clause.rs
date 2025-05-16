use crate::ast::ci_str::CIStr;
use crate::ast::common_table_expression::CommonTableExpression;
use crate::ast::with_clause::WithClause;
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::statements::common::{ident_list, identifier};
use crate::parser::statements::sub_select::sub_select;
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn with_clause(i: Input) -> IResult<WithClause> {
    alt((
        map(rule!(WITH ~ #with_list), |(_, with)| with),
        map(rule!(WITH ~ RECURSIVE ~ #with_list), |(_, _, mut with)| {
            with.is_recursive = true;
            with.ctes.iter_mut().for_each(|cte| cte.is_recursive = true);

            with
        }),
    ))(i)
}

pub fn with_list(i: Input) -> IResult<WithClause> {
    map(rule!(#common_table_expr_list), |(ctes)| WithClause {
        is_recursive: false,
        ctes,
    })(i)
}

pub fn common_table_expr(i: Input) -> IResult<CommonTableExpression> {
    map(
        rule!(#identifier ~ #ident_list_with_paren_opt ~ AS ~ #sub_select),
        |(name, col_name_list, _, query)| CommonTableExpression {
            name: CIStr::new(&name),
            query: Some(query),
            col_name_list,
            is_recursive: false,
            consumer_count: 0,
        },
    )(i)
}

pub fn common_table_expr_list(i: Input) -> IResult<Vec<CommonTableExpression>> {
    separated_list1(map(rule!(","), |_| ()), common_table_expr)(i)
}

pub fn ident_list_with_paren(i: Input) -> IResult<Vec<CIStr>> {
    map(rule!("(" ~ #ident_list ~ ")"), |(_, idents, _)| idents)(i)
}

pub fn ident_list_with_paren_opt(i: Input) -> IResult<Vec<CIStr>> {
    map(rule!(#ident_list_with_paren?), |(idents)| {
        idents.unwrap_or_else(|| vec![])
    })(i)
}
