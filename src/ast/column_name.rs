use crate::ast::ci_str::CIStr;

#[derive(Debug)]
pub struct ColumnName {
    pub schema: CIStr,
    pub table: CIStr,
    pub name: CIStr,
}