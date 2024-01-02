use crate::email_parser::parser::process_emails;
use dotenv::dotenv;
use lazy_static::lazy_static;
use rules::save_location:: setup;

use std::env::var;
extern crate imap;
extern crate native_tls;

pub mod email_parser;
pub mod rules;

lazy_static! {
    pub static ref COMPANY_EMAIL_SERVER: String =
        var("COMPANY_EMAIL_SERVER").expect("COMPANY_EMAIL_SERVER must be set.");
}
lazy_static! {
    pub static ref COMPANY_EMAIL_PORT: u16 = var("COMPANY_EMAIL_PORT")
        .expect("COMPANY_EMAIL_PORT must be set.")
        .parse()
        .expect("COMPANY_EMAIL_PORT must be a valid u16.");
}
lazy_static! {
    pub static ref COMPANY_EMAIL: String =
        var("COMPANY_EMAIL").expect("COMPANY_EMAIL must be set.");
}
lazy_static! {
    pub static ref COMPANY_EMAIL_PASSWORD: String =
        var("COMPANY_EMAIL_PASSWORD").expect("COMPANY_EMAIL_PASSWORD must be set.");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let subject = process_emails().await.unwrap();
    // println!("subject: {:?}", subject);
    let setup= setup();
    
}
