use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use std::{env::var, fs};

lazy_static! {
    pub static ref ROOT_SAVE_LOCATION_PATH: String =
        var("ROOT_SAVE_LOCATION_PATH").expect("ROOT_SAVE_LOCATION_PATH must be set.");
}

fn get_save_loaction() -> String {
    let current_year = get_current_year();
    let current_month = get_current_month();
    let save_location = format!(
        "{}/{}/{}_{}",
        ROOT_SAVE_LOCATION_PATH.as_str(),
        current_year.as_str(),
        current_year.as_str(),
        current_month.as_str()
    );
    save_location
}

fn get_current_year() -> String {
    let now: DateTime<Utc> = chrono::Utc::now();
    now.format("%Y").to_string()
}

fn get_current_month() -> String {
    let now: DateTime<Utc> = chrono::Utc::now();
    now.format("%m").to_string()
}

fn maybe_create_save_location(save_location: &String) -> Result<(), std::io::Error> {
    if !fs::metadata(&save_location).is_ok() {
        fs::create_dir_all(&save_location)?;
    }
    Ok(())
}

pub fn setup() -> Result<(), std::io::Error> {
    let save_location = get_save_loaction();
    println!("save_location: {:?}", save_location);
    maybe_create_save_location(&save_location).unwrap();
    Ok(())
}
