use crate::ast::expr_node::ExprNode;
use derive_visitor::Drive;

#[derive(Debug)]
pub enum SampleMethodType {
    SampleMethodTypeNone,
    SampleMethodTypeSystem,
    SampleMethodTypeBernoulli,
    SampleMethodTypeTiDBRegion,
}

#[derive(Debug)]
pub enum SampleClauseUnitType {
    SampleClauseUnitTypeDefault,
    SampleClauseUnitTypeRow,
    SampleClauseUnitTypePercent,
}

#[derive(Debug, Drive)]
pub struct TableSample {
    #[drive(skip)]
    pub sample_method: SampleMethodType,
    pub expr: Box<ExprNode>,
    #[drive(skip)]
    pub sample_clause_unit: SampleClauseUnitType,
    pub repeatable_seed: Box<ExprNode>,
}