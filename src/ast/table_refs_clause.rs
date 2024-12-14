use crate::ast::column_name::ColumnName;
use crate::ast::expr_node::ExprNode;
use crate::ast::result_set_node::ResultSetNode;
use derive_visitor::Drive;

#[derive(Debug)]
pub enum JoinType {
    // CrossJoin is cross join type.
    CrossJoin,
    // LeftJoin is left Join type.
    LeftJoin,
    // RightJoin is right Join type.
    RightJoin,
}

#[derive(Debug, Drive)]
pub struct TableRefsClause {
    // Left table can be TableSource or JoinNode.
    pub left: Box<ResultSetNode>,
    // Right table can be TableSource or JoinNode or nil.
    pub right: Option<Box<ResultSetNode>>,
    // Tp represents join type.
    #[drive(skip)]
    pub join_type: JoinType,
    // On represents join on condition.
    pub on: Box<ExprNode>,
    // Using represents join using clause.
    #[drive(skip)]
    pub using: Vec<ColumnName>,
    // NaturalJoin represents join is natural join.
    #[drive(skip)]
    pub natural_join: bool,
    // StraightJoin represents a straight join.
    #[drive(skip)]
    pub straight_join: bool,
    #[drive(skip)]
    pub explicit_parens: bool,
}
