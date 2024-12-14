use crate::ast::expr_node::ExprNode;
use crate::ast::functions::TimeUnitType;
use derive_visitor::Drive;

// FrameType is the type of window function frame.
// Window function frame types.
// MySQL only supports `ROWS` and `RANGES`.
#[derive(Debug)]
pub enum FrameType {
    Rows,
    Ranges,
    Groups,
}

// FrameClause represents frame clause.
#[derive(Debug, Drive)]
pub struct FrameClause {
    #[drive(skip)]
    pub tp: FrameType,
    pub extent: FrameExtent,
}

// FrameType is the type of window function frame bound.
// Frame bound types.
#[derive(Debug)]
pub enum BoundType {
    Following,
    Preceding,
    CurrentRow,
}

// FrameBound represents frame bound.
#[derive(Debug, Drive)]
pub struct FrameBound {
    #[drive(skip)]
    pub tp: BoundType,
    #[drive(skip)]
    pub un_bounded: bool,
    pub expr: Box<ExprNode>,
    // `Unit` is used to indicate the units in which the `Expr` should be interpreted.
    // For example: '2:30' MINUTE_SECOND.
    #[drive(skip)]
    pub unit: TimeUnitType,
}


#[derive(Debug, Drive)]
pub struct FrameExtent {
    pub start: FrameBound,
    pub end: FrameBound,
}