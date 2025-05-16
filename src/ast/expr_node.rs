use crate::ast::ci_str::CIStr;
use crate::ast::column_name::ColumnName;
use crate::ast::common::FulltextSearchModifier;
use crate::ast::functions::{GetFormatSelectorType, TimeUnitType, TrimDirectionType};
use crate::ast::op_code;
use crate::ast::order_by_clause::OrderByClause;
use crate::ast::subquery_expr::SubQueryExpr;
use crate::ast::table_name::TableName;
use crate::ast::window_spec::WindowSpec;
use bigdecimal::BigDecimal;
use derive_visitor::Drive;

#[derive(Debug, Drive, Default)]
pub enum ExprNode {
    #[default]
    #[drive(skip)]
    Default,
    #[drive(skip)]
    ColumnNameExpr(ColumnNameExpr),
    SubQueryExpr(SubQueryExpr),
    RowExpr(RowExpr),
    VariableExpr(VariableExpr),
    BinaryOperationExpr(BinaryOperationExpr),
    ExistsSubqueryExpr(ExistsSubqueryExpr),
    UnaryOperationExpr(UnaryOperationExpr),
    MatchAgainst(MatchAgainst),
    FuncCallExpr(FuncCallExpr),
    #[drive(skip)]
    TimeUnitExpr(TimeUnitExpr),
    #[drive(skip)]
    ValueExpr(ValueExpr),
    #[drive(skip)]
    TrimDirectionExpr(TrimDirectionExpr),
    #[drive(skip)]
    GetFormatSelectorExpr(GetFormatSelectorExpr),
    TableNameExpr(TableNameExpr),
    SetCollationExpr(SetCollationExpr),
    WindowFuncExpr(WindowFuncExpr),
    PositionExpr(PositionExpr),
    ParamMarkerExpr(ParamMarkerExpr),
    AggregateFuncExpr(AggregateFuncExpr),
}

#[derive(Debug, Drive, Default)]
pub struct VariableExpr {
    #[drive(skip)]
    pub name: String,
    #[drive(skip)]
    pub is_global: bool,
    #[drive(skip)]
    pub is_system: bool,
    #[drive(skip)]
    pub explicit_scope: bool,
    pub value: Option<Box<ExprNode>>,
}

#[derive(Debug, Drive, Default)]
pub struct BinaryOperationExpr {
    #[drive(skip)]
    // Op is the operator code for BinaryOperation.
    pub op: op_code::OpCode,
    // L is the left expression in BinaryOperation.
    pub l: Option<Box<ExprNode>>,
    // R is the right expression in BinaryOperation.
    pub r: Option<Box<ExprNode>>,
}

// ExistsSubqueryExpr is the expression for "exists (select ...)".
// See https://dev.mysql.com/doc/refman/5.7/en/exists-and-not-exists-subqueries.html
#[derive(Debug, Drive, Default)]
pub struct ExistsSubqueryExpr {
    // Sel is the subquery, may be rewritten to other type of expression.
    pub sel: Option<Box<ExprNode>>,
    // Not is true, the expression is "not exists".
    #[drive(skip)]
    pub not: bool,
}

// UnaryOperationExpr is the expression for unary operator.
#[derive(Debug, Drive, Default)]
pub struct UnaryOperationExpr {
    // Op is the operator opcode.
    #[drive(skip)]
    pub op: op_code::OpCode,
    // V is the unary expression.
    pub v: Option<Box<ExprNode>>,
}

#[derive(Debug, Default)]
pub struct ColumnNameExpr {
    pub name: ColumnName,
}

// MatchAgainst is the expression for matching against fulltext index.
#[derive(Debug, Drive, Default)]
pub struct MatchAgainst {
    // ColumnNames are the columns to match.
    #[drive(skip)]
    pub column_names: Vec<ColumnName>,
    // Against
    pub against: Option<Box<ExprNode>>,
    // Modifier
    #[drive(skip)]
    pub modifier: FulltextSearchModifier,
}

#[derive(Debug, Default)]
pub enum FuncCallExprType {
    #[default]
    Keyword,
    Generic,
}
// FuncCallExpr is for function expression.
#[derive(Debug, Drive, Default)]
pub struct FuncCallExpr {
    #[drive(skip)]
    pub tp: FuncCallExprType,
    #[drive(skip)]
    pub schema: CIStr,
    // FnName is the function name.
    #[drive(skip)]
    pub fn_name: CIStr,
    // Args is the function args.
    pub args: Vec<ExprNode>,
}

