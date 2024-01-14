use lazy_static::lazy_static;
use std::{env::var, fs};

use crate::datemath::date::{get_current_month_str, get_current_year_str, get_previous_month_year_str};

lazy_static! {
    pub static ref ROOT_SAVE_LOCATION_PATH: String =
        var("ROOT_SAVE_LOCATION_PATH").expect("ROOT_SAVE_LOCATION_PATH must be set.");
}

lazy_static! {
    pub static ref ROOT_MONTHLY_SUMMARY_BALANCE: String =
        var("ROOT_MONTHLY_SUMMARY_BALANCE").expect("ROOT_MONTHLY_SUMMARY_BALANCE must be set.");
}
lazy_static! {
    pub static ref MONTHLY_BALANCE_SUBJECT: String =
        var("MONTHLY_BALANCE_SUBJECT").expect("MONTHLY_BALANCE_SUBJECT must be set.");
}

pub fn get_save_location_invoices() -> String {
    let current_year = get_current_year_str();
    let current_month = get_current_month_str();
    let save_location = format!(
        "{}/{}/{}_{}",
        ROOT_SAVE_LOCATION_PATH.as_str(),
        current_year.as_str(),
        current_year.as_str(),
        current_month.as_str()
    );
    save_location
}

pub fn get_save_location_monthly_balance() -> String {
    let (previous_month, previous_year) = get_previous_month_year_str();
    let save_location = format!(
        "{}/{}/{}",
        ROOT_MONTHLY_SUMMARY_BALANCE.as_str(),
        previous_year.as_str(),
        previous_month.as_str(),
    );
    save_location
}

pub fn maybe_create_save_location(save_location: &String) -> Result<(), std::io::Error> {
    if !fs::metadata(&save_location).is_ok() {
        fs::create_dir_all(&save_location)?;
    }
    Ok(())
}

pub fn setup_save_location(subject: &String) -> Result<String, std::io::Error> {
    if subject.contains(MONTHLY_BALANCE_SUBJECT.as_str()) {
        let save_location = get_save_location_monthly_balance();
        maybe_create_save_location(&save_location).unwrap();
        return Ok(save_location);
    } else {
        let save_location = get_save_location_invoices();
        maybe_create_save_location(&save_location).unwrap();
        return Ok(save_location);
    }
}
