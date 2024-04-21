use crate::{
    factories::credentials::EmailAccountBuilder, COMPANY_EMAIL,
    COMPANY_EMAIL_PASSWORD, COMPANY_EMAIL_PORT, COMPANY_EMAIL_SERVER, PRIVATE_EMAIL,
    PRIVATE_EMAIL_PASSWORD, S_EMAIL, S_EMAIL_PASSWORD,
};
use std::collections::HashMap;

use super::inbox::process_all_inboxes;

pub async fn process_emails() -> Result<(), Box<dyn std::error::Error>> {
    let mut inboxes = HashMap::new();
    let latest_uid_company: i32 = 1;
    let latest_uid_private: i32 = 1;
    let latest_uid_s: i32 = 1;
    let company_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &COMPANY_EMAIL,
        &COMPANY_EMAIL_PASSWORD,
    )
    .uid_set(format!("{}:*", latest_uid_company.to_string()).as_str())
    .build();
    let private_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &PRIVATE_EMAIL,
        &PRIVATE_EMAIL_PASSWORD,
    )
    .uid_set(format!("{}:*", latest_uid_private.to_string()).as_str())
    .build();
    let s_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &S_EMAIL,
        &S_EMAIL_PASSWORD,
    )
    .uid_set(format!("{}:*", latest_uid_s.to_string()).as_str())
    .build();
    inboxes.insert("company", company_credentials);
    inboxes.insert("private", private_credentials);
    inboxes.insert("s", s_credentials);
    process_all_inboxes(inboxes).await.unwrap_or_else(|e| {
        eprintln!("Error processing all inboxes: {:?}", e);
    });
    Ok(())
}
