use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    Error, Ident,
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
pub enum Timestamp {
    RFC2822(DateTime),
    RFC3339(DateTime),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum DayOfWeekShort {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl AsRef<str> for DayOfWeekShort {
    fn as_ref(&self) -> &str {
        match self {
            DayOfWeekShort::Sun => "Sun",
            DayOfWeekShort::Mon => "Mon",
            DayOfWeekShort::Tue => "Tue",
            DayOfWeekShort::Wed => "Wed",
            DayOfWeekShort::Thu => "Thu",
            DayOfWeekShort::Fri => "Fri",
            DayOfWeekShort::Sat => "Sat",
        }
    }
}

impl Parse for DayOfWeekShort {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().to_lowercase().as_str() {
            "sun" => Ok(DayOfWeekShort::Sun),
            "mon" => Ok(DayOfWeekShort::Mon),
            "tue" => Ok(DayOfWeekShort::Tue),
            "wed" => Ok(DayOfWeekShort::Wed),
            "thu" => Ok(DayOfWeekShort::Thu),
            "fri" => Ok(DayOfWeekShort::Fri),
            "sat" => Ok(DayOfWeekShort::Sat),
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
fn test_parse_day_of_week_short() {
    use DayOfWeekShort::*;

    assert_eq!(parse2::<DayOfWeekShort>(quote!(Mon)).unwrap(), Mon);
    assert_eq!(parse2::<DayOfWeekShort>(quote!(Tue)).unwrap(), Tue);
    assert_eq!(parse2::<DayOfWeekShort>(quote!(Wed)).unwrap(), Wed);
    assert_eq!(parse2::<DayOfWeekShort>(quote!(Thu)).unwrap(), Thu);
    assert_eq!(parse2::<DayOfWeekShort>(quote!(Fri)).unwrap(), Fri);
    assert_eq!(parse2::<DayOfWeekShort>(quote!(Sat)).unwrap(), Sat);
    assert_eq!(parse2::<DayOfWeekShort>(quote!(Sun)).unwrap(), Sun);
    assert!(parse2::<DayOfWeekShort>(quote!(Monday)).is_err());
    assert!(parse2::<DayOfWeekShort>(quote!(Mo)).is_err());
}
