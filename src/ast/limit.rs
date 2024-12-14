use crate::ast::expr_node::ExprNode;
use derive_visitor::Drive;

// Limit is the limit clause.
#[derive(Debug, Drive)]
pub struct Limit {
    pub count: Box<ExprNode>,
    pub offset: Box<ExprNode>,
}