//! Timelang is a simple DSL (Domain Specific Language) for representing human-readable
//! time-related expressions including specific date/times, relative expressions like "3 hours
//! from now", time ranges, and durations.
//!
//! ## Context Free Grammar
//! Here is a rough CFG (Context Free Grammar) for timelang:
//!
//! ```cfg
//! S → TimeExpression
//! TimeExpression → PointInTime | TimeRange | Duration
//! PointInTime → AbsoluteTime | RelativeTime
//! TimeRange → 'from' PointInTime 'to' PointInTime
//! Duration → (Number TimeUnit)+
//! AbsoluteTime → Date | DateTime
//! RelativeTime → Duration 'ago' | Duration 'from now' | Duration 'before' AbsoluteTime | Duration 'after' AbsoluteTime
//! Date → DayOfMonth '/' Month '/' Year
//! DateTime → Date Time
//! Time → Hour ':' Minute AmPm?
//! Hour → Number
//! Minute → Number
//! Month → Number
//! DayOfMonth → Number
//! Year → Number
//! AmPm → 'AM' | 'PM'
//! TimeUnit → 'minutes' | 'hours' | 'days' | 'weeks' | 'months' | 'years'
//! Number → [Any positive integer value]
//! ```

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};
use syn::{
    parse::{Parse, ParseStream, Result},
    Error, Ident, LitInt, Token,
};

#[cfg(test)]
mod tests;

/// The high-level AST node for Timelang.
///
/// Usually you would want a more specific type like [`PointInTime`], [`TimeRange`], [`Time`],
/// [`DateTime`], etc., however this node type is included so we can technically consider
/// Timelang to be a distinct language.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum TimeExpression {
    Specific(PointInTime), // (LitInt, Ident) or (LitInt, Token![/])
    Range(TimeRange),      // Ident, LitInt
    Duration(Duration),    // LitInt, Ident
}

impl Parse for TimeExpression {
    fn parse(input: ParseStream) -> Result<Self> {
        if !input.peek(Ident) && !input.peek(LitInt) {
            return Err(Error::new(input.span(), "expected [number] or [keyword]"));
        }
        if input.peek(Ident) {
            return Ok(TimeExpression::Range(input.parse()?));
        }
        if input.peek(LitInt) && input.peek2(Token![/]) {
            // case 2 for PointInTime
            return Ok(TimeExpression::Specific(input.parse()?));
        }
        // now we either have a Duration or PointInTime starting with a Duration
        let fork = input.fork();
        if fork.parse::<PointInTime>().is_ok() {
            return Ok(TimeExpression::Specific(input.parse()?));
        }
        Ok(TimeExpression::Duration(input.parse()?))
    }
}

impl Display for TimeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeExpression::Specific(point) => write!(f, "{point}"),
            TimeExpression::Range(tr) => write!(f, "{tr}"),
            TimeExpression::Duration(dur) => write!(f, "{dur}"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TimeRange(pub PointInTime, pub PointInTime);

impl Parse for TimeRange {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        if ident.to_string().to_lowercase() != "from" {
            return Err(Error::new(ident.span(), "expected `from`"));
        }
        let t1 = input.parse::<PointInTime>()?;
        let ident = input.parse::<Ident>()?;
        if ident.to_string().to_lowercase() != "to" {
            return Err(Error::new(ident.span(), "expected `to`"));
        }
        let t2 = input.parse::<PointInTime>()?;
        Ok(TimeRange(t1, t2))
    }
}

impl Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "from {} to {}", self.0, self.1)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Duration {
    pub minutes: Number,
    pub hours: Number,
    pub days: Number,
    pub weeks: Number,
    pub months: Number,
    pub years: Number,
}

