use super::parser::EmailDetails;
use crate::io::save_location::setup_save_location;
use imap::Session;
use indicatif::{MultiProgress, ProgressBar, ProgressIterator, ProgressStyle};
use mailparse::{self, parse_mail, ParsedContentType, ParsedMail};
use native_tls::TlsStream;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub fn get_and_save_attachments<S: Read + Write>(
    email_details: &Vec<EmailDetails>,
    imap_session: &mut Session<TlsStream<S>>,
    multi_progress: &MultiProgress,
) -> () {
    let email_len = email_details.len();
    // Provide a custom bar style
    let pb_2 = multi_progress.add(ProgressBar::new(email_len as u64));
    pb_2.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.red}] ({pos}/{len})").unwrap(),
    );
    for email in email_details.iter().progress_with(pb_2) {
        let uid = email.uid;
        let save_location = setup_save_location(&email.subject).unwrap();
        let message_stream = imap_session.uid_fetch(uid.to_string(), "BODY[]").unwrap();
        for fetch_result in &message_stream {
            let body = fetch_result.body().unwrap();
            // Parse the MIME content
            let mail = parse_mail(body).unwrap();
            // Iterate through MIME parts
            for part in mail.subparts.iter() {
                let content_type = &part.ctype;
                match content_type.mimetype.as_str() {
                    "application/pdf" => {
                        handle_pure_pdf(uid, content_type, part, &save_location).unwrap();
                    }
                    "multipart/mixed" => {
                        handle_mixed(uid, part, &save_location).unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn handle_mixed(
    uid: u32,
    part: &ParsedMail,
    save_location: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    for sub_part in part.subparts.iter() {
        let sub_part_content_type = &sub_part.ctype;
        match sub_part_content_type.mimetype.as_str() {
            "application/pdf" => {
                handle_pure_pdf(uid, sub_part_content_type, sub_part, save_location).unwrap();
            }
            _ => {}
        }
    }
    Ok(())
}

fn handle_pure_pdf(
    uid: u32,
    content_type: &ParsedContentType,
    part: &ParsedMail,
    save_location: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = content_type
        .params
        .get("name")
        .cloned()
        .unwrap_or_else(|| format!("attachment_{}_unnamed.pdf", uid));
    let full_path_save_location = Path::new(save_location).join(&filename);
    let binary_content = part
        .get_body_raw()
        .map_err(|e| eprintln!("Failed to get body raw: {}", e))
        .expect("Failed to get body raw");
    let mut file = File::create(full_path_save_location.clone())
        .map_err(|e| eprintln!("Failed to create file: {}", e))
        .expect("Failed to create file");
    file.write_all(&binary_content)
        .map_err(|e| eprintln!("Failed to write to file: {}", e))
        .expect("Failed to write to file");
    Ok(())
}
