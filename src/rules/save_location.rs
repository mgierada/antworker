use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use std::env::var;

lazy_static! {
    pub static ref ROOT_SAVE_LOCATION_PATH : String =
        var("ROOT_SAVE_LOCATION_PATH").expect("ROOT_SAVE_LOCATION_PATH must be set.");
}

pub fn get_save_loaction() -> String {
    let save_location = format!(
        "{}/{}",
        ROOT_SAVE_LOCATION_PATH.as_str(),
        get_current_year()
    );
    save_location
}

fn get_current_year() -> String {
    let now: DateTime<Utc> = chrono::Utc::now();
    now.format("%Y").to_string()
}
