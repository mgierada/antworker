use base64::{
    engine::{self, general_purpose},
};
use dotenv::dotenv;
use lazy_static::lazy_static;

use mailparse::{self, parse_mail};
use quoted_printable::{decode, ParseMode};
use std::{env::var, fs::File, io::Write};
extern crate imap;
extern crate native_tls;

lazy_static! {
    pub static ref COMPANY_EMAIL_SERVER: String =
        var("COMPANY_EMAIL_SERVER").expect("COMPANY_EMAIL_SERVER must be set.");
}
lazy_static! {
    pub static ref COMPANY_EMAIL_PORT: u16 = var("COMPANY_EMAIL_PORT")
        .expect("COMPANY_EMAIL_PORT must be set.")
        .parse()
        .expect("COMPANY_EMAIL_PORT must be a valid u16.");
}
lazy_static! {
    pub static ref COMPANY_EMAIL: String =
        var("COMPANY_EMAIL").expect("COMPANY_EMAIL must be set.");
}
lazy_static! {
    pub static ref COMPANY_EMAIL_PASSWORD: String =
        var("COMPANY_EMAIL_PASSWORD").expect("COMPANY_EMAIL_PASSWORD must be set.");
}

async fn connect() -> imap::error::Result<Option<Vec<String>>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect(
        (COMPANY_EMAIL_SERVER.to_string(), *COMPANY_EMAIL_PORT),
        COMPANY_EMAIL_SERVER.to_string(),
        &tls,
    )
    .unwrap();

    let mut imap_session = client
        .login(
            COMPANY_EMAIL.to_string(),
            COMPANY_EMAIL_PASSWORD.to_string(),
        )
        .map_err(|e| e.0)?;

    imap_session.select("INBOX")?;

    // let messages = imap_session.fetch("RECENT", "ENVELOPE UID")?;
    let messages = imap_session.uid_fetch("15:15", "ALL")?;

    // save attachments
    for msg in messages.iter() {
        let uid = msg.uid.unwrap();
        let message_stream = imap_session.uid_fetch(uid.to_string(), "BODY[]").unwrap();
        for fetch_result in &message_stream {
            let body = fetch_result.body().unwrap();
            // Parse the MIME content
            let mail = parse_mail(body).unwrap();
            // Iterate through MIME parts
            for part in mail.subparts.iter() {
                let content_type = &part.ctype;
                println!("content_type: {:?}", content_type);

                if content_type.mimetype == "application/pdf" {
                    // Extract the filename from the Content-Type header
                    let filename = content_type
                        .params
                        .get("name")
                        .cloned()
                        .unwrap_or_else(|| format!("attachment_{}_unnamed.pdf", uid));
                    let pdf_binary = part
                        .get_body_raw()
                        .map_err(|e| eprintln!("Failed to get body raw: {}", e))
                        .expect("Failed to get body raw");
                    // Write the attachment content to a file without decoding
                    let mut file = File::create(filename.clone())
                        .map_err(|e| eprintln!("Failed to create file: {}", e))
                        .expect("Failed to create file");

                    file.write_all(&pdf_binary)
                        .map_err(|e| eprintln!("Failed to write to file: {}", e))
                        .expect("Failed to write to file");
                    println!("Attachment saved to file: {}", filename);
                }
            }
        }
    }

    let subjects: Vec<String> = messages
        .iter()
        .filter_map(|msg| {
            let envelope = msg.envelope().expect("message did not have an envelope!");
            let subject = envelope
                .subject
                .map(|subject_bytes| {
                    decode(subject_bytes, ParseMode::Robust)
                        .map(|v| String::from_utf8_lossy(&v).to_string())
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to decode subject: {}", e);
                            String::new()
                        })
                        .replace("=?UTF-8?Q?", "")
                        .replace("?= ", "")
                        .replace("_", " ")
                })
                .unwrap_or_else(|| String::new());
            Some(subject)
        })
        .collect();
    imap_session.logout()?;
    Ok(Some(subjects))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let subject = connect().await.unwrap();
    println!("subject: {:?}", subject);
}
