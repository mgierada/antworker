use crate::datemath::date::get_current_year_month_str;
use crate::db::connect::connect;
use crate::db::email::{EmailMonthly, Record};
use crate::db::enums::Tables;

#[allow(async_fn_in_trait)]
pub trait EmailDatabase {
    async fn get_emails(&self) -> surrealdb::Result<Vec<EmailMonthly>>;
    async fn get_emails_current_year_month(&self) -> surrealdb::Result<()>;
    async fn get_emails_current_year_month_mailbox(
        &self,
        mailbox: &str,
        year_month: &str,
    ) -> surrealdb::Result<()>;
    async fn get_emails_ids_for_current_year_month(&self) -> surrealdb::Result<Vec<String>>;
    async fn get_email_id_for_current_year_month_by_mailbox(
        &self,
        mailbox: &str,
    ) -> surrealdb::Result<Option<String>>;
}

pub struct MyDatabaseConnection;

impl EmailDatabase for MyDatabaseConnection {
    async fn get_emails(&self) -> surrealdb::Result<Vec<EmailMonthly>> {
        let db = connect().await?;
        let emails: Vec<EmailMonthly> = db.select(Tables::Emails.to_string()).await?;
        Ok(emails)
    }

    async fn get_emails_current_year_month(&self) -> surrealdb::Result<()> {
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
    async fn get_emails_current_year_month_mailbox(
        &self,
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

    async fn get_emails_ids_for_current_year_month(&self) -> surrealdb::Result<Vec<String>> {
        let db = connect().await?;
        let year_month = get_current_year_month_str();
        let sql = "SELECT id FROM type::table($table) WHERE year_month = $year_month;";
        let mut result = db
            .query(sql)
            .bind(("table", Tables::Emails.to_string()))
            .bind(("year_month", year_month))
            .await?;
        let raw_ids: Vec<Record> = result.take(0)?;
        let ids: Vec<String> = raw_ids.iter().map(|x| x.id.id.to_string()).collect();
        dbg!(&ids);
        Ok(ids)
    }

    async fn get_email_id_for_current_year_month_by_mailbox(
        &self,
        mailbox: &str,
    ) -> surrealdb::Result<Option<String>> {
        let db = connect().await?;
        let year_month = get_current_year_month_str();
        let sql = "SELECT id FROM type::table($table) WHERE year_month = $year_month AND emails.mailbox = $mailbox;";
        let mut result = db
            .query(sql)
            .bind(("table", Tables::Emails.to_string()))
            .bind(("year_month", year_month))
            .bind(("mailbox", mailbox))
            .await?;
        let raw_ids: Vec<Record> = result.take(0)?;
        // NOTE: There should be exactly one id per mailbox per month
        let email_id: Option<String> = raw_ids.iter().map(|x| x.id.id.to_string()).next();
        Ok(email_id)
    }
}
