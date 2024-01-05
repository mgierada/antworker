use lazy_static::lazy_static;
use lettre::{
    message::{header::ContentType, Attachment, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use std::env::var;

use crate::{io::save_location::get_save_location, COMPANY_EMAIL, COMPANY_EMAIL_PASSWORD};

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

fn format_email(email: &str) -> String {
    let name = email.split("@").collect::<Vec<&str>>()[0];
    format!("{} <{}>", name, email)
}

fn add_attachment() -> SinglePart {
    let filename = String::from("faktura_2024-01-02 01-19-48)_927.pdf");
    let save_location = get_save_location();
    let file_to_send = format!("{}/{}", save_location, filename);
    let filebody = std::fs::read(file_to_send).unwrap();
    let content_type = ContentType::parse("application/pdf").unwrap();
    let attachment = Attachment::new(filename).body(filebody, content_type);
    attachment
}

pub fn send_email() -> () {
    let attachment = add_attachment();
    let email = Message::builder()
        .to(format_email(TARGET_EMAIL.as_str()).parse().unwrap())
        .from(format_email(FROM_EMAIL.as_str()).parse().unwrap())
        .subject(SUBJECT.as_str())
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(String::from("W zalaczeniu faktury za ostatni miesiac.")),
                )
                .singlepart(attachment),
        )
        // .body(String::from("W zalaczeniu faktury za ostatni miesiac."))
        .unwrap();
    // Add the attachment
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
