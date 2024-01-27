use std::process::Command;

use crate::io::save_location::get_save_location_invoices;

pub fn open_save_location_invoices() {
    let dir_path = get_save_location_invoices();
    let status = Command::new("open")
        .arg(dir_path)
        .status()
        .expect("Failed to open directory");
    if !status.success() {
        eprintln!("Failed to open directory");
    }
}
