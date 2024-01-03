use chrono::Datelike;
use imap::types::Fetch;
use lazy_static::lazy_static;
use std::env::var;

use crate::{datemath::date::get_current_month_year, email_parser::parser::parse_date};

lazy_static! {
    #[derive(Debug)]
    pub static ref OBSERVED_SENDERS: Vec<String> = {
        let observed_senders_str = var("OBSERVED_SENDERS").expect("OBSERVED_SENDERS must be set.");
        observed_senders_str
            .split(',')
            .map(|s| s.trim_matches('"').trim().to_string())
            .collect()
    };
}

pub fn define_rules() -> FilterRules {
    return FilterRules {
        allowed_senders: OBSERVED_SENDERS.to_vec(),
        timeframe: get_current_month_year(),
    };
}

pub struct FilterRules {
    pub allowed_senders: Vec<String>,
    pub timeframe: Option<(i32, u32)>, // (year, month)
}

impl FilterRules {
    pub fn is_empty(&self) -> bool {
        self.allowed_senders.is_empty()
    }

    pub fn matches(&self, msg: &Fetch) -> bool {
        // Check whether the sender of the given email message is in the list of allowed senders
        // specified by the FilterRules struct. If the sender is allowed, the method returns true;
        // otherwise, it returns false.
        let envelope = msg.envelope().expect("message did not have an envelope!");
        // Check if the sender is allowed
        let sender_allowed = envelope.from.as_ref().map_or(false, |from_addresses| {
            from_addresses.iter().any(|address| {
                let sender = format!(
                    "{}@{}",
                    String::from_utf8_lossy(address.mailbox.unwrap_or_default()).to_string(),
                    String::from_utf8_lossy(address.host.unwrap_or_default()).to_string()
                );
                self.allowed_senders.contains(&sender)
            })
        });
        // Check if the date is within the specified timeframe
        let date_allowed = match self.timeframe {
            Some((year, month)) => {
                let email_date_raw = envelope
                    .date
                    .map(|date_bytes| String::from_utf8_lossy(&date_bytes).to_string());
                let email_date = parse_date(&email_date_raw.unwrap_or_else(|| String::new()));
                // let email_date = envelope.date.expect("message did not have a date!");
                let email_month_year = (email_date.year(), email_date.month());
                email_month_year == (year, month)
            }
            None => true, // No timeframe specified, consider all dates
        };
        sender_allowed && date_allowed
    }
}
