use crate::{
    datemath::date::get_current_year_month_str,
    db::{
        connect::connect,
        email::{CreateEmailMonthly, Emails, Record},
        enums::Tables,
    },
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
}
