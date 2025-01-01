use crate::ast::ci_str::CIStr;
use crate::ast::common::{FulltextSearchModifier, FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE};
use crate::ast::expr_node::{
    BinaryOperationExpr, ExprNode, FuncCallExpr, MatchAgainst, TimeUnitExpr, UnaryOperationExpr,
    ValueExpr, ValueExprKind, VariableExpr,
};
use crate::ast::functions;
use crate::ast::op_code::OpCode;
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::statements::common::{
    column_name_list, fulltext_search_modifier_opt, log_and, log_or, simple_ident, time_unit,
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

pub fn simple_expr(i: Input) -> IResult<ExprNode> {
    map(rule!(#simple_expr_sub_1 | #simple_expr_sub_2), |expr| expr)(i)
}

pub fn simple_expr_sub_1(i: Input) -> IResult<ExprNode> {
    alt((
        map(rule!(#simple_ident), |expr| ExprNode::ColumnNameExpr(expr)),
        map(rule!(#function_call_keyword), |expr| {
            ExprNode::FuncCallExpr(expr)
        }),
    ))(i)
}

pub fn simple_expr_sub_2(i: Input) -> IResult<ExprNode> {}

pub fn function_call_keyword(i: Input) -> IResult<FuncCallExpr> {
    alt((
        map(
            rule!(#function_name_conflict ~ "(" ~ #expression_list_opt ~ ")"),
            |(fn_name, _, exprs, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                fn_expr.args = exprs;

                fn_expr
            },
        ),
        map(
            rule!(USER ~ "(" ~ #expression_list_opt ~ ")"),
            |(t, _, exprs, _)| {
                let fn_name = t.text().to_string();

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                fn_expr.args = exprs;

                fn_expr
            },
        ),
        map(
            rule!(#function_name_optional_braces ~ #optional_braces?),
            |(fn_name, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                fn_expr
            },
        ),
        map(rule!(CURRENT_DATE ~ #optional_braces), |(t, _)| {
            let fn_name = t.text().to_string();

            let mut fn_expr = FuncCallExpr::default();
            fn_expr.fn_name = CIStr::new(&fn_name);
            fn_expr
        }),
        map(
            rule!(#function_name_datetime_precision ~ #func_datetime_prec),
            |(fn_name, value_expr)| {
                let mut args = Vec::<ExprNode>::with_capacity(1);
                if let Some(v) = value_expr {
                    args.push(ExprNode::ValueExpr(v));
                }

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                fn_expr.args = args;
                fn_expr
            },
        ),
        map(
            rule!(#function_name_datetime_precision ~ #func_datetime_prec),
            |(fn_name, value_expr)| {
                let mut args = Vec::with_capacity(1);
                if let Some(v) = value_expr {
                    args.push(ExprNode::ValueExpr(v));
                }

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                fn_expr.args = args;
                fn_expr
            },
        ),
        map(
            rule!(CHAR ~ "(" ~ #expression_list ~ ")"),
            |(_, _, mut exprs, _)| {
                let value_expr = ValueExpr::new("", ValueExprKind::None, i.charset, i.collation);
                exprs.push(ExprNode::ValueExpr(value_expr));

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(functions::CHAR_FUNC);
                fn_expr.args = exprs;
                fn_expr
            },
        ),
        map(
            rule!(CHAR ~ "(" ~ #expression_list ~ USING ~ ")"),
            |(_, _, mut exprs, _, _)| {
                let value_expr = ValueExpr::new("", ValueExprKind::None, i.charset, i.collation);
                exprs.push(ExprNode::ValueExpr(value_expr));

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(functions::CHAR_FUNC);
                fn_expr.args = exprs;
                fn_expr
            },
        ),
    ))(i)
}

pub fn function_name_conflict(i: Input) -> IResult<String> {
    alt((
        map(
            rule!(
                ASCII
                    | CHARSET
                    | COALESCE
                    | COLLATION
                    | DATE
                    | DATABASE
                    | DAY
                    | HOUR
                    | IF
                    | INTERVAL
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                LOG | FORMAT
                    | LEFT
                    | MICROSECOND
                    | MINUTE
                    | MONTH
                    | NOW
                    | POINT
                    | QUARTER
                    | REPEAT
                    | REPLACE
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                REVERSE
                    | RIGHT
                    | ROW_COUNT
                    | SECOND
                    | TIME
                    | TIMESTAMP
                    | TRUNCATE
                    | USER
                    | WEEK
                    | YEAR
            ),
            |t| t.text().to_string(),
        ),
    ))(i)
}

fn expression_list_opt(i: Input) -> IResult<Vec<ExprNode>> {
    separated_list0(map(rule!(","), |_| ()), expression)(i)
}

fn expression_list(i: Input) -> IResult<Vec<ExprNode>> {
    separated_list1(map(rule!(","), |_| ()), expression)(i)
}

fn function_name_optional_braces(i: Input) -> IResult<String> {
    map(
        rule!(CURRENT_USER | CURRENT_DATE | CURRENT_ROLE | UTC_DATE | TIDB_CURRENT_TSO),
        |t| t.text().to_string(),
    )(i)
}

fn function_name_datetime_precision(i: Input) -> IResult<String> {
    map(
        rule!(
            CURRENT_TIME
                | CURRENT_TIMESTAMP
                | LOCALTIME
                | LOCALTIMESTAMP
                | UTC_TIME
                | UTC_TIMESTAMP
        ),
        |t| t.text().to_string(),
    )(i)
}

fn optional_braces(i: Input) -> IResult<()> {
    map(rule!("(" ~ ")"), |_| ())(i)
}

fn func_datetime_prec(i: Input) -> IResult<Option<ValueExpr>> {
    alt((
        map(rule!(#optional_braces?), |_| None),
        map(rule!("(" ~ LiteralInteger ~ ")"), |(_, t, _)| {
            let value_expr = ValueExpr::new(t.text(), ValueExprKind::Isize, i.charset, i.collation);
            Some(value_expr)
        }),
    ))(i)
}
