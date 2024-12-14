#[cfg(test)]
mod tests {
    use crate::parser::input::Dialect;
    use crate::parser::parser::{parse_sql, tokenize_sql};

    #[test]
    fn select_basic_01_test() {
        let querys = vec![
            r#"
        SELECT
            "#,
            r#"
        SELECT ALL DISTINCT
            "#,
            r#"
        SELECT /*+*/ ALL DISTINCT
            "#,
            r#"
        SELECT /*+*/ ALL DISTINCT HIGH_PRIORITY SQL_SMALL_RESULT SQL_BIG_RESULT SQL_BUFFER_RESULT
        SQL_NO_CACHE SQL_CALC_FOUND_ROWS STRAIGHT_JOIN
            "#,
            r#"
        SELECT /*+*/ ALL DISTINCT HIGH_PRIORITY SQL_SMALL_RESULT SQL_BIG_RESULT SQL_BUFFER_RESULT
        SQL_NO_CACHE SQL_CALC_FOUND_ROWS STRAIGHT_JOIN
        `db1`.tb1.*
            "#,
            r#"
        SELECT /*+*/ ALL DISTINCT HIGH_PRIORITY SQL_SMALL_RESULT SQL_BIG_RESULT SQL_BUFFER_RESULT
        SQL_NO_CACHE SQL_CALC_FOUND_ROWS STRAIGHT_JOIN
        `db1`.tb1.*
        , tb1.*
        , *
            "#,
        ];

        let query = querys[querys.len() - 1];
        let tokens = tokenize_sql(query).unwrap();
        let stmt = parse_sql(&tokens, Dialect::MySQL);

        println!("{:#?}", stmt)
    }
}
