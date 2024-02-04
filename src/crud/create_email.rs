use crate::{
    datemath::date::get_current_year_month_str,
    db::{
        connect::connect,
        email::{CreateEmailMonthly, Emails, Record},
        enums::Tables,
    },
};

use crate::crud::get_email::EmailDatabase;
use crate::crud::get_email::MyDatabaseConnection;

// use super::get_email::get_email_id_for_current_year_month_by_mailbox;

pub async fn store_emails(emails: Emails) -> surrealdb::Result<()> {
    let db = connect().await?;
    let db_conn = MyDatabaseConnection;
    let existing_email_id = db_conn.get_email_id_for_current_year_month_by_mailbox(&emails.mailbox).await?;
    let updated_at = chrono::Local::now().to_rfc3339();
    match existing_email_id {
        Some(existing_email_id) => {
            println!("existing_email_id: {:?}", existing_email_id);
            let _: Option<Record> = db
                .update((Tables::Emails.to_string(), existing_email_id))
                .content(CreateEmailMonthly {
                    year_month: get_current_year_month_str(),
                    emails,
                    updated_at,
                })
                .await?;
        }
        None => {
            let _: Vec<Record> = db
                .create(Tables::Emails.to_string())
                .content(CreateEmailMonthly {
                    year_month: get_current_year_month_str(),
                    emails,
                    updated_at,
                })
                .await?;
        }
    }
    Ok(())
}
