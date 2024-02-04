use crate::crud::get_email::EmailDatabase;
use crate::crud::get_email::MyDatabaseConnection;
use crate::db::connect::connect;
use crate::db::email::EmailMonthly;
use crate::db::enums::Tables;

#[allow(async_fn_in_trait)]
pub trait DeleteEmailDbOps {
    async fn remove_emails(&self) -> surrealdb::Result<()>;
}

impl DeleteEmailDbOps for MyDatabaseConnection {
    async fn remove_emails(&self) -> surrealdb::Result<()> {
        let db = connect().await?;
        let ids_to_remove = &self.get_emails_ids_for_current_year_month().await?;
        for id in ids_to_remove {
            let _deleted_email: Option<EmailMonthly> =
                db.delete((Tables::Emails.to_string(), id.clone())).await?;
        }
        Ok(())
    }
}
