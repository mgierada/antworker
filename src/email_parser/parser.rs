use chrono::{DateTime, Utc};
use imap::{
    types::{Fetch, ZeroCopy},
    Session,
};
use native_tls::TlsStream;
use quoted_printable::{decode, ParseMode};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};
use std::io::{Read, Write};

use crate::rules::define::FilterRules;

#[derive(Default, Serialize, Deserialize, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse_date_rfc2822() {
        let date_str = "Mon, 15 Nov 2021 10:30:00 +0000";
        let parsed = parse_date(date_str);
        assert_eq!(parsed.year(), 2021);
        assert_eq!(parsed.month(), 11);
        assert_eq!(parsed.day(), 15);
        assert_eq!(parsed.hour(), 10);
        assert_eq!(parsed.minute(), 30);
    }

    #[test]
    fn test_parse_date_rfc2822_with_offset() {
        let date_str = "Wed, 25 Dec 2024 14:45:30 -0500";
        let parsed = parse_date(date_str);
        assert_eq!(parsed.year(), 2024);
        assert_eq!(parsed.month(), 12);
        assert_eq!(parsed.day(), 25);
        // Note: Time will be adjusted to UTC
        assert_eq!(parsed.hour(), 19); // 14:45 - 5 hours = 19:45 UTC
        assert_eq!(parsed.minute(), 45);
    }

    #[test]
    fn test_parse_date_invalid_fallback_to_current() {
        let date_str = "invalid date string";
        let parsed = parse_date(date_str);
        let now = Utc::now();
        // Should be close to current time (within a few seconds)
        let diff = (parsed - now).num_seconds().abs();
        assert!(diff < 5, "Parsed date should be close to current time");
    }

    #[test]
    fn test_parse_date_empty_string() {
        let date_str = "";
        let parsed = parse_date(date_str);
        let now = Utc::now();
        let diff = (parsed - now).num_seconds().abs();
        assert!(diff < 5, "Empty date string should fallback to current time");
    }

    #[test]
    fn test_email_details_default() {
        let email = EmailDetails::default();
        assert_eq!(email.subject, "");
        assert_eq!(email.from.len(), 0);
        assert_eq!(email.uid, 0);
    }

    #[test]
    fn test_email_details_clone() {
        let email = EmailDetails {
            subject: "Test Subject".to_string(),
            from: vec!["test@example.com".to_string()],
            date: Utc::now(),
            uid: 123,
        };
        let cloned = email.clone();
        assert_eq!(email.subject, cloned.subject);
        assert_eq!(email.from, cloned.from);
        assert_eq!(email.date, cloned.date);
        assert_eq!(email.uid, cloned.uid);
    }
}
