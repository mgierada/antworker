use lazy_static::lazy_static;
use std::{env::var, fs};

use crate::datemath::date::{get_current_month_str, get_current_year_str};

lazy_static! {
    pub static ref ROOT_SAVE_LOCATION_PATH: String =
        var("ROOT_SAVE_LOCATION_PATH").expect("ROOT_SAVE_LOCATION_PATH must be set.");
}

pub fn get_save_location() -> String {
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

fn maybe_create_save_location(save_location: &String) -> Result<(), std::io::Error> {
    if !fs::metadata(&save_location).is_ok() {
        fs::create_dir_all(&save_location)?;
    }
    Ok(())
}

pub fn setup_save_location() -> Result<String, std::io::Error> {
    let save_location = get_save_location();
    maybe_create_save_location(&save_location).unwrap();
    Ok(save_location)
}
