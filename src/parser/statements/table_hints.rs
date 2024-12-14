use crate::ast::ci_str::CIStr;
use crate::ast::table_optimizer_hint::{HintData, TableOptimizerHint};
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::token_kind::TokenKind::{HintPrefix, HintSuffix};
use nom::combinator::map;
use nom_rule::rule;

pub fn table_optimizer_hints(i: Input) -> IResult<Vec<TableOptimizerHint>> {
    map(rule!(HintPrefix ~ HintSuffix), |(_, _)| {
        let mut hints = vec![];
        let hint = TableOptimizerHint {
            hint_name: CIStr::new(""),
            hint_data: HintData::Bool(true),
            qb_name: CIStr::new(""),
            tables: vec![],
            indexes: vec![],
        };
        hints.push(hint);
        hints
    })(i)
}
