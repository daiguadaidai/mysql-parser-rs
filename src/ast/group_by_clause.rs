use crate::ast::expr_node::ExprNode;
use derive_visitor::Drive;

// ByItem represents an item in order by or group by.
#[derive(Debug, Drive)]
pub struct ByItem {
    pub expr: Option<Box<ExprNode>>,
    #[drive(skip)]
    pub desc: bool,
    #[drive(skip)]
    pub null_order: bool,
}

// GroupByClause represents group by clause.
#[derive(Debug, Drive)]
pub struct GroupByClause {
    pub items: Vec<ByItem>,
    #[drive(skip)]
    pub rollup: bool,
}
