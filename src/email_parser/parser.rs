use chrono::{DateTime, Utc};
use imap::{
    types::{Fetch, ZeroCopy},
    Session,
};
use indicatif::{MultiProgress, ProgressBar, ProgressIterator, ProgressStyle};
use mailparse::{self, parse_mail, ParsedContentType, ParsedMail};
use native_tls::TlsStream;
use quoted_printable::{decode, ParseMode};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};
use std::{
    fmt::{self, Debug, Formatter},
    time::Duration,
};

use crate::{
    factories::credentials::EmailAccountBuilder,
    io::save_location::setup_save_location,
    rules::define::{define_rules, FilterRules},
    COMPANY_EMAIL, COMPANY_EMAIL_PASSWORD, COMPANY_EMAIL_PORT, COMPANY_EMAIL_SERVER, PRIVATE_EMAIL,
    PRIVATE_EMAIL_PASSWORD, S_EMAIL, S_EMAIL_PASSWORD,
};

#[derive(Default)]
pub struct EmailDetails {
    pub subject: String,
    pub from: Vec<String>,
    pub date: DateTime<Utc>,
    pub uid: u32,
}

impl Debug for EmailDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "EmailDetails {{")?;
        writeln!(f, "  subject: {}", self.subject)?;
        writeln!(f, "  from: {:?}", self.from)?;
        writeln!(f, "  date: {}", self.date)?;
        writeln!(f, "  uid: {}", self.uid)?;
        write!(f, "}}")
    }
}

async fn connect(
    email_account: &EmailAccountBuilder,
) -> imap::error::Result<Session<TlsStream<std::net::TcpStream>>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect(
        (&*email_account.server, email_account.port), // Pass a reference to credentials.server
        &email_account.server,                        // Pass a reference to credentials.server
        &tls,
    )?;
    let imap_session = client
        .login(&email_account.email, &email_account.password)
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
    m: &MultiProgress,
) -> () {
    let email_len = email_details.len();
    // Provide a custom bar style
    let pb_2 = m.add(ProgressBar::new(email_len as u64));
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

async fn process_inbox(
    email_account: &EmailAccountBuilder,
    m: &MultiProgress,
) -> Result<Vec<EmailDetails>, Box<dyn std::error::Error>> {
    let mut imap_session = connect(email_account).await?;
    let messages = fetch_emails(&mut imap_session, &email_account.uid_set)?;
    let rules = define_rules();
    let email_details = get_email_details(&messages, &rules)?;
    get_and_save_attachments(&email_details, &mut imap_session, &m);
    imap_session.logout()?;
    Ok(email_details)
}

pub async fn process_all_inboxes(
    inboxes: HashMap<&str, EmailAccountBuilder>,
) -> Result<Vec<EmailDetails>, Box<dyn std::error::Error>> {
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new_spinner());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    let mut all_email_details = Vec::new();
    for (inbox_name, credentials) in inboxes.iter() {
        let inbox_name_str = format!("📥 Processing inbox: {}", inbox_name);
        pb.set_message(inbox_name_str);
        let email_details = process_inbox(credentials, &m).await?;
        all_email_details.extend(email_details);
    }
    pb.finish_with_message("🏁Done processing emails");
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
        println!("All emails processed successfully: \n{:?}", email_details);
    } else {
        eprintln!("Error processing emails for one or more inboxes");
    }
    Ok(())
}
