use chrono::{DateTime, Utc, Datelike};

pub fn get_current_year_str() -> String {
    let now: DateTime<Utc> = chrono::Utc::now();
    now.format("%Y").to_string()
}

pub fn get_current_month_str() -> String {
    let now: DateTime<Utc> = chrono::Utc::now();
    now.format("%m").to_string()
}

pub fn get_current_year_month_str() -> String {
    let now: DateTime<Utc> = chrono::Utc::now();
    now.format("%Y_%m").to_string()
}

pub fn get_current_month_year() -> Option<(i32, u32)> {
    let now: DateTime<Utc> = chrono::Utc::now();
    return Some((now.year(), now.month()));
}

pub fn get_previous_month_year() -> (i32, u32) {
    let now: DateTime<Utc> = Utc::now();
    let (year, month) = now
        .checked_sub_signed(chrono::Duration::days(now.day() as i64))
        .and_then(|dt| dt.checked_sub_signed(chrono::Duration::days(1)))
        .map(|dt| (dt.year(), dt.month()))
        .unwrap_or_else(|| (now.year(), now.month()));
    (year, month)
}

pub fn get_previous_month_year_str() -> (String, String) {
    let (previous_year, previous_month) = get_previous_month_year();
    let previous_month_str = format!("{:02}", previous_month);
    let previous_year_str = previous_year.to_string();
    (previous_month_str, previous_year_str)
}
