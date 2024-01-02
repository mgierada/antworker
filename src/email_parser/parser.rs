use imap::{
    types::{Fetch, ZeroCopy},
    Session,
};
use mailparse::{self, parse_mail};
use native_tls::TlsStream;
use quoted_printable::{decode, ParseMode};
use std::{fs::File, io::Write};

use crate::{COMPANY_EMAIL, COMPANY_EMAIL_PASSWORD, COMPANY_EMAIL_PORT, COMPANY_EMAIL_SERVER};

#[derive(Debug, Default)]
pub struct EmailDetails {
    pub date: Option<String>,
    pub subject: String,
    pub email: Vec<String>,
}

fn get_email_details(messages: &ZeroCopy<Vec<Fetch>>) -> imap::error::Result<Vec<EmailDetails>> {
    let email_details: Vec<EmailDetails> = messages
        .iter()
        .filter_map(|msg| {
            let envelope = msg.envelope().expect("message did not have an envelope!");
            // NOTE: Extract subject
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
            // NOTE: Extract date
            let date = envelope
                .date
                .map(|date_bytes| String::from_utf8_lossy(&date_bytes).to_string());
            // NOTE: Extract email
            let email: Vec<String> =
                envelope
                    .from
                    .as_ref()
                    .map_or_else(Vec::new, |from_addresses| {
                        from_addresses
                            .iter()
                            .map(|address| {
                                format!(
                                    "{}@{}",
                                    String::from_utf8_lossy(address.mailbox.unwrap_or_default()).to_string(),
                                    String::from_utf8_lossy(address.host.unwrap_or_default()).to_string()
                                )
                            })
                            .collect()
                    });

            Some(EmailDetails {
                date,
                subject,
                email,
            })
        })
        .collect();

    Ok(email_details)
}

async fn connect() -> imap::error::Result<Session<TlsStream<std::net::TcpStream>>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect(
        (COMPANY_EMAIL_SERVER.to_string(), *COMPANY_EMAIL_PORT),
        COMPANY_EMAIL_SERVER.to_string(),
        &tls,
    )?;
    let imap_session = client
        .login(
            COMPANY_EMAIL.to_string(),
            COMPANY_EMAIL_PASSWORD.to_string(),
        )
        .map_err(|e| e.0)?;
    Ok(imap_session)
}

fn fetch_emails<S: std::io::Read + std::io::Write>(
    imap_session: &mut Session<TlsStream<S>>,
    uid_set: &str,
) -> imap::error::Result<ZeroCopy<Vec<Fetch>>> {
    imap_session.select("INBOX")?;
    let messages = imap_session.uid_fetch(&uid_set, "ALL")?;
    Ok(messages)
}

pub fn save_attachments<S: std::io::Read + std::io::Write>(
    messages: &ZeroCopy<Vec<Fetch>>,
    imap_session: &mut Session<TlsStream<S>>,
) -> imap::error::Result<()> {
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
                match content_type.mimetype.as_str() {
                    "application/pdf" => {
                        let filename = content_type
                            .params
                            .get("name")
                            .cloned()
                            .unwrap_or_else(|| format!("attachment_{}_unnamed.pdf", uid));
                        let binary_content = part
                            .get_body_raw()
                            .map_err(|e| eprintln!("Failed to get body raw: {}", e))
                            .expect("Failed to get body raw");
                        let mut file = File::create(filename.clone())
                            .map_err(|e| eprintln!("Failed to create file: {}", e))
                            .expect("Failed to create file");
                        file.write_all(&binary_content)
                            .map_err(|e| eprintln!("Failed to write to file: {}", e))
                            .expect("Failed to write to file");
                        println!("Attachment saved to file: {}", filename);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

pub async fn process_emails() -> Result<Vec<EmailDetails>, Box<dyn std::error::Error>> {
    let mut imap_session = connect().await?;
    let uid_set = "23:23";
    let messages = fetch_emails(&mut imap_session, &uid_set)?;
    // let subjects = get_subjects(&messages)?;
    let get_email_details = get_email_details(&messages)?;
    // save_attachments(&messages, &mut imap_session)?;
    imap_session.logout()?;
    Ok(get_email_details)
}
