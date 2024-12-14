use crate::ast::group_by_clause::ByItem;
use derive_visitor::Drive;

// OrderByClause represents order by clause.
#[derive(Debug, Drive)]
pub struct OrderByClause {
    pub items: Vec<ByItem>,
    #[drive(skip)]
    pub for_union: bool,
}