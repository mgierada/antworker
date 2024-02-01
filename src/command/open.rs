use crate::io::save_location::{
    get_save_location_income_invoices, get_save_location_outcome_invoices,
    ROOT_SAVE_LOCATION_INCOME_INVOICES, ROOT_SAVE_LOCATION_OUTCOME_INVOICES,
};
use std::process::Command;

pub fn open_save_location_invoices(year_month_or_year: &str, is_income: bool) {
    let dir_path = match is_income {
        true => {
            let dir_path = match year_month_or_year {
                "" => get_save_location_income_invoices().to_string(),
                _ => format!(
                    "{}/{}",
                    ROOT_SAVE_LOCATION_INCOME_INVOICES.as_str(),
                    year_month_or_year
                ),
            };
            dir_path
        }
        false => {
            let dir_path = match year_month_or_year {
                "" => get_save_location_outcome_invoices().to_string(),
                _ => {
                    let year = year_month_or_year.split("_").next().unwrap();
                    format!(
                        "{}/{}/{}",
                        ROOT_SAVE_LOCATION_OUTCOME_INVOICES.as_str(),
                        year,
                        year_month_or_year
                    )
                }
            };
            dir_path
        }
    };
    let status = Command::new("open")
        .arg(dir_path.as_str())
        .status()
        .expect("Failed to open directory");

    if !status.success() {
        eprintln!("Failed to open directory");
    }
}
