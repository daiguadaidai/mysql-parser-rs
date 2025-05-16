use crate::ast::result_set_node::ResultSetNode;
use derive_visitor::Drive;

// SubqueryExpr represents a subquery.
#[derive(Debug, Drive, Default)]
pub struct SubQueryExpr {
    // Query is the query SelectNode.
    pub query: Option<ResultSetNode>,
    #[drive(skip)]
    pub evaluated: bool,
    #[drive(skip)]
    pub correlated: bool,
    #[drive(skip)]
    pub multi_rows: bool,
    #[drive(skip)]
    pub exists: bool,
}
