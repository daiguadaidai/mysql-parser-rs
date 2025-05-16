use crate::ast::ci_str::CIStr;
use crate::ast::subquery_expr::SubQueryExpr;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub struct CommonTableExpression {
    #[drive(skip)]
    pub name: CIStr,
    pub query: Option<SubQueryExpr>,
    #[drive(skip)]
    pub col_name_list: Vec<CIStr>,
    #[drive(skip)]
    pub is_recursive: bool,

    // Record how many consumers the current cte has
    #[drive(skip)]
    pub consumer_count: isize,
}
