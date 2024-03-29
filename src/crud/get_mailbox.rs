use crate::db::{
    connect::{connect, DatabaseConnection},
    email::Record,
    enums::Tables,
    mailbox::MailboxMonthly,
};

#[allow(async_fn_in_trait)]
pub trait GetMailboxDbOps {
    async fn get_mailboxes(&self) -> surrealdb::Result<Vec<MailboxMonthly>>;
    async fn get_mailbox_ids(&self) -> surrealdb::Result<Vec<String>>;
    async fn get_mailbox_id_by_mailbox(&self, mailbox: &str) -> surrealdb::Result<Option<String>>;
    async fn get_latest_uid_by_mailbox(&self, mailbox: &str) -> surrealdb::Result<Option<u32>>;
}

impl GetMailboxDbOps for DatabaseConnection {
    async fn get_mailboxes(&self) -> surrealdb::Result<Vec<MailboxMonthly>> {
        let db = connect().await?;
        let mailboxes: Vec<MailboxMonthly> = db.select(Tables::Mailbox.to_string()).await?;
        dbg!(&mailboxes);
        Ok(mailboxes)
    }

    async fn get_mailbox_ids(&self) -> surrealdb::Result<Vec<String>> {
        let db = connect().await?;
        let sql = "SELECT id FROM type::table($table);";
        let mut result = db
            .query(sql)
            .bind(("table", Tables::Mailbox.to_string()))
            .await?;
        let raw_ids: Vec<Record> = result.take(0)?;
        let ids: Vec<String> = raw_ids.iter().map(|x| x.id.id.to_string()).collect();
        dbg!(&ids);
        Ok(ids)
    }

    async fn get_mailbox_id_by_mailbox(&self, mailbox: &str) -> surrealdb::Result<Option<String>> {
        let db = connect().await?;
        let sql = "SELECT id FROM type::table($table) WHERE mailbox = $mailbox;";
        let mut result = db
            .query(sql)
            .bind(("table", Tables::Mailbox.to_string()))
            .bind(("mailbox", mailbox))
            .await?;
        // NOTE This should be a single record
        let raw_ids: Vec<Record> = result.take(0)?;
        let id = raw_ids
            .iter()
            .map(|x| x.id.id.to_string())
            .collect::<Vec<String>>()
            .pop();
        Ok(id)
    }

    async fn get_latest_uid_by_mailbox(&self, mailbox: &str) -> surrealdb::Result<Option<u32>> {
        let db = connect().await?;
        let sql = "SELECT * FROM type::table($table) WHERE mailbox = $mailbox;";
        let mut result = db
            .query(sql)
            .bind(("table", Tables::Mailbox.to_string()))
            .bind(("mailbox", mailbox))
            .await?;
        // NOTE This should be a single record
        let mailbox: Vec<MailboxMonthly> = result.take(0)?;
        let latest_uid = mailbox
            .iter()
            .map(|x| x.latest_uid)
            .collect::<Vec<u32>>()
            .pop();
        Ok(latest_uid)
    }
}
