use crate::{
    datemath::date::get_current_year_month_str,
    db::{
        connect::connect,
        email::{CreateEmailMonthly, Emails, Record},
        enums::Tables,
        mailbox::CreateMailboxMonthly,
    },
    email_parser::parser::EmailDetails,
};

use crate::crud::get_email::GetEmailDbOps;
use crate::db::connect::DatabaseConnection;

#[allow(async_fn_in_trait)]
pub trait CreateEmailDbOps {
    async fn store_emails(&self, emails: Emails) -> surrealdb::Result<()>;
}

impl CreateEmailDbOps for DatabaseConnection {
    async fn store_emails(&self, emails: Emails) -> surrealdb::Result<()> {
        let db = connect().await?;
        let existing_email_id = &self
            .get_email_id_for_current_year_month_by_mailbox(&emails.mailbox)
            .await?;
        let updated_at = chrono::Local::now().to_rfc3339();
        match existing_email_id {
            Some(existing_email_id) => {
                let _: Option<Record> = db
                    .update((Tables::Emails.to_string(), existing_email_id))
                    .content(CreateEmailMonthly {
                        year_month: get_current_year_month_str(),
                        emails: emails.clone(),
                        updated_at: updated_at.clone(),
                    })
                    .await?;
                let max_uid = emails
                    .clone()
                    .details
                    .iter()
                    .max_by_key(|x| x.uid)
                    .unwrap_or(&EmailDetails {
                        uid: 0,
                        subject: "".to_string(),
                        from: vec!["".to_string()],
                        date: chrono::Utc::now(),
                    })
                    .uid;
                let _: Vec<Record> = db
                    .create(Tables::Mailbox.to_string())
                    .content(CreateMailboxMonthly {
                        year_month: get_current_year_month_str(),
                        updated_at: updated_at.clone(),
                        latest_uid: max_uid,
                    })
                    .await?;
            }
            None => {
                let _: Vec<Record> = db
                    .create(Tables::Emails.to_string())
                    .content(CreateEmailMonthly {
                        year_month: get_current_year_month_str(),
                        emails: emails.clone(),
                        updated_at: updated_at.clone(),
                    })
                    .await?;
                let max_uid = emails
                    .clone()
                    .details
                    .iter()
                    .max_by_key(|x| x.uid)
                    .unwrap_or(&EmailDetails {
                        uid: 0,
                        subject: "".to_string(),
                        from: vec!["".to_string()],
                        date: chrono::Utc::now(),
                    })
                    .uid;
                let _: Vec<Record> = db
                    .create(Tables::Mailbox.to_string())
                    .content(CreateMailboxMonthly {
                        year_month: get_current_year_month_str(),
                        updated_at: updated_at.clone(),
                        latest_uid: max_uid,
                    })
                    .await?;
            }
        }
        Ok(())
    }
}
