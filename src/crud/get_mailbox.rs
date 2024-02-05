use crate::db::{
    connect::{connect, DatabaseConnection},
    enums::Tables,
    mailbox::MailboxMonthly,
};

#[allow(async_fn_in_trait)]
pub trait GetMailboxDbOps {
    async fn get_mailboxes(&self) -> surrealdb::Result<Vec<MailboxMonthly>>;
}

impl GetMailboxDbOps for DatabaseConnection {
    async fn get_mailboxes(&self) -> surrealdb::Result<Vec<MailboxMonthly>> {
        let db = connect().await?;
        println!("Getting mailboxes");
        let mailboxes: Vec<MailboxMonthly> = db.select(Tables::Mailbox.to_string()).await?;
        dbg!(&mailboxes);
        Ok(mailboxes)
    }
}
