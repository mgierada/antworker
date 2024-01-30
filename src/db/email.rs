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
    year_month: String,
    emails: Emails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emails {
    pub mailbox: String,
    pub details: Vec<EmailDetails>,
}

pub async fn store_emails(emails: Emails) -> surrealdb::Result<()> {
    let db = connect().await?;
    let _: Vec<Record> = db
        .create("emails")
        .content(EmailMonthly {
            year_month: get_current_year_month_str(),
            emails,
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
