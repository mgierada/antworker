use chrono::{DateTime, Utc, Datelike};

pub fn get_current_year_str() -> String {
    let now: DateTime<Utc> = chrono::Utc::now();
    now.format("%Y").to_string()
}

pub fn get_current_month_str() -> String {
    let now: DateTime<Utc> = chrono::Utc::now();
    now.format("%m").to_string()
}

pub fn get_current_month_year() -> Option<(i32, u32)> {
    let now: DateTime<Utc> = chrono::Utc::now();
    return Some((now.year(), now.month()));
}
