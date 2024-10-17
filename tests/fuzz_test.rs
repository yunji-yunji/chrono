use std::println;
use std::string::String;
use std::vec::*;
use std::boxed::Box;

use chrono::*;
use crate::bench_chrono::*;

fn run(start: i32, end: i32, div: i32,
    year: i32, month: u32, day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    nano: u32,
    pst_sec: i32,
    timestamp_secs: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    // = = = 1. in_between
    let res0 = bench_date_from_ymd(year, month, day);
    println!("- res0: {:?}", res0);

    // BUG FOUND?
    bench_datetime_to_string(year, month, day, hour, min, sec, nano, pst_sec);

    // assert!(start < end);
    let res1 = in_between(start, end, div);
    println!("- res1: {:?}", res1);

    let dt1 = &NaiveDate::from_ymd(year, month, day);
    let dt2 = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, min, sec)
        .unwrap();
    let dt3 = FixedOffset::east_opt(3600) // UTC+1 hour
        .unwrap()
        .ymd(year, month, day)
        .and_hms(hour, min, sec);
    let dt4 = FixedOffset::east_opt(3600)
    .unwrap().with_ymd_and_hms(year, month, day, hour, min, sec).unwrap();
    assert_eq!(dt3, dt4);

    // BUG?
    let timestamp_secs = 1_000_000_000; 
    // let timestamp_secs = 3294413016511009893;
    let dt5 = Utc.timestamp_opt(timestamp_secs, nano).unwrap(); 
    // let dt6 = Local::now();

    let res2 = num_days_from_ce_alt(dt1);
    println!("- res2: {:?}", res2);

    let res3 = bench_format_with_items(&dt1, &dt2, &dt3, &dt4, &dt5);
    println!("- res3: {:?}", res3);

    let res4 = bench_format_manual(&dt1, &dt2, &dt3, &dt4, &dt5);
    let res5 = bench_naivedate_add_signed(&dt1, &dt2, &dt3, &dt4, &dt5);
    let res6 = bench_datetime_with(&dt1, &dt2, &dt3, &dt4, &dt5);
    println!("res4: {:?}, res5: {:?}, res6: {:?}", res4, res5, res6);

    Ok(())
}

#[test]
fn my_fuzz() {
    let args: Vec<String> = std::env::args().collect();
    let mut data_arg: Option<String> = None;
    for arg in args.iter().skip(1) {
        if arg.starts_with("data=") {
            data_arg = Some(arg.chars().skip(5).collect());
            break;
        }
    }
    if let Some(data_raw) = data_arg {
        println!("\n- input data: {:?}", data_raw);
        let data: Vec<u8> = data_raw.split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect();

        let start = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
        let end = i32::from_ne_bytes([data[4], data[5], data[6], data[7]]);
        let mut div = i32::from_ne_bytes([data[8], data[9], data[10], data[11]]);
        if div == 0 { div = 10; } else if div < 0 { div = -div; }

        // let year = i8::from_ne_bytes([data[12]]);
        let year = i16::from_ne_bytes([data[12], data[13]]) % 9998;
        let year = year as i32;
        // .clamp(-9998, 9998);
        // TODO: remove (scale down)
        // let min_target: f32 = -9998.0;
        // let max_target: f32 = 9998.0;
        // let range_target: f32 = max_target - min_target;
        // let range_original: f32 = 65535.0; // (32767 - (-32768))
        // let scaled_value = (((year as f32) + 32768.0) / range_original) * range_target + min_target;
        // let year = scaled_value.round() as i32;

        let month = u32::from_ne_bytes([data[14], data[15], data[16], data[17]]) % 13; // Ensure month is 0-12
        let month = month.clamp(1, 12);
        let day = u32::from_ne_bytes([data[18], data[19], data[20], data[21]]) % 32;   // Ensure day is 0-31
        let day: u32 = day.clamp(1, 31);

        let hour = u32::from_ne_bytes([data[22], data[23], data[24], data[25]]) % 24;  // Ensure hour is 0-23
        let minute = u32::from_ne_bytes([data[26], data[27], data[28], data[29]]) % 60; // Ensure minute is 0-59    
        let second = u32::from_ne_bytes([data[30], data[31], data[32], data[33]]) % 60;
        let nanosecond = u32::from_ne_bytes([data[34], data[35], data[36], data[37]]) % 1000000000;
        
        let pst_sec = i16::from_ne_bytes([data[38], data[39]]);
        // .clamp(-86_399, 86_399);
        let pst_sec = pst_sec as i32;
        let timestamp_secs = i64::from_ne_bytes([data[40], data[41], data[42], data[43], data[44], data[45], data[46], data[47]]);
        println!("start: {}, end: {}, div: {}, \nyear: {}, month: {}, day: {}, \nhour: {}, minute: {}, second: {}, nanosecond: {}, pst_sec: {}, timestamp_secs: {}",
            start, end, div, year, month, day, hour, minute, second, nanosecond, pst_sec, timestamp_secs);

        let res = run(
            start, end, div, year, month, day,
            hour, minute, second, nanosecond,
            pst_sec,
            timestamp_secs,
        );
        println!("- Final result: {:?}", res);
    } else {
        panic!("input data not found");
    }
}

#[test]
fn quick_test() {
    let start: i32 = 0;
    let end: i32 = 100;
    let div: i32 = 10; // should be positive
    let year: i32 = 2023;
    let month: u32 = 7;
    let day: u32 = 29;
    let hour: u32 = 7;
    let min: u32 = 36;
    let sec: u32 = 0;
    let nano: u32 = 0;
    let timestamp_secs: i64 = 1_000_000_000;
    let pst_sec: i32 = 3600;

    let res = run(start, end, div, year, month, day, hour, min, sec, nano, pst_sec, timestamp_secs);
    println!("- Final result: {:?}", res);
}

// Bug cases
//cargo test --package chrono --test fuzz_test -- my_fuzz --exact --show-output data=224,172,41,159,79,95,75,149,97,147,232,146,63,167,228,242,65,211,114,4,179,33,61,199,206,100,107,113,178,172,217,0,50,194,238,173,126,1,200,17,215,53,214,123,9,53,84,85,164,157,164,162
//cargo test --package chrono --test fuzz_test -- my_fuzz --exact --show-output data=154,115,208,76,32,216,168,235,49,173,23,124,65,166,120,163,224,218,102,11,252,5,65,28,229,158,233,2,45,137,249,164,50,206,177,208,76,219,125,135,195,153,86,85,198,199,9,177,118,48,249,98