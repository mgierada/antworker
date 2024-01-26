use std::collections::HashMap;

use crate::{
    factories::credentials::EmailAccountBuilder, COMPANY_EMAIL, COMPANY_EMAIL_PASSWORD,
    COMPANY_EMAIL_PORT, COMPANY_EMAIL_SERVER, PRIVATE_EMAIL, PRIVATE_EMAIL_PASSWORD, S_EMAIL,
    S_EMAIL_PASSWORD,
};

use super::inbox::process_all_inboxes;

pub async fn process_emails() -> Result<(), Box<dyn std::error::Error>> {
    let mut inboxes = HashMap::new();
    let company_credentials = EmailAccountBuilder::new(
        &COMPANY_EMAIL_SERVER,
        *COMPANY_EMAIL_PORT,
        &COMPANY_EMAIL,
        &COMPANY_EMAIL_PASSWORD,
    )
    .uid_set("1:*")
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

    // Process emails for all inboxes
    if let Ok(email_details) = process_all_inboxes(inboxes).await {
        println!("All emails processed successfully: \n{:?}", email_details);
    } else {
        eprintln!("Error processing emails for one or more inboxes");
    }
    Ok(())
}
