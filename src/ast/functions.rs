// TimeUnitType is the type for time and timestamp units.
#[derive(Debug)]
pub enum TimeUnitType {
    // TimeUnitInvalid is a placeholder for an invalid time or timestamp unit
    TimeUnitInvalid,
    // TimeUnitMicrosecond is the time or timestamp unit MICROSECOND.
    TimeUnitMicrosecond,
    // TimeUnitSecond is the time or timestamp unit SECOND.
    TimeUnitSecond,
    // TimeUnitMinute is the time or timestamp unit MINUTE.
    TimeUnitMinute,
    // TimeUnitHour is the time or timestamp unit HOUR.
    TimeUnitHour,
    // TimeUnitDay is the time or timestamp unit DAY.
    TimeUnitDay,
    // TimeUnitWeek is the time or timestamp unit WEEK.
    TimeUnitWeek,
    // TimeUnitMonth is the time or timestamp unit MONTH.
    TimeUnitMonth,
    // TimeUnitQuarter is the time or timestamp unit QUARTER.
    TimeUnitQuarter,
    // TimeUnitYear is the time or timestamp unit YEAR.
    TimeUnitYear,
    // TimeUnitSecondMicrosecond is the time unit SECOND_MICROSECOND.
    TimeUnitSecondMicrosecond,
    // TimeUnitMinuteMicrosecond is the time unit MINUTE_MICROSECOND.
    TimeUnitMinuteMicrosecond,
    // TimeUnitMinuteSecond is the time unit MINUTE_SECOND.
    TimeUnitMinuteSecond,
    // TimeUnitHourMicrosecond is the time unit HOUR_MICROSECOND.
    TimeUnitHourMicrosecond,
    // TimeUnitHourSecond is the time unit HOUR_SECOND.
    TimeUnitHourSecond,
    // TimeUnitHourMinute is the time unit HOUR_MINUTE.
    TimeUnitHourMinute,
    // TimeUnitDayMicrosecond is the time unit DAY_MICROSECOND.
    TimeUnitDayMicrosecond,
    // TimeUnitDaySecond is the time unit DAY_SECOND.
    TimeUnitDaySecond,
    // TimeUnitDayMinute is the time unit DAY_MINUTE.
    TimeUnitDayMinute,
    // TimeUnitDayHour is the time unit DAY_HOUR.
    TimeUnitDayHour,
    // TimeUnitYearMonth is the time unit YEAR_MONTH.
    TimeUnitYearMonth,
}