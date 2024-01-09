use std::fs;

use super::save_location::get_save_location;

pub fn get_saved_files() -> Vec<String> {
    let save_location = get_save_location();
    let paths = fs::read_dir(save_location).unwrap();
    let mut files = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap().to_string();
        files.push(path);
    }
    files
}
