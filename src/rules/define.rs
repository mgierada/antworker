use imap::types::Fetch;
use lazy_static::lazy_static;
use std::env::var;

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
    };
}

pub struct FilterRules {
    pub allowed_senders: Vec<String>,
}

impl FilterRules {
    pub fn is_empty(&self) -> bool {
        self.allowed_senders.is_empty()
    }

    pub fn matches(&self, msg: &Fetch) -> bool {
        let envelope = msg.envelope().expect("message did not have an envelope!");
        // Check if the sender is allowed
        let sender_allowed = envelope
            .from
            .as_ref()
            .map_or(false, |from_addresses| {
                from_addresses.iter().any(|address| {
                    let sender = format!(
                        "{}@{}",
                        String::from_utf8_lossy(address.mailbox.unwrap_or_default()).to_string(),
                        String::from_utf8_lossy(address.host.unwrap_or_default()).to_string()
                    );
                    self.allowed_senders.contains(&sender)
                })
            });
        sender_allowed
    }
}
