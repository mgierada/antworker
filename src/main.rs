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

async fn connect() -> imap::error::Result<Option<Vec<String>>> {
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

    // let messages = imap_session.fetch("RECENT", "ENVELOPE UID")?;
    let messages = imap_session.fetch("1:*", "ALL")?;
    println!("messages: {:?}", messages);
    let message = if let Some(msg) = messages.iter().next() {
        msg
    } else {
        return Ok(None);
    };

    let envelope = message
        .envelope()
        .expect("message did not have an envelope!");

    let subjects: Vec<String> = messages
        .iter()
        .filter_map(|msg| {
            let envelope = msg.envelope()?;
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
            Some(subject)
        })
        .collect();
    imap_session.logout()?;
    Ok(Some(subjects))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let subject = connect().await.unwrap();
    println!("subject: {:?}", subject);
}
