use lazy_static::lazy_static;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::env::var;

use crate::{COMPANY_EMAIL, COMPANY_EMAIL_PASSWORD};

lazy_static! {
    pub static ref TARGET_EMAIL: String = var("TARGET_EMAIL").expect("TARGET_EMAIL must be set.");
}

lazy_static! {
    pub static ref FROM_EMAIL: String = var("FROM_EMAIL").expect("FROM_EMAIL must be set.");
}

lazy_static! {
    pub static ref SUBJECT: String = var("SUBJECT").expect("SUBJECT must be set.");
}

lazy_static! {
    pub static ref SMTP_TARGET_SERVER: String =
        var("SMTP_TARGET_SERVER").expect("SMTP_TARGET_SERVER must be set.");
}

pub fn send_email() -> () {
    let email = Message::builder()
        .to(TARGET_EMAIL.as_str().parse().unwrap())
        .from(FROM_EMAIL.as_str().parse().unwrap())
        .subject(SUBJECT.as_str())
        .body(String::from("Hello from Rust!"))
        .unwrap();
    let creds = Credentials::new(COMPANY_EMAIL.to_owned(), COMPANY_EMAIL_PASSWORD.to_owned());
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(SMTP_TARGET_SERVER.as_str())
        .unwrap()
        .credentials(creds)
        .build();
    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