// TimeUnitExpr is an expression representing a time or timestamp unit.
#[derive(Debug)]
pub struct TimeUnitExpr {
    // Unit is the time or timestamp unit.
    pub unit: TimeUnitType,
}

#[derive(Debug, Default)]
pub enum ValueExprData {
    #[default]
    Default,
    NULL,
    Bool(bool),
    Isize(isize),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Decimal(BigDecimal),
    BinaryLiteral(Vec<u8>),
    BitLiteral(Vec<u8>),
    HexLiteral(Vec<u8>),
    Other(String),
}

#[derive(Debug, Default)]
pub enum ValueExprKind {
    #[default]
    Default,
    None,
    Bool(bool),
    Isize(isize),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    String,
    Bytes(Vec<u8>),
    Decimal(BigDecimal),
    BinaryLiteral,
    BitLiteral,
    HexLiteral,
    Other,
}

#[derive(Debug, Default)]
pub struct ValueExpr {
    pub s: String,
    pub kind: ValueExprKind,
    pub charset: String,
    pub collation: String,
}

impl ValueExpr {
    pub fn new(s: &str, kind: ValueExprKind, charset: &str, collation: &str) -> Self {
        ValueExpr {
            s: s.to_string(),
            kind,
            charset: charset.to_string(),
            collation: collation.to_string(),
        }
    }

    pub fn get_value_i64(&self) -> Option<i64> {
        match self.kind {
            ValueExprKind::Isize(v) => Some(v as i64),
            ValueExprKind::I64(v) => Some(v),
            ValueExprKind::U64(v) => Some(v as i64),
            _ => None,
        }
    }
}

#[derive(Debug, Drive)]
pub struct RowExpr {
    pub values: Vec<ExprNode>,
}

#[derive(Debug)]
pub struct TrimDirectionExpr {
    pub direction: TrimDirectionType,
}

#[derive(Debug)]
pub struct GetFormatSelectorExpr {
    pub selector: GetFormatSelectorType,
}

#[derive(Debug, Drive)]
pub struct TableNameExpr {
    pub name: TableName,
}

#[derive(Debug, Drive)]
pub struct SetCollationExpr {
    pub expr: Option<Box<ExprNode>>,
    #[drive(skip)]
    pub collate: String,
}

// WindowFuncExpr represents window function expression.
#[derive(Debug, Drive, Default)]
pub struct WindowFuncExpr {
    // Name is the function name.
    #[drive(skip)]
    pub name: String,
    // Args is the function args.
    pub args: Vec<ExprNode>,
    // Distinct cannot be true for most window functions, except `max` and `min`.
    // We need to raise error if it is not allowed to be true.
    #[drive(skip)]
    pub distinct: bool,
    // IgnoreNull indicates how to handle null value.
    // MySQL only supports `RESPECT NULLS`, so we need to raise error if it is true.
    #[drive(skip)]
    pub ignore_null: bool,
    // FromLast indicates the calculation direction of this window function.
    // MySQL only supports calculation from first, so we need to raise error if it is true.
    #[drive(skip)]
    pub from_last: bool,
    // Spec is the specification of this window.
    pub spec: Option<WindowSpec>,
}

// PositionExpr is the expression for order by and group by position.
// MySQL use position expression started from 1, it looks a little confused inner.
// maybe later we will use 0 at first.
#[derive(Debug, Drive, Default)]
pub struct PositionExpr {
    // N is the position, started from 1 now.
    #[drive(skip)]
    pub n: isize,
    // P is the parameterized position.
    pub p: Option<Box<ExprNode>>,
}

#[derive(Debug, Drive, Default)]
pub struct ParamMarkerExpr {
    #[drive(skip)]
    pub offset: isize,
    #[drive(skip)]
    pub order: isize,
    #[drive(skip)]
    pub in_execute: bool,
    pub value_expr: Option<Box<ExprNode>>,
    #[drive(skip)]
    pub token_index: usize,
    #[drive(skip)]
    pub start_pos: usize,
    #[drive(skip)]
    pub end_pos: usize,
}

// AggregateFuncExpr represents aggregate function expression.
#[derive(Debug, Drive, Default)]
pub struct AggregateFuncExpr {
    // F is the function name.
    #[drive(skip)]
    pub f: String,
    // Args is the function args.
    pub args: Vec<ExprNode>,
    // Distinct is true, function hence only aggregate distinct values.
    // For example, column c1 values are "1", "2", "2",  "sum(c1)" is "5",
    // but "sum(distinct c1)" is "3".
    #[drive(skip)]
    pub distinct: bool,
    pub order: Option<OrderByClause>,
}
