use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use lazy_static::lazy_static;
use lettre::{
    message::{header::ContentType, Attachment, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use std::{env::var, fs};

use crate::{io::files::get_saved_files, COMPANY_EMAIL, COMPANY_EMAIL_PASSWORD};

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

fn add_attachment(filepath: &String) -> SinglePart {
    let filename = filepath
        .split("/")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string();
    // let file_to_send = format!("{}/{}", save_location, filename);
    let filebody = fs::read(filepath).unwrap();
    let content_type = ContentType::parse("application/pdf").unwrap();
    Attachment::new(filename).body(filebody, content_type)
}

fn add_attachments() -> Vec<SinglePart> {
    let save_location = get_saved_files();
    let attachments = save_location
        .iter()
        .map(|filepath| add_attachment(filepath))
        .collect();
    attachments
}

pub fn send_emails(is_dry_run: bool) {
    let attachments = add_attachments();
    let n_attachments = attachments.len();
    let pb = ProgressBar::new(n_attachments as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.red}] ({pos}/{len})").unwrap(),
    );
    if is_dry_run {
        let att = get_saved_files()
            .iter()
            .map(|filepath| filepath.to_string())
            .collect::<Vec<String>>()
            .join("\n     ");
        println!(
            "The total {} emails will be sent with the following attachments: \n    {}",
            n_attachments, att
        );
        return;
    }
    attachments.iter().progress_with(pb).for_each(|attachment| {
        send_email(attachment.clone());
    });
}

pub fn send_email(attachment: SinglePart) -> () {
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
        .unwrap();

    let from = Credentials::new(COMPANY_EMAIL.to_owned(), COMPANY_EMAIL_PASSWORD.to_owned());
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(SMTP_TARGET_SERVER.as_str())
        .unwrap()
        .credentials(from)
        .build();
    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
