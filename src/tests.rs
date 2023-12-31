use super::*;
use quote::quote;
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
        parse2::<DateTime>(quote!(5/6/2024 at 6:23 AM)).unwrap(),
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
        "1/1/2001 at 7:01 PM"
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
        "22/4/1991 at 5:01 PM"
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
        TimeDirection::AfterAbsolute(AbsoluteTime::Date(Date(
            Month::January,
            DayOfMonth(1),
            Year(2024)
        )))
    );
    assert_eq!(
        parse2::<TimeDirection>(quote!(before 23/4/2025)).unwrap(),
        TimeDirection::BeforeAbsolute(AbsoluteTime::Date(Date(
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

#[test]
fn test_parse_relative_time() {
    assert_eq!(
        parse2::<RelativeTime>(quote!(5 days from now)).unwrap(),
        RelativeTime::Directional {
            duration: Duration {
                minutes: 0.into(),
                hours: 0.into(),
                days: 5.into(),
                weeks: 0.into(),
                months: 0.into(),
                years: 0.into(),
            },
            dir: TimeDirection::FromNow
        }
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(24787 years, 32 days ago)).unwrap(),
        RelativeTime::Directional {
            duration: Duration {
                minutes: 0.into(),
                hours: 0.into(),
                days: 32.into(),
                weeks: 0.into(),
                months: 0.into(),
                years: 24787.into(),
            },
            dir: TimeDirection::Ago
        }
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(3 weeks after 18/4/2024)).unwrap(),
        RelativeTime::Directional {
            duration: Duration {
                minutes: 0.into(),
                hours: 0.into(),
                days: 0.into(),
                weeks: 3.into(),
                months: 0.into(),
                years: 0.into(),
            },
            dir: TimeDirection::AfterAbsolute(AbsoluteTime::Date(Date(
                Month::April,
                DayOfMonth(18),
                Year(2024)
            )))
        }
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(7 days before 14/3/2026 5:04 PM)).unwrap(),
        RelativeTime::Directional {
            duration: Duration {
                minutes: 0.into(),
                hours: 0.into(),
                days: 7.into(),
                weeks: 0.into(),
                months: 0.into(),
                years: 0.into(),
            },
            dir: TimeDirection::BeforeAbsolute(AbsoluteTime::DateTime(DateTime(
                Date(Month::March, DayOfMonth(14), Year(2026)),
                Time(Hour::Hour12(5, AmPm::PM), Minute(4))
            )))
        }
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(7 days before 14/3/2026 5:04 PM))
            .unwrap()
            .to_string(),
        "7 days before 14/3/2026 at 5:04 PM"
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(yesterday)).unwrap(),
        RelativeTime::Named(NamedRelativeTime::Yesterday)
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(tomorrow)).unwrap(),
        RelativeTime::Named(NamedRelativeTime::Tomorrow)
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(Day Before Yesterday)).unwrap(),
        RelativeTime::Named(NamedRelativeTime::DayBeforeYesterday)
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(day after tomorrow)).unwrap(),
        RelativeTime::Named(NamedRelativeTime::DayAfterTomorrow)
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(now)).unwrap(),
        RelativeTime::Named(NamedRelativeTime::Now)
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(tomorrow))
            .unwrap()
            .to_string(),
        "tomorrow"
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(the day after tomorrow))
            .unwrap()
            .to_string(),
        "the day after tomorrow"
    );
    assert_eq!(
        parse2::<RelativeTime>(quote!(the day before yesterday))
            .unwrap()
            .to_string(),
        "the day before yesterday"
    );
}

