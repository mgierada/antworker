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

pub fn define_rules() {
    println!("OBSERVED_SENDERS: {:?}", OBSERVED_SENDERS.to_vec());
}
