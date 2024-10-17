#![deny(missing_docs)]
//! Benchmarks for chrono that just depend on std

use crate::format::StrftimeItems;
use crate::prelude::*;
#[cfg(feature = "unstable-locales")]
use crate::Locale;
use crate::{DateTime, FixedOffset, Local, TimeDelta, Utc,};

/// 1. 
pub fn bench_date_from_ymd(year: i32, month: u32, day: u32) {
    let expected = NaiveDate::from_ymd_opt(year, month, day);
    let (y, m, d) = (year, month, day);
    assert_eq!(NaiveDate::from_ymd_opt(y, m, d), expected)
}

/// mutation too complicated
pub fn bench_datetime_parse_from_rfc2822() {
    let str = "Wed, 18 Feb 2015 23:16:09 +0000";
    let res = DateTime::parse_from_rfc2822(str).unwrap();
    println!("{:?}", res);
}

/// 2. combine 3 fuzz_target functions
pub fn bench_datetime_to_string(
    year: i32, month: u32, day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    nano: u32,
    pst_sec: i32,
) {
    // let pst = FixedOffset::east_opt(8 * 60 * 60).unwrap();
    let pst = FixedOffset::east_opt(pst_sec).unwrap();
    let dt = pst
        .from_local_datetime(
            &NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_nano_opt(hour, min, sec, nano)
                .unwrap(),
        )
        .unwrap();
    let res2822 = dt.to_rfc2822();
    // let res2822 = "a";
    let res3339 = dt.to_rfc3339();
    let res3339_opts = dt.to_rfc3339_opts(SecondsFormat::Nanos, true);

    println!("res2822 {}, res3339 {}, res opt {}", res2822, res3339, res3339_opts);
}

// no input data
// pub fn bench_get_local_time() {
//     let _ = Local::now();
// }

/// Returns the number of multiples of `div` in the range `start..end`.
///
/// If the range `start..end` is back-to-front, i.e. `start` is greater than `end`, the
/// behaviour is defined by the following equation:
/// `in_between(start, end, div) == - in_between(end, start, div)`.
///
/// When `div` is 1, this is equivalent to `end - start`, i.e. the length of `start..end`.
///
/// # Panics
///
/// Panics if `div` is not positive.
pub fn in_between(start: i32, end: i32, div: i32) -> i32 {
    assert!(div > 0, "in_between: nonpositive div = {}", div);
    let start = (start.div_euclid(div), start.rem_euclid(div));
    let end = (end.div_euclid(div), end.rem_euclid(div));
    // The lowest multiple of `div` greater than or equal to `start`, divided.
    let start = start.0 + (start.1 != 0) as i32;
    // The lowest multiple of `div` greater than or equal to   `end`, divided.
    let end = end.0 + (end.1 != 0) as i32;
    end - start
}

/// 4.
/// Alternative implementation to `Datelike::num_days_from_ce`
pub fn num_days_from_ce_alt<Date: Datelike>(date: &Date) -> i32 {
    let year = date.year();
    let diff = move |div| in_between(1, year, div);
    // 365 days a year, one more in leap years. In the gregorian calendar, leap years are all
    // the multiples of 4 except multiples of 100 but including multiples of 400.
    date.ordinal() as i32 + 365 * diff(1) + diff(4) - diff(100) + diff(400)
}

// pub fn bench_num_days_from_ce() {
//     for year in &[1, 500, 2000, 2019] {
//         let d = NaiveDate::from_ymd_opt(*year, 1, 1).unwrap();
//         let res1 = num_days_from_ce_alt(y);
//         group.bench_with_input(BenchmarkId::new("classic", year), &d, |b, y| {
//             b.iter(|| y.num_days_from_ce())
//         });
//     }
// }

/// input too complicated
pub fn bench_parse_strftime() {
    let str = "%a, %d %b %Y %H:%M:%S GMT";
    let items = StrftimeItems::new(str);
    let res = items.collect::<Vec<_>>();
    println!("{:?}", res);
}

// use crate::format::locales::Locale;
// #[cfg(feature = "unstable-locales")]
// pub fn bench_parse_strftime_localized() {
//     let str = black_box("%a, %d %b %Y %H:%M:%S GMT");
//     let items = StrftimeItems::new_with_locale(str, Locale::nl_NL);
//     black_box(items.collect::<Vec<_>>());
// }

/// no input data
pub fn bench_format() {
    let dt = Local::now();
    let res = format!("{}", dt.format("%Y-%m-%dT%H:%M:%S%.f%:z"));
    println!("{}", res);
}

