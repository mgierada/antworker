use std::fs;

use lazy_static::lazy_static;

use crate::{
    datemath::date::{
        get_current_month_str, get_current_year_month_str, get_current_year_str,
        get_previous_month_year_str,
    },
    io::{
        files::get_saved_files,
        save_location::{
            get_save_location_income_invoices, get_save_location_monthly_balance,
            get_save_location_outcome_invoices, maybe_create_save_location,
        },
    },
};

lazy_static! {
    static ref TEMP_DIR: tempfile::TempDir = tempfile::tempdir().unwrap();
}

#[test]
fn test_get_saved_files() {
    let temp_dir_path = TEMP_DIR.path().to_str().unwrap().to_string();
    let current_year = get_current_year_str();
    let current_month = get_current_month_str();
    let save_location = format!(
        "{}/{}/{}_{}",
        temp_dir_path.to_string().as_str(),
        current_year.as_str(),
        current_year.as_str(),
        current_month.as_str()
    );

    // Set the ROOT_SAVE_LOCATION_OUTCOME_INVOICES  environment variable for testing
    std::env::set_var(
        "ROOT_SAVE_LOCATION_OUTCOME_INVOICES",
        &temp_dir_path,
    );

    // Create some files in the temporary directory
    fs::create_dir_all(&save_location).unwrap();
    fs::write(format!("{}/file1.txt", save_location), "content1").unwrap();
    fs::write(format!("{}/file2.txt", save_location), "content2").unwrap();

    // Test the function
    let saved_files = get_saved_files();

    // Assert that the files are present in the result
    assert!(saved_files.contains(&format!("{}/file1.txt", save_location)));
    assert!(saved_files.contains(&format!("{}/file2.txt", save_location)));
}

#[test]
fn test_get_saved_files_filters_ds_store() {
    let temp_dir_path = TEMP_DIR.path().to_str().unwrap().to_string();
    let current_year = get_current_year_str();
    let current_year_month = get_current_year_month_str();
    let save_location = format!("{}/{}/{}", temp_dir_path, current_year, current_year_month);

    std::env::set_var(
        "ROOT_SAVE_LOCATION_OUTCOME_INVOICES",
        &temp_dir_path,
    );

    // Create files including .DS_Store
    fs::create_dir_all(&save_location).unwrap();
    fs::write(format!("{}/file1.txt", save_location), "content1").unwrap();
    fs::write(format!("{}/.DS_Store", save_location), "mac_stuff").unwrap();

    let saved_files = get_saved_files();

    // Should contain file1.txt but not .DS_Store
    assert!(saved_files.iter().any(|f| f.contains("file1.txt")));
    assert!(!saved_files.iter().any(|f| f.contains(".DS_Store")));
}

#[test]
fn test_files_filtering_logic() {
    // Test the filtering logic directly with a temp directory
    let unique_temp_dir = tempfile::tempdir().unwrap();
    let test_dir = unique_temp_dir.path();

    // Create test files including ones that should be ignored
    fs::create_dir_all(test_dir).unwrap();
    fs::write(test_dir.join("file1.pdf"), "content1").unwrap();
    fs::write(test_dir.join("file2.txt"), "content2").unwrap();
    fs::write(test_dir.join("file3.pdf"), "content3").unwrap();
    fs::write(test_dir.join(".DS_Store"), "mac_stuff").unwrap();
    fs::write(test_dir.join("file4.doc"), "content4").unwrap();

    // Read and filter files similar to get_saved_files logic
    let paths = fs::read_dir(test_dir).unwrap();
    let mut files = Vec::new();
    const IGNORE_LIST: [&str; 3] = [".", "..", ".DS_Store"];

    for path in paths {
        let path_buf = path.unwrap().path();
        let file_name = path_buf.file_name().unwrap().to_string_lossy().to_string();
        if !IGNORE_LIST.contains(&file_name.as_str()) {
            files.push(file_name);
        }
    }

    // Should have 4 files (excluding .DS_Store)
    assert_eq!(files.len(), 4);
    assert!(!files.iter().any(|f| f.contains(".DS_Store")));
    assert!(files.iter().any(|f| f.contains("file1.pdf")));
    assert!(files.iter().any(|f| f.contains("file2.txt")));
    assert!(files.iter().any(|f| f.contains("file3.pdf")));
    assert!(files.iter().any(|f| f.contains("file4.doc")));
}

#[test]
fn test_get_save_location_outcome_invoices() {
    std::env::set_var(
        "ROOT_SAVE_LOCATION_OUTCOME_INVOICES",
        TEMP_DIR.path().to_str().unwrap(),
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
    let save_location = get_save_location_outcome_invoices();
    assert_eq!(save_location, expected_save_location);
}

#[test]
fn test_get_save_location_income_invoices() {
    std::env::set_var(
        "ROOT_SAVE_LOCATION_INCOME_INVOICES",
        TEMP_DIR.path().to_str().unwrap(),
    );
    let current_year = get_current_year_str();
    let expected_save_location = format!(
        "{}/{}",
        TEMP_DIR.path().to_str().unwrap(),
        current_year.as_str(),
    );
    let save_location = get_save_location_income_invoices();
    assert_eq!(save_location, expected_save_location);
}

#[test]
fn test_get_save_location_monthly_balance() {
    std::env::set_var("ROOT_MONTHLY_SUMMARY_BALANCE", "/root/monthly/balance");
    let save_location = get_save_location_monthly_balance();
    let (previous_month, previous_year) = get_previous_month_year_str();
    let expected_save_location = format!(
        "{}/{}/{}",
        "/root/monthly/balance",
        previous_year.as_str(),
        previous_month.as_str()
    );
    assert_eq!(save_location, expected_save_location);
}

#[test]
fn test_maybe_create_save_location_new_directory() {
    let temp_dir_path = TEMP_DIR.path().to_str().unwrap().to_string();
    let new_dir = format!("{}/new_test_dir", temp_dir_path);

    // Ensure the directory doesn't exist
    let _ = fs::remove_dir_all(&new_dir);

    // Test creating a new directory
    let result = maybe_create_save_location(&new_dir);
    assert!(result.is_ok());
    assert!(fs::metadata(&new_dir).is_ok());

    // Cleanup
    let _ = fs::remove_dir_all(&new_dir);
}

#[test]
fn test_maybe_create_save_location_existing_directory() {
    let temp_dir_path = TEMP_DIR.path().to_str().unwrap().to_string();
    let existing_dir = format!("{}/existing_test_dir", temp_dir_path);

    // Create the directory first
    fs::create_dir_all(&existing_dir).unwrap();

    // Test with existing directory (should not error)
    let result = maybe_create_save_location(&existing_dir);
    assert!(result.is_ok());
    assert!(fs::metadata(&existing_dir).is_ok());

    // Cleanup
    let _ = fs::remove_dir_all(&existing_dir);
}

#[test]
fn test_get_save_location_outcome_with_year_month() {
    std::env::set_var(
        "ROOT_SAVE_LOCATION_OUTCOME_INVOICES",
        TEMP_DIR.path().to_str().unwrap(),
    );

    let save_location = get_save_location_outcome_invoices();
    let current_year = get_current_year_str();
    let current_year_month = get_current_year_month_str();

    assert!(save_location.contains(&current_year));
    assert!(save_location.contains(&current_year_month));
    let expected_format = format!(
        "{}/{}/{}",
        TEMP_DIR.path().to_str().unwrap(),
        current_year,
        current_year_month
    );
    assert_eq!(save_location, expected_format);
}
