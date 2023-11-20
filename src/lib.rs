use chumsky::prelude::*;

// S → TimeExpression
// TimeExpression → AbsoluteTime | RelativeTime | TimeCalculation
// AbsoluteTime → 'Date' | 'DateTime' | ISOFormat
// RelativeTime → Number TimeUnit TimeDirection
// TimeUnit → 'minutes' | 'hours' | 'days' | 'weeks' | 'months' | 'years'
// TimeDirection → 'ago' | 'from now'
// TimeCalculation → Number TimeUnit 'before' TimeExpression | Number TimeUnit 'after' TimeExpression
// Number → [Any integer value]
// Date → Month Day, Year
// DateTime → Month Day, Year 'at' Time
// Month → 'January' | 'February' | ... | 'December'
// Day → [1-31]
// Year → [Any year]
// Time → Hour ':' Minute [AM/PM]
// Hour → [1-12]
// Minute → [00-59]
// ISOFormat → 'ISO Date' | 'ISO DateTime' | 'ISO Week Date' | 'ISO Ordinal Date'

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum TimeExpression {
    Specific(PointInTime),
    Range(PointInTime, PointInTime),
    Duration(Duration),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Duration {
    pub minutes: Number,
    pub hours: Number,
    pub days: Number,
    pub years: Number,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum PointInTime {
    Absolute(AbsoluteTime),
    Relative(RelativeTime),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum AbsoluteTime {
    Date(Date),
    DateTime(DateTime),
    ISO(Timestamp),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct RelativeTime {
    pub num: Number,
    pub unit: TimeUnit,
    pub dir: TimeDirection,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Date(Month, DayOfMonth, Year);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Time(Hour, Minute, Option<AmPm>);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct DateTime(Date, Time);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct DayOfMonth(u8);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Year(u16);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Hour(u8);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Minute(u8);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(u8)]
pub enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum AmPm {
    Am,
    Pm,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum TimeUnit {
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum TimeDirection {
    After(DateTime),
    Before(DateTime),
    Ago,
    FromNow,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Number(u64);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Timestamp(u64);