/// Example of using StrftimeItems
pub fn bench_format_with_items(_dt1: &NaiveDate, dt2: &NaiveDateTime, dt3: &DateTime<FixedOffset>, dt4: &DateTime<FixedOffset>, dt5: &DateTime<Utc>) {

    let format_strings = vec![
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%d-%b-%Y %I:%M:%S %p",
        "%B %d, %Y %H:%M",
        "%Y-%j %H:%M:%S",
    ];
    
    // 1) general
    for format_str in format_strings.iter() {
        let items: Vec<_> = StrftimeItems::new(format_str).collect();
        // let res1 = format!("{}", dt1.format_with_items(items.iter()));
        let res2 = format!("{}", dt2.format_with_items(items.iter()));
        let res3 = format!("{}", dt3.format_with_items(items.iter()));
        let res4 = format!("{}", dt4.format_with_items(items.iter()));
        let res5 = format!("{}", dt5.format_with_items(items.iter()));
        println!("res2: {}, res3: {}, res4: {} res5: {}", res2, res3, res4, res5);
    }

    // 2) offset (z/Z)
    let offset_strings = vec![
        "%Y-%m-%dT%H:%M:%S%.f%:z",
        "%H:%M:%S %Z",
        "%a, %d %b %Y %H:%M:%S %Z",
        "%Y-%m-%dT%H:%M:%S%.fZ"
    ];
    for format_str in offset_strings.iter() {
        let items: Vec<_> = StrftimeItems::new(format_str).collect();
        let res3 = format!("{}", dt3.format_with_items(items.iter()));
        let res4 = format!("{}", dt4.format_with_items(items.iter()));
        println!("offset res3: {}, res4: {}", res3, res4);
    }

    // 3) nano
    let items: Vec<_> = StrftimeItems::new("%Y-%m-%dT%H:%M:%S%.f").collect();
    let res5 = format!("{}", dt5.format_with_items(items.iter()));
    println!("nano res5: {}", res5);
}

/// Example of manual formatting
pub fn bench_format_manual(dt1: &NaiveDate, dt2: &NaiveDateTime, dt3: &DateTime<FixedOffset>, dt4: &DateTime<FixedOffset>, dt5: &DateTime<Utc>) {
    // let dt = Local::now();
    let res1 = format!(
        "{:04}-{:02}-{:02}T",
        dt1.year(),
        dt1.month(),
        dt1.day(),
    );
    let res2 = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}",
        dt2.year(),
        dt2.month(),
        dt2.day(),
        dt2.hour(),
        dt2.minute(),
        dt2.second(),
        dt2.nanosecond(),
    );
    let res3 = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}{:+02}:{:02}",
        dt3.year(),
        dt3.month(),
        dt3.day(),
        dt3.hour(),
        dt3.minute(),
        dt3.second(),
        dt3.nanosecond(),
        dt3.offset().fix().local_minus_utc() / 3600,
        dt3.offset().fix().local_minus_utc() / 60,
    );
    let res4 = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}{:+02}:{:02}",
        dt4.year(),
        dt4.month(),
        dt4.day(),
        dt4.hour(),
        dt4.minute(),
        dt4.second(),
        dt4.nanosecond(),
        dt4.offset().fix().local_minus_utc() / 3600,
        dt4.offset().fix().local_minus_utc() / 60,
    );
    let res5 = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}{:+02}:{:02}",
        dt5.year(),
        dt5.month(),
        dt5.day(),
        dt5.hour(),
        dt5.minute(),
        dt5.second(),
        dt5.nanosecond(),
        dt5.offset().fix().local_minus_utc() / 3600,
        dt5.offset().fix().local_minus_utc() / 60,
    );
    println!("res1: {}, res2: {}, res3: {}, res4: {}, res5: {}", res1, res2, res3, res4, res5);
}

/// 5.
pub fn bench_naivedate_add_signed(dt1: &NaiveDate, dt2: &NaiveDateTime, dt3: &DateTime<FixedOffset>, dt4: &DateTime<FixedOffset>, dt5: &DateTime<Utc>) {
    let extra = TimeDelta::try_days(25).unwrap();
    let res1 = dt1.checked_add_signed(extra).unwrap();
    let res2 = dt2.checked_add_signed(extra).unwrap();
    let res3 = dt3.checked_add_signed(extra).unwrap();
    let res4 = dt4.checked_add_signed(extra).unwrap();
    let res5 = dt5.checked_add_signed(extra).unwrap();
    println!("res1: {}, res2: {}, res3: {}, res4: {}, res5: {}", res1, res2, res3, res4, res5);
}

/// 6.
pub fn bench_datetime_with(_dt1: &NaiveDate, dt2: &NaiveDateTime, dt3: &DateTime<FixedOffset>, dt4: &DateTime<FixedOffset>, dt5: &DateTime<Utc>) {
    // let dt = FixedOffset::east_opt(3600).unwrap().with_ymd_and_hms(2023, 9, 23, 7, 36, 0).unwrap();
    // let res = dt1.with_hour(12).unwrap();
    let res2 = dt2.with_hour(12).unwrap();
    let res3 = dt3.with_hour(12).unwrap();
    let res4 = dt4.with_hour(12).unwrap();
    let res5 = dt5.with_hour(12).unwrap();
    println!("res2: {}, res3: {}, res4: {}, res5: {}", res2, res3, res4, res5);
}

/// Used for bug report
#[test]
pub fn test_datetime_to_string() {

    // Error causing inputs: `year` is causing error
    let (year, month, day, hour, min, sec, nanosec, pst_sec) 
        = (-2979, 7, 6, 9, 57, 9, 679244465, -30851);

    let pst = FixedOffset::east_opt(pst_sec).unwrap();
    let dt = pst
        .from_local_datetime(
            &NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_nano_opt(hour, min, sec, nanosec)
                .unwrap(),
        )
        .unwrap();

    // Error caused by this line
    let res2822 = dt.to_rfc2822();
    let res3339 = dt.to_rfc3339();
    let res3339_opts = dt.to_rfc3339_opts(SecondsFormat::Nanos, true);

    println!("res 2822 {}, res 3339 {}, res opt {}", res2822, res3339, res3339_opts);
}