use crate::ast::limit::Limit;
use crate::ast::node::Node;
use crate::ast::order_by_clause::OrderByClause;
use crate::ast::with_clause::WithClause;
use derive_visitor::Drive;
use std::rc::Rc;

// SetOprStmt represents "union/except/intersect statement"
// See https://dev.mysql.com/doc/refman/5.7/en/union.html
// See https://mariadb.com/kb/en/intersect/
// See https://mariadb.com/kb/en/except/
#[derive(Debug, Drive, Default)]
pub struct SetOprStmt {
    #[drive(skip)]
    pub is_in_braces: bool,
    pub select_list: Option<SetOprSelectList>,
    pub order_by: Option<Rc<OrderByClause>>,
    pub limit: Option<Rc<Limit>>,
    pub with: Option<Rc<WithClause>>,
}

// SetOprSelectList represents the SelectStmt/TableStmt/ValuesStmt list in a union statement.
#[derive(Debug, Drive, Default)]
pub struct SetOprSelectList {
    pub with: Option<Rc<WithClause>>,
    pub after_set_operator: Option<SetOprType>,
    pub selects: Vec<Node>,
    pub limit: Option<Rc<Limit>>,
    pub order_by: Option<Rc<OrderByClause>>,
}

#[derive(Debug, Clone)]
pub enum SetOprType {
    Union,
    UnionAll,
    Except,
    ExceptAll,
    Intersect,
    IntersectAll,
}
