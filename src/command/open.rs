use std::process::Command;

use crate::io::save_location::{get_save_location_invoices, ROOT_SAVE_LOCATION_PATH};

pub fn open_save_location_invoices(year_month: &str) {
    let dir_path: String;
    match year_month {
        "" => {
            dir_path = get_save_location_invoices().to_string();
        }
        _ => {
            let year = year_month.split("_").collect::<Vec<&str>>()[0];
            dir_path  = format!(
                "{}/{}/{}",
                ROOT_SAVE_LOCATION_PATH.as_str(),
                year,
                year_month
            );
        }
    }
    let status = Command::new("open")
        .arg(dir_path)
        .status()
        .expect("Failed to open directory");
    if !status.success() {
        eprintln!("Failed to open directory");
    }
}
