use crate::ast::ci_str::CIStr;
use crate::ast::column_name::ColumnName;
use crate::ast::common::{
    FULLTEXT_SEARCH_MODIFIER_BOOLEAN_MODE, FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE,
    FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION,
};
use crate::ast::expr_node::ColumnNameExpr;
use crate::ast::functions::TimeUnitType;
use crate::mysql::consts::PriorityEnum;
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::token_kind::TokenKind::{
    Ident, PipesAsOr, AND, BOOLEAN, DAY, DAY_HOUR, DAY_MICROSECOND, DAY_MINUTE, DAY_SECOND,
    EXPANSION, HOUR, HOUR_MICROSECOND, HOUR_MINUTE, HOUR_SECOND, IN, LANGUAGE,
    MICROSECOND, MINUTE, MINUTE_MICROSECOND, MINUTE_SECOND, MODE, MONTH, NATURAL, OR, QUARTER,
    QUERY, SECOND, SECOND_MICROSECOND, SQL_TSI_DAY, SQL_TSI_HOUR, SQL_TSI_MINUTE, SQL_TSI_MONTH,
    SQL_TSI_QUARTER, SQL_TSI_SECOND, SQL_TSI_WEEK, SQL_TSI_YEAR, WEEK, WITH, YEAR,
};
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

pub fn time_unit(i: Input) -> IResult<TimeUnitType> {
    alt((
        map(rule!(#timestamp_unit), |(t)| t),
        map(rule!(#timestamp_unit_sql_tsi), |(t)| t),
        map(rule!(#time_unit_1), |(t)| t),
    ))(i)
}

pub fn timestamp_unit(i: Input) -> IResult<TimeUnitType> {
    alt((
        map(rule!(MICROSECOND), |_| TimeUnitType::TimeUnitMicrosecond),
        map(rule!(SECOND), |_| TimeUnitType::TimeUnitSecond),
        map(rule!(MINUTE), |_| TimeUnitType::TimeUnitMinute),
        map(rule!(HOUR), |_| TimeUnitType::TimeUnitHour),
        map(rule!(DAY), |_| TimeUnitType::TimeUnitDay),
        map(rule!(WEEK), |_| TimeUnitType::TimeUnitWeek),
        map(rule!(MONTH), |_| TimeUnitType::TimeUnitMonth),
        map(rule!(QUARTER), |_| TimeUnitType::TimeUnitQuarter),
        map(rule!(YEAR), |_| TimeUnitType::TimeUnitYear),
    ))(i)
}

pub fn timestamp_unit_sql_tsi(i: Input) -> IResult<TimeUnitType> {
    alt((
        map(rule!(SQL_TSI_SECOND), |_| TimeUnitType::TimeUnitSecond),
        map(rule!(SQL_TSI_MINUTE), |_| TimeUnitType::TimeUnitMinute),
        map(rule!(SQL_TSI_HOUR), |_| TimeUnitType::TimeUnitHour),
        map(rule!(SQL_TSI_DAY), |_| TimeUnitType::TimeUnitDay),
        map(rule!(SQL_TSI_WEEK), |_| TimeUnitType::TimeUnitWeek),
        map(rule!(SQL_TSI_MONTH), |_| TimeUnitType::TimeUnitMonth),
        map(rule!(SQL_TSI_QUARTER), |_| TimeUnitType::TimeUnitQuarter),
        map(rule!(SQL_TSI_YEAR), |_| TimeUnitType::TimeUnitYear),
    ))(i)
}

pub fn time_unit_1(i: Input) -> IResult<TimeUnitType> {
    alt((
        map(rule!(SECOND_MICROSECOND), |_| {
            TimeUnitType::TimeUnitSecondMicrosecond
        }),
        map(rule!(MINUTE_MICROSECOND), |_| {
            TimeUnitType::TimeUnitMinuteMicrosecond
        }),
        map(rule!(MINUTE_SECOND), |_| TimeUnitType::TimeUnitMinuteSecond),
        map(rule!(HOUR_MICROSECOND), |_| {
            TimeUnitType::TimeUnitHourMicrosecond
        }),
        map(rule!(HOUR_SECOND), |_| TimeUnitType::TimeUnitHourSecond),
        map(rule!(HOUR_MINUTE), |_| TimeUnitType::TimeUnitHourMinute),
        map(rule!(DAY_MICROSECOND), |_| {
            TimeUnitType::TimeUnitDayMicrosecond
        }),
        map(rule!(DAY_SECOND), |_| TimeUnitType::TimeUnitDaySecond),
        map(rule!(DAY_MINUTE), |_| TimeUnitType::TimeUnitDayMinute),
        map(rule!(DAY_HOUR), |_| TimeUnitType::TimeUnitDayHour),
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
