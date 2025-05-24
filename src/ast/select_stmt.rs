use crate::ast::expr_node::{ExprNode, RowExpr};
use crate::ast::group_by_clause::GroupByClause;
use crate::ast::limit::Limit;
use crate::ast::order_by_clause::OrderByClause;
use crate::ast::select_field::SelectField;
use crate::ast::select_into_option::SelectIntoOption;
use crate::ast::select_lock_info::SelectLockInfo;
use crate::ast::set_opr_stmt::SetOprType;
use crate::ast::table_optimizer_hint::TableOptimizerHint;
use crate::ast::table_refs_clause::TableRefsClause;
use crate::ast::window_spec::WindowSpec;
use crate::ast::with_clause::WithClause;
use crate::mysql;
use derive_visitor::Drive;
use std::rc::Rc;

#[derive(Debug, Default)]
pub enum SelectStmtKind {
    #[default]
    SelectStmtKindSelect,
    SelectStmtKindTable,
    SelectStmtKindValues,
}

// SelectStmtOpts wrap around select hints and switches
#[derive(Debug, Default)]
pub struct SelectStmtOpts {
    pub distinct: bool,
    pub sql_big_result: bool,
    pub sql_buffer_result: bool,
    pub sql_cache: bool,
    pub sql_small_result: bool,
    pub calc_found_rows: bool,
    pub straight_join: bool,
    pub priority: mysql::consts::PriorityEnum,
    pub table_hints: Vec<TableOptimizerHint>,
    pub explicit_all: bool,
}

// SelectStmt represents the select query node.
// See https://dev.mysql.com/doc/refman/5.7/en/select.html
#[derive(Debug, Drive, Default)]
pub struct SelectStmt {
    #[drive(skip)]
    pub select_stmt_opts: SelectStmtOpts,
    // From is the from clause of the query.
    pub from: Option<TableRefsClause>,
    // Where is the where clause in select statement.
    pub where_clause: Option<ExprNode>,
    // Fields is the select expression list.
    pub fields: Vec<SelectField>,
    // GroupBy is the group by expression list.
    pub group_by: Option<GroupByClause>,
    // Having is the having condition.
    pub having: Option<ExprNode>,
    // WindowSpecs is the window specification list.
    pub window_specs: Vec<WindowSpec>,
    // OrderBy is the ordering expression list.
    pub order_by: Option<Rc<OrderByClause>>,
    // Limit is the limit clause.
    pub limit: Option<Rc<Limit>>,
    // LockInfo is the lock type
    pub lock_info: Option<SelectLockInfo>,
    // IsInBraces indicates whether it's a stmt in brace.
    #[drive(skip)]
    pub is_in_braces: bool,
    // WithBeforeBraces indicates whether stmt's with clause is before the brace.
    // It's used to distinguish (with xxx select xxx) and with xxx (select xxx)
    #[drive(skip)]
    pub with_before_braces: bool,
    // QueryBlockOffset indicates the order of this SelectStmt if counted from left to right in the sql text.
    #[drive(skip)]
    pub query_block_offset: isize,
    // SelectIntoOpt is the select-into option.
    #[drive(skip)]
    pub select_into_opt: Option<SelectIntoOption>,
    // AfterSetOperator indicates the SelectStmt after which type of set operator
    #[drive(skip)]
    pub after_set_operator: Option<SetOprType>,
    // Kind refer to three kind of statement: SelectStmt, TableStmt and ValuesStmt
    #[drive(skip)]
    pub kind: SelectStmtKind,
    // Lists is filled only when Kind == SelectStmtKindValues
    pub lists: Vec<RowExpr>,
    pub with: Option<Rc<WithClause>>,
    // AsViewSchema indicates if this stmt provides the schema for the view. It is only used when creating the view
    #[drive(skip)]
    pub as_view_schema: bool,
}
