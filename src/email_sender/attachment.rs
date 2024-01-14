use std::fs;

use lettre::message::{header::ContentType, Attachment, SinglePart};

use crate::io::files::get_saved_files;

pub fn add_attachment(filepath: &String) -> SinglePart {
    let filename = filepath
        .split("/")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string();
    let filebody = fs::read(filepath).unwrap();
    let content_type = ContentType::parse("application/pdf").unwrap();
    Attachment::new(filename).body(filebody, content_type)
}

pub fn add_attachments() -> Vec<SinglePart> {
    let save_location = get_saved_files();
    let attachments = save_location
        .iter()
        .map(|filepath| add_attachment(filepath))
        .collect();
    attachments
}
