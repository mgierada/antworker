use std::process::Command;

use crate::io::save_location::get_save_location_invoices;

pub fn open_save_location_invoices() {
    // Replace "path/to/your/directory" with the actual path to the directory you want to open
    let dir_path = get_save_location_invoices();

    // Execute the "open" command with the directory path
    let status = Command::new("open")
        .arg(dir_path)
        .status()
        .expect("Failed to open directory");

    if !status.success() {
        eprintln!("Failed to open directory");
    }
}

