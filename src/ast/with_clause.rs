use crate::ast::common_table_expression::CommonTableExpression;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub struct WithClause {
    #[drive(skip)]
    pub is_recursive: bool,
    pub ctes: Vec<CommonTableExpression>,
}