use crate::ast::select_stmt::SelectStmt;
use crate::ast::table_name::TableName;
use crate::ast::table_refs_clause::TableRefsClause;
use crate::ast::table_source::TableSource;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub enum ResultSetNode {
    TableRefsClause(Box<TableRefsClause>),
    SelectStmt(Box<SelectStmt>),
    TableName(Box<TableName>),
    TableSource(Box<TableSource>),
}