impl Parse for Duration {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut minutes: Option<Number> = None;
        let mut hours: Option<Number> = None;
        let mut days: Option<Number> = None;
        let mut weeks: Option<Number> = None;
        let mut months: Option<Number> = None;
        let mut years: Option<Number> = None;
        while input.peek(LitInt) {
            let num = input.parse::<Number>()?;
            let unit = input.parse::<TimeUnit>()?;
            match unit {
                TimeUnit::Minutes => minutes = Some(minutes.unwrap_or(Number(0)) + num),
                TimeUnit::Hours => hours = Some(hours.unwrap_or(Number(0)) + num),
                TimeUnit::Days => days = Some(days.unwrap_or(Number(0)) + num),
                TimeUnit::Weeks => weeks = Some(weeks.unwrap_or(Number(0)) + num),
                TimeUnit::Months => months = Some(months.unwrap_or(Number(0)) + num),
                TimeUnit::Years => years = Some(years.unwrap_or(Number(0)) + num),
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        if minutes.is_none()
            && hours.is_none()
            && days.is_none()
            && weeks.is_none()
            && months.is_none()
            && years.is_none()
        {
            return Err(Error::new(
                input.span(),
                "expected [number] followed by one of `minutes`, `hours`, `days`, `years`",
            ));
        }
        Ok(Duration {
            minutes: minutes.unwrap_or(Number(0)),
            hours: hours.unwrap_or(Number(0)),
            days: days.unwrap_or(Number(0)),
            weeks: weeks.unwrap_or(Number(0)),
            months: months.unwrap_or(Number(0)),
            years: years.unwrap_or(Number(0)),
        })
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut before = false;
        if self.years > 0 {
            before = true;
        }
        if self.years == 1 {
            write!(f, "1 year")?;
        } else if self.years > 1 {
            write!(f, "{} years", self.years)?;
        }
        if self.months > 0 {
            if before {
                write!(f, ", ")?;
            }
            before = true;
        }
        if self.months == 1 {
            write!(f, "1 month")?;
        } else if self.months > 1 {
            write!(f, "{} months", self.months)?;
        }
        if self.weeks > 0 {
            if before {
                write!(f, ", ")?;
            }
            before = true;
        }
        if self.weeks == 1 {
            write!(f, "1 week")?;
        } else if self.weeks > 1 {
            write!(f, "{} weeks", self.weeks)?;
        }
        if self.days > 0 {
            if before {
                write!(f, ", ")?;
            }
            before = true;
        }
        if self.days == 1 {
            write!(f, "1 day")?;
        } else if self.days > 1 {
            write!(f, "{} days", self.days)?;
        }
        if self.hours > 0 {
            if before {
                write!(f, ", ")?;
            }
            before = true;
        }
        if self.hours == 1 {
            write!(f, "1 hour")?;
        } else if self.hours > 1 {
            write!(f, "{} hours", self.hours)?;
        }
        if self.minutes > 0 {
            if before {
                write!(f, ", ")?;
            }
        }
        if self.minutes == 1 {
            write!(f, "1 minute")?;
        } else if self.minutes > 1 {
            write!(f, "{} minutes", self.minutes)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum PointInTime {
    Absolute(AbsoluteTime),
    Relative(RelativeTime),
}

impl Parse for PointInTime {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitInt) && input.peek2(Token![/]) {
            Ok(PointInTime::Absolute(input.parse::<AbsoluteTime>()?))
        } else {
            Ok(PointInTime::Relative(input.parse::<RelativeTime>()?))
        }
    }
}

impl Display for PointInTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PointInTime::Absolute(abs) => write!(f, "{abs}"),
            PointInTime::Relative(rel) => write!(f, "{rel}"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum AbsoluteTime {
    Date(Date),
    DateTime(DateTime),
}

impl Parse for AbsoluteTime {
    fn parse(input: ParseStream) -> Result<Self> {
        let date = input.parse::<Date>()?;
        if input.peek(LitInt) && input.peek2(Token![:]) && input.peek3(LitInt) {
            let time = input.parse::<Time>()?;
            return Ok(AbsoluteTime::DateTime(DateTime(date, time)));
        }
        Ok(AbsoluteTime::Date(date))
    }
}

impl Display for AbsoluteTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbsoluteTime::Date(date) => write!(f, "{}", date),
            AbsoluteTime::DateTime(date_time) => write!(f, "{}", date_time),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct RelativeTime {
    // 5 days, 3 hours after 22/5/2024
    pub duration: Duration,
    pub dir: TimeDirection,
}

impl Parse for RelativeTime {
    fn parse(input: ParseStream) -> Result<Self> {
        let duration = input.parse::<Duration>()?;
        let dir = input.parse::<TimeDirection>()?;
        Ok(RelativeTime { duration, dir })
    }
}

impl Display for RelativeTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.duration, self.dir)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Date(pub Month, pub DayOfMonth, pub Year);

impl Parse for Date {
    fn parse(input: ParseStream) -> Result<Self> {
        let day = input.parse::<DayOfMonth>()?;
        input.parse::<Token![/]>()?;
        let month = input.parse::<Month>()?;
        input.parse::<Token![/]>()?;
        let year = input.parse::<Year>()?;
        Ok(Date(month, day, year))
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}/{}", self.1, self.0, self.2))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct DateTime(pub Date, pub Time); // 22/4/1991 5:25 PM

impl Parse for DateTime {
    fn parse(input: ParseStream) -> Result<Self> {
        let date = input.parse::<Date>()?;
        let time = input.parse::<Time>()?;
        Ok(DateTime(date, time))
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.0, self.1))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Time(pub Hour, pub Minute);

impl Parse for Time {
    fn parse(input: ParseStream) -> Result<Self> {
        let hour_lit = input.parse::<LitInt>()?;
        let hour_val = hour_lit.base10_parse::<u8>()?;
        input.parse::<Token![:]>()?;
        let min = input.parse::<Minute>()?;
        if let Ok(am_pm) = input.parse::<AmPm>() {
            if hour_val > 12 || hour_val == 0 {
                return Err(Error::new(
                    hour_lit.span(),
                    "hour must be between 1 and 12 (inclusive)",
                ));
            }
            return Ok(Time(Hour::Hour12(hour_val, am_pm), min));
        }
        if hour_val > 24 {
            return Err(Error::new(
                hour_lit.span(),
                "hour must be between 0 and 24 (inclusive)",
            ));
        }
        Ok(Time(Hour::Hour24(hour_val), min))
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time(Hour::Hour12(hour, am_pm), minute) => {
                write!(f, "{}:{:02} {}", hour, minute, am_pm)
            }
            Time(Hour::Hour24(hour), minute) => write!(f, "{}:{:02}", hour, minute),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct DayOfMonth(pub u8);

impl Parse for DayOfMonth {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<LitInt>()?;
        let int_val = lit.base10_parse::<u8>()?;
        if int_val > 31 || int_val == 0 {
            return Err(Error::new(
                lit.span(),
                "day must be between 1 and 31 (inclusive)",
            ));
        }
        Ok(DayOfMonth(int_val))
    }
}

impl Display for DayOfMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Year(pub u16);

impl Parse for Year {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<LitInt>()?;
        let int_val = lit.base10_parse::<u16>()?;
        Ok(Year(int_val))
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Hour {
    Hour12(u8, AmPm),
    Hour24(u8),
}

impl Parse for Hour {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<LitInt>()?;
        let int_val = lit.base10_parse::<u8>()?;
        if let Ok(am_pm) = input.parse::<AmPm>() {
            if int_val > 12 || int_val == 0 {
                return Err(Error::new(
                    lit.span(),
                    "hour must be between 1 and 12 (inclusive)",
                ));
            }
            return Ok(Hour::Hour12(int_val, am_pm));
        }
        if int_val > 24 {
            return Err(Error::new(
                lit.span(),
                "hour must be between 0 and 24 (inclusive)",
            ));
        }
        Ok(Hour::Hour24(int_val))
    }
}

impl Display for Hour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hour::Hour12(hour, am_pm) => f.write_fmt(format_args!("{hour} {am_pm}",)),
            Hour::Hour24(hour) => f.write_fmt(format_args!("{hour}")),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Minute(pub u8);

impl Parse for Minute {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<LitInt>()?;
        let int_val = lit.base10_parse::<u8>()?;
        if int_val > 60 {
            return Err(Error::new(
                lit.span(),
                "minute must be between 0 and 60 (inclusive)",
            ));
        }
        Ok(Minute(int_val))
    }
}

impl Display for Minute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}", self.0))
    }
}

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

