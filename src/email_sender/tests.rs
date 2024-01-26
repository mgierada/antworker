// use std::path::Path;
// use std::{fs, path::PathBuf};
//
// // use lazy_static::lazy_static;
//
// use lazy_static::lazy_static;
// use tempfile::tempdir;
//
// use crate::{
//     datemath::date::{get_current_month_str, get_current_year_str},
//     email_sender::attachment::{add_attachment, add_attachments},
// };
//
// lazy_static! {
//     static ref TEMP_DIR_ATTACHMENTS: tempfile::TempDir = tempfile::tempdir().unwrap();
// }

// #[test]
// fn test_add_attachments() {
//     // Create a temporary directory and some sample files for testing
//     let temp_dir = tempdir().expect("Failed to create temporary directory");
//     let temp_dir_path = temp_dir.path().to_str().unwrap().to_string();
//     // let temp_dir_path = TEMP_DIR_ATTACHMENTS.path().to_str().unwrap().to_string();
//     // let temp_dir_path = Path::new("temp_dir_attachments").to_str().unwrap().to_string();
//     let current_year = get_current_year_str();
//     let current_month = get_current_month_str();
//     let save_location = format!(
//         "{}/{}/{}_{}",
//         temp_dir_path.to_string().as_str(),
//         current_year.as_str(),
//         current_year.as_str(),
//         current_month.as_str()
//     );
//     println!("temp_dir_path: {}", temp_dir_path);
//
//     // Set the ROOT_SAVE_LOCATION_PATH environment variable for testing
//     // std::env::set_var("ROOT_SAVE_LOCATION_PATH", temp_dir_path);
//     std::env::set_var(
//         "ROOT_SAVE_LOCATION_PATH",
//         Path::new("temp_dir_attachments")
//             .to_str()
//             .unwrap()
//             .to_string(),
//     );
//
//     let file_paths = [
//         PathBuf::from(save_location.clone()).join("file1.pdf"),
//         PathBuf::from(save_location.clone()).join("file2.pdf"),
//     ];
//
//     // create the save location
//     fs::create_dir_all(save_location.to_string()).unwrap();
//
//     // Create empty files to simulate attachments
//     for path in &file_paths {
//         fs::write(path, "").unwrap();
//     }
//
//     // Call add_attachments function
//     let attachments = add_attachments();
//
//     // Check if the number of attachments matches the number of files in the temporary directory
//     assert_eq!(attachments.len(), file_paths.len());
//
//     std::env::remove_var("ROOT_SAVE_LOCATION_PATH");
// }
