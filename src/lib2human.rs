#![allow(dead_code)]

// Functions used to calculate humanized date formats
pub fn year(e:i64) -> i64 {
    let year = e/31556926;
    1970 + year
}

pub fn year_since_epoch(e:i64) -> i64 {
    e/31556926
}

pub fn leap_year_from_epoch(e:i64) -> bool {
    let humanized_year = year(e);
    leap_year_from_human(humanized_year)
}

pub fn leap_year_from_human(e:i64) -> bool {
    e%4 == 0 && (e%100 != 0 || e%400 == 0)
}

// This returns a month in current year
pub fn month(e:i64) -> i64 {
    1+e/2629743%12
}

pub fn month_since_epoch(e:i64) -> i64 {
    e/2629743
}

// This returns a week in current year
pub fn week(e:i64) -> i64 {
    day_in_year(e)/7
}

pub fn week_since_epoch(e:i64) -> i64 {
    e/604800
}

// This returns a day in current month
pub fn day(e:i64) -> i64 {
    let monthdays = if leap_year_from_epoch(e) {
            match month(e) {
                1 => 0,
                2 => 31,
                3 => 60,
                4 => 91,
                5 => 121,
                6 => 152,
                7 => 182,
                8 => 213,
                9 => 244,
                10 => 274,
                11 => 305,
                12 => 335,
                _ => 0
            }
        }
        else {
            match month(e) {
                1 => 0,
                2 => 31,
                3 => 59,
                4 => 90,
                5 => 120,
                6 => 151,
                7 => 181,
                8 => 212,
                9 => 243,
                10 => 273,
                11 => 304,
                12 => 334,
                _ => 0
            }
        };
    // Substract the number of days in previous months
    day_in_year(e)-monthdays
}

pub fn day_since_epoch(e:i64) -> i64 {
        e/86400
}

pub fn day_in_year(e:i64) -> i64 {
    /*
        To calculate day in a year properly, I need to substract the number of leap years that passed since 1970
        For example: In 2023 they've been 13 of them
     */
    let passed_leap_years = year_since_epoch(e)/4;
    if leap_year_from_epoch(e) {
        1+e/86400%366-passed_leap_years
    }
    else {
        1+e/86400%365-passed_leap_years
    }
}

pub fn hour(e:i64) -> i64 {
    e/3600%24
}

pub fn min(e:i64) -> i64 {
    e/60%60
}

pub fn sec(e:i64) -> i64 {
    e%60
}

pub fn size(s:u64) -> String {
    if s >= 1099511627776 {
        format!("{}TB", s/1099511627776)
    }
    else if s >= 1073741824 {
        format!("{}GB", s/1073741824)
    }
    else if s >= 1048576 {
        format!("{}MB", s/1048576)
    }
    else if s >= 1024 {
        format!("{}KB", s/1024)
    }
    else {
        format!("{s}B")
    }
}

pub fn perms(p:u32, normal_perms:bool) -> String {
    if normal_perms {
        match p {
            0 => String::from("---"),
            1 => String::from("--x"),
            2 => String::from("-w-"),
            3 => String::from("-wx"),
            4 => String::from("r--"),
            5 => String::from("r-x"),
            6 => String::from("rw-"),
            7 => String::from("rwx"),
            _ => String::from("???"),
        }
    }
    else {
        match p {
            0 => String::from("-"),
            1 => String::from("t"),
            2 => String::from("g"),
            4 => String::from("u"),
            _ => String::from("?"),
        }
    }
    
}