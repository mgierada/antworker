use dotenv::dotenv;
use lazy_static::lazy_static;
use quoted_printable::{decode, ParseMode};
use std::env::var;
extern crate imap;
extern crate native_tls;

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

async fn connect() -> imap::error::Result<Option<String>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect(
        (COMPANY_EMAIL_SERVER.to_string(), *COMPANY_EMAIL_PORT),
        COMPANY_EMAIL_SERVER.to_string(),
        &tls,
    )
    .unwrap();

    let mut imap_session = client
        .login(
            COMPANY_EMAIL.to_string(),
            COMPANY_EMAIL_PASSWORD.to_string(),
        )
        .map_err(|e| e.0)?;

    imap_session.select("INBOX")?;

    let messages = imap_session.fetch("15", "ENVELOPE")?;
    let message = if let Some(msg) = messages.iter().next() {
        msg
    } else {
        return Ok(None);
    };

    let envelope = message
        .envelope()
        .expect("message did not have an envelope!");

    let subject = if let Some(subject_bytes) = envelope.subject {
        let decoded_subject = decode(subject_bytes, ParseMode::Robust)
            .map(|v| String::from_utf8_lossy(&v).to_string())
            .unwrap_or_else(|e| {
                eprintln!("Failed to decode subject: {}", e);
                String::new()
            });

        // HACK: Who the hell knows why the resulted string has those utf related
        // encoding characters.
        let decoded_subject = decoded_subject
            .replace("=?UTF-8?Q?", "")
            .replace("?= ", "")
            .replace("_", " ");

        decoded_subject
    } else {
        // Handle the case when the subject is None (no subject in the envelope)
        String::new()
    };

    imap_session.logout()?;

    Ok(Some(subject))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let subject = connect().await.unwrap();
    println!("subject: {:?}", subject);
}
