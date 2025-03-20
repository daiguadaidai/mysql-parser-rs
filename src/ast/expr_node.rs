use crate::ast::ci_str::CIStr;
use crate::ast::column_name::ColumnName;
use crate::ast::common::FulltextSearchModifier;
use crate::ast::functions::{GetFormatSelectorType, TimeUnitType, TrimDirectionType};
use crate::ast::op_code;
use crate::ast::subquery_expr::SubqueryExpr;
use crate::ast::table_name::TableName;
use bigdecimal::BigDecimal;
use derive_visitor::Drive;

#[derive(Debug, Drive, Default)]
pub enum ExprNode {
    #[default]
    #[drive(skip)]
    Default,
    #[drive(skip)]
    ColumnNameExpr(ColumnNameExpr),
    SubqueryExpr(SubqueryExpr),
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
