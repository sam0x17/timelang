use std::fmt::Display;
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
    // 5 days after 22/5/2024
    pub num: Number,
    pub unit: TimeUnit,
    pub dir: TimeDirection,
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
use syn::parse2;

#[cfg(test)]
use quote::quote;

#[test]
fn test_parse_minutes() {
    assert_eq!(parse2::<Minute>(quote!(59)).unwrap(), Minute(59));
    assert_eq!(parse2::<Minute>(quote!(0)).unwrap(), Minute(0));
    assert!(parse2::<Minute>(quote!(-1)).is_err());
    assert!(parse2::<Minute>(quote!(61)).is_err());
    assert!(parse2::<Minute>(quote!(259)).is_err());
}

#[test]
fn test_parse_hours() {
    use AmPm::*;

    assert_eq!(parse2::<Hour>(quote!(23)).unwrap(), Hour::Hour24(23));
    assert_eq!(parse2::<Hour>(quote!(0)).unwrap(), Hour::Hour24(0));
    assert!(parse2::<Hour>(quote!(25)).is_err());
    assert!(parse2::<Hour>(quote!(259)).is_err());

    assert_eq!(parse2::<Hour>(quote!(11 AM)).unwrap(), Hour::Hour12(11, AM));
    assert_eq!(parse2::<Hour>(quote!(1 PM)).unwrap(), Hour::Hour12(1, PM));
    assert!(parse2::<Hour>(quote!(0 AM)).is_err());
    assert!(parse2::<Hour>(quote!(21 PM)).is_err());
    assert!(parse2::<Hour>(quote!(26 AM)).is_err());
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

#[test]
fn test_parse_month() {
    use Month::*;

    assert_eq!(parse2::<Month>(quote!(6)).unwrap(), June);
    assert!(parse2::<Month>(quote!(0)).is_err());
    assert!(parse2::<Month>(quote!(13)).is_err());
    assert_eq!(
        format!("{}", parse2::<Month>(quote!(8)).unwrap()).as_str(),
        "8"
    );
}

#[test]
fn test_parse_am_pm() {
    use AmPm::*;

    assert_eq!(parse2::<AmPm>(quote!(Am)).unwrap(), AM);
    assert_eq!(parse2::<AmPm>(quote!(AM)).unwrap(), AM);
    assert_eq!(parse2::<AmPm>(quote!(PM)).unwrap(), PM);
    assert!(parse2::<AmPm>(quote!(Aam)).is_err());
    assert_eq!(format!("{}", AM).as_str(), "AM");
    assert_eq!(PM.as_ref(), "PM");
}

#[test]
fn test_parse_date() {
    assert_eq!(
        parse2::<Date>(quote!(22 / 4 / 1991)).unwrap(),
        Date(Month::April, DayOfMonth(22), Year(1991))
    );
    assert!(parse2::<Date>(quote!(0 / 3 / 1993)).is_err());
    assert!(parse2::<Date>(quote!(11 / 4)).is_err());
    assert_eq!(
        Date(Month::July, DayOfMonth(5), Year(1991))
            .to_string()
            .as_str(),
        "5/7/1991"
    );
}

#[test]
fn test_parse_time() {
    use AmPm::*;
    assert_eq!(
        parse2::<Time>(quote!(4:34 PM)).unwrap(),
        Time(Hour::Hour12(4, PM), Minute(34))
    );
    assert_eq!(
        parse2::<Time>(quote!(12:00 AM)).unwrap(),
        Time(Hour::Hour12(12, AM), Minute(00))
    );
    assert_eq!(
        parse2::<Time>(quote!(1:13 PM)).unwrap(),
        Time(Hour::Hour12(1, PM), Minute(13))
    );
    assert_eq!(
        parse2::<Time>(quote!(00:00)).unwrap(),
        Time(Hour::Hour24(0), Minute(00))
    );
    assert!(parse2::<Time>(quote!(13:24 AM)).is_err());
    assert_eq!(
        parse2::<Time>(quote!(4:34 PM))
            .unwrap()
            .to_string()
            .as_str(),
        "4:34 PM"
    );
    assert_eq!(
        parse2::<Time>(quote!(23:44)).unwrap().to_string().as_str(),
        "23:44"
    );
    assert_eq!(
        parse2::<Time>(quote!(23:01)).unwrap().to_string().as_str(),
        "23:01"
    );
}

#[test]
fn test_parse_date_time() {
    use AmPm::*;

    assert_eq!(
        parse2::<DateTime>(quote!(5/6/2024 6:23 AM)).unwrap(),
        DateTime(
            Date(Month::June, DayOfMonth(5), Year(2024)),
            Time(Hour::Hour12(6, AM), Minute(23))
        )
    );
    assert_eq!(
        parse2::<DateTime>(quote!(5/6/2024 23:01)).unwrap(),
        DateTime(
            Date(Month::June, DayOfMonth(5), Year(2024)),
            Time(Hour::Hour24(23), Minute(01))
        )
    );
    assert_eq!(
        parse2::<DateTime>(quote!(1/1/2001 7:01 PM))
            .unwrap()
            .to_string(),
        "1/1/2001 7:01 PM"
    );
}

#[test]
fn test_parse_absolute_time() {
    use AmPm::*;

    assert_eq!(
        parse2::<AbsoluteTime>(quote!(22 / 4 / 1991)).unwrap(),
        AbsoluteTime::Date(Date(Month::April, DayOfMonth(22), Year(1991)))
    );
    assert_eq!(
        parse2::<AbsoluteTime>(quote!(22/4/1991 5:01 PM)).unwrap(),
        AbsoluteTime::DateTime(DateTime(
            Date(Month::April, DayOfMonth(22), Year(1991)),
            Time(Hour::Hour12(5, PM), Minute(01))
        ))
    );
    assert_eq!(
        parse2::<AbsoluteTime>(quote!(22/4/1991 5:01 PM))
            .unwrap()
            .to_string(),
        "22/4/1991 5:01 PM"
    );
    assert_eq!(
        parse2::<AbsoluteTime>(quote!(22 / 4 / 1991))
            .unwrap()
            .to_string(),
        "22/4/1991"
    );
}

#[test]
fn test_parse_time_unit() {
    assert_eq!(
        parse2::<TimeUnit>(quote!(Minutes)).unwrap(),
        TimeUnit::Minutes
    );
    assert_eq!(TimeUnit::Months.as_ref(), "months");
}

#[test]
fn test_parse_time_direction() {
    assert_eq!(
        parse2::<TimeDirection>(quote!(after 1/1/2024)).unwrap(),
        TimeDirection::After(AbsoluteTime::Date(Date(
            Month::January,
            DayOfMonth(1),
            Year(2024)
        )))
    );
    assert_eq!(
        parse2::<TimeDirection>(quote!(before 23/4/2025)).unwrap(),
        TimeDirection::Before(AbsoluteTime::Date(Date(
            Month::April,
            DayOfMonth(23),
            Year(2025)
        )))
    );
    assert_eq!(
        parse2::<TimeDirection>(quote!(ago)).unwrap(),
        TimeDirection::Ago
    );
    assert_eq!(
        parse2::<TimeDirection>(quote!(from now)).unwrap(),
        TimeDirection::FromNow
    );
    assert_eq!(
        parse2::<TimeDirection>(quote!(before 23/4/2025))
            .unwrap()
            .to_string(),
        "before 23/4/2025"
    );
    assert_eq!(
        parse2::<TimeDirection>(quote!(after 1/1/2024))
            .unwrap()
            .to_string(),
        "after 1/1/2024"
    );
    assert_eq!(TimeDirection::Ago.to_string(), "ago");
    assert_eq!(TimeDirection::FromNow.to_string(), "from now");
}
