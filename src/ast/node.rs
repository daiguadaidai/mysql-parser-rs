use crate::ast::result_set_node::ResultSetNode;
use crate::ast::set_opr_stmt::SetOprSelectList;
use crate::ast::statement::Statement;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub enum Node {
    Statement(Statement),
    ResultSetNode(ResultSetNode),
    SetOprSelectList(Box<SetOprSelectList>),
}