impl Parse for Month {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<LitInt>()?;
        let int_val = lit.base10_parse::<u8>()?;
        if int_val > 12 || int_val == 0 {
            return Err(Error::new(
                lit.span(),
                "month must be between 1 and 12 (inclusive)",
            ));
        }
        use Month::*;
        Ok(match int_val {
            1 => January,
            2 => February,
            3 => March,
            4 => April,
            5 => May,
            6 => June,
            7 => July,
            8 => August,
            9 => September,
            10 => October,
            11 => November,
            12 => December,
            _ => unreachable!(),
        })
    }
}

impl From<Month> for u8 {
    fn from(value: Month) -> Self {
        use Month::*;
        match value {
            January => 1,
            February => 2,
            March => 3,
            April => 4,
            May => 5,
            June => 6,
            July => 7,
            August => 8,
            September => 9,
            October => 10,
            November => 11,
            December => 12,
        }
    }
}

impl From<&Month> for u8 {
    fn from(value: &Month) -> Self {
        (*value).into()
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_u8: u8 = self.into();
        f.write_fmt(format_args!("{}", as_u8))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum AmPm {
    AM,
    PM,
}

impl Parse for AmPm {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().to_lowercase().as_str() {
            "am" => Ok(AmPm::AM),
            "pm" => Ok(AmPm::PM),
            _ => Err(Error::new(ident.span(), "expected `AM` or `PM`")),
        }
    }
}

impl Display for AmPm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmPm::AM => f.write_str("AM"),
            AmPm::PM => f.write_str("PM"),
        }
    }
}

