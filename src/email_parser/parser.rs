use chrono::{DateTime, Utc};
use imap::{
    types::{Fetch, ZeroCopy},
    Session,
};
use mailparse::{self, parse_mail};
use native_tls::TlsStream;
use quoted_printable::{decode, ParseMode};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::{
    io::save_location::setup_save_location,
    rules::define::{define_rules, FilterRules},
    COMPANY_EMAIL, COMPANY_EMAIL_PASSWORD, COMPANY_EMAIL_PORT, COMPANY_EMAIL_SERVER, PRIVATE_EMAIL,
    PRIVATE_EMAIL_PASSWORD, factories::credentials::{Credentials, EmailAccountBuilder}, S_EMAIL, S_EMAIL_PASSWORD,
};

#[derive(Debug, Default)]
pub struct EmailDetails {
    pub subject: String,
    pub from: Vec<String>,
    pub date: DateTime<Utc>,
    pub uid: u32,
}


async fn connect(
    credentials: &Credentials,
) -> imap::error::Result<Session<TlsStream<std::net::TcpStream>>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect(
        (&*credentials.server, credentials.port), // Pass a reference to credentials.server
        &credentials.server,                      // Pass a reference to credentials.server
        &tls,
    )?;
    let imap_session = client
        .login(
            &credentials.email,    // Pass a reference to credentials.email
            &credentials.password, // Pass a reference to credentials.password
        )
        .map_err(|e| e.0)?;
    Ok(imap_session)
}

fn fetch_emails<S: Read + Write>(
    imap_session: &mut Session<TlsStream<S>>,
    uid_set: &str,
) -> imap::error::Result<ZeroCopy<Vec<Fetch>>> {
    imap_session.select("INBOX")?;
    let messages = imap_session.uid_fetch(&uid_set, "ALL")?;
    Ok(messages)
}

pub fn parse_date(date_str: &str) -> DateTime<Utc> {
    // HACK: This is kind of a quacky solution.
    // Attempt to parse the date string using RFC 2822 format
    let date = DateTime::parse_from_rfc2822(date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|_| {
            // If parsing with RFC 2822 fails, try a custom format with full timezone name
            let date_format = "%a %b %d %H:%M:%S %Z %Y";
            DateTime::parse_from_str(date_str, date_format).map(|dt| dt.with_timezone(&Utc))
        })
        .unwrap_or_else(|_| {
            // If all parsing attempts fail, fallback to current time in UTC
            Utc::now()
        });
    date
}

fn get_email_details(
    messages: &ZeroCopy<Vec<Fetch>>,
    rules: &FilterRules,
) -> imap::error::Result<Vec<EmailDetails>> {
    let email_details: Vec<EmailDetails> = messages
        .iter()
        .filter_map(|msg| {
            // Apply filtering rules
            if !rules.is_empty() && !rules.matches(msg) {
                return None;
            }
            let uid = msg.uid.unwrap();
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
            let raw_date = envelope
                .date
                .map(|date_bytes| String::from_utf8_lossy(&date_bytes).to_string());
            let date = parse_date(&raw_date.unwrap_or_else(|| String::new()));
            // NOTE: Extract email
            let from: Vec<String> =
                envelope
                    .from
                    .as_ref()
                    .map_or_else(Vec::new, |from_addresses| {
                        from_addresses
                            .iter()
                            .map(|address| {
                                format!(
                                    "{}@{}",
                                    String::from_utf8_lossy(address.mailbox.unwrap_or_default())
                                        .to_string(),
                                    String::from_utf8_lossy(address.host.unwrap_or_default())
                                        .to_string()
                                )
                            })
                            .collect()
                    });
            Some(EmailDetails {
                date,
                subject,
                from,
                uid,
            })
        })
        .collect();
    Ok(email_details)
}

fn get_and_save_attachments<S: Read + Write>(
    email_details: &Vec<EmailDetails>,
    imap_session: &mut Session<TlsStream<S>>,
    save_location: &String,
) -> () {
    for email in email_details.iter() {
        let uid = email.uid;
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
                        println!(
                            "Attachment saved to file: {}",
                            full_path_save_location.to_str().unwrap()
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

// Define a function to fetch emails for a given set of credentials
async fn process_inbox(
    credentials: &Credentials,
) -> Result<Vec<EmailDetails>, Box<dyn std::error::Error>> {
    let mut imap_session = connect(credentials).await?;
    // let uid_set = "1:*";
    let messages = fetch_emails(&mut imap_session, &credentials.uid_set)?;
    let rules = define_rules();
    let email_details = get_email_details(&messages, &rules)?;
    let save_location = setup_save_location()?;
    get_and_save_attachments(&email_details, &mut imap_session, &save_location);
    imap_session.logout()?;
    Ok(email_details)
}

// Define a function to process emails for multiple inboxes
pub async fn process_all_inboxes(
    inboxes: HashMap<&str, Credentials>,
) -> Result<Vec<EmailDetails>, Box<dyn std::error::Error>> {
    let mut all_email_details = Vec::new();

    for (inbox_name, credentials) in inboxes.iter() {
        println!("Processing inbox: {}", inbox_name);
        let email_details = process_inbox(credentials).await?;
        all_email_details.extend(email_details);
    }
    Ok(all_email_details)
}

pub async fn process_emails() -> Result<(), Box<dyn std::error::Error>> {
    let mut inboxes = HashMap::new();

    let company_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &COMPANY_EMAIL,
        &COMPANY_EMAIL_PASSWORD,
    )
    .uid_set("1:*")
    .build();

    let private_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &PRIVATE_EMAIL,
        &PRIVATE_EMAIL_PASSWORD,
    )
    .uid_set("6000:*")
    .build();
    
    let s_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &S_EMAIL,
        &S_EMAIL_PASSWORD,
    )
    .uid_set("6000:*")
    .build();

    inboxes.insert("company", company_credentials);
    inboxes.insert("private", private_credentials);
    inboxes.insert("s", s_credentials);

    // Process emails for all inboxes
    if let Ok(email_details) = process_all_inboxes(inboxes).await {
        println!("All emails processed successfully: {:?}", email_details);
    } else {
        eprintln!("Error processing emails for one or more inboxes");
    }
    Ok(())
}
