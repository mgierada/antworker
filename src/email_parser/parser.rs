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
    // pub from: Option<String>,
    // pub mailbox: Option<String>,
    // pub sender: Option<String>,
    // pub reply_to: Option<String>,
}

pub fn get_email_details(messages: &ZeroCopy<Vec<Fetch>>) -> imap::error::Result<Vec<EmailDetails>> {
    let email_details: Vec<EmailDetails> = messages
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

            let email_detail = EmailDetails {
                date: envelope.date.map(|date_bytes| {
                    String::from_utf8_lossy(&date_bytes).to_string()
                }),
                subject,
                // from: envelope.from.map(|addresses| stringify_address(&addresses)),
                // mailbox: envelope.to.and_then(|addresses| {
                //     addresses
                //         .iter()
                //         .next()
                //         .and_then(|addr| addr.mailbox.as_ref().map(|mb| String::from_utf8_lossy(mb).to_string()))
                // }),
                // sender: envelope.sender.map(|addresses| stringify_address(&addresses)),
                // reply_to: envelope.reply_to.map(|addresses| stringify_address(&addresses)),
                // Add more fields as needed
            };

            Some(email_detail)
        })
        .collect();

    Ok(email_details)
}

// fn stringify_address(addresses: &[Address]) -> String {
//     addresses
//         .iter()
//         .map(|addr| {
//             addr.mailbox
//                 .as_ref()
//                 .map_or_else(|| String::new(), |mb| String::from_utf8_lossy(mb).to_string())
//         })
//         .collect::<Vec<String>>()
//         .join(", ")
// }

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
    uid_set: &str,
) -> imap::error::Result<ZeroCopy<Vec<Fetch>>> {
    imap_session.select("INBOX")?;
    // let messages = imap_session.fetch("RECENT", "ENVELOPE UID")?;
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
