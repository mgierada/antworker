use imap::{
    types::{Fetch, ZeroCopy},
    Session,
};
use mailparse::{self, parse_mail};
use native_tls::TlsStream;
use quoted_printable::{decode, ParseMode};
use std::{fs::File, io::Write};

use crate::{COMPANY_EMAIL, COMPANY_EMAIL_PASSWORD, COMPANY_EMAIL_PORT, COMPANY_EMAIL_SERVER};

pub async fn connect() -> imap::error::Result<Session<TlsStream<std::net::TcpStream>>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect(
        (COMPANY_EMAIL_SERVER.to_string(), *COMPANY_EMAIL_PORT),
        COMPANY_EMAIL_SERVER.to_string(),
        &tls,
    )
    .unwrap();
    let imap_session = client
        .login(
            COMPANY_EMAIL.to_string(),
            COMPANY_EMAIL_PASSWORD.to_string(),
        )
        .map_err(|e| e.0)?;
    Ok(imap_session)
}

pub fn fetch_emails<S: std::io::Read + std::io::Write>(
    imap_session: &mut Session<TlsStream<S>>,
) -> imap::error::Result<ZeroCopy<Vec<Fetch>>> {
    imap_session.select("INBOX")?;
    // let messages = imap_session.fetch("RECENT", "ENVELOPE UID")?;
    let messages = imap_session.uid_fetch("23:23", "ALL")?;
    Ok(messages)
}

pub fn save_attachments<S: std::io::Read + std::io::Write>(
    messages: &ZeroCopy<Vec<Fetch>>,
    imap_session: &mut Session<TlsStream<S>>,
) -> imap::error::Result<Option<Vec<String>>> {
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
    Ok(None)
}

pub fn get_subjects(messages: &ZeroCopy<Vec<Fetch>>) -> imap::error::Result<Option<Vec<String>>> {
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
    // imap_session.logout()?;
    Ok(Some(subjects))
}

pub async fn process_emails() -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
    let mut imap_session = connect().await?;
    let messages = fetch_emails(&mut imap_session)?;
    let subjects = get_subjects(&messages)?;
    save_attachments(&messages, &mut imap_session);
    imap_session.logout()?;
    Ok(subjects)
}
