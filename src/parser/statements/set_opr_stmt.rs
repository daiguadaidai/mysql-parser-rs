use crate::ast::limit::Limit;
use crate::ast::node::Node;
use crate::ast::order_by_clause::OrderByClause;
use crate::ast::result_set_node::ResultSetNode;
use crate::ast::set_opr_stmt::{SetOprSelectList, SetOprStmt, SetOprType};
use crate::ast::statement::Statement;
use crate::ast::with_clause::WithClause;
use crate::parser::common::*;
use crate::parser::input::Input;
use crate::parser::statements::common::default_true_distinct_opt;
use crate::parser::statements::select_statement::{select_stmt, sub_select};
use crate::parser::statements::with_clause::with_clause;
use crate::parser::token_kind::TokenKind::*;
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;
use std::rc::Rc;

pub fn set_opr_stmt(i: Input) -> IResult<SetOprStmt> {
    alt((
        map(rule!(#set_opr_stmt_wout_limit_order_by), |(stmt)| stmt),
        map(rule!(#set_opr_stmt_with_limit_order_by), |(stmt)| stmt),
        map(
            rule!(#with_clause ~ #set_opr_stmt_with_limit_order_by),
            |(with, mut set_opr_with)| {
                set_opr_with.with = Some(with);

                set_opr_with
            },
        ),
        map(
            rule!(#with_clause ~ #set_opr_stmt_wout_limit_order_by),
            |(with, mut set_opr_with)| {
                set_opr_with.with = Some(with);

                set_opr_with
            },
        ),
    ))(i)
}

// See https://dev.mysql.com/doc/refman/5.7/en/union.html
// See https://mariadb.com/kb/en/intersect/
// See https://mariadb.com/kb/en/except/
pub fn set_opr_stmt_wout_limit_order_by(i: Input) -> IResult<SetOprStmt> {
    alt((
        map(
            rule!(#set_opr_clause_list ~ #set_opr ~ #select_stmt),
            |(list, op, mut st)| {
                let mut ssl = SetOprSelectList::default();
                ssl.selects = list;

                let mut so = SetOprStmt::default();
                so.select_list = Some(ssl);
                so.limit = st.limit;
                so.order_by = st.order_by;

                st.limit = None;
                st.order_by = None;
                st.after_set_operator = Some(op);

                so.select_list
                    .as_mut()
                    .unwrap()
                    .selects
                    .push(Node::new_select_stmt(st));

                so
            },
        ),
        map(
            rule!(#set_opr_clause_list ~ #set_opr ~ #sub_select),
            |(mut list, op, mut st)| {
                let mut set_opr_list2 = Vec::<Node>::new();
                let mut with2: Option<Rc<WithClause>> = None;
                let mut limit2: Option<Rc<Limit>> = None;
                let mut order_by2: Option<Rc<OrderByClause>> = None;

                if let Some(q) = st.query {
                    match q {
                        ResultSetNode::SelectStmt(ss) => {
                            with2 = ss.with.clone();
                            set_opr_list2.push(Node::new_select_stmt_by_ref(ss));
                        }
                        ResultSetNode::SetOprStmt(ss) => {
                            // child setOprStmt's limit and order should also make sense
                            // we should separate it out from other normal SetOprSelectList.
                            with2 = ss.with.clone();
                            limit2 = ss.limit.clone();
                            order_by2 = ss.order_by.clone();
                            set_opr_list2 = match ss.select_list {
                                None => vec![],
                                Some(v) => v.selects,
                            }
                        }
                        _ => {}
                    }
                }

                let next_set_opr_list = SetOprSelectList {
                    with: with2,
                    after_set_operator: Some(op),
                    selects: set_opr_list2,
                    limit: limit2,
                    order_by: order_by2,
                };

                list.push(Node::new_set_opr_select_list(next_set_opr_list));
                let mut sosl = SetOprSelectList::default();
                sosl.selects = list;

                let mut so = SetOprStmt::default();
                so.select_list = Some(sosl);

                so
            },
        ),
    ))(i)
}

pub fn set_opr_stmt_with_limit_order_by(i: Input) -> IResult<SetOprStmt> {}

pub fn set_opr_clause(i: Input) -> IResult<Vec<Node>> {
    alt((
        map(rule!(#select_stmt), |(stmt)| {
            vec![Node::Statement(Statement::SelectStmt(Box::new(stmt)))]
        }),
        map(rule!(#sub_select), |(stmt)| match stmt.query {
            None => {
                vec![]
            }
            Some(v) => match v {
                ResultSetNode::SelectStmt(s_stmt) => {
                    let mut select_list = SetOprSelectList::default();
                    select_list.selects = vec![Node::new_select_stmt_by_ref(s_stmt)];

                    vec![Node::SetOprSelectList(Box::new(select_list))]
                }
                ResultSetNode::SetOprStmt(so_stmt) => {
                    let selects = match so_stmt.select_list {
                        None => vec![],
                        Some(ss) => ss.selects,
                    };

                    let select_list = SetOprSelectList {
                        with: so_stmt.with,
                        after_set_operator: None,
                        selects,
                        limit: so_stmt.limit,
                        order_by: so_stmt.order_by,
                    };

                    vec![Node::new_set_opr_select_list(select_list)]
                }
                _ => {
                    vec![]
                }
            },
        }),
    ))(i)
}

pub fn set_opr_opt(i: Input) -> IResult<bool> {
    map(rule!(#default_true_distinct_opt), |(b)| b)(i)
}

pub fn set_opr(i: Input) -> IResult<SetOprType> {
    alt((
        map(rule!(UNION ~ #set_opr_opt), |(_, b)| {
            if b {
                SetOprType::Union
            } else {
                SetOprType::UnionAll
            }
        }),
        map(rule!(EXCEPT ~ #set_opr_opt), |(_, b)| {
            if b {
                SetOprType::Except
            } else {
                SetOprType::ExceptAll
            }
        }),
        map(rule!(INTERSECT ~ #set_opr_opt), |(_, b)| {
            if b {
                SetOprType::Intersect
            } else {
                SetOprType::IntersectAll
            }
        }),
    ))(i)
}
pub fn set_opr_clause_list(i: Input) -> IResult<Vec<Node>> {
    // 检查是否为空
    if i.is_empty() {
        return Ok((i, vec![]));
    }

    let mut result = Vec::<Node>::new();

    // 解析第一个元素
    let (_, first_ele) = set_opr_clause(i)?;
    result.extend(first_ele);

    // 解析后续的元素
    let (_, rest) = map(rule!(#set_opr_clause_list_sub*), |(clauses)| clauses)(i)?;
    for clause in rest {
        result.extend(clause);
    }

    Ok((i, result))
}

pub fn set_opr_clause_list_sub(i: Input) -> IResult<Vec<Node>> {
    map(rule!(#set_opr ~ #set_opr_clause), |(opr, mut clauses)| {
        if clauses.is_empty() {
            return clauses;
        }

        let mut first_clause = clauses.get_mut(0).unwrap();
        match first_clause {
            Node::Statement(mut v) => match v {
                Statement::SelectStmt(mut stmt) => {
                    stmt.after_set_operator = Some(opr);
                }
                _ => {}
            },
            Node::SetOprSelectList(mut v) => {
                v.after_set_operator = Some(opr);
            }
            _ => {}
        }

        clauses
    })(i)
}
