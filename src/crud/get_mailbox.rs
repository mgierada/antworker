use crate::{
    datemath::date::get_current_year_month_str,
    db::{
        connect::{connect, DatabaseConnection},
        email::Record,
        enums::Tables,
        mailbox::MailboxMonthly,
    },
};

#[allow(async_fn_in_trait)]
pub trait GetMailboxDbOps {
    async fn get_mailboxes(&self) -> surrealdb::Result<Vec<MailboxMonthly>>;
    async fn get_mailbox_ids_for_current_year_month(&self) -> surrealdb::Result<Vec<String>>;
    async fn get_mailbox_id_by_mailbox(&self, mailbox: &str) -> surrealdb::Result<Option<String>>;
}

impl GetMailboxDbOps for DatabaseConnection {
    async fn get_mailboxes(&self) -> surrealdb::Result<Vec<MailboxMonthly>> {
        let db = connect().await?;
        let mailboxes: Vec<MailboxMonthly> = db.select(Tables::Mailbox.to_string()).await?;
        dbg!(&mailboxes);
        Ok(mailboxes)
    }

    async fn get_mailbox_ids_for_current_year_month(&self) -> surrealdb::Result<Vec<String>> {
        let db = connect().await?;
        let year_month = get_current_year_month_str();
        let sql = "SELECT id FROM type::table($table) WHERE year_month = $year_month;";
        let mut result = db
            .query(sql)
            .bind(("table", Tables::Mailbox.to_string()))
            .bind(("year_month", year_month))
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
        let raw_ids: Vec<Record> = result.take(0)?;
        let id = raw_ids.iter().map(|x| x.id.id.to_string()).collect::<Vec<String>>().pop();
        dbg!(&id);
        Ok(id)
    }
}
