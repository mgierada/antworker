use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::datemath::date::get_current_year_month_str;
use crate::db::connect::connect;
use crate::email_parser::parser::EmailDetails;

#[derive(Debug, Serialize)]
struct EmailMonthly<'a> {
    year_month: &'a str,
    emails: Vec<Email<'a>>,
}

#[derive(Debug, Serialize)]
struct Email<'a> {
    mailbox: &'a str,
    details: EmailDetails,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn store_emails() -> surrealdb::Result<()> {
    let db = connect().await?;
    let created: Vec<Record> = db
        .create("emails")
        .content(EmailMonthly{
            year_month: &&get_current_year_month_str(),
            emails: vec![],
        })
        .await?;
    dbg!(created);
    Ok(())
}
