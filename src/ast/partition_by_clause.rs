use crate::ast::group_by_clause::ByItem;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub struct PartitionByClause {
    pub items: Vec<ByItem>,
}
