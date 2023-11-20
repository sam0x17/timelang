use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    Error, Ident, LitInt, Token,
};

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

impl Parse for Number {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<LitInt>()?;
        let int_val = lit.base10_parse::<u64>()?;
        Ok(Number(int_val))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Timestamp {
    RFC2822(DateTime),
    RFC3339(DateTime),
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
use syn::parse2;

#[test]
fn test_parse_minutes() {
    assert_eq!(parse2::<Minute>(quote!(59)).unwrap(), Minute(59));
    assert_eq!(parse2::<Minute>(quote!(0)).unwrap(), Minute(0));
    assert!(parse2::<Minute>(quote!(-1)).is_err());
    assert!(parse2::<Minute>(quote!(61)).is_err());
    assert!(parse2::<Minute>(quote!(259)).is_err());
}

#[test]
fn test_parse_numbers() {
    assert_eq!(parse2::<Number>(quote!(32323)).unwrap(), Number(32323));
    assert_eq!(parse2::<Number>(quote!(0)).unwrap(), Number(0));
    assert!(parse2::<Number>(quote!(-1)).is_err());
}

#[test]
fn test_parse_day_of_week_short() {
    use DayOfWeek::*;

    assert_eq!(parse2::<DayOfWeek>(quote!(Mon)).unwrap(), Mon);
    assert_eq!(parse2::<DayOfWeek>(quote!(Tue)).unwrap(), Tue);
    assert_eq!(parse2::<DayOfWeek>(quote!(Wed)).unwrap(), Wed);
    assert_eq!(parse2::<DayOfWeek>(quote!(Thu)).unwrap(), Thu);
    assert_eq!(parse2::<DayOfWeek>(quote!(Fri)).unwrap(), Fri);
    assert_eq!(parse2::<DayOfWeek>(quote!(Sat)).unwrap(), Sat);
    assert_eq!(parse2::<DayOfWeek>(quote!(Sun)).unwrap(), Sun);
    assert_eq!(parse2::<DayOfWeek>(quote!(sun)).unwrap(), Sun);
    assert_eq!(parse2::<DayOfWeek>(quote!(Monday)).unwrap(), Mon);
    assert!(parse2::<DayOfWeek>(quote!(Mo)).is_err());
}
