use std::i32;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

use crate::{
    datemath::date::get_current_year_month_str,
    db::{connect::connect, email::Record, enums::Tables},
};

use super::get_email::get_emails_current_year_month;

use crate::email_parser::parser::EmailDetails;

#[derive(Debug, Serialize, Deserialize)]
struct EmailMonthly {
    id: Thing,
    year_month: String,
    emails: Emails,
}

#[derive(Debug, Serialize, Deserialize)]
struct Emails {
    mailbox: String,
    details: Vec<EmailDetails>,
}

pub async fn remove_emails() -> surrealdb::Result<()> {
    let db = connect().await?;
    let year_month = get_current_year_month_str();
    let sql = "
    SELECT id FROM type::table($table) WHERE year_month = $year_month;
";
    let mut result = db
        .query(sql)
        .bind(("table", Tables::Emails.to_string()))
        .bind(("year_month", year_month))
        .await?;
    // let emails: Vec<EmailMonthly> = result.take(0)?;
    let raw_ids: Vec<Record> = result.take(0)?;
    let ids: Vec<String> = raw_ids.iter().map(|x| x.id.id.to_string()).collect();

    // delete all ids
    //
    ids.iter().for_each(|id| {
        let email: Option<EmailMonthly> =
            db.delete((Tables::Emails.to_string(), ids.clone()));
    });

    // let email: Option<EmailMonthly> = db
    //     .delete((Tables::Emails.to_string(), ids.clone()))
    //     .await?;

    // let ids: Option<Record> = result.take(0)?;
    // println!("ids: {:?}", ids);
    // println!("emails: {:?}", emails);
    dbg!(&ids);
    // dbg!(&emails);
    Ok(())
}
