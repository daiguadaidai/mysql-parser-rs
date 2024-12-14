use crate::ast::ci_str::CIStr;

// HintTimeRange is the payload of `TIME_RANGE` hint
#[derive(Debug, Clone)]
pub struct HintTimeRange {
    pub from: String,
    pub to: String,
}

// HintSetVar is the payload of `SET_VAR` hint
#[derive(Debug, Clone)]
pub struct HintSetVar {
    pub var_name: String,
    pub value: String,
}

// HintTable is table in the hint. It may have query block info.
#[derive(Debug, Clone)]
pub struct HintTable {
    pub db_name: CIStr,
    pub table_name: CIStr,
    pub qb_name: CIStr,
    pub partition_list: Vec<CIStr>,
}

#[derive(Debug, Clone)]
pub enum HintData {
    Int64(i64),
    Uint64(u64),
    Bool(bool),
    CIStr(CIStr),
    HintTimeRange(HintTimeRange),
    HintSetVar(HintSetVar),
}

#[derive(Debug, Clone)]
pub struct TableOptimizerHint {
    // HintName is the name or alias of the table(s) which the hint will affect.
    // Table hints has no schema info
    // It allows only table name or alias (if table has an alias)
    pub hint_name: CIStr,
    // HintData is the payload of the hint. The actual type of this field
    // is defined differently as according `HintName`. Define as following:
    //
    // Statement Execution Time Optimizer Hints
    // See https://dev.mysql.com/doc/refman/5.7/en/optimizer-hints.html#optimizer-hints-execution-time
    // - MAX_EXECUTION_TIME  => uint64
    // - MEMORY_QUOTA        => int64
    // - QUERY_TYPE          => CIStr
    //
    // Time Range is used to hint the time range of inspection tables
    // e.g: select /*+ time_range('','') */ * from information_schema.inspection_result.
    // - TIME_RANGE          => ast.HintTimeRange
    // - READ_FROM_STORAGE   => CIStr
    // - USE_TOJA            => bool
    // - NTH_PLAN            => int64
    pub hint_data: HintData,
    // QBName is the default effective query block of this hint.
    pub qb_name: CIStr,
    pub tables: Vec<HintTable>,
    pub indexes: Vec<CIStr>,
}
