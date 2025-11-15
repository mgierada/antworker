use crate::datemath::date::{
    get_current_month_str, get_current_month_year, get_current_year_month_str,
    get_current_year_str, get_previous_month_year, get_previous_month_year_str,
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

#[test]
fn test_get_current_year_month_str() {
    let current_year_month = get_current_year_month_str();
    let now: DateTime<Utc> = Utc::now();
    let expected_year_month = now.format("%Y_%m").to_string();
    assert_eq!(current_year_month, expected_year_month);
}

#[test]
fn test_get_current_year_month_str_format() {
    let current_year_month = get_current_year_month_str();
    // Should contain underscore separator
    assert!(current_year_month.contains("_"));
    // Should be in format YYYY_MM (length 7)
    assert_eq!(current_year_month.len(), 7);
    // Should start with a 4-digit year
    let parts: Vec<&str> = current_year_month.split('_').collect();
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].len(), 4); // year part
    assert_eq!(parts[1].len(), 2); // month part
}

#[test]
fn test_month_formats_consistency() {
    let month_str = get_current_month_str();
    let (_year, month) = get_current_month_year().unwrap();

    // Month string should be zero-padded 2 digits
    assert_eq!(month_str.len(), 2);

    // Month string should match the month from get_current_month_year
    let expected_month_str = format!("{:02}", month);
    assert_eq!(month_str, expected_month_str);
}

#[test]
fn test_year_formats_consistency() {
    let year_str = get_current_year_str();
    let (year, _) = get_current_month_year().unwrap();

    // Year string should be 4 digits
    assert_eq!(year_str.len(), 4);

    // Year string should match the year from get_current_month_year
    assert_eq!(year_str, year.to_string());
}

#[test]
fn test_previous_month_boundary() {
    // Test that previous month calculation is consistent
    let (prev_year, prev_month) = get_previous_month_year();
    let (prev_month_str, prev_year_str) = get_previous_month_year_str();

    // Verify consistency between the two functions
    assert_eq!(prev_year.to_string(), prev_year_str);
    assert_eq!(format!("{:02}", prev_month), prev_month_str);

    // Verify month is in valid range
    assert!((1..=12).contains(&prev_month));
}

#[test]
fn test_current_month_year_returns_some() {
    let result = get_current_month_year();
    assert!(result.is_some());
    let (year, month) = result.unwrap();
    // Verify reasonable ranges
    assert!((2020..=2100).contains(&year));
    assert!((1..=12).contains(&month));
}
