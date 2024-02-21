use crate::crud::get_mailbox::GetMailboxDbOps;
use crate::{
    db::connect::DatabaseConnection, factories::credentials::EmailAccountBuilder, COMPANY_EMAIL,
    COMPANY_EMAIL_PASSWORD, COMPANY_EMAIL_PORT, COMPANY_EMAIL_SERVER, PRIVATE_EMAIL,
    PRIVATE_EMAIL_PASSWORD, S_EMAIL, S_EMAIL_PASSWORD,
};
use std::collections::HashMap;

use super::inbox::process_all_inboxes;

pub async fn process_emails() -> Result<(), Box<dyn std::error::Error>> {
    let db_conn = DatabaseConnection;
    let mut inboxes = HashMap::new();
    let latest_uid_company = db_conn.get_latest_uid_by_mailbox("company").await?;
    dbg!(&latest_uid_company);
    let company_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &COMPANY_EMAIL,
        &COMPANY_EMAIL_PASSWORD,
    )
    // .uid_set("1:*")
    .uid_set(format!("{}:*", latest_uid_company.unwrap_or(1)).as_str())
    .build();
    let private_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &PRIVATE_EMAIL,
        &PRIVATE_EMAIL_PASSWORD,
    )
    .uid_set("6000:*")
    .build();
    let s_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &S_EMAIL,
        &S_EMAIL_PASSWORD,
    )
    .uid_set("1:*")
    .build();
    inboxes.insert("company", company_credentials);
    inboxes.insert("private", private_credentials);
    inboxes.insert("s", s_credentials);
    process_all_inboxes(inboxes).await.unwrap_or_else(|e| {
        eprintln!("Error processing all inboxes: {:?}", e);
    });
    Ok(())
}
