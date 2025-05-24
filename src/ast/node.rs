use crate::ast::result_set_node::ResultSetNode;
use crate::ast::select_stmt::SelectStmt;
use crate::ast::set_opr_stmt::SetOprSelectList;
use crate::ast::statement::Statement;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub enum Node {
    Statement(Statement),
    ResultSetNode(ResultSetNode),
    SetOprSelectList(Box<SetOprSelectList>),
}

impl Node {
    pub fn new_select_stmt(stmt: SelectStmt) -> Self {
        Node::Statement(Statement::SelectStmt(Box::new(stmt)))
    }

    pub fn new_select_stmt_by_ref(stmt: Box<SelectStmt>) -> Self {
        Node::Statement(Statement::SelectStmt(stmt))
    }

    pub fn new_set_opr_select_list(list: SetOprSelectList) -> Self {
        Node::SetOprSelectList(Box::new(list))
    }
}
