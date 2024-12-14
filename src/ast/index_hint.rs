use crate::ast::ci_str::CIStr;

#[derive(Debug)]
pub enum IndexHintType {
    HintUse,
    HintIgnore,
    HintForce,
    HintOrderIndex,
    HintNoOrderIndex,
}

#[derive(Debug)]
pub enum IndexHintScope {
    HintForScan,
    HintForJoin,
    HintForOrderBy,
    HintForGroupBy,
}

#[derive(Debug)]
pub struct IndexHint {
    pub index_names: Vec<CIStr>,
    pub hint_type: IndexHintType,
    pub hint_scope: IndexHintScope,
}