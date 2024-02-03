use crate::datemath::date::get_current_year_month_str;
use crate::db::connect::connect;
use crate::db::email::EmailMonthly;
use crate::db::enums::Tables;

pub async fn get_emails() -> surrealdb::Result<Vec<EmailMonthly>> {
    let db = connect().await?;
    let emails: Vec<EmailMonthly> = db.select(Tables::Emails.to_string()).await?;
    dbg!(&emails);
    Ok(emails)
}

pub async fn get_emails_current_year_month() -> surrealdb::Result<()> {
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

pub async fn get_emails_current_year_month_mailbox(
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
