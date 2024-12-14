use crate::mysql::consts::PriorityEnum;
use crate::parser::common::*;
use crate::parser::input::Input;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn priority(i: Input) -> IResult<PriorityEnum> {
    alt((
        map(rule!("LOW_PRIORITY"), |_| PriorityEnum::LowPriority),
        map(rule!("HIGH_PRIORITY"), |_| PriorityEnum::HighPriority),
        map(rule!("DELAYED"), |_| PriorityEnum::DelayedPriority),
    ))(i)
}
