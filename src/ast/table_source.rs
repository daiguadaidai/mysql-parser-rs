use crate::ast::ci_str::CIStr;
use crate::ast::result_set_node::ResultSetNode;
use derive_visitor::Drive;

#[derive(Debug, Drive)]
pub struct TableSource {
    pub source: Box<ResultSetNode>,
    #[drive(skip)]
    pub as_name: CIStr,
}