#[test]
fn test_parse_duration() {
    assert_eq!(
        parse2::<Duration>(quote!(6 years 5 months and 4 weeks, 3 days, 2 hours, 1 minute))
            .unwrap(),
        Duration {
            years: 6.into(),
            months: 5.into(),
            weeks: 4.into(),
            days: 3.into(),
            hours: 2.into(),
            minutes: 1.into(),
        }
    );
    assert_eq!(
        parse2::<Duration>(quote!(6 years, 2 hours)).unwrap(),
        Duration {
            years: 6.into(),
            months: 0.into(),
            weeks: 0.into(),
            days: 0.into(),
            hours: 2.into(),
            minutes: 0.into(),
        }
    );
    assert_eq!(
        parse2::<Duration>(quote!(3 minutes and 2 hours)).unwrap(),
        Duration {
            years: 0.into(),
            months: 0.into(),
            weeks: 0.into(),
            days: 0.into(),
            hours: 2.into(),
            minutes: 3.into(),
        }
    );
    assert_eq!(
        parse2::<Duration>(quote!(77 Weeks)).unwrap(),
        Duration {
            years: 0.into(),
            months: 0.into(),
            weeks: 77.into(),
            days: 0.into(),
            hours: 0.into(),
            minutes: 0.into(),
        }
    );
    assert_eq!(
        Duration {
            years: 1.into(),
            months: 2.into(),
            weeks: 3.into(),
            days: 4.into(),
            hours: 5.into(),
            minutes: 6.into(),
        }
        .to_string(),
        "1 year, 2 months, 3 weeks, 4 days, 5 hours, 6 minutes"
    );
    assert_eq!(
        Duration {
            years: 2.into(),
            months: 0.into(),
            weeks: 0.into(),
            days: 0.into(),
            hours: 0.into(),
            minutes: 1.into(),
        }
        .to_string(),
        "2 years, 1 minute"
    );
    assert_eq!(
        Duration {
            years: 0.into(),
            months: 0.into(),
            weeks: 0.into(),
            days: 0.into(),
            hours: 0.into(),
            minutes: 2.into(),
        }
        .to_string(),
        "2 minutes"
    );
}

#[test]
fn test_parse_point_in_time() {
    use AmPm::*;

    assert_eq!(
        parse2::<PointInTime>(quote!(5 days from now)).unwrap(),
        PointInTime::Relative(RelativeTime::Directional {
            duration: Duration {
                minutes: 0.into(),
                hours: 0.into(),
                days: 5.into(),
                weeks: 0.into(),
                months: 0.into(),
                years: 0.into(),
            },
            dir: TimeDirection::FromNow
        })
    );
    assert_eq!(
        parse2::<PointInTime>(quote!(22/4/1991 5:01 PM)).unwrap(),
        PointInTime::Absolute(AbsoluteTime::DateTime(DateTime(
            Date(Month::April, DayOfMonth(22), Year(1991)),
            Time(Hour::Hour12(5, PM), Minute(01))
        )))
    );
    assert_eq!(
        PointInTime::Absolute(AbsoluteTime::DateTime(DateTime(
            Date(Month::April, DayOfMonth(22), Year(1991)),
            Time(Hour::Hour12(5, PM), Minute(01))
        )))
        .to_string(),
        "22/4/1991 at 5:01 PM"
    );
    assert_eq!(
        PointInTime::Relative(RelativeTime::Directional {
            duration: Duration {
                minutes: 0.into(),
                hours: 0.into(),
                days: 5.into(),
                weeks: 0.into(),
                months: 0.into(),
                years: 0.into(),
            },
            dir: TimeDirection::FromNow
        })
        .to_string(),
        "5 days from now"
    );
}

#[test]
fn test_parse_time_range() {
    parse2::<TimeRange>(quote!(from 3 days, 1 hour, 23 minutes ago to 22/4/2029)).unwrap();
    assert_eq!(
        parse2::<TimeRange>(quote!(from 8789 hours ago to 37 days from now))
            .unwrap()
            .to_string(),
        "from 8789 hours ago to 37 days from now"
    );
}

#[test]
fn test_parse_time_expressions() {
    parse2::<TimeExpression>(quote!(3 hours)).unwrap();
    parse2::<TimeExpression>(quote!(3 hours before 2/1/1822 11:59 PM)).unwrap();
    parse2::<TimeExpression>(quote!(2/1/1822 22:34)).unwrap();
    assert_eq!(
        parse2::<TimeExpression>(quote!(2/1/1822 22:34))
            .unwrap()
            .to_string(),
        "2/1/1822 at 22:34"
    );
    assert_eq!(
        parse2::<TimeExpression>(quote!(3 hours before 2/1/1822 11:59 PM))
            .unwrap()
            .to_string(),
        "3 hours before 2/1/1822 at 11:59 PM"
    );
    assert_eq!(
        parse2::<TimeExpression>(quote!(3 hours))
            .unwrap()
            .to_string(),
        "3 hours"
    );
    assert_eq!(
        parse2::<TimeExpression>(quote!(tomorrow))
            .unwrap()
            .to_string(),
        "tomorrow"
    );
    assert_eq!(
        parse2::<TimeExpression>(quote!(3 days before yesterday))
            .unwrap()
            .to_string(),
        "3 days before yesterday"
    );
}
