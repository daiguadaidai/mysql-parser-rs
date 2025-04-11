use crate::ast::ci_str::CIStr;
use crate::ast::common::{FulltextSearchModifier, FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE};
use crate::ast::expr_node::{
    BinaryOperationExpr, ExprNode, FuncCallExpr, FuncCallExprType, GetFormatSelectorExpr,
    MatchAgainst, ParamMarkerExpr, PositionExpr, SetCollationExpr, TableNameExpr, TimeUnitExpr,
    TrimDirectionExpr, UnaryOperationExpr, ValueExpr, ValueExprKind, VariableExpr, WindowFuncExpr,
};
use crate::ast::frame_clause::{BoundType, FrameBound, FrameClause, FrameExtent, FrameType};
use crate::ast::functions;
use crate::ast::functions::TimeUnitType;
use crate::ast::group_by_clause::ByItem;
use crate::ast::op_code::OpCode;
use crate::ast::order_by_clause::OrderByClause;
use crate::ast::partition_by_clause::PartitionByClause;
use crate::ast::window_spec::WindowSpec;
use crate::common::misc::is_in_token_map;
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::statements::column_name::{column_name_list, simple_ident};
use crate::parser::statements::common::{
    collation_name, field_len, fulltext_search_modifier_opt, func_datetime_prec,
    func_datetime_prec_list_opt, log_and, log_or, opt_from_first_last, opt_null_treatment,
    optional_braces, string_lit, table_name, time_unit,
};
use crate::parser::statements::keywords::{
    function_name_conflict, function_name_date_arith, function_name_date_arith_multi_forms,
    function_name_datetime_precision, function_name_optional_braces, get_format_selector,
    timestamp_unit, timestamp_unit_sql_tsi, trim_direction,
};
use crate::parser::statements::literal::literal;
use crate::parser::statements::num_literal::{num_literal, signed_num};
use crate::parser::statements::vairable::vairable;
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
        map(rule!(#function_call_keyword), |expr| expr),
        map(rule!(#function_call_non_keyword), |expr| expr),
        map(rule!(#function_call_generic), |expr| {
            ExprNode::FuncCallExpr(expr)
        }),
        map(
            rule!(#simple_expr ~ COLLATE ~ #collation_name),
            |(expr, _, collate)| {
                ExprNode::SetCollationExpr(SetCollationExpr {
                    expr: Some(Box::new(expr)),
                    collate,
                })
            },
        ),
        map(rule!(#window_func_call), |(expr)| {
            ExprNode::WindowFuncExpr(expr)
        }),
        map(rule!(#literal), |(expr)| expr),
        map(rule!("?"), |(t)| {
            ExprNode::ParamMarkerExpr(ParamMarkerExpr {
                offset: 0,
                order: 0,
                in_execute: false,
                value_expr: None,
                token_index: t.pos,
                start_pos: t.span.start as usize,
                end_pos: t.span.end as usize,
            })
        }),
        map(rule!(#vairable), |(expr)| expr),
    ))(i)
}

pub fn simple_expr_sub_2(i: Input) -> IResult<ExprNode> {}

pub fn function_call_keyword(i: Input) -> IResult<ExprNode> {
    alt((
        map(
            rule!(#function_name_conflict ~ "(" ~ #expression_list_opt ~ ")"),
            |(fn_name, _, exprs, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                fn_expr.args = exprs;

                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(USER ~ "(" ~ #expression_list_opt ~ ")"),
            |(t, _, exprs, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(t.text());
                fn_expr.args = exprs;

                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(#function_name_optional_braces ~ #optional_braces?),
            |(fn_name, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(rule!(CURDATE ~ #optional_braces), |(t, _)| {
            let mut fn_expr = FuncCallExpr::default();
            fn_expr.fn_name = CIStr::new(t.text());
            ExprNode::FuncCallExpr(fn_expr)
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
                ExprNode::FuncCallExpr(fn_expr)
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
                ExprNode::FuncCallExpr(fn_expr)
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
                ExprNode::FuncCallExpr(fn_expr)
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
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(rule!(DATE ~ #string_lit), |(_, s)| {
            let value_expr = ValueExpr::new(&s, ValueExprKind::String, "", "");
            let exprs = vec![ExprNode::ValueExpr(value_expr)];

            let mut fn_expr = FuncCallExpr::default();
            fn_expr.fn_name = CIStr::new(functions::DATE_LITERAL);
            fn_expr.args = exprs;
            ExprNode::FuncCallExpr(fn_expr)
        }),
        map(rule!(TIME ~ #string_lit), |(_, s)| {
            let value_expr = ValueExpr::new(&s, ValueExprKind::String, "", "");
            let exprs = vec![ExprNode::ValueExpr(value_expr)];

            let mut fn_expr = FuncCallExpr::default();
            fn_expr.fn_name = CIStr::new(functions::TIME_LITERAL);
            fn_expr.args = exprs;
            ExprNode::FuncCallExpr(fn_expr)
        }),
        map(rule!(TIMESTAMP ~ #string_lit), |(_, val)| {
            let value_expr = ValueExpr::new(&val, ValueExprKind::String, "", "");
            let exprs = vec![ExprNode::ValueExpr(value_expr)];

            let mut fn_expr = FuncCallExpr::default();
            fn_expr.fn_name = CIStr::new(functions::TIMESTAMP_LITERAL);
            fn_expr.args = exprs;
            ExprNode::FuncCallExpr(fn_expr)
        }),
        map(rule!(INSERT ~ #expression_list_opt), |(_, exprs)| {
            let mut fn_expr = FuncCallExpr::default();
            fn_expr.fn_name = CIStr::new(functions::INSERT_FUNC);
            fn_expr.args = exprs;
            ExprNode::FuncCallExpr(fn_expr)
        }),
        map(
            rule!(MOD ~ "(" ~ #bit_expr ~ "," ~ #bit_expr ~ ")"),
            |(_, _, l, _, r, _)| {
                ExprNode::BinaryOperationExpr(BinaryOperationExpr {
                    op: OpCode::Mod,
                    l: Some(Box::new(l)),
                    r: Some(Box::new(r)),
                })
            },
        ),
        map(
            rule!(PASSWORD ~ "(" ~ #expression_list_opt ~ ")"),
            |(_, _, exprs, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(functions::PASSWORD_FUNC);
                fn_expr.args = exprs;
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
    ))(i)
}

pub fn function_call_non_keyword(i: Input) -> IResult<ExprNode> {
    map(
        rule!(#function_call_non_keyword_1 | #function_call_non_keyword_2),
        |(e)| e,
    )(i)
}

pub fn function_call_non_keyword_1(i: Input) -> IResult<ExprNode> {
    alt((
        map(
            rule!(CURTIME ~ "(" ~ #func_datetime_prec_list_opt ~ ")"),
            |(fn_name, _, exprs, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = exprs;
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(SYSDATE ~ "(" ~ #func_datetime_prec_list_opt ~ ")"),
            |(fn_name, _, exprs, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = exprs;
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(#function_name_date_arith_multi_forms ~ "(" ~ #expression ~ "," ~ #expression ~ ")"),
            |(fn_name, _, expr1, _, expr2, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                fn_expr.args = vec![
                    expr1,
                    expr2,
                    ExprNode::TimeUnitExpr(TimeUnitExpr {
                        unit: TimeUnitType::TimeUnitDay,
                    }),
                ];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(#function_name_date_arith_multi_forms ~ "(" ~ #expression ~ "," ~ INTERVAL ~ #expression ~ #time_unit ~ ")"),
            |(fn_name, _, expr1, _, _, expr2, unit, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                fn_expr.args = vec![expr1, expr2, ExprNode::TimeUnitExpr(TimeUnitExpr { unit })];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(#function_name_date_arith ~ "(" ~ #expression ~ "," ~ INTERVAL ~ #expression ~ #time_unit ~ ")"),
            |(fn_name, _, expr1, _, _, expr2, unit, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(&fn_name);
                fn_expr.args = vec![expr1, expr2, ExprNode::TimeUnitExpr(TimeUnitExpr { unit })];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(EXTRACT ~ "(" ~ #time_unit ~ FROM ~ #expression ~ ")"),
            |(fn_name, _, unit, _, expr, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![ExprNode::TimeUnitExpr(TimeUnitExpr { unit })];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(GET_FORMAT ~ "(" ~ #get_format_selector ~ "," ~ #expression ~ ")"),
            |(fn_name, _, typ, _, expr, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![
                    ExprNode::GetFormatSelectorExpr(GetFormatSelectorExpr { selector: typ }),
                    expr,
                ];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(POSITION ~ "(" ~ #bit_expr ~ IN ~ #expression ~ ")"),
            |(fn_name, _, bit_expr, _, expr, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![bit_expr, expr];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(SUBSTRING ~ "(" ~ #expression ~ "," ~ #expression ~ ")"),
            |(fn_name, _, expr1, _, expr2, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr1, expr2];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(SUBSTRING ~ "(" ~ #expression ~ FROM ~ #expression ~ ")"),
            |(fn_name, _, expr1, _, expr2, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr1, expr2];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(SUBSTRING ~ "(" ~ #expression ~ "," ~ #expression ~ "," ~ #expression ~ ")"),
            |(fn_name, _, expr1, _, expr2, _, expr3, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr1, expr2, expr3];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(SUBSTRING ~ "(" ~ #expression ~ FROM ~ #expression ~ FOR ~ #expression ~ ")"),
            |(fn_name, _, expr1, _, expr2, _, expr3, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr1, expr2, expr3];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(TIMESTAMPADD ~ "(" ~ (#timestamp_unit|#timestamp_unit_sql_tsi) ~ "," ~ #expression ~ "," ~ #expression ~ ")"),
            |(fn_name, _, unit, _, expr1, _, expr2, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![ExprNode::TimeUnitExpr(TimeUnitExpr { unit }), expr1, expr2];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(TIMESTAMPDIFF ~ "(" ~ (#timestamp_unit|#timestamp_unit_sql_tsi) ~ "," ~ #expression ~ "," ~ #expression ~ ")"),
            |(fn_name, _, unit, _, expr1, _, expr2, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![ExprNode::TimeUnitExpr(TimeUnitExpr { unit }), expr1, expr2];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(TRIM ~ "(" ~ #expression ~ ")"),
            |(fn_name, _, expr, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(TRIM ~ "(" ~ #expression ~ FROM ~ #expression ~ ")"),
            |(fn_name, _, expr1, _, expr2, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr2, expr1];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
    ))(i)
}

pub fn function_call_non_keyword_2(i: Input) -> IResult<ExprNode> {
    alt((
        map(
            rule!(TRIM ~ "(" ~ #expression ~ ")"),
            |(fn_name, _, expr, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(TRIM ~ "(" ~ #expression ~ FROM ~ #expression ~ ")"),
            |(fn_name, _, expr1, _, expr2, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr2, expr1];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(TRIM ~ "(" ~ #trim_direction ~ FROM ~ #expression ~ ")"),
            |(fn_name, _, trim_type, _, expr, _)| {
                let space_val = ExprNode::ValueExpr(ValueExpr::new(
                    " ",
                    ValueExprKind::String,
                    i.charset,
                    i.collation,
                ));

                let dirction = ExprNode::TrimDirectionExpr(TrimDirectionExpr {
                    direction: trim_type,
                });
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr, space_val, dirction];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(TRIM ~ "(" ~ #trim_direction ~ #expression ~ FROM ~ #expression ~ ")"),
            |(fn_name, _, trim_type, expr1, _, expr2, _)| {
                let dirction = ExprNode::TrimDirectionExpr(TrimDirectionExpr {
                    direction: trim_type,
                });
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr2, expr1, dirction];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(WEIGHT_STRING ~ "(" ~ #expression ~ ")"),
            |(fn_name, _, expr, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![expr];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(
            rule!(WEIGHT_STRING ~ "(" ~ #expression ~ AS ~ BINARY ~ #field_len ~ ")"),
            |(fn_name, _, expr, _, _, len, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(fn_name.text());
                fn_expr.args = vec![
                    expr,
                    ExprNode::ValueExpr(ValueExpr::new(
                        "BINARY",
                        ValueExprKind::BinaryLiteral,
                        i.charset,
                        i.collation,
                    )),
                    ExprNode::ValueExpr(ValueExpr::new(
                        &len.to_string(),
                        ValueExprKind::Isize(len),
                        i.charset,
                        i.collation,
                    )),
                ];
                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
        map(rule!(#function_name_sequence), |expr| {
            ExprNode::FuncCallExpr(expr)
        }),
        map(
            rule!(TRANSLATE ~ "(" ~ #expression ~ "," ~ #expression ~ "," ~ #expression ~ ")"),
            |(_, _, expr1, _, expr2, _, expr3, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(functions::TRANSLATE);
                fn_expr.args = vec![expr1, expr2, expr3];

                ExprNode::FuncCallExpr(fn_expr)
            },
        ),
    ))(i)
}

pub fn expression_list_opt(i: Input) -> IResult<Vec<ExprNode>> {
    separated_list0(map(rule!(","), |_| ()), expression)(i)
}

pub fn expression_list(i: Input) -> IResult<Vec<ExprNode>> {
    separated_list1(map(rule!(","), |_| ()), expression)(i)
}

pub fn function_name_sequence(i: Input) -> IResult<FuncCallExpr> {
    alt((
        map(
            rule!(LASTVAL ~ "(" ~ #table_name ~ ")"),
            |(_, _, table_name, _)| {
                let obj_name_expr = ExprNode::TableNameExpr(TableNameExpr { name: table_name });

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(functions::LAST_VAL);
                fn_expr.args = vec![obj_name_expr];

                fn_expr
            },
        ),
        map(
            rule!(SETVAL ~ "(" ~ #table_name ~ "," ~ #signed_num ~ ")"),
            |(_, _, table_name, _, num, _)| {
                let obj_name_expr = ExprNode::TableNameExpr(TableNameExpr { name: table_name });
                let value_expr = ExprNode::ValueExpr(ValueExpr::new(
                    &num.to_string(),
                    ValueExprKind::I64(num),
                    i.charset,
                    i.collation,
                ));

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(functions::SET_VAL);
                fn_expr.args = vec![obj_name_expr, value_expr];

                fn_expr
            },
        ),
        map(rule!(#next_value_for_sequence), |(expr)| expr),
    ))(i)
}

pub fn next_value_for_sequence(i: Input) -> IResult<FuncCallExpr> {
    alt((
        map(
            rule!(NEXT ~ VALUE ~ FOR ~ #table_name),
            |(_, _, _, table_name)| {
                let obj_name_expr = ExprNode::TableNameExpr(TableNameExpr { name: table_name });

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(functions::NEXT_VAL);
                fn_expr.args = vec![obj_name_expr];

                fn_expr
            },
        ),
        map(
            rule!(NEXTVAL ~ "(" ~ #table_name ~ ")"),
            |(_, _, table_name, _)| {
                let obj_name_expr = ExprNode::TableNameExpr(TableNameExpr { name: table_name });

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(functions::NEXT_VAL);
                fn_expr.args = vec![obj_name_expr];

                fn_expr
            },
        ),
    ))(i)
}

pub fn function_call_generic(i: Input) -> IResult<FuncCallExpr> {
    alt((
        map(
            rule!(Ident ~ "(" ~ #expression_list_opt ~ ")"),
            |(t, _, exprs, _)| {
                let mut fn_expr = FuncCallExpr::default();
                fn_expr.fn_name = CIStr::new(t.get_trim_start_end_text('`'));
                fn_expr.args = exprs;

                fn_expr
            },
        ),
        map(
            rule!(Ident ~ "." ~ Ident ~ "(" ~ #expression_list_opt ~ ")"),
            |(t1, _, t2, _, exprs, _)| {
                let tp = if is_in_token_map(&t2.text().to_uppercase()) {
                    FuncCallExprType::Keyword
                } else {
                    FuncCallExprType::Generic
                };

                let mut fn_expr = FuncCallExpr::default();
                fn_expr.schema = CIStr::new(t1.get_trim_start_end_text('`'));
                fn_expr.fn_name = CIStr::new(t2.text());
                fn_expr.args = exprs;

                fn_expr
            },
        ),
    ))(i)
}

pub fn window_func_call(i: Input) -> IResult<WindowFuncExpr> {
    alt((
        map(
            rule!(ROW_NUMBER ~ "(" ~ ")" ~ #windowing_clause),
            |(t, _, _, spec)| {
                let name = t.text();
                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(RANK ~ "(" ~ ")" ~ #windowing_clause),
            |(t, _, _, spec)| {
                let name = t.text();
                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(DENSE_RANK ~ "(" ~ ")" ~ #windowing_clause),
            |(t, _, _, spec)| {
                let name = t.text();
                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(CUME_DIST ~ "(" ~ ")" ~ #windowing_clause),
            |(t, _, _, spec)| {
                let name = t.text();
                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(PERCENT_RANK ~ "(" ~ ")" ~ #windowing_clause),
            |(t, _, _, spec)| {
                let name = t.text();
                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(NTILE ~ "(" ~ #simple_expr ~ ")" ~ #windowing_clause),
            |(t, _, e, _, spec)| {
                let name = t.text();
                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.args = vec![e];
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(LEAD ~ "(" ~ #expression ~ #opt_lead_lag_info ~ ")" ~ #opt_null_treatment ~ #windowing_clause),
            |(t, _, e, infos, _, treatment, spec)| {
                let name = t.text();
                let mut args = vec![e];
                if infos.len() > 0 {
                    args.extend(infos);
                }

                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.args = args;
                expr.ignore_null = treatment;
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(LAG ~ "(" ~ #expression ~ #opt_lead_lag_info ~ ")" ~ #opt_null_treatment ~ #windowing_clause),
            |(t, _, e, infos, _, treatment, spec)| {
                let name = t.text();
                let mut args = vec![e];
                if infos.len() > 0 {
                    args.extend(infos);
                }

                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.args = args;
                expr.ignore_null = treatment;
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(FIRST_VALUE ~ "(" ~ #expression ~ ")" ~ #opt_null_treatment ~ #windowing_clause),
            |(t, _, e, _, treatment, spec)| {
                let name = t.text();
                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.args = vec![e];
                expr.ignore_null = treatment;
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(LAST_VALUE ~ "(" ~ #expression ~ ")" ~ #opt_null_treatment ~ #windowing_clause),
            |(t, _, e, _, treatment, spec)| {
                let name = t.text();
                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.args = vec![e];
                expr.ignore_null = treatment;
                expr.spec = Some(spec);

                expr
            },
        ),
        map(
            rule!(LAG ~ "(" ~ #expression ~ "," ~ #simple_expr ~ ")" ~ #opt_from_first_last ~ #opt_null_treatment ~ #windowing_clause),
            |(t, _, e, _, s_expr, _, from_last, treatment, spec)| {
                let name = t.text();
                let mut expr = WindowFuncExpr::default();
                expr.name = name.to_string();
                expr.args = vec![e, s_expr];
                expr.from_last = from_last;
                expr.ignore_null = treatment;
                expr.spec = Some(spec);

                expr
            },
        ),
    ))(i)
}

pub fn windowing_clause(i: Input) -> IResult<WindowSpec> {
    map(rule!(OVER ~ #window_name_or_spec), |(_, spec)| spec)(i)
}

pub fn window_name_or_spec(i: Input) -> IResult<WindowSpec> {
    alt((
        map(rule!(#window_name), |(name)| {
            let mut spec = WindowSpec::default();
            spec.name = name;
            spec
        }),
        map(rule!(#window_spec), |spec| spec),
    ))(i)
}
pub fn window_name(i: Input) -> IResult<CIStr> {
    map(rule!(Ident), |(t)| {
        let s = t.get_trim_start_end_text('`');
        CIStr::new(s)
    })
}

pub fn window_spec(i: Input) -> IResult<WindowSpec> {
    map(rule!("(" ~ #window_spec_details ~ ")"), |(_, spec, _)| spec)(i)
}

pub fn window_spec_details(i: Input) -> IResult<WindowSpec> {
    map(
        rule!(#opt_existing_window_name ~ #opt_partition_clause? ~ #opt_window_order_by_clause? ~ #opt_window_frame_clause?),
        |(name, partition_clause, order_by_clause, frame_clause)| {
            let mut spec = WindowSpec {
                name: CIStr::default(),
                references: name,
                partition_by: partition_clause,
                order_by: order_by_clause,
                frame: frame_clause,
                only_alias: false,
            };

            spec
        },
    )(i)
}

pub fn opt_existing_window_name(i: Input) -> IResult<CIStr> {
    map(rule!(#window_name?), |(name)| {
        name.unwrap_or_else(|| CIStr::new(""))
    })
}

pub fn opt_partition_clause(i: Input) -> IResult<PartitionByClause> {
    map(rule!(PARTITION ~ BY ~ #by_list), |(_, _, items)| {
        PartitionByClause { items }
    })
}

pub fn by_list(i: Input) -> IResult<Vec<ByItem>> {
    separated_list1(map(rule!(","), |_| ()), by_item)(i)
}

pub fn by_item(i: Input) -> IResult<ByItem> {
    alt((
        map(rule!(#expression), |(expr)| {
            let new_expr = if let ExprNode::ValueExpr(value_expr) = expr {
                if let Some(position) = value_expr.get_value_i64() {
                    ExprNode::PositionExpr(PositionExpr {
                        n: position as isize,
                        p: None,
                    })
                } else {
                    expr
                }
            } else {
                expr
            };

            ByItem {
                expr: Some(Box::new(new_expr)),
                desc: false,
                null_order: true,
            }
        }),
        map(rule!(#expression ~ #order), |(expr, desc)| {
            let new_expr = if let ExprNode::ValueExpr(value_expr) = expr {
                if let Some(position) = value_expr.get_value_i64() {
                    ExprNode::PositionExpr(PositionExpr {
                        n: position as isize,
                        p: None,
                    })
                } else {
                    expr
                }
            } else {
                expr
            };

            ByItem {
                expr: Some(Box::new(new_expr)),
                desc,
                null_order: false,
            }
        }),
    ))(i)
}

pub fn order(i: Input) -> IResult<bool> {
    alt((map(rule!(ASC), |_| false), map(rule!(DESC), |_| true)))(i)
}

pub fn opt_order(i: Input) -> IResult<bool> {
    map(rule!(#order?), |(b)| b.unwrap_or_else(|| false))(i)
}

pub fn opt_window_order_by_clause(i: Input) -> IResult<OrderByClause> {
    map(rule!(ORDER ~ BY ~ #by_list), |(_, _, items)| {
        OrderByClause {
            items,
            for_union: false,
        }
    })
}

pub fn opt_window_frame_clause(i: Input) -> IResult<FrameClause> {
    map(
        rule!(#window_frame_units ~ #window_frame_extent),
        |(tp, items)| FrameClause {
            tp,
            extent: Some(items),
        },
    )
}

pub fn window_frame_units(i: Input) -> IResult<FrameType> {
    alt((
        map(rule!(ROWS), |_| FrameType::Rows),
        map(rule!(RANGE), |_| FrameType::Ranges),
        map(rule!(GROUPS), |_| FrameType::Groups),
    ))(i)
}

pub fn window_frame_extent(i: Input) -> IResult<FrameExtent> {
    alt((
        map(rule!(#window_frame_start), |(frame)| FrameExtent {
            start: Some(frame),
            end: Some(FrameBound {
                tp: BoundType::CurrentRow,
                un_bounded: false,
                expr: None,
                unit: TimeUnitType::TimeUnitInvalid,
            }),
        }),
        map(rule!(#window_frame_between), |(extent)| extent),
    ))(i)
}

pub fn window_frame_start(i: Input) -> IResult<FrameBound> {
    alt((
        map(rule!(UNBOUNDED ~ PRECEDING), |_| FrameBound {
            tp: BoundType::Preceding,
            un_bounded: true,
            expr: None,
            unit: TimeUnitType::TimeUnitInvalid,
        }),
        map(rule!(#num_literal ~ PRECEDING), |(value_expr, _)| {
            FrameBound {
                tp: BoundType::Preceding,
                un_bounded: false,
                expr: Some(Box::new(ExprNode::ValueExpr(value_expr))),
                unit: TimeUnitType::TimeUnitInvalid,
            }
        }),
        map(rule!("?" ~ PRECEDING), |(t, _)| FrameBound {
            tp: BoundType::Preceding,
            un_bounded: false,
            expr: Some(Box::new(ExprNode::ParamMarkerExpr(ParamMarkerExpr {
                offset: 0,
                order: 0,
                in_execute: false,
                value_expr: None,
                token_index: t.pos,
                start_pos: t.span.start as usize,
                end_pos: t.span.end as usize,
            }))),
            unit: TimeUnitType::TimeUnitInvalid,
        }),
        map(
            rule!(INTERVAL ~ #expression ~ #time_unit ~ PRECEDING),
            |(_, expr, tu, _)| FrameBound {
                tp: BoundType::Preceding,
                un_bounded: false,
                expr: Some(Box::new(expr)),
                unit: tu,
            },
        ),
        map(rule!(CURRENT ~ ROW), |(_, _)| FrameBound {
            tp: BoundType::CurrentRow,
            un_bounded: false,
            expr: None,
            unit: TimeUnitType::TimeUnitInvalid,
        }),
    ))(i)
}

pub fn window_frame_between(i: Input) -> IResult<FrameExtent> {
    map(
        rule!(BETWEEN ~ #window_frame_bound ~ AND ~ #window_frame_bound),
        |(_, start, _, end)| FrameExtent {
            start: Some(start),
            end: Some(end),
        },
    )(i)
}

pub fn window_frame_bound(i: Input) -> IResult<FrameBound> {
    alt((
        map(rule!(#window_frame_start), |(bound)| bound),
        map(rule!(UNBOUNDED ~ FOLLOWING), |(_, _)| FrameBound {
            tp: BoundType::Following,
            un_bounded: true,
            expr: None,
            unit: TimeUnitType::TimeUnitInvalid,
        }),
        map(rule!(#num_literal ~ FOLLOWING), |(num, _)| FrameBound {
            tp: BoundType::Following,
            un_bounded: false,
            expr: Some(Box::new(ExprNode::ValueExpr(num))),
            unit: TimeUnitType::TimeUnitInvalid,
        }),
        map(rule!("?" ~ FOLLOWING), |(t, _)| FrameBound {
            tp: BoundType::Following,
            un_bounded: false,
            expr: Some(Box::new(ExprNode::ParamMarkerExpr(ParamMarkerExpr {
                offset: 0,
                order: 0,
                in_execute: false,
                value_expr: None,
                token_index: t.pos,
                start_pos: t.span.start as usize,
                end_pos: t.span.end as usize,
            }))),
            unit: TimeUnitType::TimeUnitInvalid,
        }),
        map(
            rule!(INTERVAL ~ #expression ~ #time_unit ~ FOLLOWING),
            |(_, expr, tu, _)| FrameBound {
                tp: BoundType::Following,
                un_bounded: false,
                expr: Some(Box::new(expr)),
                unit: tu,
            },
        ),
    ))(i)
}
pub fn opt_lead_lag_info(i: Input) -> IResult<Vec<ExprNode>> {
    alt((
        map(
            rule!("," ~ #num_literal ~ #opt_ll_default?),
            |(_, value_expr, expr)| {
                let mut args = vec![ExprNode::ValueExpr(value_expr)];
                if let Some(expr) = expr {
                    args.push(expr);
                }

                args
            },
        ),
        map(rule!("," ~ "?" ~ #opt_ll_default), |(_, t, expr)| {
            let mut mark_expr = ParamMarkerExpr::default();
            mark_expr.token_index = t.pos;
            mark_expr.start_pos = t.span.start as usize;
            mark_expr.end_pos = t.span.end as usize;
            let marker_expr = ExprNode::ParamMarkerExpr(mark_expr);

            let mut args = vec![marker_expr];
            if let Some(expr) = expr {
                args.push(expr);
            }

            args
        }),
    ))(i)
}

pub fn opt_ll_default(i: Input) -> IResult<ExprNode> {
    map(rule!("," ~ #expression), |(_, expr)| expr)
}
