use crate::ast::ci_str::CIStr;
use crate::ast::common::ASTType;
use crate::ast::expr_node::ExprNode;
use derive_visitor::Drive;

// WildCardField is a special type of select field content.
#[derive(Debug, Default)]
pub struct WildCardField {
    pub table: CIStr,
    pub schema: CIStr,
}

#[derive(Debug, Drive, Default)]
pub enum Field {
    #[default]
    Unkonw,
    // WildCard is not nil, Expr will be nil.
    #[drive(skip)]
    WildCardField(WildCardField),
    // Expr is not nil, WildCard will be nil.
    Expr(ExprNode),
}

// SelectField represents fields in select statement.
// There are two type of select field: wildcard
// and expression with optional alias name.
#[derive(Debug, Drive, Default)]
pub struct SelectField {
    // Offset is used to get original text.
    #[drive(skip)]
    pub offset: isize,
    pub field: Field,
    // AsName is alias name for Expr.
    #[drive(skip)]
    pub as_name: CIStr,
    // Auxiliary stands for if this field is auxiliary.
    // When we add a Field into SelectField list which is used for having/orderby clause but the field is not in select clause,
    // we should set its Auxiliary to true. Then the TrimExec will trim the field.
    #[drive(skip)]
    pub auxiliary: bool,
    #[drive(skip)]
    pub auxiliary_col_in_agg: bool,
    #[drive(skip)]
    pub auxiliary_col_in_order_by: bool,
}

#[derive(Debug)]
pub struct FieldItem {
    pub tp: ASTType,
    pub value: String,
    pub opt_enclosed: bool,
}

// FieldsClause represents fields references clause in load data statement.
#[derive(Debug)]
pub struct FieldsClause {
    pub terminated: Option<String>,
    pub enclosed: Option<String>, // length always <= 1 if not nil, see parser.y
    pub escaped: Option<String>,  // length always <= 1 if not nil, see parser.y
    pub opt_enclosed: bool,
    pub defined_null_by: Option<String>,
    pub null_value_opt_enclosed: bool,
}
