use crate::datemath::date::{
    get_current_month_str, get_current_month_year, get_current_year_str, get_previous_month_year,
    get_previous_month_year_str,
};
use chrono::{DateTime, Datelike, Duration, Utc};

#[test]
fn test_get_current_year_str() {
    let current_year = get_current_year_str();
    let now: DateTime<Utc> = Utc::now();
    let expected_year = now.format("%Y").to_string();
    assert_eq!(current_year, expected_year);
}

#[test]
fn test_get_current_month_str() {
    let current_month = get_current_month_str();
    let now: DateTime<Utc> = Utc::now();
    let expected_month = now.format("%m").to_string();
    assert_eq!(current_month, expected_month);
}

#[test]
fn test_get_current_month_year() {
    let current_month_year = get_current_month_year().unwrap();
    let now: DateTime<Utc> = Utc::now();
    let expected_month_year = (now.year(), now.month());
    assert_eq!(current_month_year, expected_month_year);
}

#[test]
fn test_get_previous_month_year() {
    let (previous_year, previous_month) = get_previous_month_year();
    let now: DateTime<Utc> = Utc::now();
    let expected_year_month = now
        .checked_sub_signed(Duration::days(now.day() as i64))
        .and_then(|dt| dt.checked_sub_signed(Duration::days(1)))
        .map(|dt| (dt.year(), dt.month()))
        .unwrap_or_else(|| (now.year(), now.month()));
    assert_eq!((previous_year, previous_month), expected_year_month);
}

#[test]
fn test_get_previous_month_year_str() {
    let (previous_month_str, previous_year_str) = get_previous_month_year_str();
    let (previous_year, previous_month) = get_previous_month_year();
    let expected_month_str = format!("{:02}", previous_month);
    let expected_year_str = previous_year.to_string();
    assert_eq!(previous_month_str, expected_month_str);
    assert_eq!(previous_year_str, expected_year_str);
}
