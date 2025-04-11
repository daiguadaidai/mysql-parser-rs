use crate::ast::expr_node::{ExprNode, ValueExpr, VariableExpr};
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::token_kind::TokenKind::{DoubleAtIdent, SingleAtIdent};
use nom::combinator::map;
use nom_rule::rule;

pub fn vairable(i: Input) -> IResult<ExprNode> {
    map(rule!(#system_variable | #user_variable), |(e)| e)
}

pub fn system_variable(i: Input) -> IResult<ExprNode> {
    map(rule!(DoubleAtIdent), |(t)| {
        let v = t.text().to_lowercase();
        let (name, is_global, explicit_scope) = if v.starts_with("@@global.") {
            (v.trim_start_matches("@@global.").to_string(), true, true)
        } else if v.starts_with("@@session.") {
            (v.trim_start_matches("@@global.").to_string(), false, true)
        } else if v.starts_with("@@local.") {
            (v.trim_start_matches("@@local.").to_string(), false, true)
        } else if v.starts_with("@@") {
            (v.trim_start_matches("@@").to_string(), false, false)
        } else {
            (v, false, true)
        };

        ExprNode::VariableExpr(VariableExpr {
            name,
            is_global,
            is_system: true,
            explicit_scope,
            value: None,
        })
    })(i)
}

pub fn user_variable(i: Input) -> IResult<ExprNode> {
    map(rule!(SingleAtIdent), |(t)| {
        ExprNode::VariableExpr(VariableExpr {
            name: t.text().trim_start_matches("@").to_string(),
            is_global: false,
            is_system: false,
            explicit_scope: false,
            value: None,
        })
    })(i)
}
