use crate::crud::get_mailbox::GetMailboxDbOps;
use crate::db::connect::connect;
use crate::db::connect::DatabaseConnection;
use crate::db::enums::Tables;
use crate::db::mailbox::MailboxMonthly;

#[allow(async_fn_in_trait)]
pub trait DeleteMailboxDbOps {
    async fn remove_mailbox(&self) -> surrealdb::Result<()>;
}

impl DeleteMailboxDbOps for DatabaseConnection {
    async fn remove_mailbox(&self) -> surrealdb::Result<()> {
        let db = connect().await?;
        let ids_to_remove = &self.get_mailbox_ids_for_current_year_month().await?;
        for id in ids_to_remove {
            let _deleted_mailbox: Option<MailboxMonthly> =
                db.delete((Tables::Mailbox.to_string(), id.clone())).await?;
        }
        Ok(())
    }
}
