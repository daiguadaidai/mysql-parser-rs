use derive_visitor::Drive;

#[derive(Debug, Drive, Default)]
pub enum OpCode {
    #[default]
    Default,
    LogicAnd,
    LeftShift,
    RightShift,
    LogicOr,
    GE,
    LE,
    EQ,
    NE,
    LT,
    GT,
    Plus,
    Minus,
    And,
    Or,
    Mod,
    Xor,
    Div,
    Mul,
    Not,
    Not2,
    BitNeg,
    IntDiv,
    LogicXor,
    NullEQ,
    In,
    Like,
    Case,
    Regexp,
    IsNull,
    IsTruth,
    IsFalsity,
}