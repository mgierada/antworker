use std::fs;

use lazy_static::lazy_static;

use crate::{
    datemath::date::{get_current_month_str, get_current_year_str, get_previous_month_year_str},
    io::{
        files::get_saved_files,
        save_location::{get_save_location_invoices, get_save_location_monthly_balance},
    },
};

lazy_static! {
    static ref TEMP_DIR: tempfile::TempDir = tempfile::tempdir().unwrap();
}

#[test]
fn test_get_saved_files() {
    let temp_dir_path = TEMP_DIR.path().to_str().unwrap().to_string();
    // std::env::set_var(
    //     "ROOT_SAVE_LOCATION_PATH",
    //     Path::new("temp_dir_attachments")
    //         .to_str()
    //         .unwrap()
    //         .to_string(),
    // );
    let current_year = get_current_year_str();
    let current_month = get_current_month_str();
    let save_location = format!(
        "{}/{}/{}_{}",
        temp_dir_path.to_string().as_str(),
        current_year.as_str(),
        current_year.as_str(),
        current_month.as_str()
    );

    // Set the ROOT_SAVE_LOCATION_PATH environment variable for testing
    std::env::set_var("ROOT_SAVE_LOCATION_PATH", temp_dir_path.to_string());

    // Create some files in the temporary directory
    fs::create_dir_all(save_location.to_string()).unwrap();
    fs::write(format!("{}/file1.txt", save_location), "content1").unwrap();
    fs::write(format!("{}/file2.txt", save_location), "content2").unwrap();

    // Test the function
    let saved_files = get_saved_files();

    // Assert that the files are present in the result
    assert!(saved_files.contains(&format!("{}/file1.txt", save_location)));
    assert!(saved_files.contains(&format!("{}/file2.txt", save_location)));
}

#[test]
fn test_get_save_location_invoices() {
    std::env::set_var(
        "ROOT_SAVE_LOCATION_PATH",
        TEMP_DIR.path().to_str().unwrap().to_string(),
    );


    // std::env::set_var(
    //     "ROOT_SAVE_LOCATION_PATH",
    //     Path::new("temp_dir_attachments")
    //         .to_str()
    //         .unwrap()
    //         .to_string(),
    // );
    //
    let current_year = get_current_year_str();
    let current_month = get_current_month_str();
    let expected_save_location = format!(
        "{}/{}/{}_{}",
        TEMP_DIR.path().to_str().unwrap(),
        current_year.as_str(),
        current_year.as_str(),
        current_month.as_str()
    );
    let save_location = get_save_location_invoices();
    assert_eq!(save_location, expected_save_location);
}

#[test]
fn test_get_save_location_monthly_balance() {
    std::env::set_var("ROOT_MONTHLY_SUMMARY_BALANCE", "/root/monthly/balance");
    let save_location = get_save_location_monthly_balance();
    let (previous_month, previous_year) = get_previous_month_year_str();
    let expected_save_location = format!(
        "{}/{}/{}",
        "/root/monthly/balance".to_string(),
        previous_year.as_str(),
        previous_month.as_str()
    );
    assert_eq!(save_location, expected_save_location);
}
