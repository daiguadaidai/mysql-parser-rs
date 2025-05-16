use crate::ast::select_stmt::SelectStmt;
use crate::ast::set_opr_stmt::SetOprStmt;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub enum Statement {
    SelectStmt(Box<SelectStmt>),
    SetOprStmt(Box<SetOprStmt>),
}
