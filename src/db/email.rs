use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::datemath::date::get_current_year_month_str;
use crate::db::connect::connect;
use crate::email_parser::parser::EmailDetails;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailMonthly {
    pub year_month: String,
    pub items: Vec<Mailbox>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mailbox {
    pub mailbox: Vec<MailboxDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailboxDetails {
    pub name: String,
    pub emails: Vec<EmailDetails>,
}

pub async fn store_emails(items: Vec<Mailbox>) -> surrealdb::Result<()> {
    let db = connect().await?;
    let _: Vec<Record> = db
        .create("emails")
        .content(EmailMonthly {
            year_month: get_current_year_month_str(),
            items,
        })
        .await?;
    Ok(())
}

pub async fn get_emails() -> surrealdb::Result<Vec<EmailMonthly>> {
    let db = connect().await?;
    let emails: Vec<EmailMonthly> = db.select("emails").await?;
    dbg!(&emails);
    Ok(emails)
}
