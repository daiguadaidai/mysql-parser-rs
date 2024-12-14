use crate::ast::select_stmt::SelectStmt;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub enum Statement {
    SelectStmt(Box<SelectStmt>)
}