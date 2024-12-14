// PriorityEnum is defined for Priority const values.
#[derive(Debug, Default, Copy, Clone)]
pub enum PriorityEnum {
    // Priority const values.
    // See https://dev.mysql.com/doc/refman/5.7/en/insert.html
    #[default]
    NoPriority,
    LowPriority,
    HighPriority,
    DelayedPriority,
}
