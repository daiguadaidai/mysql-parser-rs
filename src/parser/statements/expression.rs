use crate::ast::ci_str::CIStr;
use crate::ast::common::{FulltextSearchModifier, FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE};
use crate::ast::expr_node::{
    BinaryOperationExpr, ExprNode, FuncCallExpr, MatchAgainst, TimeUnitExpr, UnaryOperationExpr,
    VariableExpr,
};
use crate::ast::op_code::OpCode;
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::statements::common::{
    column_name_list, fulltext_search_modifier_opt, log_and, log_or, time_unit,
};
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn expression(i: Input) -> IResult<ExprNode> {
    alt((
        map(
            rule!(SingleAtIdent ~ AssignmentEq ~ #expression),
            |(s_ident, _, expr_node)| {
                let s_ident = s_ident.text().trim_start_matches(|c| c == '@');
                let mut expr = VariableExpr::default();
                expr.name = s_ident.to_string();
                expr.value = Some(Box::new(expr_node));
                ExprNode::VariableExpr(expr)
            },
        ),
        map(
            rule!(#expression ~ #log_or ~ #expression),
            |(expr1, _, expr2)| {
                ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                    op: OpCode::LogicOr,
                    l: Some(Box::new(expr1)),
                    r: Some(Box::new(expr2)),
                })
            },
        ),
        map(
            rule!(#expression ~ "XOR" ~ #expression),
            |(expr1, _, expr2)| {
                ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                    op: OpCode::LogicXor,
                    l: Some(Box::new(expr1)),
                    r: Some(Box::new(expr2)),
                })
            },
        ),
        map(
            rule!(#expression ~ #log_and ~ #expression),
            |(expr1, _, expr2)| {
                ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                    op: OpCode::LogicAnd,
                    l: Some(Box::new(expr1)),
                    r: Some(Box::new(expr2)),
                })
            },
        ),
        map(rule!(NOT ~ #expression), |(_, mut expr)| match expr {
            ExprNode::ExistsSubqueryExpr(mut v) => {
                v.not = !v.not;
                ExprNode::ExistsSubqueryExpr(v)
            }
            _ => {
                let e = UnaryOperationExpr {
                    op: OpCode::Not,
                    v: Some(Box::new(expr)),
                };
                ExprNode::UnaryOperationExpr(e)
            }
        }),
        map(
            rule!(MATCH ~ "(" ~ #column_name_list ~ ")" ~ AGAINST ~ "(" ~ #bit_expr ~ #fulltext_search_modifier_opt? ~ ")"),
            |(_, _, columns, _, _, _, expr, opt_value, _)| {
                let modifier =
                    opt_value.unwrap_or_else(|| FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE);

                ExprNode::MatchAgainst(MatchAgainst {
                    column_names: columns,
                    against: Some(Box::new(expr)),
                    modifier: FulltextSearchModifier { v: modifier },
                })
            },
        ),
    ))(i)
}

pub fn bit_expr(i: Input) -> IResult<ExprNode> {
    alt((
        map(rule!(#bit_expr ~ "|" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::Or,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ "&" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::And,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ "<<" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::LeftShift,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ ">>" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::RightShift,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ "+" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::Plus,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ "-" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::Minus,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(
            rule!(#bit_expr ~ "+" ~ INTERVAL ~ #expression ~ #time_unit),
            |(be, _, _, expr, tu)| {
                let time_expr = ExprNode::TimeUnitExpr(TimeUnitExpr { unit: tu });

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new("DATE_ADD");
                fn_expr.args = vec![be, expr, time_expr];

                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(#bit_expr ~ "-" ~ INTERVAL ~ #expression ~ #time_unit),
            |(be, _, _, expr, tu)| {
                let time_expr = ExprNode::TimeUnitExpr(TimeUnitExpr { unit: tu });

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new("DATE_SUB");
                fn_expr.args = vec![be, expr, time_expr];

                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(INTERVAL ~ #expression ~ #time_unit ~ "+" ~ #bit_expr),
            |(_, expr, tu, _, be)| {
                let time_expr = ExprNode::TimeUnitExpr(TimeUnitExpr { unit: tu });

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new("DATE_ADD");
                fn_expr.args = vec![be, expr, time_expr];

                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(rule!(#bit_expr ~ "*" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::Mul,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ "/" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::Div,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ "%" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::Mod,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ DIV ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::IntDiv,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ MOD ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::Mod,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#bit_expr ~ "^" ~ #bit_expr), |(l, _, r)| {
            ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                op: OpCode::Xor,
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            })
        }),
        map(rule!(#simple_expr), |_| {}),
    ))(i)
}

pub fn simple_expr(i: Input) -> IResult<ExprNode> {}
