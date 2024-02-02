use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::datemath::date::get_current_year_month_str;
use crate::db::connect::connect;
use crate::email_parser::parser::EmailDetails;

use super::enums::Tables;

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
        .create(Tables::Emails.to_string())
        .content(EmailMonthly {
            year_month: get_current_year_month_str(),
            emails,
        })
        .await?;
    Ok(())
}

pub async fn get_emails() -> surrealdb::Result<Vec<EmailMonthly>> {
    let db = connect().await?;
    let emails: Vec<EmailMonthly> = db.select(Tables::Emails.to_string()).await?;
    dbg!(&emails);
    Ok(emails)
}

pub async fn get_emails_current_year_month() -> surrealdb::Result<()> {
    let db = connect().await?;
    let year_month = get_current_year_month_str();
    let sql = "
    SELECT * FROM type::table($table) WHERE year_month = $year_month;
";
    let mut result = db
        .query(sql)
        .bind(("table", Tables::Emails.to_string()))
        .bind(("year_month", year_month))
        .await?;
    let emails: Vec<EmailMonthly> = result.take(0)?;
    dbg!(&emails);
    Ok(())
}

pub async fn get_emails_current_year_month_mailbox(
    mailbox: &str,
    year_month: &str,
) -> surrealdb::Result<()> {
    let db = connect().await?;
    let year_month_str = match year_month {
        "" => get_current_year_month_str(),
        _ => year_month.to_string(),
    };
    let sql = "
    SELECT * FROM type::table($table) WHERE year_month = $year_month AND emails.mailbox = $mailbox;
    ";
    let mut result = db
        .query(sql)
        .bind(("table", Tables::Emails.to_string()))
        .bind(("year_month", year_month_str))
        .bind(("mailbox", mailbox))
        .await?;
    let emails: Vec<EmailMonthly> = result.take(0)?;
    dbg!(&emails);
    Ok(())
}
