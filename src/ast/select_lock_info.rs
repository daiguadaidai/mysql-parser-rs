use crate::ast::table_name::TableName;
use derive_visitor::Drive;

// SelectLockType is the lock type for SelectStmt.
// Select lock types.
#[derive(Debug)]
pub enum SelectLockType {
    SelectLockNone,
    SelectLockForUpdate,
    SelectLockForShare,
    SelectLockForUpdateNoWait,
    SelectLockForUpdateWaitN,
    SelectLockForShareNoWait,
    SelectLockForUpdateSkipLocked,
    SelectLockForShareSkipLocked,
}

#[derive(Debug, Drive)]
pub struct SelectLockInfo {
    #[drive(skip)]
    pub lock_type: SelectLockType,
    #[drive(skip)]
    pub wait_sec: u64,
    pub tables: Vec<TableName>,
}