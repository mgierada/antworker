use std::fs;

use super::save_location::get_save_location_invoices;

const IGNORE_LIST: [&'static str; 3] = [".", "..", ".DS_Store"];

pub fn get_saved_files() -> Vec<String> {
    let save_location = get_save_location_invoices();
    println!("save_location: {}", save_location);
    let paths = fs::read_dir(save_location).unwrap();
    let mut files = Vec::new();

    for path in paths {
        let path_buf = path.unwrap().path();
        let file_name = path_buf.file_name().unwrap().to_string_lossy().to_string();
        if !IGNORE_LIST.contains(&file_name.as_str()) {
            let path_str = path_buf.to_str().unwrap().to_string();
            files.push(path_str);
        }
    }
    files
}
