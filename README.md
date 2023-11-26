# 🕗 Timelang

[![Crates.io](https://img.shields.io/crates/v/timelang)](https://crates.io/crates/timelang)
[![docs.rs](https://img.shields.io/docsrs/timelang?label=docs)](https://docs.rs/timelang/latest/timelang/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sam0x17/timelang/ci.yaml)](https://github.com/sam0x17/timelang/actions/workflows/ci.yaml?query=branch%3Amain)
[![MIT License](https://img.shields.io/github/license/sam0x17/timelang)](https://github.com/sam0x17/timelang/blob/main/LICENSE)

Timelang is a simple DSL (Domain Specific Language) for representing human-readable
time-related expressions including specific date/times, relative expressions like "3 hours from
now", time ranges, and durations.


 ## Getting Started

To use timelang, you should take a look at [TimeExpression], which is the top-level entry point
of the AST, or some of the more specific types like [Duration], [PointInTime], and [TimeRange].

All nodes in timelang impl [FromStr] as well as [syn::parse::Parse] which is used for the
internal parsing logic. The standard [Display] impl is used on all node types as the preferred
means of outputting them to a string.

Note that for the moment, only years, months, weeks, days, hours, and minutes are supported in
timelang, but seconds and more might be added later. Generally better than minute resolution is
not needed in many of the common use-cases for timelang.


 ## Examples

The following are all examples of valid expressions in timelang:
- `now`
- `tomorrow`
- `next tuesday`
- `day after tomorrow`
- `the day before yesterday`
- `20/4/2021`
- `11:21 AM`
- `15/6/2022 at 3:58 PM`
- `2 hours, 37 minutes`
- `5 years, 2 months, 3 weeks and 11 minutes`
- `7 days ago`
- `2 years and 10 minutes from now`
- `5 days, 3 weeks, 6 minutes after 15/4/2025 at 9:27 AM`
- `from 1/1/2023 at 14:07 to 15/1/2023`
- `from 19/3/2024 at 10:07 AM to 3 months 2 days after 3/9/2027 at 5:27 PM`
- `2 days and 14 hours after the day after tomorrow`
- `11 days before the day before yesterday`
- `5 days after next tuesday`

## Examples

Specific Date:
```rust
use timelang::*;
assert_eq!(
    "20/4/2021".parse::<TimeExpression>().unwrap(),
    TimeExpression::Specific(PointInTime::Absolute(AbsoluteTime::Date(Date(
        Month::April,
        DayOfMonth(20),
        Year(2021)
    ))))
);
```

Specific DateTime:
```rust
use timelang::*;
assert_eq!(
    "15/6/2022 at 14:00".parse::<AbsoluteTime>().unwrap(),
    AbsoluteTime::DateTime(DateTime(
        Date(Month::June, DayOfMonth(15), Year(2022)),
        Time(Hour::Hour24(14), Minute(0))
    ))
);
```

Time Range:
```rust
use timelang::*;
assert_eq!(
    "from 1/1/2023 to 15/1/2023"
        .parse::<TimeExpression>()
        .unwrap(),
    TimeExpression::Range(TimeRange(
        PointInTime::Absolute(AbsoluteTime::Date(Date(
            Month::January,
            DayOfMonth(1),
            Year(2023)
        ))),
        PointInTime::Absolute(AbsoluteTime::Date(Date(
            Month::January,
            DayOfMonth(15),
            Year(2023)
        )))
    ))
);
```

Duration (multiple units with comma):
```rust
use timelang::*;
assert_eq!(
    "2 hours, 30 minutes".parse::<TimeExpression>().unwrap(),
    TimeExpression::Duration(Duration {
        hours: Number(2),
        minutes: Number(30),
        days: Number(0),
        weeks: Number(0),
        months: Number(0),
        years: Number(0)
    })
);
```

Duration (multiple units with `and`):
```rust
use timelang::*;
assert_eq!(
    "1 year and 6 months".parse::<TimeExpression>().unwrap(),
    TimeExpression::Duration(Duration {
        years: Number(1),
        months: Number(6),
        days: Number(0),
        weeks: Number(0),
        hours: Number(0),
        minutes: Number(0)
    })
);
```

Relative Time (using `ago`):
```rust
use timelang::*;
assert_eq!(
    "3 days ago".parse::<TimeExpression>().unwrap(),
    TimeExpression::Specific(PointInTime::Relative(RelativeTime::Directional {
        duration: Duration {
            days: Number(3),
            minutes: Number(0),
            hours: Number(0),
            weeks: Number(0),
            months: Number(0),
            years: Number(0)
        },
        dir: TimeDirection::Ago
    }))
);
```

Relative Time (using `from now`):
```rust
use timelang::*;
assert_eq!(
    "5 days, 10 hours, and 35 minutes from now"
        .parse::<TimeExpression>()
        .unwrap(),
    TimeExpression::Specific(PointInTime::Relative(RelativeTime::Directional {
        duration: Duration {
            minutes: Number(35),
            hours: Number(10),
            days: Number(5),
            weeks: Number(0),
            months: Number(0),
            years: Number(0)
        },
        dir: TimeDirection::FromNow
    }))
);
```

Relative Time (`after` a specific date):
```rust
use timelang::*;
assert_eq!(
    "2 hours, 3 minutes after 10/10/2022"
        .parse::<TimeExpression>()
        .unwrap(),
    TimeExpression::Specific(PointInTime::Relative(RelativeTime::Directional {
        duration: Duration {
            hours: Number(2),
            minutes: Number(3),
            days: Number(0),
            weeks: Number(0),
            months: Number(0),
            years: Number(0)
        },
        dir: TimeDirection::AfterAbsolute(AbsoluteTime::Date(Date(
            Month::October,
            DayOfMonth(10),
            Year(2022)
        )))
    }))
);
```

Relative Time (`before` a specific date/time):
```rust
use timelang::*;
assert_eq!(
    "1 day before 31/12/2023 at 11:13 PM"
        .parse::<TimeExpression>()
        .unwrap(),
    TimeExpression::Specific(PointInTime::Relative(RelativeTime::Directional {
        duration: Duration {
            days: Number(1),
            minutes: Number(0),
            hours: Number(0),
            weeks: Number(0),
            months: Number(0),
            years: Number(0)
        },
        dir: TimeDirection::BeforeAbsolute(AbsoluteTime::DateTime(DateTime(
            Date(Month::December, DayOfMonth(31), Year(2023)),
            Time(Hour::Hour12(11, AmPm::PM), Minute(13))
        )))
    }))
);
```

Time Range (with specific date/times):
```rust
use timelang::*;
assert_eq!(
    "from 1/1/2024 at 10:00 to 2/1/2024 at 15:30"
        .parse::<TimeExpression>()
        .unwrap(),
    TimeExpression::Range(TimeRange(
        PointInTime::Absolute(AbsoluteTime::DateTime(DateTime(
            Date(Month::January, DayOfMonth(1), Year(2024)),
            Time(Hour::Hour24(10), Minute(0))
        ))),
        PointInTime::Absolute(AbsoluteTime::DateTime(DateTime(
            Date(Month::January, DayOfMonth(2), Year(2024)),
            Time(Hour::Hour24(15), Minute(30))
        )))
    ))
);
```

Relative Time:
```rust
use timelang::*;
assert_eq!("now".parse::<RelativeTime>().unwrap(), RelativeTime::Named(NamedRelativeTime::Now));
assert_eq!(
    "tomorrow".parse::<RelativeTime>().unwrap(),
    RelativeTime::Named(NamedRelativeTime::Tomorrow)
);
assert_eq!(
    "yesterday".parse::<RelativeTime>().unwrap(),
    RelativeTime::Named(NamedRelativeTime::Yesterday)
);
assert_eq!(
    "day before yesterday".parse::<RelativeTime>().unwrap(),
    RelativeTime::Named(NamedRelativeTime::DayBeforeYesterday)
);
// note the optional `the`
assert_eq!(
    "the day after tomorrow".parse::<RelativeTime>().unwrap(),
    RelativeTime::Named(NamedRelativeTime::DayAfterTomorrow)
);
assert_eq!(
    "next tuesday".parse::<RelativeTime>().unwrap(),
    RelativeTime::Next(RelativeTimeUnit::Tuesday)
);
assert_eq!(
    "last wednesday".parse::<RelativeTime>().unwrap(),
    RelativeTime::Last(RelativeTimeUnit::Wednesday)
);
assert_eq!(
    "3 days before yesterday".parse::<RelativeTime>().unwrap(),
    RelativeTime::Directional {
        duration: Duration {
            minutes: Number(0),
            hours: Number(0),
            days: Number(3),
            weeks: Number(0),
            months: Number(0),
            years: Number(0)
        },
        dir: TimeDirection::BeforeNamed(NamedRelativeTime::Yesterday)
    }
);
assert_eq!(
    "2 days and 14 hours after the day after tomorrow".parse::<RelativeTime>().unwrap(),
    RelativeTime::Directional {
        duration: Duration {
            minutes: Number(0),
            hours: Number(14),
            days: Number(2),
            weeks: Number(0),
            months: Number(0),
            years: Number(0)
        },
        dir: TimeDirection::AfterNamed(NamedRelativeTime::DayAfterTomorrow)
    }
);
assert_eq!(
    "2 weeks before last sunday".parse::<RelativeTime>().unwrap(),
    RelativeTime::Directional {
        duration: Duration {
            minutes: Number(0),
            hours: Number(0),
            days: Number(0),
            weeks: Number(2),
            months: Number(0),
            years: Number(0)
        },
        dir: TimeDirection::BeforeLast(RelativeTimeUnit::Sunday)
    }
);
assert_eq!(
    "3 years, 2 weeks after next thursday".parse::<RelativeTime>().unwrap(),
    RelativeTime::Directional {
        duration: Duration {
            minutes: Number(0),
            hours: Number(0),
            days: Number(0),
            weeks: Number(2),
            months: Number(0),
            years: Number(3)
        },
        dir: TimeDirection::AfterNext(RelativeTimeUnit::Thursday)
    }
);
```