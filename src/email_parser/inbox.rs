use crate::crud::create_email::CreateEmailDbOps;
use std::{collections::HashMap, time::Duration};

use imap::Session;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use native_tls::TlsStream;

use crate::{
    db::connect::DatabaseConnection, db::email::Emails,
    factories::credentials::EmailAccountBuilder, rules::define::define_rules,
};

use super::{
    attachment::get_and_save_attachments,
    parser::{fetch_emails, get_email_details, EmailDetails},
};

async fn connect(
    email_account: &EmailAccountBuilder,
) -> imap::error::Result<Session<TlsStream<std::net::TcpStream>>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect(
        (&*email_account.server, email_account.port), // Pass a reference to credentials.server
        &email_account.server,                        // Pass a reference to credentials.server
        &tls,
    )?;
    let imap_session = client
        .login(&email_account.email, &email_account.password)
        .map_err(|e| e.0)?;
    Ok(imap_session)
}

async fn process_inbox(
    email_account: &EmailAccountBuilder,
    multi_progress: &MultiProgress,
) -> Result<Vec<EmailDetails>, Box<dyn std::error::Error>> {
    let mut imap_session = connect(email_account).await?;
    let messages = fetch_emails(&mut imap_session, &email_account.uid_set)?;
    let rules = define_rules();
    let email_details = get_email_details(&messages, &rules)?;
    get_and_save_attachments(&email_details, &mut imap_session, &multi_progress);
    imap_session.logout()?;
    Ok(email_details)
}

pub async fn process_all_inboxes(
    inboxes: HashMap<&str, EmailAccountBuilder>,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_conn = DatabaseConnection;
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new_spinner());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    for (inbox_name, credentials) in inboxes.iter() {
        let inbox_name_str = format!("📥 Processing inbox: {}", inbox_name);
        pb.set_message(inbox_name_str.clone());
        let email_details = process_inbox(credentials, &m).await?;
        db_conn
            .store_emails(Emails {
                mailbox: inbox_name.to_string(),
                details: email_details.clone(),
            })
            .await?;
    }
    pb.finish_with_message("🏁 Done processing emails");
    Ok(())
}
