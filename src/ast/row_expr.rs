use crate::ast::expr_node::ExprNode;
use derive_visitor::Drive;

// RowExpr is the expression for row constructor.
// See https://dev.mysql.com/doc/refman/5.7/en/row-subqueries.html
#[derive(Debug, Drive)]
pub struct RowExpr {
    pub values: Vec<ExprNode>,
}