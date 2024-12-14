use crate::ast::ci_str::CIStr;
use crate::ast::frame_clause::FrameClause;
use crate::ast::group_by_clause::ByItem;
use crate::ast::order_by_clause::OrderByClause;
use derive_visitor::Drive;

// WindowSpec is the specification of a window.
#[derive(Debug, Drive)]
pub struct WindowSpec {
    #[drive(skip)]
    pub name: CIStr,
    // Ref is the reference window of this specification. For example, in `w2 as (w1 order by a)`,
    // the definition of `w2` references `w1`.
    #[drive(skip)]
    pub references: CIStr,

    pub partition_by: Vec<ByItem>,
    pub order_by: Box<OrderByClause>,
    pub frame: Box<FrameClause>,

    // OnlyAlias will set to true of the first following case.
    // To make compatible with MySQL, we need to distinguish `select func over w` from `select func over (w)`.
    #[drive(skip)]
    pub only_alias: bool,
}