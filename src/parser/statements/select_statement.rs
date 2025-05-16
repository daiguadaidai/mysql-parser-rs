use crate::ast::ci_str::CIStr;
use crate::ast::select_field::{Field, SelectField, WildCardField};
use crate::ast::select_stmt::{SelectStmt, SelectStmtKind, SelectStmtOpts};
use crate::ast::statement::Statement;
use crate::ast::table_optimizer_hint::TableOptimizerHint;
use crate::mysql::consts::PriorityEnum;
use crate::parser::common::*;
use crate::parser::error::ErrorKind;
use crate::parser::input::Input;
use crate::parser::statements::common::priority;
use crate::parser::statements::expression::expression;
use crate::parser::statements::table_hints::table_optimizer_hints;
use crate::parser::token_kind::TokenKind::{
    Ident, LiteralString, ALL, DISTINCT, DISTINCTROW, SELECT,
};
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom_rule::rule;

pub fn select_statement(i: Input) -> IResult<Statement> {
    map(rule!(#select_stmt), |(sel_stmt)| {
        Statement::SelectStmt(Box::new(sel_stmt))
    })(i)
}

pub fn select_stmt(i: Input) -> IResult<SelectStmt> {
    let s1 = map(
        rule!(
            #select_stmt_basic
        ),
        |mut sel_stmt| sel_stmt,
    );

    let s2 = map(
        rule!(
            #select_stmt_basic
        ),
        |mut sel_stmt| sel_stmt,
    );

    alt((s1, s2))(i)
}

pub fn select_stmt_basic(i: Input) -> IResult<SelectStmt> {
    map(
        rule!(SELECT ~ #select_stmt_opts ~ #select_stmt_field_list),
        |(_, sso, fields)| {
            let mut st = SelectStmt::default();
            st.kind = SelectStmtKind::SelectStmtKindSelect;
            st.select_stmt_opts = sso;
            st.fields = fields;

            st
        },
    )(i)
}

pub fn select_stmt_basic2(i: Input) -> IResult<SelectStmt> {
    map(rule!(SELECT), |_| {
        let mut st = SelectStmt::default();
        st.kind = SelectStmtKind::SelectStmtKindSelect;

        st
    })(i)
}

pub fn select_stmt_opts(i: Input) -> IResult<SelectStmtOpts> {
    map_res(many0(select_stmt_opt), |opts| {
        let mut new_opt = SelectStmtOpts::default();
        new_opt.sql_cache = true;

        for opt in &opts {
            if opt.table_hints.len() > 0 {
                new_opt.table_hints = opt
                    .table_hints
                    .iter()
                    .map(|hint| hint.clone())
                    .collect::<Vec<TableOptimizerHint>>()
            }
            if opt.distinct {
                new_opt.distinct = true
            }
            match opt.priority {
                PriorityEnum::NoPriority => {}
                _ => new_opt.priority = opt.priority,
            }
            if opt.sql_small_result {
                new_opt.sql_small_result = true
            }
            if opt.sql_big_result {
                new_opt.sql_big_result = true
            }
            if opt.sql_buffer_result {
                new_opt.sql_buffer_result = opt.sql_buffer_result
            }
            if !opt.sql_cache {
                new_opt.sql_cache = false
            }
            if opt.calc_found_rows {
                new_opt.calc_found_rows = true
            }
            if opt.straight_join {
                new_opt.straight_join = true
            }
            if opt.explicit_all {
                new_opt.explicit_all = true
            }

            if opt.distinct && opt.explicit_all {
                return Err(nom::Err::Error(ErrorKind::ExpectText(
                    "ALL and DISTINCT error wrong usage",
                )));
            }
        }

        Ok(new_opt)
    })(i)
}

fn select_stmt_sql_cache(i: Input) -> IResult<bool> {
    alt((
        map(rule!("SQL_CACHE"), |_| true),
        map(rule!("SQL_NO_CACHE"), |_| false),
    ))(i)
}

pub fn select_stmt_opt(i: Input) -> IResult<SelectStmtOpts> {
    alt((
        map(table_optimizer_hints, |hints| {
            let mut opt = SelectStmtOpts::default();
            opt.table_hints = hints;
            opt
        }),
        map(rule!(ALL? ~ (DISTINCT | DISTINCTROW)), |(_, _)| {
            let mut opt = SelectStmtOpts::default();
            opt.distinct = true;
            opt
        }),
        map(priority, |p| {
            let mut opt = SelectStmtOpts::default();
            opt.priority = p;
            opt
        }),
        map(rule!("SQL_SMALL_RESULT"), |_| {
            let mut opt = SelectStmtOpts::default();
            opt.sql_small_result = true;
            opt
        }),
        map(rule!("SQL_BIG_RESULT"), |_| {
            let mut opt = SelectStmtOpts::default();
            opt.sql_big_result = true;
            opt
        }),
        map(rule!("SQL_BUFFER_RESULT"), |_| {
            let mut opt = SelectStmtOpts::default();
            opt.sql_buffer_result = true;
            opt
        }),
        map(select_stmt_sql_cache, |_| {
            let mut opt = SelectStmtOpts::default();
            opt.sql_cache = true;
            opt
        }),
        map(rule!("SQL_CALC_FOUND_ROWS"), |_| {
            let mut opt = SelectStmtOpts::default();
            opt.calc_found_rows = true;
            opt
        }),
        map(rule!("STRAIGHT_JOIN"), |_| {
            let mut opt = SelectStmtOpts::default();
            opt.straight_join = true;
            opt
        }),
    ))(i)
}

pub fn select_stmt_field_list(i: Input) -> IResult<Vec<SelectField>> {
    separated_list1(map(rule!(","), |_| ()), field)(i)
}

pub fn field(i: Input) -> IResult<SelectField> {
    alt((
        map(rule!("*"), |_| {
            let mut sf = SelectField::default();
            sf.field = Field::WildCardField(Default::default());
            sf
        }),
        map(rule!(Ident ~ "." ~ "*"), |(table_name, _, _)| {
            let tb_name = table_name.get_trim_start_end_text('`');

            let mut sf = SelectField::default();
            sf.field = Field::WildCardField(WildCardField {
                schema: Default::default(),
                table: CIStr::new(tb_name),
            });
            sf
        }),
        map(
            rule!(Ident ~ "." ~ Ident ~ "." ~ "*"),
            |(schema_name, _, table_name, _, _)| {
                let s_name = schema_name.get_trim_start_end_text('`');
                let tb_name = table_name.get_trim_start_end_text('`');

                let mut sf = SelectField::default();
                sf.field = Field::WildCardField(WildCardField {
                    schema: CIStr::new(s_name),
                    table: CIStr::new(tb_name),
                });
                sf
            },
        ),
        map(rule!(#expression ~ #field_as_name?), |(expr, as_name)| {
            let mut sf = SelectField::default();
            if let Some(v) = as_name {
                sf.as_name = CIStr::new(&v)
            }
            sf.field = Field::Expr(expr);
            sf
        }),
    ))(i)
}

pub fn field_as_name(i: Input) -> IResult<String> {
    map(rule!("AS"? ~ (Ident | LiteralString)), |(_, s)| {
        s.text().to_string()
    })(i)
}
