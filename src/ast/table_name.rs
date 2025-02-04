use crate::ast::ci_str::CIStr;
use crate::ast::expr_node::ExprNode;
use crate::ast::index_hint::IndexHint;
use crate::ast::table_sample::TableSample;
use derive_visitor::Drive;

#[derive(Debug, Default, Drive)]
pub struct TableName {
    #[drive(skip)]
    pub schema: CIStr,
    #[drive(skip)]
    pub name: CIStr,

    #[drive(skip)]
    pub index_hints: Vec<IndexHint>,
    #[drive(skip)]
    pub partition_names: Vec<CIStr>,
    pub table_sample: Option<Box<TableSample>>,

    // AS OF is used to see the data as it was at a specific point in time.
    pub as_of: Option<Box<ExprNode>>,

    // IsAlias is true if this table name is an alias.
    //  sometime, we need to distinguish the table name is an alias or not.
    //   for example ```delete tt1 from t1 tt1,(select max(id) id from t2)tt2 where tt1.id<=tt2.id```
    //   ```tt1``` is a alias name. so we need to set IsAlias to true and restore the table name without database name.
    #[drive(skip)]
    pub is_alias: bool,
}
