use std::fs;

use crate::{io::files::get_saved_files, datemath::date::{get_current_year_str, get_current_month_str}};


#[test]
fn test_get_saved_files() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path().to_str().unwrap().to_string();
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
    std::env::set_var("ROOT_SAVE_LOCATION_PATH", temp_dir_path.to_string());   // Create a temporary directory for testing

    // Create some files in the temporary directory
    fs::create_dir_all(save_location.to_string()).unwrap();
    fs::write(format!("{}/file1.txt", save_location), "content1").unwrap();
    fs::write(format!("{}/file2.txt", save_location), "content2").unwrap();

    // Test the function
    let saved_files = get_saved_files();

    // Assert that the files are present in the result
    assert!(saved_files.contains(&format!("{}/file1.txt", save_location)));
    assert!(saved_files.contains(&format!("{}/file2.txt", save_location)));

    // Cleanup: Remove the temporary directory
    temp_dir.close().unwrap();
}
