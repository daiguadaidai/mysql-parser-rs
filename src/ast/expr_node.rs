use crate::ast::column_name::ColumnName;
use crate::ast::row_expr::RowExpr;
use crate::ast::subquery_expr::SubqueryExpr;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub enum ExprNode {
    #[drive(skip)]
    ColumnNameExpr(ColumnName),
    SubqueryExpr(SubqueryExpr),
    RowExpr(RowExpr),
}
