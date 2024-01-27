use chrono::{DateTime, Utc};
use imap::{
    types::{Fetch, ZeroCopy},
    Session,
};
use native_tls::TlsStream;
use quoted_printable::{decode, ParseMode};
use serde::Serialize;
use std::fmt::{self, Debug, Formatter};
use std::io::{Read, Write};

use crate::rules::define::FilterRules;

#[derive(Default, Serialize)]
pub struct EmailDetails {
    // pub subject: &'a str,
    pub subject: String,
    // pub from: Vec< &'a str>,
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

pub fn fetch_emails<S: Read + Write>(
    imap_session: &mut Session<TlsStream<S>>,
    uid_set: &str,
) -> imap::error::Result<ZeroCopy<Vec<Fetch>>> {
    imap_session.select("INBOX")?;
    let messages = imap_session.uid_fetch(&uid_set, "ALL")?;
    Ok(messages)
}

pub fn get_email_details(
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
