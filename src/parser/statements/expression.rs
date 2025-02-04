use crate::ast::ci_str::CIStr;
use crate::ast::common::{FulltextSearchModifier, FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE};
use crate::ast::expr_node::{
    BinaryOperationExpr, ExprNode, FuncCallExpr, FuncCallExprType, GetFormatSelectorExpr,
    MatchAgainst, TableNameExpr, TimeUnitExpr, TrimDirectionExpr, UnaryOperationExpr, ValueExpr,
    ValueExprKind, VariableExpr,
};
use crate::ast::functions;
use crate::ast::functions::TimeUnitType;
use crate::ast::op_code::OpCode;
use crate::common::misc::is_in_token_map;
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::statements::common::{
    column_name_list, field_len, fulltext_search_modifier_opt, func_datetime_prec,
    func_datetime_prec_list_opt, function_name_conflict, function_name_date_arith,
    function_name_date_arith_multi_forms, function_name_datetime_precision,
    function_name_optional_braces, get_format_selector, log_and, log_or, optional_braces,
    signed_num, simple_ident, string_lit, table_name, time_unit, timestamp_unit,
    timestamp_unit_sql_tsi, trim_direction,
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
        map(rule!(#function_call_keyword), |expr| expr),
        map(rule!(#function_call_non_keyword), |expr| expr),
        map(rule!(#function_call_generic), |expr| {
            ExprNode::FuncCallExpr(expr)
        }),
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