impl AsRef<str> for AmPm {
    fn as_ref(&self) -> &str {
        match self {
            AmPm::AM => "AM",
            AmPm::PM => "PM",
        }
    }
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

impl Parse for TimeUnit {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        use TimeUnit::*;
        Ok(match ident.to_string().to_lowercase().as_str() {
            "mins" | "minutes" | "minute" | "min" => Minutes,
            "hours" | "hrs" | "hour" | "hr" => Hours,
            "days" | "day" => Days,
            "weeks" | "week" => Weeks,
            "months" | "month" => Months,
            "years" | "yr" | "year" => Years,
            _ => {
                return Err(Error::new(
                    ident.span(),
                    "expected one of `minutes`, `hours`, `days`, `weeks`, `months`, and `years`",
                ))
            }
        })
    }
}

impl AsRef<str> for TimeUnit {
    fn as_ref(&self) -> &str {
        match self {
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Weeks => "minutes",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum TimeDirection {
    After(AbsoluteTime),
    Before(AbsoluteTime),
    Ago,
    FromNow,
}

impl Parse for TimeDirection {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident1 = input.parse::<Ident>()?;
        match ident1.to_string().to_lowercase().as_str() {
            "after" => Ok(TimeDirection::After(input.parse::<AbsoluteTime>()?)),
            "before" => Ok(TimeDirection::Before(input.parse::<AbsoluteTime>()?)),
            "ago" => Ok(TimeDirection::Ago),
            "from" => {
                let ident2 = input.parse::<Ident>()?;
                if ident2.to_string().to_lowercase().as_str() != "now" {
                    return Err(Error::new(ident2.span(), "expected `now`"));
                }
                Ok(TimeDirection::FromNow)
            }
            _ => Err(Error::new(
                ident1.span(),
                "expected one of `after`, `before`, `ago`, `from`",
            )),
        }
    }
}

impl Display for TimeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeDirection::After(abs_time) => write!(f, "after {abs_time}"),
            TimeDirection::Before(abs_time) => write!(f, "before {abs_time}"),
            TimeDirection::Ago => f.write_str("ago"),
            TimeDirection::FromNow => f.write_str("from now"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Number(pub u64);

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number(value)
    }
}

impl From<Number> for u64 {
    fn from(value: Number) -> Self {
        value.0
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl PartialEq<u64> for Number {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u64> for Number {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Parse for Number {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<LitInt>()?;
        let int_val = lit.base10_parse::<u64>()?;
        Ok(Number(int_val))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum DayOfWeek {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl AsRef<str> for DayOfWeek {
    fn as_ref(&self) -> &str {
        match self {
            DayOfWeek::Sun => "Sun",
            DayOfWeek::Mon => "Mon",
            DayOfWeek::Tue => "Tue",
            DayOfWeek::Wed => "Wed",
            DayOfWeek::Thu => "Thu",
            DayOfWeek::Fri => "Fri",
            DayOfWeek::Sat => "Sat",
        }
    }
}

impl Display for DayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl Parse for DayOfWeek {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().to_lowercase().as_str() {
            "sun" | "sunday" => Ok(DayOfWeek::Sun),
            "mon" | "monday" => Ok(DayOfWeek::Mon),
            "tue" | "tuesday" => Ok(DayOfWeek::Tue),
            "wed" | "wednesday" => Ok(DayOfWeek::Wed),
            "thu" | "thursday" => Ok(DayOfWeek::Thu),
            "fri" | "friday" => Ok(DayOfWeek::Fri),
            "sat" | "saturday" => Ok(DayOfWeek::Sat),
            _ => Err(Error::new(
                ident.span(),
                "expected one of `Sun`, `Mon`, `Tue`, `Wed`, `Thu`, `Fri`, `Sat`",
            )),
        }
    }
}

#[cfg(test)]
macro_rules! assert_impl_all {
    ($($typ:ty),* : $($tt:tt)*) => {{
        const fn _assert_impl<T>() where T: $($tt)*, {}
        $(_assert_impl::<$typ>();)*
    }};
}

#[test]
fn test_traits() {
    assert_impl_all!(
        DayOfWeek,
        TimeDirection,
        TimeUnit,
        AmPm,
        Month,
        Hour,
        AbsoluteTime,
        Duration,
        RelativeTime,
        PointInTime,
        TimeExpression : Copy
        + Clone
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + core::fmt::Debug
        + core::fmt::Display
        + Parse
        + core::hash::Hash
    );
}
