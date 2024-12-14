use crate::ast::line_clause::LinesClause;
use crate::ast::select_field::FieldsClause;

#[derive(Debug)]
pub enum SelectIntoType {
    SelectIntoOutfile,
    SelectIntoDumpfile,
    SelectIntoVars,
}

#[derive(Debug)]
pub struct SelectIntoOption {
    pub tp: SelectIntoType,
    pub file_name: String,
    pub fields_info: Option<FieldsClause>,
    pub lines_info: Option<LinesClause>,
}
